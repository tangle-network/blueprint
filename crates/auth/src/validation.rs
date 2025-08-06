use std::collections::{BTreeMap, HashSet};

/// Maximum number of additional headers allowed
const MAX_HEADERS: usize = 8;

/// Maximum length for header names and values
const MAX_HEADER_NAME_LEN: usize = 256;
const MAX_HEADER_VALUE_LEN: usize = 512;

/// Headers that should not be forwarded (hop-by-hop headers)
const FORBIDDEN_HEADERS: &[&str] = &[
    "connection",
    "keep-alive",
    "proxy-authenticate",
    "proxy-authorization",
    "te",
    "trailer",
    "transfer-encoding",
    "upgrade",
    "host",
    "content-length",
];

/// Validates and sanitizes additional headers
pub fn validate_headers(
    headers: &BTreeMap<String, String>,
) -> Result<BTreeMap<String, String>, ValidationError> {
    if headers.len() > MAX_HEADERS {
        return Err(ValidationError::TooManyHeaders {
            max: MAX_HEADERS,
            provided: headers.len(),
        });
    }

    let forbidden_set: HashSet<String> =
        FORBIDDEN_HEADERS.iter().map(|h| h.to_lowercase()).collect();

    let mut validated = BTreeMap::new();

    for (name, value) in headers {
        // Validate header name
        let name_lower = name.to_lowercase();

        if forbidden_set.contains(&name_lower) {
            return Err(ValidationError::ForbiddenHeader {
                header: name.clone(),
            });
        }

        if name.len() > MAX_HEADER_NAME_LEN {
            return Err(ValidationError::HeaderNameTooLong {
                header: name.clone(),
                max: MAX_HEADER_NAME_LEN,
            });
        }

        if value.len() > MAX_HEADER_VALUE_LEN {
            return Err(ValidationError::HeaderValueTooLong {
                header: name.clone(),
                max: MAX_HEADER_VALUE_LEN,
            });
        }

        // Validate that header name only contains valid characters
        if !is_valid_header_name(name) {
            return Err(ValidationError::InvalidHeaderName {
                header: name.clone(),
            });
        }

        // Validate that header value only contains valid characters
        if !is_valid_header_value(value) {
            return Err(ValidationError::InvalidHeaderValue {
                header: name.clone(),
                value: value.clone(),
            });
        }

        // Store with canonical casing (preserve original)
        validated.insert(name.clone(), value.clone());
    }

    Ok(validated)
}

/// Check if a header name contains only valid characters
fn is_valid_header_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

/// Check if a header value contains only valid characters
fn is_valid_header_value(value: &str) -> bool {
    value.chars().all(|c| {
        // Allow printable ASCII characters and spaces
        (c.is_ascii() && !c.is_control()) || c == '\t'
    })
}

/// Hash a user ID to create a tenant ID
pub fn hash_user_id(user_id: &str) -> String {
    use tiny_keccak::{Hasher, Keccak};
    let mut hasher = Keccak::v256();
    hasher.update(user_id.as_bytes());
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    // Use first 16 bytes of hash for a compact representation
    hex::encode(&output[..16])
}

/// Process headers with PII protection
/// Hashes user IDs and emails in known PII headers
pub fn process_headers_with_pii_protection(
    headers: &BTreeMap<String, String>,
) -> BTreeMap<String, String> {
    let mut processed = BTreeMap::new();

    for (name, value) in headers {
        let processed_value = match name.to_lowercase().as_str() {
            // Hash PII fields
            "x-user-id" | "x-user-email" | "x-customer-email" => hash_user_id(value),
            // For tenant ID, check if it looks like an email or raw ID
            "x-tenant-id" => {
                if value.contains('@') {
                    // It's an email, hash it
                    hash_user_id(value)
                } else if value.len() == 32 && value.chars().all(|c| c.is_ascii_hexdigit()) {
                    // Already looks like a hash, keep it
                    value.clone()
                } else {
                    // Raw ID, hash it for privacy
                    hash_user_id(value)
                }
            }
            // Keep other headers as-is
            _ => value.clone(),
        };
        processed.insert(name.clone(), processed_value);
    }

    processed
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Too many headers provided: {provided} (max: {max})")]
    TooManyHeaders { max: usize, provided: usize },

    #[error("Forbidden header: {header}")]
    ForbiddenHeader { header: String },

    #[error("Header name too long: {header} (max: {max} bytes)")]
    HeaderNameTooLong { header: String, max: usize },

    #[error("Header value too long for {header} (max: {max} bytes)")]
    HeaderValueTooLong { header: String, max: usize },

    #[error("Invalid header name: {header}")]
    InvalidHeaderName { header: String },

    #[error("Invalid header value for {header}: {value}")]
    InvalidHeaderValue { header: String, value: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_headers_valid() {
        let mut headers = BTreeMap::new();
        headers.insert("X-Tenant-Id".to_string(), "abc123".to_string());
        headers.insert("X-User-Type".to_string(), "premium".to_string());

        let result = validate_headers(&headers);
        assert!(result.is_ok());
        let validated = result.unwrap();
        assert_eq!(validated.len(), 2);
    }

    #[test]
    fn test_validate_headers_too_many() {
        let mut headers = BTreeMap::new();
        for i in 0..10 {
            headers.insert(format!("X-Header-{}", i), "value".to_string());
        }

        let result = validate_headers(&headers);
        assert!(matches!(
            result,
            Err(ValidationError::TooManyHeaders { .. })
        ));
    }

    #[test]
    fn test_validate_headers_forbidden() {
        let mut headers = BTreeMap::new();
        headers.insert("Connection".to_string(), "close".to_string());

        let result = validate_headers(&headers);
        assert!(matches!(
            result,
            Err(ValidationError::ForbiddenHeader { .. })
        ));
    }

    #[test]
    fn test_validate_headers_invalid_name() {
        let mut headers = BTreeMap::new();
        headers.insert("X-Invalid Header".to_string(), "value".to_string());

        let result = validate_headers(&headers);
        assert!(matches!(
            result,
            Err(ValidationError::InvalidHeaderName { .. })
        ));
    }

    #[test]
    fn test_validate_headers_name_too_long() {
        let mut headers = BTreeMap::new();
        let long_name = "X-".to_string() + &"a".repeat(300);
        headers.insert(long_name, "value".to_string());

        let result = validate_headers(&headers);
        assert!(matches!(
            result,
            Err(ValidationError::HeaderNameTooLong { .. })
        ));
    }

    #[test]
    fn test_validate_headers_value_too_long() {
        let mut headers = BTreeMap::new();
        let long_value = "a".repeat(600);
        headers.insert("X-Test".to_string(), long_value);

        let result = validate_headers(&headers);
        assert!(matches!(
            result,
            Err(ValidationError::HeaderValueTooLong { .. })
        ));
    }

    #[test]
    fn test_hash_user_id() {
        let user_id = "user123@example.com";
        let hash1 = hash_user_id(user_id);
        let hash2 = hash_user_id(user_id);

        // Should be deterministic
        assert_eq!(hash1, hash2);

        // Should be 32 characters (16 bytes hex encoded)
        assert_eq!(hash1.len(), 32);

        // Different inputs should produce different hashes
        let hash3 = hash_user_id("different@example.com");
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_valid_header_name() {
        assert!(is_valid_header_name("X-Tenant-Id"));
        assert!(is_valid_header_name("X_User_Type"));
        assert!(is_valid_header_name("Authorization"));

        assert!(!is_valid_header_name(""));
        assert!(!is_valid_header_name("X Tenant Id"));
        assert!(!is_valid_header_name("X-Tenant:Id"));
    }

    #[test]
    fn test_valid_header_value() {
        assert!(is_valid_header_value("abc123"));
        assert!(is_valid_header_value("Bearer token123"));
        assert!(is_valid_header_value("value with spaces"));

        assert!(!is_valid_header_value("value\nwith\nnewlines"));
        assert!(!is_valid_header_value("value\0with\0nulls"));
    }
}
