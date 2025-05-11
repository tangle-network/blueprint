use serde::{Deserialize, Serialize};

/// Common headers used in the authentication process.
pub mod headers {
    pub const AUTHORIZATION: &str = "Authorization";
    pub const X_SERVICE_ID: &str = "X-Service-Id";
}

/// Represents the ID a service in the authentication process.
///
/// The `ServiceId` is a tuple of two `u64` values, which can be used to uniquely identify a service.
/// The first `u64` represents the main service ID, while the second `u64` represents a sub-service or a specific instance of the service.
///
/// This structure is useful for identifying services in a distributed system, where multiple instances of the same service may co-exist on the
/// same service instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ServiceId(pub u64, pub u64);

impl From<(u64, u64)> for ServiceId {
    fn from(value: (u64, u64)) -> Self {
        ServiceId(value.0, value.1)
    }
}

impl ServiceId {
    /// Creates a new `ServiceId` instance with the given main service ID.
    pub fn new(main: u64) -> Self {
        ServiceId(main, 0)
    }

    /// Creates a new `ServiceId` instance with the given main service ID and sub-service ID.
    pub fn with_subservice(self, sub: u64) -> Self {
        ServiceId(self.0, sub)
    }

    /// The main service ID.
    pub fn id(&self) -> u64 {
        self.0
    }

    /// The sub-service ID.
    pub fn sub_id(&self) -> u64 {
        self.1
    }

    /// Checks if the `ServiceId` has a sub-service ID.
    ///
    /// Returns `true` if the sub-service ID is not zero, indicating that it is a specific instance of the service.
    pub fn has_sub_id(&self) -> bool {
        self.1 != 0
    }

    /// Converts the `ServiceId` to a big-endian byte array.
    pub const fn to_be_bytes(&self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        let hi = self.0.to_be_bytes();
        let lo = self.1.to_be_bytes();
        let mut i = 0;
        while i < 8 {
            bytes[i] = hi[i];
            bytes[i + 8] = lo[i];
            i += 1;
        }
        bytes
    }

    /// Creates a `ServiceId` from a big-endian byte array.
    pub const fn from_be_bytes(bytes: [u8; 16]) -> Self {
        let mut hi = [0u8; 8];
        let mut lo = [0u8; 8];
        let mut i = 0;
        while i < 8 {
            hi[i] = bytes[i];
            lo[i] = bytes[i + 8];
            i += 1;
        }
        ServiceId(u64::from_be_bytes(hi), u64::from_be_bytes(lo))
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ServiceIdParseError {
    /// Error parsing the main or sub-service ID as a `u64`.
    #[error(transparent)]
    ParseInt(#[from] core::num::ParseIntError),
    /// Error parsing the `ServiceId` from a string.
    #[error("Invalid ServiceId format, expected <main_id>[:<sub_id>]")]
    Malformed,
}

impl std::str::FromStr for ServiceId {
    type Err = ServiceIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        if let Some(main_str) = parts.next() {
            if let Some(sub_str) = parts.next() {
                if parts.next().is_none() {
                    let main = main_str.parse::<u64>()?;
                    let sub = sub_str.parse::<u64>()?;
                    return Ok(ServiceId(main, sub));
                }
            } else {
                let main = main_str.parse::<u64>()?;
                return Ok(ServiceId::new(main));
            }
        }
        Err(ServiceIdParseError::Malformed)
    }
}

impl core::fmt::Display for ServiceId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.has_sub_id() {
            write!(f, "{}:{}", self.0, self.1)
        } else {
            write!(f, "{}:0", self.0)
        }
    }
}

impl<S> axum::extract::FromRequestParts<S> for ServiceId
where
    S: Send + Sync,
{
    type Rejection = axum::response::Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        use axum::http::StatusCode;
        use axum::response::IntoResponse;

        let header = match parts.headers.get(crate::types::headers::X_SERVICE_ID) {
            Some(header) => header,
            None => {
                return Err((
                    StatusCode::PRECONDITION_REQUIRED,
                    "Missing X-Service-Id header",
                )
                    .into_response());
            }
        };

        let header_str = match header.to_str() {
            Ok(header_str) => header_str,
            Err(_) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Invalid X-Service-Id header; not a string",
                )
                    .into_response());
            }
        };

        match header_str.parse::<ServiceId>() {
            Ok(service_id) => Ok(service_id),
            Err(_) => Err((
                StatusCode::BAD_REQUEST,
                "Invalid X-Service-Id header; not a valid ServiceId",
            )
                .into_response()),
        }
    }
}

/// Represents the different types of cryptographic keys used in the authentication process.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    prost::Enumeration,
)]
#[repr(i32)]
pub enum KeyType {
    Unknown = 0,
    /// Ecdsa key type
    Ecdsa = 1,
    /// Sr25519 key type
    Sr25519 = 2,
}

/// Represents the challenge request sent from the client to the server to request a challenge.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChallengeRequest {
    /// The public key representing the user in hex format
    #[serde(with = "hex")]
    pub pub_key: Vec<u8>,
    /// The type of the public key
    pub key_type: KeyType,
}

/// Represents the challenge response sent from the server to the client after a successful challenge request.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChallengeResponse {
    /// The challenge string sent from the server to the client to be signed by the user
    #[serde(with = "hex")]
    pub challenge: [u8; 32],
    /// Expires at timestamp in milliseconds since epoch
    /// the time when the challenge will expire and should not be used anymore
    pub expires_at: u64,
}

/// Represents the challenge solution sent from the client to the server after signing the challenge string.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifyChallengeRequest {
    /// The original challenge request sent from the server to the client in the first step
    #[serde(flatten)]
    pub challenge_request: ChallengeRequest,
    /// The challenge string sent from the server to the client to be signed by the user
    #[serde(with = "hex")]
    pub challenge: [u8; 32],
    /// The signed challenge string sent from the client to the server
    #[serde(with = "hex")]
    pub signature: Vec<u8>,
    /// The timestamp in seconds since epoch at which the token will expire
    pub expires_at: u64,
}

/// Represents the response sent from the server to the client after verifying the challenge solution.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "status", content = "data")]
pub enum VerifyChallengeResponse {
    /// The challenge was verified successfully
    Verified {
        /// The access token to be used for authentication from now on
        access_token: String,
        /// A UNIX timestamp in milliseconds since epoch at which the access token will expire
        expires_at: u64,
    },
    /// The challenge was not verified because the challenge has expired
    Expired,
    /// The challenge was not verified because the signature is invalid
    InvalidSignature,

    /// The challenge was not verified because the service ID is not found
    ServiceNotFound,

    /// The challenge was not verified because the service ID is not authorized
    Unauthorized,

    /// An unexpected error occurred during verification
    UnexpectedError {
        /// The error message
        message: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_id_creation() {
        // Create with just main ID
        let service_id = ServiceId::new(42);
        assert_eq!(service_id.0, 42);
        assert_eq!(service_id.1, 0);

        // Create with main ID and add subservice
        let service_id = ServiceId::new(42).with_subservice(7);
        assert_eq!(service_id.0, 42);
        assert_eq!(service_id.1, 7);

        // Create from tuple
        let service_id = ServiceId::from((42, 7));
        assert_eq!(service_id.0, 42);
        assert_eq!(service_id.1, 7);
    }

    #[test]
    fn test_service_id_accessors() {
        let service_id = ServiceId(42, 7);

        assert_eq!(service_id.id(), 42);
        assert_eq!(service_id.sub_id(), 7);
        assert!(service_id.has_sub_id());

        let service_id = ServiceId(42, 0);
        assert!(!service_id.has_sub_id());
    }

    #[test]
    fn test_service_id_bytes_conversion() {
        let service_id = ServiceId(42, 7);

        let bytes = service_id.to_be_bytes();
        assert_eq!(bytes.len(), 16);

        let reconstructed = ServiceId::from_be_bytes(bytes);
        assert_eq!(reconstructed, service_id);

        // Test with different values
        let service_id = ServiceId(0xDEADBEEF, 0xCAFEBABE);
        let bytes = service_id.to_be_bytes();
        let reconstructed = ServiceId::from_be_bytes(bytes);
        assert_eq!(reconstructed, service_id);
    }

    #[test]
    fn test_service_id_parsing() {
        // Valid formats
        assert_eq!("42".parse::<ServiceId>().unwrap(), ServiceId(42, 0));
        assert_eq!("42:7".parse::<ServiceId>().unwrap(), ServiceId(42, 7));

        // Invalid formats
        let empty_result = "".parse::<ServiceId>();
        assert!(empty_result.is_err());

        assert!(matches!(
            "abc".parse::<ServiceId>(),
            Err(ServiceIdParseError::ParseInt(_))
        ));
        assert!(matches!(
            "42:7:9".parse::<ServiceId>(),
            Err(ServiceIdParseError::Malformed)
        ));
        assert!(matches!(
            "42:abc".parse::<ServiceId>(),
            Err(ServiceIdParseError::ParseInt(_))
        ));
    }

    #[test]
    fn test_service_id_display() {
        assert_eq!(ServiceId(42, 0).to_string(), "42:0");
        assert_eq!(ServiceId(42, 7).to_string(), "42:7");
    }

    #[test]
    fn test_key_type_conversion() {
        // Test KeyType to i32 conversion (as used in the ServiceOwnerModel)
        assert_eq!(KeyType::Unknown as i32, 0);
        assert_eq!(KeyType::Ecdsa as i32, 1);
        assert_eq!(KeyType::Sr25519 as i32, 2);

        // Test i32 to KeyType conversion (using transmute for simplicity in tests)
        let key_type: KeyType = unsafe { std::mem::transmute(1i32) };
        assert_eq!(key_type, KeyType::Ecdsa);
    }

    #[test]
    fn test_headers_constants() {
        assert_eq!(headers::AUTHORIZATION, "Authorization");
        assert_eq!(headers::X_SERVICE_ID, "X-Service-Id");
    }
}
