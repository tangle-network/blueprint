//! TEE key exchange service.
//!
//! Manages ephemeral key exchange sessions with TTL enforcement
//! and capacity limits. Designed to be wrapped as a `BackgroundService`
//! by the runner integration.

use crate::config::TeeKeyExchangeConfig;
use crate::errors::TeeError;
use crate::exchange::protocol::KeyExchangeSession;
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Service for TEE key exchange and session management.
///
/// Manages ephemeral key exchange sessions with configurable TTL,
/// capacity limits, and background cleanup. This is an adapter that
/// wraps session state and provides the API consumed by the runner's
/// TEE key-exchange endpoint.
///
/// # Usage
///
/// ```rust,ignore
/// use blueprint_tee::exchange::TeeAuthService;
/// use blueprint_tee::TeeKeyExchangeConfig;
///
/// let mut service = TeeAuthService::new(TeeKeyExchangeConfig::default());
/// service.start_cleanup_loop();
/// ```
pub struct TeeAuthService {
    config: TeeKeyExchangeConfig,
    sessions: Arc<Mutex<BTreeMap<String, KeyExchangeSession>>>,
    /// Abort handle to the background cleanup task, if started.
    /// Stored so the task can be cancelled on drop and to prevent the
    /// caller from silently discarding it.
    cleanup_handle: Option<tokio::task::AbortHandle>,
}

impl TeeAuthService {
    /// Create a new TEE auth service with the given configuration.
    pub fn new(config: TeeKeyExchangeConfig) -> Self {
        Self {
            config,
            sessions: Arc::new(Mutex::new(BTreeMap::new())),
            cleanup_handle: None,
        }
    }

    /// Create a new key exchange session.
    ///
    /// Returns the session ID and public key, or an error if the
    /// maximum session count is reached.
    pub async fn create_session(&self) -> Result<(String, Vec<u8>), TeeError> {
        let mut sessions = self.sessions.lock().await;

        // Evict expired sessions first
        sessions.retain(|_, s| !s.is_expired());

        if sessions.len() >= self.config.max_sessions {
            return Err(TeeError::KeyExchange(format!(
                "maximum session limit reached ({})",
                self.config.max_sessions
            )));
        }

        let session = KeyExchangeSession::new(self.config.session_ttl_secs);
        let session_id = session.session_id.clone();
        let public_key = session.public_key.clone();

        sessions.insert(session_id.clone(), session);

        tracing::debug!(session_id = %session_id, "created key exchange session");
        Ok((session_id, public_key))
    }

    /// Consume a session by ID, returning the session if valid.
    ///
    /// Sessions are atomically removed from the map on consumption (one-time use).
    /// Validation (expiry check) is performed before removal to avoid losing
    /// error context.
    pub async fn consume_session(&self, session_id: &str) -> Result<KeyExchangeSession, TeeError> {
        let mut sessions = self.sessions.lock().await;

        // Check validity before removing so we can return precise errors
        let session = sessions
            .get(session_id)
            .ok_or_else(|| TeeError::KeyExchange(format!("session not found: {session_id}")))?;

        if session.is_expired() {
            // Remove the expired session from the map as cleanup
            sessions.remove(session_id);
            return Err(TeeError::KeyExchange(format!(
                "session expired: {session_id}"
            )));
        }

        // Session is valid — remove and return it
        let session = sessions.remove(session_id).expect("session exists; checked above");

        tracing::debug!(session_id = %session_id, "consumed key exchange session");
        Ok(session)
    }

    /// Get the number of active (non-expired) sessions.
    pub async fn active_session_count(&self) -> usize {
        let sessions = self.sessions.lock().await;
        sessions.values().filter(|s| !s.is_expired()).count()
    }

    /// Get the public key for a session, if it exists and is valid.
    pub async fn get_session_public_key(&self, session_id: &str) -> Result<Vec<u8>, TeeError> {
        let sessions = self.sessions.lock().await;
        let session = sessions
            .get(session_id)
            .ok_or_else(|| TeeError::KeyExchange(format!("session not found: {session_id}")))?;

        if session.is_expired() {
            return Err(TeeError::KeyExchange(format!(
                "session expired: {session_id}"
            )));
        }

        Ok(session.public_key.clone())
    }

    /// Start the background cleanup loop for expired sessions.
    ///
    /// Spawns a tokio task that periodically evicts expired sessions.
    /// The `JoinHandle` is stored internally so the task is not silently dropped.
    /// Returns a clone of the handle for external monitoring if needed.
    pub fn start_cleanup_loop(&mut self) -> tokio::task::JoinHandle<()> {
        let sessions = self.sessions.clone();
        let ttl_secs = self.config.session_ttl_secs;

        let handle = tokio::spawn(async move {
            tracing::info!("TEE auth service cleanup loop started");

            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(ttl_secs.max(30))).await;

                let mut sessions = sessions.lock().await;
                let before = sessions.len();
                sessions.retain(|_, s| !s.is_expired());
                let evicted = before - sessions.len();

                if evicted > 0 {
                    tracing::debug!(
                        evicted = evicted,
                        remaining = sessions.len(),
                        "evicted expired key exchange sessions"
                    );
                }
            }
        });

        self.cleanup_handle = Some(handle.abort_handle());
        handle
    }

    /// Get the TTL configuration.
    pub fn session_ttl_secs(&self) -> u64 {
        self.config.session_ttl_secs
    }

    /// Get the max sessions configuration.
    pub fn max_sessions(&self) -> usize {
        self.config.max_sessions
    }
}

impl Drop for TeeAuthService {
    fn drop(&mut self) {
        if let Some(handle) = self.cleanup_handle.take() {
            handle.abort();
        }
    }
}
