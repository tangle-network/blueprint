use alloy_json_abi::Param;
use alloy_primitives::Bytes;
use color_eyre::eyre::{Result, eyre};

pub(super) fn hex_to_bytes(value: Option<&str>) -> Result<Bytes> {
    if let Some(raw) = value {
        if raw.trim().is_empty() {
            return Ok(Bytes::new());
        }
        let trimmed = raw.trim();

        // Check if this looks like JSON schema (starts with '[')
        if trimmed.starts_with('[') {
            return encode_json_schema_to_tlv(trimmed);
        }

        // Check if hex-encoded data might be JSON (0x5b = '[')
        let hex_trimmed = trimmed.strip_prefix("0x").unwrap_or(trimmed);
        if hex_trimmed.starts_with("5b") || hex_trimmed.starts_with("5B") {
            // Decode hex first, then check if it's JSON
            if let Ok(decoded) = hex::decode(hex_trimmed) {
                if let Ok(json_str) = std::str::from_utf8(&decoded) {
                    if json_str.trim().starts_with('[') {
                        return encode_json_schema_to_tlv(json_str);
                    }
                }
            }
        }

        // Otherwise treat as raw hex bytes (pre-encoded TLV or other binary format)
        let decoded = hex::decode(hex_trimmed)
            .map_err(|e| eyre!("failed to decode hex schema {raw}: {e}"))?;
        Ok(Bytes::from(decoded))
    } else {
        Ok(Bytes::new())
    }
}

/// Schema version constants
const SCHEMA_VERSION_2: u8 = 0x02;

/// Encode a JSON ABI schema to the TLV binary format expected by the contract.
///
/// The TLV format (version 2 with field names) is:
/// - 1 byte: version (0x02)
/// - 2 bytes: uint16 field count (big-endian)
/// - For each field (5 bytes header + name + children recursively):
///   - 1 byte: BlueprintFieldKind enum (0-22)
///   - 2 bytes: uint16 arrayLength (big-endian)
///   - 2 bytes: uint16 childCount (big-endian)
///   - 1-4 bytes: compact-encoded name length
///   - N bytes: field name UTF-8 string
fn encode_json_schema_to_tlv(json_str: &str) -> Result<Bytes> {
    let params: Vec<Param> =
        serde_json::from_str(json_str).map_err(|e| eyre!("failed to parse JSON schema: {e}"))?;

    if params.is_empty() {
        return Ok(Bytes::new());
    }

    let field_count = params.len();
    if field_count > u16::MAX as usize {
        return Err(eyre!("schema has too many fields (max {})", u16::MAX));
    }

    // Calculate total size needed: 1 byte version + 2 bytes field count + fields with names
    let total_size: usize = 1 + 2 + params.iter().map(calculate_field_size).sum::<usize>();

    let mut buffer = vec![0u8; total_size];

    // Write version byte
    buffer[0] = SCHEMA_VERSION_2;

    // Write field count (big-endian)
    buffer[1] = (field_count >> 8) as u8;
    buffer[2] = (field_count & 0xFF) as u8;

    // Write each field
    let mut cursor = 3;
    for param in &params {
        cursor = write_field(&mut buffer, cursor, param)?;
    }

    Ok(Bytes::from(buffer))
}

/// Calculate the total encoded size of a field including header, name, and children.
fn calculate_field_size(param: &Param) -> usize {
    // Header (5 bytes) + name length encoding + name bytes
    let name_len = param.name.len();
    let mut size = 5 + compact_length_size(name_len) + name_len;

    // For array types (type[] or type[n]), we need to count the element type as a child
    if param.ty.ends_with(']') && param.components.is_empty() {
        if let Some(bracket_pos) = param.ty.rfind('[') {
            let base_type = &param.ty[..bracket_pos];
            if !base_type.is_empty() {
                // Create synthetic child for element type (with empty name)
                let element_param = Param {
                    name: String::new(),
                    ty: base_type.to_string(),
                    components: vec![],
                    internal_type: None,
                };
                size += calculate_field_size(&element_param);
                return size;
            }
        }
    }

    // Add children recursively
    for child in &param.components {
        size += calculate_field_size(child);
    }

    size
}

/// Calculate the number of bytes needed for compact length encoding.
fn compact_length_size(len: usize) -> usize {
    if len < 0x80 {
        1
    } else if len < 0x4000 {
        2
    } else if len < 0x200000 {
        3
    } else {
        4
    }
}

/// Write a single field to the buffer and return the new cursor position.
fn write_field(buffer: &mut [u8], cursor: usize, param: &Param) -> Result<usize> {
    let (kind, array_length) = parse_solidity_type(&param.ty)?;

    // For array types (kind 20 or 21) without explicit components, create synthetic child
    let (child_count, synthetic_child) =
        if (kind == 20 || kind == 21) && param.components.is_empty() && param.ty.ends_with(']') {
            if let Some(bracket_pos) = param.ty.rfind('[') {
                let base_type = &param.ty[..bracket_pos];
                if !base_type.is_empty() {
                    let element_param = Param {
                        name: String::new(),
                        ty: base_type.to_string(),
                        components: vec![],
                        internal_type: None,
                    };
                    (1, Some(element_param))
                } else {
                    (param.components.len(), None)
                }
            } else {
                (param.components.len(), None)
            }
        } else {
            (param.components.len(), None)
        };

    if child_count > u16::MAX as usize {
        return Err(eyre!("field has too many children (max {})", u16::MAX));
    }

    // Write 5-byte header
    buffer[cursor] = kind;
    buffer[cursor + 1] = (array_length >> 8) as u8;
    buffer[cursor + 2] = (array_length & 0xFF) as u8;
    buffer[cursor + 3] = (child_count >> 8) as u8;
    buffer[cursor + 4] = (child_count & 0xFF) as u8;

    let mut next_cursor = cursor + 5;

    // Write field name (compact length + UTF-8 bytes)
    next_cursor = write_compact_string(buffer, next_cursor, &param.name);

    // If we have a synthetic child for array element type, write it
    if let Some(ref element) = synthetic_child {
        next_cursor = write_field(buffer, next_cursor, element)?;
    }

    // Write explicit children (for tuples, etc.)
    for child in &param.components {
        next_cursor = write_field(buffer, next_cursor, child)?;
    }

    Ok(next_cursor)
}

/// Write a compact-encoded string to the buffer.
fn write_compact_string(buffer: &mut [u8], cursor: usize, s: &str) -> usize {
    let bytes = s.as_bytes();
    let len = bytes.len();

    // Write compact length
    let cursor = write_compact_length(buffer, cursor, len);

    // Write string bytes
    buffer[cursor..cursor + len].copy_from_slice(bytes);

    cursor + len
}

/// Write a compact-encoded length to the buffer.
fn write_compact_length(buffer: &mut [u8], cursor: usize, len: usize) -> usize {
    if len < 0x80 {
        buffer[cursor] = len as u8;
        cursor + 1
    } else if len < 0x4000 {
        buffer[cursor] = (0x80 | (len >> 8)) as u8;
        buffer[cursor + 1] = (len & 0xFF) as u8;
        cursor + 2
    } else if len < 0x200000 {
        buffer[cursor] = (0xC0 | (len >> 16)) as u8;
        buffer[cursor + 1] = ((len >> 8) & 0xFF) as u8;
        buffer[cursor + 2] = (len & 0xFF) as u8;
        cursor + 3
    } else {
        buffer[cursor] = (0xE0 | (len >> 24)) as u8;
        buffer[cursor + 1] = ((len >> 16) & 0xFF) as u8;
        buffer[cursor + 2] = ((len >> 8) & 0xFF) as u8;
        buffer[cursor + 3] = (len & 0xFF) as u8;
        cursor + 4
    }
}

/// Parse a Solidity type string and return (BlueprintFieldKind, arrayLength).
///
/// BlueprintFieldKind enum values:
/// - Void=0, Bool=1, Uint8=2, Int8=3, Uint16=4, Int16=5, Uint32=6, Int32=7
/// - Uint64=8, Int64=9, Uint128=10, Int128=11, Uint256=12, Int256=13
/// - Address=14, Bytes32=15, FixedBytes=16, String=17, Bytes=18
/// - Optional=19, Array=20, List=21, Struct=22
fn parse_solidity_type(ty: &str) -> Result<(u8, u16)> {
    let ty = ty.trim();

    // Handle arrays: type[] or type[n]
    if ty.ends_with(']') {
        if let Some(bracket_pos) = ty.rfind('[') {
            let _base_type = &ty[..bracket_pos];
            let size_str = &ty[bracket_pos + 1..ty.len() - 1];

            if size_str.is_empty() {
                // Dynamic array: type[] → List (21) with child being the element type
                // For List, arrayLength=0 (dynamic), the actual element type goes in children
                return Ok((21, 0)); // List
            } else {
                // Fixed array: type[n] → Array (20) with arrayLength=n
                let size: u16 = size_str
                    .parse()
                    .map_err(|_| eyre!("invalid array size in type '{ty}'"))?;
                return Ok((20, size)); // Array
            }
        }
    }

    // Handle tuple (struct)
    if ty == "tuple" {
        return Ok((22, 0)); // Struct
    }

    // Handle basic types
    let kind = match ty {
        "bool" => 1,
        "uint8" => 2,
        "int8" => 3,
        "uint16" => 4,
        "int16" => 5,
        "uint32" => 6,
        "int32" => 7,
        "uint64" => 8,
        "int64" => 9,
        "uint128" => 10,
        "int128" => 11,
        "uint256" | "uint" => 12,
        "int256" | "int" => 13,
        "address" => 14,
        "bytes32" => 15,
        "string" => 17,
        "bytes" => 18,
        _ => {
            // Handle fixed bytes (bytes1 to bytes32)
            if let Some(size_str) = ty.strip_prefix("bytes") {
                if let Ok(size) = size_str.parse::<u16>() {
                    if size >= 1 && size <= 32 {
                        if size == 32 {
                            return Ok((15, 0)); // Bytes32
                        }
                        return Ok((16, size)); // FixedBytes with arrayLength=size
                    }
                }
            }
            // Handle uint with explicit size
            if let Some(size_str) = ty.strip_prefix("uint") {
                if let Ok(bits) = size_str.parse::<u16>() {
                    return Ok((
                        match bits {
                            8 => 2,
                            16 => 4,
                            32 => 6,
                            64 => 8,
                            128 => 10,
                            256 => 12,
                            _ => return Err(eyre!("unsupported uint size: uint{bits}")),
                        },
                        0,
                    ));
                }
            }
            // Handle int with explicit size
            if let Some(size_str) = ty.strip_prefix("int") {
                if let Ok(bits) = size_str.parse::<u16>() {
                    return Ok((
                        match bits {
                            8 => 3,
                            16 => 5,
                            32 => 7,
                            64 => 9,
                            128 => 11,
                            256 => 13,
                            _ => return Err(eyre!("unsupported int size: int{bits}")),
                        },
                        0,
                    ));
                }
            }
            return Err(eyre!("unsupported Solidity type: '{ty}'"));
        }
    };

    Ok((kind, 0))
}
