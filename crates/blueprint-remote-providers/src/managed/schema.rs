//! Metadata is projected from the same credential variants the factory constructs.
use super::{ProviderCredentials, ProviderErrorCode};
use crate::{CloudProvider, Error, Result};
use schemars::{JsonSchema, generate::SchemaSettings};
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct CredentialField {
    pub field: String,
    pub env: Option<String>,
    pub required: bool,
}

#[derive(Serialize, JsonSchema)]
pub struct ProviderDescriptor {
    pub id: CloudProvider,
    pub credentials: Vec<CredentialField>,
}

/// Offline descriptors; the exhaustive factory checks every variant's managed trait.
pub fn provider_catalog() -> Result<Vec<ProviderDescriptor>> {
    let invalid = || Error::Provider(ProviderErrorCode::ProviderError);
    let schema = SchemaSettings::draft2020_12()
        .for_deserialize()
        .with(|settings| settings.inline_subschemas = true)
        .into_generator()
        .into_root_schema_for::<ProviderCredentials>();
    let schema = serde_json::to_value(schema).map_err(|_| invalid())?;
    let mut providers = Vec::new();
    for variant in schema["oneOf"].as_array().ok_or_else(invalid)? {
        let fields = variant["properties"].as_object().ok_or_else(invalid)?;
        let provider = fields
            .get("provider")
            .and_then(|field| field.get("const"))
            .cloned()
            .ok_or_else(invalid)?;
        let id = serde_json::from_value(provider).map_err(|_| invalid())?;
        let required = variant["required"].as_array().ok_or_else(invalid)?;
        let credentials = fields
            .iter()
            .filter(|(name, _)| *name != "provider")
            .map(|(name, field)| CredentialField {
                field: name.clone(),
                env: field["x-env"].as_str().map(str::to_owned),
                required: required
                    .iter()
                    .any(|value| value.as_str() == Some(name.as_str())),
            })
            .collect();
        providers.push(ProviderDescriptor { id, credentials });
    }
    providers.sort_by_key(|provider| provider.id.catalog_key());
    Ok(providers)
}
