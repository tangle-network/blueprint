//! Certificate Authority utilities for mTLS implementation.
//!
//! The helper manages a per-service certificate authority capable of issuing
//! server and client certificates that chain back to the stored root. All
//! private material is derived from [`TlsEnvelope`] encrypted storage.

use blueprint_core::debug;
use rcgen::{
    BasicConstraints, CertificateParams, DistinguishedName, DnType, ExtendedKeyUsagePurpose, IsCa,
    Issuer, KeyPair, KeyUsagePurpose, SanType, SerialNumber,
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::tls_envelope::TlsEnvelope;
use crate::types::ServiceId;

/// Certificate authority wrapper that signs leaf certificates for a service.
pub struct CertificateAuthority {
    ca_key_pair: KeyPair,
    ca_cert_pem: String,
    tls_envelope: TlsEnvelope,
}

impl CertificateAuthority {
    /// Create a brand new CA certificate and key.
    pub fn new(tls_envelope: TlsEnvelope) -> Result<Self, crate::Error> {
        let mut params = CertificateParams::default();
        params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
        params.key_usages = vec![
            KeyUsagePurpose::KeyCertSign,
            KeyUsagePurpose::CrlSign,
            KeyUsagePurpose::DigitalSignature,
        ];
        params.extended_key_usages = vec![
            ExtendedKeyUsagePurpose::ServerAuth,
            ExtendedKeyUsagePurpose::ClientAuth,
        ];

        let mut dn = DistinguishedName::new();
        dn.push(DnType::CommonName, "Tangle Network CA");
        dn.push(DnType::OrganizationName, "Tangle Network");
        params.distinguished_name = dn;

        let ca_key_pair = KeyPair::generate()?;
        let ca_cert = params.self_signed(&ca_key_pair)?;
        let ca_cert_pem = ca_cert.pem();

        debug!("Created new certificate authority");
        Ok(Self {
            ca_key_pair,
            ca_cert_pem,
            tls_envelope,
        })
    }

    /// Restore a CA from persisted certificate and key PEM strings.
    pub fn from_components(
        ca_cert_pem: &str,
        ca_key_pem: &str,
        tls_envelope: TlsEnvelope,
    ) -> Result<Self, crate::Error> {
        // Basic sanity validation – ensure the PEM really contains a certificate.
        let blocks = pem::parse_many(ca_cert_pem).map_err(|e| {
            crate::Error::Io(std::io::Error::other(format!(
                "Failed to parse CA certificate PEM: {e}"
            )))
        })?;
        if blocks.is_empty() || blocks.iter().all(|b| b.tag != "CERTIFICATE") {
            return Err(crate::Error::Io(std::io::Error::other(
                "CA bundle is missing a CERTIFICATE block",
            )));
        }

        let ca_key_pair = KeyPair::from_pem(ca_key_pem).map_err(|e| {
            crate::Error::Io(std::io::Error::other(format!(
                "Failed to parse CA private key PEM: {e}"
            )))
        })?;

        debug!("Loaded existing certificate authority");
        Ok(Self {
            ca_key_pair,
            ca_cert_pem: ca_cert_pem.to_string(),
            tls_envelope,
        })
    }

    /// Return the CA certificate in PEM encoding.
    pub fn ca_certificate_pem(&self) -> String {
        self.ca_cert_pem.clone()
    }

    /// Return the CA private key PEM.
    pub fn ca_private_key_pem(&self) -> String {
        self.ca_key_pair.serialize_pem()
    }

    /// Helper to create an issuer used for signing.
    fn issuer(&self) -> Result<Issuer<'static, KeyPair>, crate::Error> {
        let issuer_key = KeyPair::from_pem(&self.ca_key_pair.serialize_pem()).map_err(|e| {
            crate::Error::Io(std::io::Error::other(format!(
                "Failed to clone CA private key: {e}"
            )))
        })?;
        Issuer::from_ca_cert_pem(&self.ca_cert_pem, issuer_key).map_err(|e| {
            crate::Error::Io(std::io::Error::other(format!(
                "Failed to build issuer from CA cert: {e}"
            )))
        })
    }

    /// Generate a server certificate for the upstream service.
    pub fn generate_server_certificate(
        &self,
        service_id: ServiceId,
        dns_names: Vec<String>,
    ) -> Result<(String, String), crate::Error> {
        let mut params = CertificateParams::default();
        params.key_usages = vec![
            KeyUsagePurpose::DigitalSignature,
            KeyUsagePurpose::KeyEncipherment,
        ];
        params.extended_key_usages = vec![ExtendedKeyUsagePurpose::ServerAuth];

        let mut dn = DistinguishedName::new();
        dn.push(DnType::CommonName, format!("Service {service_id}"));
        dn.push(DnType::OrganizationName, "Tangle Network");
        params.distinguished_name = dn;

        params.subject_alt_names = dns_names
            .into_iter()
            .map(try_dns_name)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(SanType::DnsName)
            .collect();

        params.serial_number = Some(random_serial()?);

        let leaf_key = KeyPair::generate()?;
        let issuer = self.issuer()?;
        let cert = params.signed_by(&leaf_key, &issuer)?;

        Ok((cert.pem(), leaf_key.serialize_pem()))
    }

    /// Generate a client certificate respecting TTL and metadata expectations.
    pub fn generate_client_certificate(
        &self,
        common_name: String,
        subject_alt_names: Vec<String>,
        ttl_hours: u32,
    ) -> Result<ClientCertificate, crate::Error> {
        let mut params = CertificateParams::default();
        params.key_usages = vec![
            KeyUsagePurpose::DigitalSignature,
            KeyUsagePurpose::KeyAgreement,
        ];
        params.extended_key_usages = vec![ExtendedKeyUsagePurpose::ClientAuth];

        let mut dn = DistinguishedName::new();
        dn.push(DnType::CommonName, common_name.clone());
        dn.push(DnType::OrganizationName, "Tangle Network");
        params.distinguished_name = dn;

        for san in subject_alt_names {
            if let Some(rest) = san.strip_prefix("DNS:") {
                let dns = try_dns_name(rest.to_string())?;
                params.subject_alt_names.push(SanType::DnsName(dns));
                continue;
            }
            if let Some(rest) = san.strip_prefix("URI:") {
                let uri = try_uri_name(rest.to_string())?;
                params.subject_alt_names.push(SanType::URI(uri));
                continue;
            }
            let dns = try_dns_name(san.clone())?;
            params.subject_alt_names.push(SanType::DnsName(dns));
        }

        let now = OffsetDateTime::now_utc();
        params.not_before = now;
        let ttl = time::Duration::hours(i64::from(ttl_hours));
        let expiry = now + ttl;
        params.not_after = expiry;
        params.serial_number = Some(random_serial()?);

        let client_key = KeyPair::generate()?;
        let issuer = self.issuer()?;
        let cert = params.signed_by(&client_key, &issuer)?;
        let serial = params
            .serial_number
            .as_ref()
            .map(|s| hex::encode(s.to_bytes()))
            .unwrap_or_else(|| "missing-serial".to_string());

        Ok(ClientCertificate {
            certificate_pem: cert.pem(),
            private_key_pem: client_key.serialize_pem(),
            ca_bundle_pem: self.ca_cert_pem.clone(),
            serial: serial.clone(),
            expires_at: expiry.unix_timestamp().max(0) as u64,
            revocation_url: Some(format!("/v1/auth/certificates/{serial}/revoke")),
        })
    }

    /// Encrypt helper used by persistence routines.
    pub fn envelope(&self) -> &TlsEnvelope {
        &self.tls_envelope
    }
}

/// Random 128-bit serial compliant with RFC 5280 (avoid negative).
fn random_serial() -> Result<SerialNumber, crate::Error> {
    use blueprint_std::rand::RngCore;

    let mut rng = blueprint_std::BlueprintRng::new();
    let mut bytes = [0u8; 16];
    rng.fill_bytes(&mut bytes);
    bytes[0] &= 0x7F; // ensure positive (highest bit zero)
    if bytes.iter().all(|b| *b == 0) {
        bytes[15] = 1; // avoid zero serials
    }
    Ok(SerialNumber::from_slice(&bytes))
}

fn try_dns_name(value: String) -> Result<rcgen::string::Ia5String, crate::Error> {
    rcgen::string::Ia5String::try_from(value.clone()).map_err(|e| {
        crate::Error::Io(std::io::Error::other(format!(
            "Invalid DNS subjectAltName `{value}`: {e}"
        )))
    })
}

fn try_uri_name(value: String) -> Result<rcgen::string::Ia5String, crate::Error> {
    rcgen::string::Ia5String::try_from(value.clone()).map_err(|e| {
        crate::Error::Io(std::io::Error::other(format!(
            "Invalid URI subjectAltName `{value}`: {e}"
        )))
    })
}

/// Issued client certificate bundle returned to callers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCertificate {
    pub certificate_pem: String,
    pub private_key_pem: String,
    pub ca_bundle_pem: String,
    pub serial: String,
    pub expires_at: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_url: Option<String>,
}

/// Request payload for creating or updating a TLS profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTlsProfileRequest {
    pub require_client_mtls: bool,
    pub client_cert_ttl_hours: u32,
    pub subject_alt_name_template: Option<String>,
    pub allowed_dns_names: Option<Vec<String>>,
}

/// Request payload for client certificate issuance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueCertificateRequest {
    pub service_id: u64,
    pub common_name: String,
    pub subject_alt_names: Vec<String>,
    pub ttl_hours: u32,
}

/// Response returned when updating a TLS profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsProfileResponse {
    pub tls_enabled: bool,
    pub require_client_mtls: bool,
    pub client_cert_ttl_hours: u32,
    pub mtls_listener: String,
    pub subject_alt_name_template: Option<String>,
}

/// Validate a certificate issuance request against the stored TLS profile.
pub fn validate_certificate_request(
    request: &IssueCertificateRequest,
    profile: &crate::models::TlsProfile,
) -> Result<(), crate::Error> {
    if !profile.require_client_mtls {
        return Err(crate::Error::Io(std::io::Error::other(
            "Client mTLS is not enabled for this service",
        )));
    }

    if request.ttl_hours > profile.client_cert_ttl_hours {
        return Err(crate::Error::Io(std::io::Error::other(format!(
            "Certificate TTL {} hours exceeds maximum allowed {} hours",
            request.ttl_hours, profile.client_cert_ttl_hours
        ))));
    }

    // TODO: enforce SAN allowlists when the profile stores them.
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tls_envelope::TlsEnvelope;

    #[test]
    fn ca_material_round_trips() {
        let ca = CertificateAuthority::new(TlsEnvelope::new()).expect("fresh ca");

        let cert_pem = ca.ca_certificate_pem();
        let key_pem = ca.ca_private_key_pem();

        let restored =
            CertificateAuthority::from_components(&cert_pem, &key_pem, TlsEnvelope::new())
                .expect("restore ca");

        assert_eq!(restored.ca_certificate_pem(), cert_pem);
        assert_eq!(restored.ca_private_key_pem(), key_pem);
    }

    #[test]
    fn client_cert_respects_ttl_and_serial() {
        let ca = CertificateAuthority::new(TlsEnvelope::new()).expect("ca");
        let cert = ca
            .generate_client_certificate("tenant-alpha".into(), vec!["localhost".into()], 1)
            .expect("client cert");

        assert_ne!(cert.serial, "missing-serial");
        assert!(cert.ca_bundle_pem.contains("BEGIN CERTIFICATE"));
        assert!(
            cert.revocation_url
                .as_ref()
                .expect("revocation url")
                .ends_with(&format!("{}/revoke", cert.serial))
        );

        let now = OffsetDateTime::now_utc().unix_timestamp();
        assert!(cert.expires_at >= now as u64);
    }
}
