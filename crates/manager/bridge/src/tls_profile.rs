use blueprint_auth::models::TlsProfile;

use crate::TlsProfileConfig;

impl From<TlsProfile> for TlsProfileConfig {
    fn from(profile: TlsProfile) -> Self {
        Self {
            tls_enabled: profile.tls_enabled,
            require_client_mtls: profile.require_client_mtls,
            encrypted_server_cert: profile.encrypted_server_cert,
            encrypted_server_key: profile.encrypted_server_key,
            encrypted_client_ca_bundle: profile.encrypted_client_ca_bundle,
            encrypted_upstream_ca_bundle: profile.encrypted_upstream_ca_bundle,
            encrypted_upstream_client_cert: profile.encrypted_upstream_client_cert,
            encrypted_upstream_client_key: profile.encrypted_upstream_client_key,
            client_cert_ttl_hours: profile.client_cert_ttl_hours,
            sni: profile.sni,
            subject_alt_name_template: profile.subject_alt_name_template,
            allowed_dns_names: profile.allowed_dns_names,
        }
    }
}

impl From<TlsProfileConfig> for TlsProfile {
    fn from(config: TlsProfileConfig) -> Self {
        Self {
            tls_enabled: config.tls_enabled,
            require_client_mtls: config.require_client_mtls,
            encrypted_server_cert: config.encrypted_server_cert,
            encrypted_server_key: config.encrypted_server_key,
            encrypted_client_ca_bundle: config.encrypted_client_ca_bundle,
            encrypted_upstream_ca_bundle: config.encrypted_upstream_ca_bundle,
            encrypted_upstream_client_cert: config.encrypted_upstream_client_cert,
            encrypted_upstream_client_key: config.encrypted_upstream_client_key,
            client_cert_ttl_hours: config.client_cert_ttl_hours,
            sni: config.sni,
            subject_alt_name_template: config.subject_alt_name_template,
            allowed_dns_names: config.allowed_dns_names,
        }
    }
}
