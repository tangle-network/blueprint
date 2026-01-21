use crate::command::deploy::definition::{BlueprintDefinition, decode_blueprint_definition};
use alloy_dyn_abi::{DynSolType, DynSolValue, Specifier};
use alloy_json_abi::Param;
use alloy_primitives::{Address, Bytes, Function, I256, U256, hex};
use alloy_sol_types::Word;
use blueprint_client_tangle_evm::TangleEvmClient;
use color_eyre::eyre::{Context, Result, ensure, eyre};
use dialoguer::{Input, console::style};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use std::fs;
use std::path::Path;
use std::str::FromStr;

/// Load the on-chain job schema for the specified blueprint/job pair.
pub async fn load_job_schema(
    client: &TangleEvmClient,
    blueprint_id: u64,
    job_index: u8,
) -> Result<JobSchema> {
    let definition = fetch_blueprint_definition(client, blueprint_id).await?;
    JobSchema::from_definition(&definition, job_index)
}

/// Fetch and decode the blueprint definition stored on-chain.
pub async fn fetch_blueprint_definition(
    client: &TangleEvmClient,
    blueprint_id: u64,
) -> Result<BlueprintDefinition> {
    let raw_definition = client
        .get_raw_blueprint_definition(blueprint_id)
        .await
        .map_err(|e| eyre!(e.to_string()))?;
    let definition = decode_blueprint_definition(&raw_definition)?;
    warn_if_unverified_sources(&definition);
    Ok(definition)
}

/// Parsed ABI schema for a blueprint job.
#[derive(Debug, Clone)]
pub struct JobSchema {
    job_index: u8,
    job_name: String,
    params: Vec<SchemaParam>,
    results: Vec<SchemaParam>,
}

impl JobSchema {
    fn from_definition(definition: &BlueprintDefinition, job: u8) -> Result<Self> {
        let jobs = &definition.jobs;
        let job_definition = jobs.get(job as usize).ok_or_else(|| {
            eyre!(
                "job index {} is not defined ({} total jobs)",
                job,
                jobs.len()
            )
        })?;

        let params = parse_schema_payload(job_definition.paramsSchema.as_ref(), false)?;
        let results = parse_schema_payload(job_definition.resultSchema.as_ref(), true)?;

        Ok(Self {
            job_index: job,
            job_name: job_definition.name.clone(),
            params,
            results,
        })
    }

    /// Associated job index.
    #[must_use]
    pub fn job_index(&self) -> u8 {
        self.job_index
    }

    /// Human-readable job name, when provided in the definition.
    #[must_use]
    pub fn job_name(&self) -> &str {
        &self.job_name
    }

    /// Whether this job definition includes a parameter schema.
    #[must_use]
    pub fn has_params(&self) -> bool {
        !self.params.is_empty()
    }

    /// Whether this job definition includes a result schema.
    #[must_use]
    pub fn has_results(&self) -> bool {
        !self.results.is_empty()
    }

    /// Human-readable parameter descriptions (`name: type`).
    #[must_use]
    pub fn describe_params(&self) -> Vec<String> {
        describe_schema(&self.params)
    }

    /// Human-readable result descriptions (`name: type`).
    #[must_use]
    pub fn describe_results(&self) -> Vec<String> {
        describe_schema(&self.results)
    }

    fn require_params(&self) -> Result<&[SchemaParam]> {
        ensure!(
            !self.params.is_empty(),
            "job {} does not define a parameter schema",
            self.job_index
        );
        Ok(&self.params)
    }

    /// Encode arguments from a JSON file matching the job schema.
    pub fn encode_params_from_file(&self, path: &Path) -> Result<Bytes> {
        let params = self.require_params()?;
        let json = fs::read_to_string(path)
            .with_context(|| format!("failed to read parameter file {}", path.display()))?;
        let value: Value = serde_json::from_str(&json)
            .with_context(|| format!("failed to parse JSON from {}", path.display()))?;

        let inputs = match value {
            Value::Array(items) => {
                ensure!(
                    items.len() == params.len(),
                    "expected {} arguments but file contains {} values",
                    params.len(),
                    items.len()
                );
                items
            }
            Value::Object(map) => {
                let mut ordered = Vec::with_capacity(params.len());
                for (index, schema) in params.iter().enumerate() {
                    let name = schema.name.as_ref().ok_or_else(|| {
                        eyre!(
                            "parameter {} is unnamed; provide an array instead of an object",
                            index
                        )
                    })?;
                    let value = map.get(name).ok_or_else(|| {
                        eyre!(
                            "parameter file {} is missing the `{}` field",
                            path.display(),
                            name
                        )
                    })?;
                    ordered.push(value.clone());
                }
                ordered
            }
            other => {
                return Err(eyre!(
                    "parameters file {} must be a JSON array or object, found {}",
                    path.display(),
                    other
                ));
            }
        };

        self.encode_params_from_values(inputs)
    }

    /// Prompt the operator for each parameter interactively.
    pub fn prompt_for_params(&self) -> Result<Bytes> {
        let params = self.require_params()?;
        if params.is_empty() {
            return Ok(Bytes::new());
        }

        println!(
            "Enter parameter values for job `{}` (index {}). Use Solidity literal syntax for arrays/tuples.",
            self.job_name, self.job_index
        );

        let mut values = Vec::with_capacity(params.len());
        for (index, schema) in params.iter().enumerate() {
            let prompt = format!("{} [{}]", schema.display_name(index), schema.type_label());
            let input: String = Input::new()
                .with_prompt(prompt)
                .allow_empty(true)
                .interact_text()?;
            let literal = if input.is_empty() && matches!(schema.ty, DynSolType::String) {
                "\"\"".to_string()
            } else {
                input
            };
            let value = schema.ty.coerce_str(&literal).map_err(|e| {
                eyre!(
                    "failed to parse value `{literal}` as {}: {e}",
                    schema.type_label()
                )
            })?;
            values.push(value);
        }

        Ok(encode_arguments(values))
    }

    /// Encode parameters from parsed JSON values.
    fn encode_params_from_values(&self, values: Vec<Value>) -> Result<Bytes> {
        let params = self.require_params()?;
        let mut encoded = Vec::with_capacity(params.len());
        for (schema, value) in params.iter().zip(values.iter()) {
            encoded.push(coerce_value(value, schema)?);
        }
        Ok(encode_arguments(encoded))
    }

    /// Attempt to decode and pretty-print job results based on the schema.
    pub fn decode_and_format_results(&self, data: &[u8]) -> Result<Option<Vec<String>>> {
        if self.results.is_empty() {
            return Ok(None);
        }

        let types: Vec<DynSolType> = self.results.iter().map(|param| param.ty.clone()).collect();
        if types.is_empty() {
            return Ok(None);
        }

        let tuple_type = DynSolType::Tuple(types);
        let decoded = if data.is_empty() {
            DynSolValue::Tuple(Vec::new())
        } else {
            tuple_type
                .abi_decode_params(data)
                .map_err(|e| eyre!("failed to decode result payload: {e}"))?
        };

        let values = match decoded {
            DynSolValue::Tuple(items) => items,
            other => vec![other],
        };

        let formatted = values
            .iter()
            .enumerate()
            .map(|(idx, value)| {
                let label = self
                    .results
                    .get(idx)
                    .and_then(|param| param.name.as_deref())
                    .map(|name| name.to_string())
                    .unwrap_or_else(|| format!("result[{idx}]"));
                let ty = self
                    .results
                    .get(idx)
                    .map(|param: &SchemaParam| param.type_label())
                    .unwrap_or_else(|| "unknown".to_string());
                format!("{label} ({ty}) = {}", format_dyn_value(value))
            })
            .collect();

        Ok(Some(formatted))
    }
}

fn warn_if_unverified_sources(definition: &BlueprintDefinition) {
    let missing_indices = definition
        .sources
        .iter()
        .enumerate()
        .filter(|(_, source)| source.binaries.is_empty())
        .map(|(idx, _)| idx + 1)
        .collect::<Vec<_>>();
    if missing_indices.is_empty() {
        return;
    }
    let label = missing_indices
        .iter()
        .map(|idx| format!("#{idx}"))
        .collect::<Vec<_>>()
        .join(", ");
    eprintln!(
        "{} blueprint definition includes source entries without binary hashes ({label}); \
         operator downloads cannot be verified until the blueprint is redeployed with hashes.",
        style("warning").yellow().bold()
    );
}

#[derive(Debug, Clone)]
struct SchemaParam {
    name: Option<String>,
    ty: DynSolType,
    components: Vec<SchemaParam>,
}

impl SchemaParam {
    fn from_param(param: &Param) -> Result<Self> {
        let ty = param
            .resolve()
            .map_err(|e| eyre!("failed to parse ABI type `{}`: {e}", param.ty))?;
        let components = if param.components.is_empty() {
            Vec::new()
        } else {
            param
                .components
                .iter()
                .map(SchemaParam::from_param)
                .collect::<Result<Vec<_>>>()?
        };

        let name = param.name.trim();
        Ok(Self {
            name: if name.is_empty() {
                None
            } else {
                Some(name.to_string())
            },
            ty,
            components,
        })
    }

    fn display_name(&self, index: usize) -> String {
        self.name
            .as_ref()
            .cloned()
            .unwrap_or_else(|| format!("arg_{index}"))
    }

    fn type_label(&self) -> String {
        self.ty.sol_type_name().into_owned()
    }

    fn describe(&self, index: usize) -> String {
        format!("{}: {}", self.display_name(index), self.type_label())
    }

    fn tuple_components(&self) -> Vec<SchemaParam> {
        if !self.components.is_empty() {
            return self.components.clone();
        }

        match &self.ty {
            DynSolType::Tuple(inner) => inner
                .iter()
                .map(|ty| SchemaParam {
                    name: None,
                    ty: ty.clone(),
                    components: Vec::new(),
                })
                .collect(),
            _ => Vec::new(),
        }
    }

    fn element_schema(&self, ty: DynSolType) -> SchemaParam {
        SchemaParam {
            name: None,
            ty,
            components: self.components.clone(),
        }
    }
}

fn describe_schema(params: &[SchemaParam]) -> Vec<String> {
    params
        .iter()
        .enumerate()
        .map(|(index, param)| param.describe(index))
        .collect()
}

/// Summary of a single job definition stored in a blueprint.
#[derive(Debug, Clone, Serialize)]
pub struct JobSummary {
    pub index: u8,
    pub name: Option<String>,
    pub description: Option<String>,
    pub metadata_uri: Option<String>,
    pub parameters: SchemaDescription,
    pub results: SchemaDescription,
}

/// Schema details for job inputs or outputs.
#[derive(Debug, Clone, Serialize)]
pub struct SchemaDescription {
    pub defined: bool,
    pub fields: Vec<String>,
}

impl SchemaDescription {
    fn not_defined() -> Self {
        Self {
            defined: false,
            fields: Vec::new(),
        }
    }

    fn from_fields(defined: bool, fields: Vec<String>, error: Option<String>) -> Self {
        if !defined {
            return Self::not_defined();
        }
        let mut rendered = fields;
        if rendered.is_empty() {
            if let Some(message) = error {
                rendered.push(format!("(failed to decode schema: {message})"));
            }
        }
        Self {
            defined,
            fields: rendered,
        }
    }
}

/// Detailed metadata for a specific job call.
#[derive(Debug, Clone, Serialize)]
pub struct JobCallDetails {
    pub service_id: u64,
    pub call_id: u64,
    pub blueprint_id: u64,
    pub job_index: u8,
    pub job_name: Option<String>,
    pub job_description: Option<String>,
    pub job_metadata_uri: Option<String>,
    pub caller: String,
    pub created_at: u64,
    pub result_count: u32,
    pub payment_wei: String,
    pub completed: bool,
    pub parameters: SchemaDescription,
    pub results: SchemaDescription,
}

/// Fetch and summarize all jobs defined under a blueprint.
pub async fn list_jobs(client: &TangleEvmClient, blueprint_id: u64) -> Result<Vec<JobSummary>> {
    let definition = fetch_blueprint_definition(client, blueprint_id).await?;
    let mut jobs = Vec::with_capacity(definition.jobs.len());
    for (index, job) in definition.jobs.iter().enumerate() {
        let schema = JobSchema::from_definition(&definition, index as u8);
        let (param_fields, result_fields, schema_err) = match schema {
            Ok(schema) => (
                schema.describe_params(),
                schema.describe_results(),
                None::<String>,
            ),
            Err(err) => (Vec::new(), Vec::new(), Some(err.to_string())),
        };
        let params_defined = !job.paramsSchema.is_empty();
        let results_defined = !job.resultSchema.is_empty();
        jobs.push(JobSummary {
            index: index as u8,
            name: optional_field(job.name.as_ref()),
            description: optional_field(job.description.as_ref()),
            metadata_uri: optional_field(job.metadataUri.as_ref()),
            parameters: SchemaDescription::from_fields(
                params_defined,
                param_fields,
                schema_err.clone(),
            ),
            results: SchemaDescription::from_fields(
                results_defined,
                result_fields,
                schema_err.clone(),
            ),
        });
    }
    Ok(jobs)
}

/// Print job definitions as either human-readable text or JSON.
pub fn print_job_summaries(jobs: &[JobSummary], json_output: bool) {
    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(jobs).expect("serialize job summaries to json")
        );
        return;
    }

    if jobs.is_empty() {
        println!("{}", style("No jobs found for this blueprint").yellow());
        return;
    }

    println!("\n{}", style("Jobs").cyan().bold());
    println!(
        "{}",
        style("=============================================").dim()
    );

    for job in jobs {
        println!(
            "{} {}",
            style("Job").green().bold(),
            style(job.index).green()
        );
        if let Some(name) = &job.name {
            println!("  {} {}", style("Name:").green(), name);
        }
        if let Some(description) = &job.description {
            println!("  {} {}", style("Description:").green(), description);
        }
        if let Some(uri) = &job.metadata_uri {
            println!("  {} {}", style("Metadata URI:").green(), uri);
        }
        print_schema_block("Parameters", &job.parameters);
        print_schema_block("Results", &job.results);
        println!(
            "{}",
            style("=============================================").dim()
        );
    }
}

/// Load metadata for a job call, including job definition context.
pub async fn load_job_call_details(
    client: &TangleEvmClient,
    blueprint_id: u64,
    service_id: u64,
    call_id: u64,
) -> Result<JobCallDetails> {
    let call = client
        .get_job_call(service_id, call_id)
        .await
        .map_err(|e| eyre!(e.to_string()))?;
    let definition = fetch_blueprint_definition(client, blueprint_id).await?;
    let job_index = call.jobIndex;
    let job_entry = definition.jobs.get(job_index as usize);

    let schema = JobSchema::from_definition(&definition, job_index);
    let (param_fields, result_fields, schema_err) = match schema {
        Ok(schema) => (schema.describe_params(), schema.describe_results(), None),
        Err(err) => (Vec::new(), Vec::new(), Some(err.to_string())),
    };

    let (job_name, job_description, job_metadata_uri, params_defined, results_defined) =
        if let Some(job) = job_entry {
            (
                optional_field(job.name.as_ref()),
                optional_field(job.description.as_ref()),
                optional_field(job.metadataUri.as_ref()),
                !job.paramsSchema.is_empty(),
                !job.resultSchema.is_empty(),
            )
        } else {
            (None, None, None, false, false)
        };

    Ok(JobCallDetails {
        service_id,
        call_id,
        blueprint_id,
        job_index,
        job_name,
        job_description,
        job_metadata_uri,
        caller: format!("{:#x}", call.caller),
        created_at: call.createdAt,
        result_count: call.resultCount,
        payment_wei: call.payment.to_string(),
        completed: call.completed,
        parameters: SchemaDescription::from_fields(
            params_defined,
            param_fields,
            schema_err.clone(),
        ),
        results: SchemaDescription::from_fields(results_defined, result_fields, schema_err),
    })
}

/// Print job call metadata in a human-readable format (or JSON).
pub fn print_job_call_details(details: &JobCallDetails, json_output: bool) {
    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(details).expect("serialize job call details to json")
        );
        return;
    }

    println!(
        "\n{}",
        style(format!("Job Call {}", details.call_id)).cyan().bold()
    );
    println!("{} {}", style("Service ID:").green(), details.service_id);
    println!(
        "{} {}",
        style("Blueprint ID:").green(),
        details.blueprint_id
    );
    println!("{} {}", style("Job Index:").green(), details.job_index);
    if let Some(name) = &details.job_name {
        println!("{} {}", style("Job Name:").green(), name);
    }
    if let Some(description) = &details.job_description {
        println!("{} {}", style("Description:").green(), description);
    }
    if let Some(uri) = &details.job_metadata_uri {
        println!("{} {}", style("Metadata URI:").green(), uri);
    }
    println!("{} {}", style("Caller:").green(), details.caller);
    println!("{} {}", style("Created At:").green(), details.created_at);
    println!(
        "{} {}",
        style("Result Count:").green(),
        details.result_count
    );
    println!(
        "{} {}",
        style("Payment (wei):").green(),
        details.payment_wei
    );
    println!("{} {}", style("Completed:").green(), details.completed);
    print_schema_block("Parameters", &details.parameters);
    print_schema_block("Results", &details.results);
    println!(
        "{}",
        style("=============================================").dim()
    );
}

fn print_schema_block(label: &str, schema: &SchemaDescription) {
    if !schema.defined {
        println!(
            "  {} {}",
            style(format!("{label}:")).green(),
            "(not provided)"
        );
        return;
    }

    if schema.fields.is_empty() {
        println!("  {} {}", style(format!("{label}:")).green(), "(none)");
        return;
    }

    println!("  {} ", style(format!("{label}:")).green());
    for entry in &schema.fields {
        println!("    - {}", entry);
    }
}

fn optional_field(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn encode_arguments(arguments: Vec<DynSolValue>) -> Bytes {
    if arguments.is_empty() {
        Bytes::new()
    } else {
        // Use compact binary encoding (not ABI encoding) to match tnt-core SchemaLib format
        let mut buffer = Vec::new();
        for arg in arguments {
            encode_compact_value(&arg, &mut buffer);
        }
        Bytes::from(buffer)
    }
}

/// Encode a value in compact binary format matching tnt-core SchemaLib.
fn encode_compact_value(value: &DynSolValue, buffer: &mut Vec<u8>) {
    match value {
        DynSolValue::Bool(b) => buffer.push(if *b { 1 } else { 0 }),
        DynSolValue::Uint(n, bits) => {
            let bytes = n.to_be_bytes::<32>();
            let byte_len = (*bits + 7) / 8;
            buffer.extend_from_slice(&bytes[32 - byte_len..]);
        }
        DynSolValue::Int(n, bits) => {
            let bytes = n.to_be_bytes::<32>();
            let byte_len = (*bits + 7) / 8;
            buffer.extend_from_slice(&bytes[32 - byte_len..]);
        }
        DynSolValue::Address(addr) => buffer.extend_from_slice(&addr[..]),
        DynSolValue::FixedBytes(word, len) => buffer.extend_from_slice(&word[..*len]),
        DynSolValue::String(s) => {
            encode_compact_length(s.len(), buffer);
            buffer.extend_from_slice(s.as_bytes());
        }
        DynSolValue::Bytes(b) => {
            encode_compact_length(b.len(), buffer);
            buffer.extend_from_slice(b);
        }
        DynSolValue::Array(items) | DynSolValue::FixedArray(items) => {
            // For dynamic arrays, include length; for fixed arrays, just encode elements
            if matches!(value, DynSolValue::Array(_)) {
                encode_compact_length(items.len(), buffer);
            }
            for item in items {
                encode_compact_value(item, buffer);
            }
        }
        DynSolValue::Tuple(fields) => {
            // Structs: encode field count + fields
            encode_compact_length(fields.len(), buffer);
            for field in fields {
                encode_compact_value(field, buffer);
            }
        }
        DynSolValue::Function(f) => buffer.extend_from_slice(f.0.as_slice()),
    }
}

/// Encode a length using compact encoding matching tnt-core SchemaLib._readCompactLength.
/// - 0x00-0x7F: single byte
/// - 0x80-0xBF + 1 byte: 2-byte encoding (14 bits = max 16383)
/// - 0xC0-0xDF + 2 bytes: 3-byte encoding (21 bits = max 2097151)
/// - 0xE0-0xEF + 3 bytes: 4-byte encoding (28 bits = max 268435455)
fn encode_compact_length(len: usize, buffer: &mut Vec<u8>) {
    if len < 0x80 {
        buffer.push(len as u8);
    } else if len < 0x4000 {
        buffer.push(0x80 | ((len >> 8) as u8));
        buffer.push((len & 0xFF) as u8);
    } else if len < 0x200000 {
        buffer.push(0xC0 | ((len >> 16) as u8));
        buffer.push(((len >> 8) & 0xFF) as u8);
        buffer.push((len & 0xFF) as u8);
    } else if len < 0x10000000 {
        buffer.push(0xE0 | ((len >> 24) as u8));
        buffer.push(((len >> 16) & 0xFF) as u8);
        buffer.push(((len >> 8) & 0xFF) as u8);
        buffer.push((len & 0xFF) as u8);
    } else {
        // For very large lengths, use 5-byte format
        buffer.push(0xF0);
        buffer.push(((len >> 24) & 0xFF) as u8);
        buffer.push(((len >> 16) & 0xFF) as u8);
        buffer.push(((len >> 8) & 0xFF) as u8);
        buffer.push((len & 0xFF) as u8);
    }
}

fn parse_schema_payload(bytes: &[u8], prefer_outputs: bool) -> Result<Vec<SchemaParam>> {
    if bytes.is_empty() {
        return Ok(Vec::new());
    }

    if let Ok(params) = serde_json::from_slice::<Vec<Param>>(bytes) {
        return params.iter().map(SchemaParam::from_param).collect();
    }

    if let Ok(doc) = serde_json::from_slice::<SchemaDocument>(bytes) {
        let params = doc.into_params(prefer_outputs);
        return params.iter().map(SchemaParam::from_param).collect();
    }

    // Try to decode as TLV binary format (used by tnt-core contract)
    if let Ok(params) = decode_tlv_schema(bytes) {
        return Ok(params);
    }

    let text =
        std::str::from_utf8(bytes).map_err(|_| eyre!("schema payload is not valid UTF-8"))?;
    parse_schema_text(text, prefer_outputs)
}

/// Decode a TLV binary schema format used by the tnt-core contract.
///
/// Schema version constant
const SCHEMA_VERSION_2: u8 = 0x02;

/// TLV format (version 2 with field names):
/// - 1 byte: version (0x02)
/// - 2 bytes: uint16 field count (big-endian)
/// - For each field (5 bytes header + name + children recursively):
///   - 1 byte: BlueprintFieldKind enum (0-22)
///   - 2 bytes: uint16 arrayLength (big-endian)
///   - 2 bytes: uint16 childCount (big-endian)
///   - 1-4 bytes: compact-encoded name length
///   - N bytes: field name UTF-8 string
fn decode_tlv_schema(bytes: &[u8]) -> Result<Vec<SchemaParam>> {
    if bytes.len() < 3 {
        return Err(eyre!("TLV schema too short"));
    }

    // Verify version byte
    if bytes[0] != SCHEMA_VERSION_2 {
        return Err(eyre!(
            "Invalid schema version: expected 0x{:02x}, got 0x{:02x}",
            SCHEMA_VERSION_2,
            bytes[0]
        ));
    }

    let field_count = u16::from_be_bytes([bytes[1], bytes[2]]) as usize;
    let mut cursor = 3; // 1 byte version + 2 bytes field count
    let mut params = Vec::with_capacity(field_count);

    for i in 0..field_count {
        let (param, new_cursor) = decode_tlv_field(bytes, cursor, i)?;
        params.push(param);
        cursor = new_cursor;
    }

    Ok(params)
}

fn decode_tlv_field(
    bytes: &[u8],
    cursor: usize,
    field_idx: usize,
) -> Result<(SchemaParam, usize)> {
    if cursor + 5 > bytes.len() {
        return Err(eyre!("TLV field {} truncated", field_idx));
    }

    let kind = bytes[cursor];
    let array_length = u16::from_be_bytes([bytes[cursor + 1], bytes[cursor + 2]]);
    let child_count = u16::from_be_bytes([bytes[cursor + 3], bytes[cursor + 4]]) as usize;

    let mut next_cursor = cursor + 5;

    // Read field name (always present in v2 format)
    let (name_len, after_len) = read_compact_length(bytes, next_cursor)?;
    next_cursor = after_len;

    if next_cursor + name_len > bytes.len() {
        return Err(eyre!("TLV field {} name truncated", field_idx));
    }

    let name_bytes = &bytes[next_cursor..next_cursor + name_len];
    let name_str = String::from_utf8_lossy(name_bytes).to_string();
    next_cursor += name_len;

    // Return Some only if name is non-empty
    let name = if name_str.is_empty() {
        None
    } else {
        Some(name_str)
    };

    let mut components = Vec::with_capacity(child_count);

    for i in 0..child_count {
        let (child, new_cursor) = decode_tlv_field(bytes, next_cursor, i)?;
        components.push(child);
        next_cursor = new_cursor;
    }

    let ty = tlv_kind_to_dyn_sol_type(kind, array_length, &components)?;

    Ok((
        SchemaParam {
            name,
            ty,
            components,
        },
        next_cursor,
    ))
}

/// Read a compact-encoded length from bytes.
fn read_compact_length(bytes: &[u8], cursor: usize) -> Result<(usize, usize)> {
    if cursor >= bytes.len() {
        return Err(eyre!("compact length truncated"));
    }

    let first = bytes[cursor];

    if first & 0x80 == 0 {
        // Single byte: 0x00-0x7F
        Ok((first as usize, cursor + 1))
    } else if first & 0xC0 == 0x80 {
        // Two bytes: 0x80-0xBF
        if cursor + 2 > bytes.len() {
            return Err(eyre!("compact length truncated"));
        }
        let value = (((first & 0x3F) as usize) << 8) | (bytes[cursor + 1] as usize);
        Ok((value, cursor + 2))
    } else if first & 0xE0 == 0xC0 {
        // Three bytes: 0xC0-0xDF
        if cursor + 3 > bytes.len() {
            return Err(eyre!("compact length truncated"));
        }
        let value = (((first & 0x1F) as usize) << 16)
            | ((bytes[cursor + 1] as usize) << 8)
            | (bytes[cursor + 2] as usize);
        Ok((value, cursor + 3))
    } else {
        // Four bytes: 0xE0-0xEF
        if cursor + 4 > bytes.len() {
            return Err(eyre!("compact length truncated"));
        }
        let value = (((first & 0x0F) as usize) << 24)
            | ((bytes[cursor + 1] as usize) << 16)
            | ((bytes[cursor + 2] as usize) << 8)
            | (bytes[cursor + 3] as usize);
        Ok((value, cursor + 4))
    }
}

/// Convert BlueprintFieldKind enum to DynSolType.
///
/// BlueprintFieldKind values:
/// - Void=0, Bool=1, Uint8=2, Int8=3, Uint16=4, Int16=5, Uint32=6, Int32=7
/// - Uint64=8, Int64=9, Uint128=10, Int128=11, Uint256=12, Int256=13
/// - Address=14, Bytes32=15, FixedBytes=16, String=17, Bytes=18
/// - Optional=19, Array=20, List=21, Struct=22
fn tlv_kind_to_dyn_sol_type(
    kind: u8,
    array_length: u16,
    components: &[SchemaParam],
) -> Result<DynSolType> {
    match kind {
        0 => Ok(DynSolType::Tuple(vec![])), // Void - empty tuple
        1 => Ok(DynSolType::Bool),
        2 => Ok(DynSolType::Uint(8)),
        3 => Ok(DynSolType::Int(8)),
        4 => Ok(DynSolType::Uint(16)),
        5 => Ok(DynSolType::Int(16)),
        6 => Ok(DynSolType::Uint(32)),
        7 => Ok(DynSolType::Int(32)),
        8 => Ok(DynSolType::Uint(64)),
        9 => Ok(DynSolType::Int(64)),
        10 => Ok(DynSolType::Uint(128)),
        11 => Ok(DynSolType::Int(128)),
        12 => Ok(DynSolType::Uint(256)),
        13 => Ok(DynSolType::Int(256)),
        14 => Ok(DynSolType::Address),
        15 => Ok(DynSolType::FixedBytes(32)), // Bytes32
        16 => Ok(DynSolType::FixedBytes(array_length as usize)), // FixedBytes with size in arrayLength
        17 => Ok(DynSolType::String),
        18 => Ok(DynSolType::Bytes),
        19 => {
            // Optional - treat as the inner type (first component) wrapped in a way that allows null
            // For simplicity, we'll treat it as the inner type directly
            if components.is_empty() {
                Ok(DynSolType::Bytes) // Fallback
            } else {
                Ok(components[0].ty.clone())
            }
        }
        20 => {
            // Array (fixed size) - arrayLength contains the size
            if components.is_empty() {
                // If no components, assume bytes
                Ok(DynSolType::FixedArray(
                    Box::new(DynSolType::Bytes),
                    array_length as usize,
                ))
            } else {
                Ok(DynSolType::FixedArray(
                    Box::new(components[0].ty.clone()),
                    array_length as usize,
                ))
            }
        }
        21 => {
            // List (dynamic array)
            if components.is_empty() {
                Ok(DynSolType::Array(Box::new(DynSolType::Bytes)))
            } else {
                Ok(DynSolType::Array(Box::new(components[0].ty.clone())))
            }
        }
        22 => {
            // Struct (tuple)
            let inner_types: Vec<DynSolType> = components.iter().map(|c| c.ty.clone()).collect();
            Ok(DynSolType::Tuple(inner_types))
        }
        _ => Err(eyre!("unknown BlueprintFieldKind: {}", kind)),
    }
}

fn parse_schema_text(text: &str, prefer_outputs: bool) -> Result<Vec<SchemaParam>> {
    let trimmed = text.trim().trim_matches(|c| c == '"' || c == '\'');
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    if let Some(hex_payload) = trimmed.strip_prefix("0x") {
        let decoded =
            hex::decode(hex_payload).map_err(|e| eyre!("invalid hex schema payload: {e}"))?;
        return parse_schema_payload(&decoded, prefer_outputs);
    }

    if let Ok(params) = serde_json::from_str::<Vec<Param>>(trimmed) {
        return params.iter().map(SchemaParam::from_param).collect();
    }

    if let Ok(doc) = serde_json::from_str::<SchemaDocument>(trimmed) {
        let params = doc.into_params(prefer_outputs);
        return params.iter().map(SchemaParam::from_param).collect();
    }

    // Fallback to comma-separated Solidity types.
    let mut resolved = Vec::new();
    for raw in trimmed.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let ty = raw
            .resolve()
            .map_err(|e| eyre!("failed to parse `{raw}` as a Solidity type: {e}"))?;
        resolved.push(SchemaParam {
            name: None,
            ty,
            components: Vec::new(),
        });
    }

    if resolved.is_empty() {
        Err(eyre!("unable to parse schema payload"))
    } else {
        Ok(resolved)
    }
}

#[derive(Debug, Deserialize)]
struct SchemaDocument {
    #[serde(default)]
    inputs: Vec<Param>,
    #[serde(default)]
    outputs: Vec<Param>,
    #[serde(default)]
    params: Vec<Param>,
}

impl SchemaDocument {
    fn into_params(self, prefer_outputs: bool) -> Vec<Param> {
        if prefer_outputs {
            if !self.outputs.is_empty() {
                return self.outputs;
            }
            if !self.inputs.is_empty() {
                return self.inputs;
            }
        } else if !self.inputs.is_empty() {
            return self.inputs;
        } else if !self.outputs.is_empty() {
            return self.outputs;
        }

        if !self.params.is_empty() {
            self.params
        } else {
            Vec::new()
        }
    }
}

fn coerce_value(value: &Value, schema: &SchemaParam) -> Result<DynSolValue> {
    match &schema.ty {
        DynSolType::Bool => Ok(DynSolValue::Bool(parse_bool(value)?)),
        DynSolType::Uint(size) => {
            let number = parse_uint(value)?;
            Ok(DynSolValue::Uint(number, *size))
        }
        DynSolType::Int(size) => {
            let number = parse_int(value)?;
            Ok(DynSolValue::Int(number, *size))
        }
        DynSolType::Address => Ok(DynSolValue::Address(parse_address(value)?)),
        DynSolType::String => Ok(DynSolValue::String(parse_string(value)?)),
        DynSolType::Bytes => Ok(DynSolValue::Bytes(parse_bytes(value)?)),
        DynSolType::FixedBytes(len) => {
            let len = *len;
            let data = parse_bytes(value)?;
            ensure!(
                data.len() == len,
                "expected {len} bytes but received {}",
                data.len()
            );
            let mut word = Word::default();
            word[..len].copy_from_slice(&data);
            Ok(DynSolValue::FixedBytes(word, len))
        }
        DynSolType::Array(inner) => {
            let arr = value
                .as_array()
                .ok_or_else(|| eyre!("expected an array for parameter {}", schema.type_label()))?;
            let mut values = Vec::with_capacity(arr.len());
            let element_schema = schema.element_schema((**inner).clone());
            for item in arr {
                values.push(coerce_value(item, &element_schema)?);
            }
            Ok(DynSolValue::Array(values))
        }
        DynSolType::FixedArray(inner, len) => {
            let arr = value
                .as_array()
                .ok_or_else(|| eyre!("expected an array for parameter {}", schema.type_label()))?;
            ensure!(
                arr.len() == *len,
                "expected {len} elements but received {}",
                arr.len()
            );
            let mut values = Vec::with_capacity(*len);
            let element_schema = schema.element_schema((**inner).clone());
            for item in arr {
                values.push(coerce_value(item, &element_schema)?);
            }
            Ok(DynSolValue::FixedArray(values))
        }
        DynSolType::Tuple(_) => Ok(DynSolValue::Tuple(parse_tuple(value, schema)?)),
        DynSolType::Function => {
            let data = parse_bytes(value)?;
            ensure!(
                data.len() == 24,
                "function values must be 24 bytes (address + selector)"
            );
            let mut raw = [0u8; 24];
            raw.copy_from_slice(&data);
            Ok(DynSolValue::Function(Function::from(raw)))
        }
        #[allow(unreachable_patterns)]
        other => Err(eyre!("unsupported type {other:?} for job parameters")),
    }
}

fn parse_tuple(value: &Value, schema: &SchemaParam) -> Result<Vec<DynSolValue>> {
    let component_types = schema.tuple_components();
    let mut values = Vec::with_capacity(component_types.len());

    match value {
        Value::Array(items) => {
            ensure!(
                items.len() == component_types.len(),
                "expected {} tuple fields but received {}",
                component_types.len(),
                items.len()
            );
            for (item, component_schema) in items.iter().zip(component_types.iter()) {
                values.push(coerce_value(item, component_schema)?);
            }
        }
        Value::Object(map) => {
            for component_schema in &component_types {
                let name = component_schema.name.as_ref().ok_or_else(|| {
                    eyre!("tuple component is unnamed; provide an array instead of an object")
                })?;
                let entry = map
                    .get(name)
                    .ok_or_else(|| eyre!("missing tuple field `{name}`"))?;
                values.push(coerce_value(entry, component_schema)?);
            }
        }
        _ => {
            return Err(eyre!(
                "tuples must be provided as JSON arrays or objects in parameter files"
            ));
        }
    }

    Ok(values)
}

fn parse_bool(value: &Value) -> Result<bool> {
    match value {
        Value::Bool(b) => Ok(*b),
        Value::Number(num) => {
            if let Some(int) = num.as_u64() {
                match int {
                    0 => Ok(false),
                    1 => Ok(true),
                    _ => Err(eyre!("boolean numbers must be 0 or 1")),
                }
            } else {
                Err(eyre!("boolean numbers must be integers"))
            }
        }
        Value::String(s) => match s.trim().to_lowercase().as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            "1" => Ok(true),
            "0" => Ok(false),
            other => Err(eyre!("unable to parse `{other}` as a boolean")),
        },
        _ => Err(eyre!("boolean values must be true/false or 0/1")),
    }
}

fn parse_uint(value: &Value) -> Result<U256> {
    match value {
        Value::Number(num) => num.as_u64().map(U256::from).ok_or_else(|| {
            eyre!("unsigned integers must fit within u64 or be provided as strings")
        }),
        Value::String(s) => parse_uint_from_str(s),
        _ => Err(eyre!("unsigned integers must be numbers or strings")),
    }
}

fn parse_uint_from_str(value: &str) -> Result<U256> {
    let trimmed = value.trim();
    if let Some(hex) = trimmed.strip_prefix("0x") {
        U256::from_str_radix(hex, 16).map_err(|e| eyre!("invalid hex integer `{trimmed}`: {e}"))
    } else {
        U256::from_str_radix(trimmed, 10).map_err(|e| eyre!("invalid integer `{trimmed}`: {e}"))
    }
}

fn parse_int(value: &Value) -> Result<I256> {
    match value {
        Value::Number(num) => {
            if let Some(int) = num.as_i64() {
                I256::from_dec_str(&int.to_string())
                    .map_err(|e| eyre!("invalid integer `{int}`: {e}"))
            } else {
                Err(eyre!(
                    "signed integers must fit within i64 or be provided as strings"
                ))
            }
        }
        Value::String(s) => parse_int_from_str(s),
        _ => Err(eyre!("signed integers must be numbers or strings")),
    }
}

fn parse_int_from_str(value: &str) -> Result<I256> {
    let trimmed = value.trim();
    if let Some(hex) = trimmed.strip_prefix("0x") {
        let unsigned = U256::from_str_radix(hex, 16)
            .map_err(|e| eyre!("invalid hex integer `{trimmed}`: {e}"))?;
        Ok(I256::from_raw(unsigned))
    } else {
        I256::from_dec_str(trimmed).map_err(|e| eyre!("invalid integer `{trimmed}`: {e}"))
    }
}

fn parse_address(value: &Value) -> Result<Address> {
    let s = value
        .as_str()
        .ok_or_else(|| eyre!("addresses must be provided as strings"))?;
    Address::from_str(s).map_err(|_| eyre!("invalid address `{s}`"))
}

fn parse_string(value: &Value) -> Result<String> {
    match value {
        Value::String(s) => Ok(s.clone()),
        Value::Number(n) => Ok(n.to_string()),
        Value::Bool(b) => Ok(b.to_string()),
        _ => Err(eyre!("strings must be provided as JSON strings")),
    }
}

fn parse_bytes(value: &Value) -> Result<Vec<u8>> {
    match value {
        Value::String(s) => parse_bytes_from_str(s),
        Value::Array(items) => {
            let mut bytes = Vec::with_capacity(items.len());
            for item in items {
                let number = item
                    .as_u64()
                    .ok_or_else(|| eyre!("byte arrays must contain numbers between 0 and 255"))?;
                ensure!(number <= 0xFF, "byte values must be between 0 and 255");
                bytes.push(number as u8);
            }
            Ok(bytes)
        }
        _ => Err(eyre!("byte values must be strings or arrays of numbers")),
    }
}

fn parse_bytes_from_str(value: &str) -> Result<Vec<u8>> {
    let trimmed = value.trim_matches('"');
    if let Some(hex) = trimmed.strip_prefix("0x") {
        hex::decode(hex).map_err(|e| eyre!("invalid hex string `{trimmed}`: {e}"))
    } else {
        Ok(trimmed.as_bytes().to_vec())
    }
}

#[allow(unreachable_patterns)]
fn format_dyn_value(value: &DynSolValue) -> String {
    match value {
        DynSolValue::Bool(b) => b.to_string(),
        DynSolValue::Uint(v, _) => format!("{v}"),
        DynSolValue::Int(v, _) => format!("{v}"),
        DynSolValue::Address(addr) => format!("{addr:#x}"),
        DynSolValue::String(s) => format!("\"{s}\""),
        DynSolValue::Bytes(bytes) => format!("0x{}", hex::encode(bytes)),
        DynSolValue::FixedBytes(word, size) => format!("0x{}", hex::encode(&word[..*size])),
        DynSolValue::Array(values) | DynSolValue::FixedArray(values) => {
            let inner = values
                .iter()
                .map(format_dyn_value)
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{inner}]")
        }
        DynSolValue::Tuple(values) => {
            let inner = values
                .iter()
                .map(format_dyn_value)
                .collect::<Vec<_>>()
                .join(", ");
            format!("({inner})")
        }
        DynSolValue::Function(func) => format!("0x{}", hex::encode(func.as_slice())),
        _ => format!("{value:?}"),
    }
}

impl fmt::Display for JobSchema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "JobSchema(index={}, name={})",
            self.job_index, self.job_name
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn encodes_inputs_from_json_objects() -> Result<()> {
        let schema_bytes = br#"
        [
            {"name":"value","type":"uint64"},
            {"name":"config","type":"tuple","components":[
                {"name":"account","type":"address"},
                {"name":"values","type":"uint256[]"}
            ]}
        ]"#;
        let params = parse_schema_payload(schema_bytes, false)?;
        let results = parse_schema_payload(br#"[]"#, true)?;
        let schema = JobSchema {
            job_index: 0,
            job_name: "test".to_string(),
            params,
            results,
        };

        let mut tmp = NamedTempFile::new()?;
        let payload = json!({
            "value": 5,
            "config": {
                "account": "0x0000000000000000000000000000000000000001",
                "values": ["0x2a", 3]
            }
        });
        write!(tmp, "{payload}")?;

        let encoded = schema.encode_params_from_file(tmp.path())?;
        let types: Vec<DynSolType> = schema.params.iter().map(|param| param.ty.clone()).collect();
        let decoded = DynSolType::Tuple(types)
            .abi_decode_params(encoded.as_ref())
            .expect("decode params");
        let DynSolValue::Tuple(values) = decoded else {
            panic!("expected tuple");
        };
        assert_eq!(values.len(), 2);
        assert_eq!(format_dyn_value(&values[0]), "5");
        assert_eq!(
            format_dyn_value(&values[1]),
            "(0x0000000000000000000000000000000000000001, [42, 3])"
        );
        Ok(())
    }

    #[test]
    fn decodes_and_formats_results() -> Result<()> {
        let schema_bytes = br#"[{"name":"result","type":"uint256"}]"#;
        let params = parse_schema_payload(br#"[]"#, false)?;
        let results = parse_schema_payload(schema_bytes, true)?;
        let schema = JobSchema {
            job_index: 1,
            job_name: "result-test".to_string(),
            params,
            results,
        };

        let value = DynSolValue::Tuple(vec![DynSolValue::Uint(U256::from(123u64), 256)]);
        let bytes = Bytes::from(value.abi_encode_params());
        let formatted = schema
            .decode_and_format_results(bytes.as_ref())
            .expect("decode schema")
            .expect("formatted results");
        assert_eq!(formatted, vec!["result (uint256) = 123"]);
        Ok(())
    }
}
