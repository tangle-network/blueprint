//! Native Modal HTTP/2 transport. Account credentials never leave gRPC metadata.

use super::wire::{AuthToken, Empty};
use crate::{Error, Result};
use base64::Engine;
use prost::Message;
use std::time::{Duration, SystemTime};
use tokio::sync::OnceCell;
use tonic::{
    Request, Status,
    client::Grpc,
    codec::ProstCodec,
    codegen::http::uri::PathAndQuery,
    transport::{Channel, ClientTlsConfig, Endpoint},
};
use zeroize::Zeroizing;

pub(super) struct Rpc {
    endpoint: String,
    host: String,
    token_id: Zeroizing<String>,
    token_secret: Zeroizing<String>,
    channel: OnceCell<Channel>,
    token: tokio::sync::Mutex<Option<(Zeroizing<String>, SystemTime)>>,
}

impl Rpc {
    pub fn new(token_id: String, token_secret: String, endpoint: &str) -> Result<Self> {
        let url = url::Url::parse(endpoint)
            .map_err(|_| Error::ConfigurationError("Invalid Modal endpoint".into()))?;
        let host = url
            .host_str()
            .ok_or_else(|| Error::ConfigurationError("Invalid Modal endpoint".into()))?;
        let loopback = host == "localhost"
            || host
                .parse::<std::net::IpAddr>()
                .is_ok_and(|ip| ip.is_loopback());
        if !(url.scheme() == "https" || (url.scheme() == "http" && loopback))
            || !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
            || url.path() != "/"
            || token_id.is_empty()
            || token_secret.is_empty()
        {
            return Err(Error::ConfigurationError(
                "Invalid Modal credentials or endpoint".into(),
            ));
        }
        Ok(Self {
            endpoint: endpoint.into(),
            host: host.into(),
            token_id: Zeroizing::new(token_id),
            token_secret: Zeroizing::new(token_secret),
            channel: OnceCell::new(),
            token: tokio::sync::Mutex::new(None),
        })
    }

    async fn grpc(&self) -> std::result::Result<Grpc<Channel>, Status> {
        let channel = self
            .channel
            .get_or_try_init(|| async {
                let mut endpoint = Endpoint::from_shared(self.endpoint.clone())
                    .map_err(|_| Status::invalid_argument("Invalid provider endpoint"))?
                    .connect_timeout(Duration::from_secs(10))
                    .timeout(Duration::from_secs(60));
                if self.endpoint.starts_with("https:") {
                    endpoint = endpoint
                        .tls_config(ClientTlsConfig::new().with_webpki_roots())
                        .map_err(|_| Status::invalid_argument("Invalid TLS endpoint"))?;
                }
                endpoint
                    .connect()
                    .await
                    .map_err(|_| Status::unavailable("Provider connection failed"))
            })
            .await?;
        let mut client = Grpc::new(channel.clone());
        client
            .ready()
            .await
            .map_err(|_| Status::unavailable("Provider connection unavailable"))?;
        Ok(client)
    }

    fn request<Q>(
        &self,
        message: Q,
        token: Option<&str>,
    ) -> std::result::Result<Request<Q>, Status> {
        let mut request = Request::new(message);
        let metadata = request.metadata_mut();
        for (key, value) in [
            ("x-modal-token-id", self.token_id.as_str()),
            ("x-modal-token-secret", self.token_secret.as_str()),
            ("x-modal-host", self.host.as_str()),
            ("x-modal-client-type", "7"),
            ("x-modal-client-version", "1.0.0"),
            (
                "x-modal-libmodal-version",
                concat!("blueprint-remote-providers/", env!("CARGO_PKG_VERSION")),
            ),
        ] {
            metadata.insert(
                key,
                value
                    .parse()
                    .map_err(|_| Status::invalid_argument("Invalid provider credentials"))?,
            );
        }
        if let Some(token) = token {
            metadata.insert(
                "x-modal-auth-token",
                token
                    .parse()
                    .map_err(|_| Status::unauthenticated("Invalid provider token"))?,
            );
        }
        Ok(request)
    }

    async fn auth(&self) -> std::result::Result<Zeroizing<String>, Status> {
        let mut cached = self.token.lock().await;
        let now = SystemTime::now();
        if let Some((token, expires)) = cached.as_ref() {
            if now
                .checked_add(Duration::from_secs(10))
                .is_some_and(|time| time < *expires)
            {
                return Ok(Zeroizing::new(token.to_string()));
            }
        }
        let response: tonic::Response<AuthToken> = self
            .grpc()
            .await?
            .unary(
                self.request(Empty {}, None)?,
                PathAndQuery::from_static("/modal.client.ModalClient/AuthTokenGet"),
                ProstCodec::default(),
            )
            .await?;
        let token = Zeroizing::new(response.into_inner().token);
        if token.is_empty() {
            return Err(Status::unauthenticated("Provider token unavailable"));
        }
        // Expiry controls caching only; the provider verifies every authenticated RPC.
        let expires = token
            .split('.')
            .nth(1)
            .and_then(|payload| {
                base64::engine::general_purpose::URL_SAFE_NO_PAD
                    .decode(payload)
                    .ok()
            })
            .and_then(|payload| serde_json::from_slice::<serde_json::Value>(&payload).ok())
            .and_then(|claims| claims["exp"].as_u64())
            .and_then(|seconds| SystemTime::UNIX_EPOCH.checked_add(Duration::from_secs(seconds)))
            .unwrap_or(now);
        *cached = Some((Zeroizing::new(token.to_string()), expires));
        Ok(token)
    }

    pub async fn call<Q: Message + Send + Sync + 'static, R: Message + Default + Send + 'static>(
        &self,
        path: &'static str,
        message: Q,
    ) -> std::result::Result<R, Status> {
        let token = self.auth().await?;
        self.grpc()
            .await?
            .unary(
                self.request(message, Some(token.as_str()))?,
                PathAndQuery::from_static(path),
                ProstCodec::default(),
            )
            .await
            .map(tonic::Response::into_inner)
    }

    pub async fn stream<
        Q: Message + Send + Sync + 'static,
        R: Message + Default + Send + 'static,
    >(
        &self,
        path: &'static str,
        message: Q,
    ) -> std::result::Result<tonic::Streaming<R>, Status> {
        let token = self.auth().await?;
        self.grpc()
            .await?
            .server_streaming(
                self.request(message, Some(token.as_str()))?,
                PathAndQuery::from_static(path),
                ProstCodec::default(),
            )
            .await
            .map(tonic::Response::into_inner)
    }
}

pub(super) fn error(status: Status) -> Error {
    match status.code() {
        tonic::Code::Unauthenticated | tonic::Code::PermissionDenied => Error::AuthRejected,
        tonic::Code::InvalidArgument | tonic::Code::FailedPrecondition => {
            Error::ConfigurationError("Modal rejected this allocation request".into())
        }
        _ => Error::HttpError("Modal RPC failed".into()),
    }
}
