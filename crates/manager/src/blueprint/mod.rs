use crate::rt::service::Service;
use blueprint_std::collections::HashMap;

pub type ActiveBlueprints = HashMap<u64, HashMap<u64, Service>>;
pub mod native;
