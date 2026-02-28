//! TEE key exchange background service.
//!
//! Implements [`BackgroundService`] for managing ephemeral key exchange
//! sessions with TTL enforcement and capacity limits.

use crate::config::TeeKeyExchangeConfig;
use crate::errors::TeeError;
use crate::exchange::protocol::KeyExchangeSession;
use blueprint_runner::error::RunnerError;
use blueprint_runner::BackgroundService;
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::{Mutex, oneshot};

/// Background service for TEE key exchange and session management.
///
/// Manages ephemeral key exchange sessions with:
/// - Configurable TTL for session keys
/// - Maximum concurrent session limit
/// - Automatic cleanup of expired sessions
///
/// # Examples
///
/// ```rust,ignore
/// use blueprint_tee::exchange::TeeAuthService;
/// use blueprint_tee::TeeKeyExchangeConfig;
///
/// let service = TeeAuthService::new(TeeKeyExchangeConfig::default());
///
/// // Register as a background service with the runner
/// BlueprintRunner::builder(config, env)
///     .background_service(service)
///     .run()
///     .await?;
/// ```
pub struct TeeAuthService {
    config: TeeKeyExchangeConfig,
    sessions: Arc<Mutex<BTreeMap<String, KeyExchangeSession>>>,
}

impl TeeAuthService {
    /// Create a new TEE auth service with the given configuration.
    pub fn new(config: TeeKeyExchangeConfig) -> Self {
        Self {
            config,
            sessions: Arc::new(Mutex::new(BTreeMap::new())),
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
    /// A consumed session cannot be reused (one-time handoff).
    pub async fn consume_session(
        &self,
        session_id: &str,
    ) -> Result<KeyExchangeSession, TeeError> {
        let mut sessions = self.sessions.lock().await;

        let session = sessions.remove(session_id).ok_or_else(|| {
            TeeError::KeyExchange(format!("session not found: {session_id}"))
        })?;

        if session.is_expired() {
            return Err(TeeError::KeyExchange(format!(
                "session expired: {session_id}"
            )));
        }

        if session.consumed {
            return Err(TeeError::KeyExchange(format!(
                "session already consumed: {session_id}"
            )));
        }

        tracing::debug!(session_id = %session_id, "consumed key exchange session");
        Ok(session)
    }

    /// Get the number of active (non-expired) sessions.
    pub async fn active_session_count(&self) -> usize {
        let sessions = self.sessions.lock().await;
        sessions.values().filter(|s| s.is_valid()).count()
    }

    /// Get the public key for a session, if it exists and is valid.
    pub async fn get_session_public_key(
        &self,
        session_id: &str,
    ) -> Result<Vec<u8>, TeeError> {
        let sessions = self.sessions.lock().await;
        let session = sessions.get(session_id).ok_or_else(|| {
            TeeError::KeyExchange(format!("session not found: {session_id}"))
        })?;

        if !session.is_valid() {
            return Err(TeeError::KeyExchange(format!(
                "session no longer valid: {session_id}"
            )));
        }

        Ok(session.public_key.clone())
    }
}

impl BackgroundService for TeeAuthService {
    async fn start(&self) -> Result<oneshot::Receiver<Result<(), RunnerError>>, RunnerError> {
        let (tx, rx) = oneshot::channel();
        let sessions = self.sessions.clone();
        let ttl_secs = self.config.session_ttl_secs;

        tokio::spawn(async move {
            tracing::info!("TEE auth service started");

            // Periodic cleanup loop
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

            // This loop runs indefinitely; if we ever exit, signal completion
            #[allow(unreachable_code)]
            {
                let _ = tx.send(Ok(()));
            }
        });

        Ok(rx)
    }
}
