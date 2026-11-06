//! Consumed fields of Modal's Apache-2.0 protocol, pinned to
//! modal-labs/modal-client 7ccff7bd084b7127451d5fcc431bcf565696b428.
//! Field numbers are upstream ABI; unknown response fields remain ignored.

use prost::Message;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Message)]
pub(super) struct Empty {}
#[derive(Clone, PartialEq, Message)]
pub(super) struct AuthToken {
    #[prost(string, tag = "1")]
    pub token: String,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct Rates {
    #[prost(map = "string, string", tag = "1")]
    pub rates: HashMap<String, String>,
    #[prost(map = "string, string", tag = "2")]
    pub warnings: HashMap<String, String>,
    #[prost(map = "string, string", tag = "3")]
    pub errors: HashMap<String, String>,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct AppRequest {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(string, tag = "2")]
    pub environment: String,
    #[prost(int32, tag = "3")]
    pub creation_type: i32,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct Id {
    #[prost(string, tag = "1")]
    pub id: String,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct Environment {
    #[prost(message, optional, tag = "2")]
    pub metadata: Option<EnvironmentMetadata>,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct EnvironmentMetadata {
    #[prost(message, optional, tag = "2")]
    pub settings: Option<EnvironmentSettings>,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct EnvironmentSettings {
    #[prost(string, tag = "1")]
    pub builder: String,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct Gpu {
    #[prost(uint32, tag = "2")]
    pub count: u32,
    #[prost(string, tag = "4")]
    pub kind: String,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct Resources {
    #[prost(uint32, tag = "2")]
    pub memory_mib: u32,
    #[prost(uint32, tag = "3")]
    pub milli_cpu: u32,
    #[prost(message, optional, tag = "4")]
    pub gpu: Option<Gpu>,
    #[prost(uint32, tag = "5")]
    pub memory_limit_mib: u32,
    #[prost(uint32, tag = "7")]
    pub milli_cpu_limit: u32,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct Image {
    #[prost(string, repeated, tag = "6")]
    pub commands: Vec<String>,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct ImageRequest {
    #[prost(message, optional, tag = "2")]
    pub image: Option<Image>,
    #[prost(string, tag = "4")]
    pub app_id: String,
    #[prost(string, tag = "9")]
    pub builder: String,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct ResultInfo {
    #[prost(int32, tag = "1")]
    pub status: i32,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct ImageResponse {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(message, optional, tag = "2")]
    pub result: Option<ResultInfo>,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct ImageJoinRequest {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(float, tag = "2")]
    pub timeout: f32,
    #[prost(string, tag = "3")]
    pub last_entry: String,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct ImageJoinResponse {
    #[prost(message, optional, tag = "1")]
    pub result: Option<ResultInfo>,
    #[prost(string, tag = "3")]
    pub entry: String,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct SecretRequest {
    #[prost(string, tag = "3")]
    pub environment: String,
    #[prost(int32, tag = "4")]
    pub creation_type: i32,
    #[prost(map = "string, string", tag = "5")]
    pub values: HashMap<String, String>,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct Tag {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(string, tag = "2")]
    pub value: String,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct Ports {
    #[prost(message, repeated, tag = "1")]
    pub ports: Vec<Port>,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct Port {
    #[prost(uint32, tag = "1")]
    pub port: u32,
    #[prost(bool, tag = "2")]
    pub unencrypted: bool,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct Sandbox {
    #[prost(string, repeated, tag = "1")]
    pub command: Vec<String>,
    #[prost(string, tag = "3")]
    pub image: String,
    #[prost(string, repeated, tag = "4")]
    pub secrets: Vec<String>,
    #[prost(message, optional, tag = "5")]
    pub resources: Option<Resources>,
    #[prost(uint32, tag = "7")]
    pub ttl: u32,
    #[prost(message, optional, tag = "20")]
    pub ports: Option<Ports>,
    #[prost(string, optional, tag = "28")]
    pub runtime: Option<String>,
    #[prost(string, optional, tag = "30")]
    pub name: Option<String>,
    #[prost(string, repeated, tag = "39")]
    pub inbound_cidrs: Vec<String>,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct Create {
    #[prost(string, tag = "1")]
    pub app_id: String,
    #[prost(message, optional, tag = "2")]
    pub definition: Option<Sandbox>,
    #[prost(message, repeated, tag = "4")]
    pub tags: Vec<Tag>,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct List {
    #[prost(string, tag = "1")]
    pub app_id: String,
    #[prost(double, tag = "2")]
    pub before: f64,
    #[prost(string, tag = "3")]
    pub environment: String,
    #[prost(bool, tag = "4")]
    pub finished: bool,
    #[prost(message, repeated, tag = "5")]
    pub tags: Vec<Tag>,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct ListResponse {
    #[prost(message, repeated, tag = "1")]
    pub resources: Vec<Info>,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct Task {
    #[prost(double, tag = "2")]
    pub started: f64,
    #[prost(message, optional, tag = "4")]
    pub result: Option<ResultInfo>,
    #[prost(string, tag = "6")]
    pub gpu_type: String,
    #[prost(message, optional, tag = "9")]
    pub gpu: Option<Gpu>,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct Tunnel {
    #[prost(string, tag = "1")]
    pub host: String,
    #[prost(uint32, tag = "2")]
    pub port: u32,
    #[prost(uint32, tag = "5")]
    pub container_port: u32,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct Info {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(double, tag = "3")]
    pub created: f64,
    #[prost(message, optional, tag = "4")]
    pub task: Option<Task>,
    #[prost(message, repeated, tag = "6")]
    pub tags: Vec<Tag>,
    #[prost(string, repeated, tag = "10")]
    pub regions: Vec<String>,
    #[prost(uint32, tag = "11")]
    pub ttl: u32,
    #[prost(double, optional, tag = "13")]
    pub ready: Option<f64>,
    #[prost(message, repeated, tag = "15")]
    pub tunnels: Vec<Tunnel>,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct Poll {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(float, tag = "2")]
    pub timeout: f32,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct PollResponse {
    #[prost(message, optional, tag = "1")]
    pub result: Option<ResultInfo>,
}
#[derive(Clone, PartialEq, Message)]
pub(super) struct Tunnels {
    #[prost(message, optional, tag = "1")]
    pub result: Option<ResultInfo>,
    #[prost(message, repeated, tag = "2")]
    pub tunnels: Vec<Tunnel>,
}
