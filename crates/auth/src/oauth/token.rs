use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

/// OAuth 2.0 JWT Bearer Assertion token endpoint (RFC 7523)
/// Accepts application/x-www-form-urlencoded with:
/// - grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer
/// - assertion=<JWT>
pub async fn oauth_token(
    State(s): State<crate::proxy::AuthenticatedProxyState>,
    headers: axum::http::HeaderMap,
    axum::extract::Form(form): axum::extract::Form<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    let no_store_headers = [
        (axum::http::header::CACHE_CONTROL, "no-store"),
        (axum::http::header::PRAGMA, "no-cache"),
    ];
    let grant_type = form.get("grant_type").map(String::as_str);
    let assertion = form.get("assertion").map(String::as_str);

    if grant_type != Some("urn:ietf:params:oauth:grant-type:jwt-bearer") {
        return (
            StatusCode::BAD_REQUEST,
            (
                no_store_headers,
                Json(serde_json::json!({
                    "error": "unsupported_grant_type",
                    "error_description": "grant_type must be JWT bearer"
                })),
            ),
        );
    }

    let assertion = match assertion {
        Some(a) if !a.is_empty() => a,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                (
                    no_store_headers,
                    Json(serde_json::json!({
                        "error": "invalid_request",
                        "error_description": "missing assertion"
                    })),
                ),
            );
        }
    };

    // Map to service_id via header for MVP (consistent with existing endpoints)
    let service_id = match headers
        .get(crate::types::headers::X_SERVICE_ID)
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.parse::<crate::types::ServiceId>().ok())
    {
        Some(id) => id,
        None => {
            return (
                StatusCode::PRECONDITION_REQUIRED,
                (
                    no_store_headers,
                    Json(serde_json::json!({
                        "error": "invalid_request",
                        "error_description": "Missing X-Service-Id header"
                    })),
                ),
            );
        }
    };

    // Load per-service OAuth policy
    let policy = match crate::oauth::ServiceOAuthPolicy::load(service_id, s.db_ref()) {
        Ok(Some(p)) => p,
        Ok(None) => {
            return (
                StatusCode::BAD_REQUEST,
                (
                    no_store_headers,
                    Json(serde_json::json!({
                        "error": "invalid_request",
                        "error_description": "OAuth not enabled for this service"
                    })),
                ),
            );
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                (
                    no_store_headers,
                    Json(serde_json::json!({
                        "error": "server_error",
                        "error_description": "Failed to load service policy"
                    })),
                ),
            );
        }
    };

    // Best-effort per-IP rate limiting: 120 requests/minute per service
    if let Some(limit_err) =
        crate::oauth::rate_limit_check(s.db_ref(), &headers, service_id, 60, 120).err()
    {
        tracing::debug!("rate_limit_check error: {}", limit_err);
    }

    // Verify assertion (placeholder implementation returns NotConfigured until wired)
    let verifier = crate::oauth::AssertionVerifier::new(s.db_ref());
    match verifier.verify(assertion, &policy) {
        Ok(verified) => {
            // Intersect requested scopes with allowed scopes if present
            let scopes = match (&verified.scopes, &policy.allowed_scopes) {
                (Some(req), Some(allowed)) => {
                    let allowed_set: std::collections::BTreeSet<_> = allowed.iter().collect();
                    let filtered: Vec<String> = req
                        .iter()
                        .filter(|s| allowed_set.contains(s))
                        .cloned()
                        .collect();
                    if filtered.is_empty() {
                        None
                    } else {
                        Some(filtered)
                    }
                }
                (Some(_), None) => None, // scopes not allowed
                _ => None,
            };

            // Build headers and derive tenant from subject (privacy-safe hash)
            let mut headers_map = std::collections::BTreeMap::new();
            let tenant_hash = crate::validation::hash_user_id(&verified.subject);
            headers_map.insert("x-tenant-id".to_string(), tenant_hash);
            let validated_headers = match crate::validation::validate_headers(&headers_map) {
                Ok(h) => h,
                Err(e) => {
                    return (
                        StatusCode::BAD_REQUEST,
                        (
                            no_store_headers,
                            Json(serde_json::json!({
                                "error": "invalid_request",
                                "error_description": format!("Header validation failed: {}", e)
                            })),
                        ),
                    );
                }
            };
            let protected_headers =
                crate::validation::process_headers_with_pii_protection(&validated_headers);

            // Cap TTL by policy
            let ttl = std::time::Duration::from_secs(policy.max_access_token_ttl_secs.max(60));

            let token = match s.paseto_manager_ref().generate_token(
                service_id,
                "oauth".to_string(),
                protected_headers.get("x-tenant-id").cloned(),
                protected_headers,
                Some(ttl),
                scopes,
            ) {
                Ok(t) => t,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        (
                            no_store_headers,
                            Json(serde_json::json!({
                                "error": "server_error",
                                "error_description": "Failed to generate access token"
                            })),
                        ),
                    );
                }
            };

            let expires_at = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                + ttl.as_secs();
            let response = crate::auth_token::TokenExchangeResponse::new(token, expires_at);

            (
                StatusCode::OK,
                (
                    no_store_headers,
                    Json(serde_json::to_value(response).unwrap()),
                ),
            )
        }
        Err(crate::oauth::VerificationError::NotConfigured) => (
            StatusCode::BAD_REQUEST,
            (
                no_store_headers,
                Json(serde_json::json!({
                    "error": "invalid_grant",
                    "error_description": "JWT bearer assertion verification not configured"
                })),
            ),
        ),
        Err(crate::oauth::VerificationError::PolicyViolation(msg)) => (
            StatusCode::BAD_REQUEST,
            (
                no_store_headers,
                Json(serde_json::json!({
                    "error": "invalid_request",
                    "error_description": msg
                })),
            ),
        ),
        Err(crate::oauth::VerificationError::InvalidGrant(msg)) => (
            StatusCode::BAD_REQUEST,
            (
                no_store_headers,
                Json(serde_json::json!({
                    "error": "invalid_grant",
                    "error_description": msg
                })),
            ),
        ),
        Err(crate::oauth::VerificationError::InvalidRequest(msg)) => (
            StatusCode::BAD_REQUEST,
            (
                no_store_headers,
                Json(serde_json::json!({
                    "error": "invalid_request",
                    "error_description": msg
                })),
            ),
        ),
    }
}
