use std::collections::HashMap;

/// Lightweight auth context derived from canonical headers set by the proxy
#[derive(Clone, Debug, Default)]
pub struct AuthContext {
    inner: HashMap<String, String>,
}

impl AuthContext {
    /// Create AuthContext from a generic header map to avoid axum dependency
    pub fn from_headers<H>(headers: &H) -> Self
    where
        H: std::ops::Deref<Target = std::collections::HashMap<String, String>>,
    {
        let mut inner = HashMap::new();

        // x-tenant-id is set by proxy after PII hashing
        if let Some(tenant_hash) = headers.get("x-tenant-id") {
            inner.insert("tenant_hash".to_string(), tenant_hash.clone());
        }

        // x-scopes is a space-delimited list; normalize to lowercase
        if let Some(scopes_str) = headers.get("x-scopes") {
            inner.insert("scopes".to_string(), scopes_str.clone());
        }

        AuthContext { inner }
    }

    /// Create AuthContext from axum HeaderMap for backward compatibility
    pub fn from_axum_headers(headers: &axum::http::HeaderMap) -> Self {
        let mut inner = HashMap::new();

        // x-tenant-id is set by proxy after PII hashing
        if let Some(tenant_hash) = headers.get("x-tenant-id").and_then(|v| v.to_str().ok()) {
            inner.insert("tenant_hash".to_string(), tenant_hash.to_string());
        }

        // x-scopes is a space-delimited list; normalize to lowercase
        if let Some(scopes_str) = headers.get("x-scopes").and_then(|v| v.to_str().ok()) {
            inner.insert("scopes".to_string(), scopes_str.to_string());
        }

        AuthContext { inner }
    }

    pub fn tenant_hash(&self) -> Option<&str> {
        self.inner.get("tenant_hash").map(|s| s.as_str())
    }

    pub fn scopes(&self) -> Vec<String> {
        self.inner
            .get("scopes")
            .map(|s| {
                s.split(' ')
                    .filter(|p| !p.is_empty())
                    .map(|p| p.to_ascii_lowercase())
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn has_scope(&self, scope: &str) -> bool {
        let scope = scope.to_ascii_lowercase();
        self.scopes().contains(&scope)
    }

    pub fn has_any_scope<'a>(&self, names_or_prefixes: impl IntoIterator<Item = &'a str>) -> bool {
        let scopes = self.scopes();
        for n in names_or_prefixes {
            let n = n.to_ascii_lowercase();
            if n.ends_with(':') {
                // prefix match
                if scopes.iter().any(|s| s.starts_with(&n)) {
                    return true;
                }
            } else if scopes.contains(&n) {
                return true;
            }
        }
        false
    }
}
