use std::collections::HashSet;

/// Lightweight auth context derived from canonical headers set by the proxy
#[derive(Clone, Debug, Default)]
pub struct AuthContext {
    pub tenant_hash: Option<String>,
    pub scopes: HashSet<String>,
}

impl AuthContext {
    pub fn from_headers(headers: &axum::http::HeaderMap) -> Self {
        // x-tenant-id is set by proxy after PII hashing
        let tenant_hash = headers
            .get("x-tenant-id")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        // x-scopes is a space-delimited list; normalize to lowercase set
        let scopes = headers
            .get("x-scopes")
            .and_then(|v| v.to_str().ok())
            .map(|s| {
                s.split(' ')
                    .filter(|p| !p.is_empty())
                    .map(|p| p.to_ascii_lowercase())
                    .collect::<HashSet<_>>()
            })
            .unwrap_or_default();

        AuthContext {
            tenant_hash,
            scopes,
        }
    }

    pub fn has_scope(&self, scope: &str) -> bool {
        self.scopes.contains(&scope.to_ascii_lowercase())
    }

    pub fn has_any_scope<'a>(&self, names_or_prefixes: impl IntoIterator<Item = &'a str>) -> bool {
        let mut has = false;
        for n in names_or_prefixes {
            let n = n.to_ascii_lowercase();
            if n.ends_with(':') {
                // prefix match
                if self.scopes.iter().any(|s| s.starts_with(&n)) {
                    has = true;
                    break;
                }
            } else if self.scopes.contains(&n) {
                has = true;
                break;
            }
        }
        has
    }
}
