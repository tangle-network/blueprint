use crate::sources::ProcessHandle;
use blueprint_std::collections::HashMap;

pub type ActiveGadgets = HashMap<u64, HashMap<u64, ProcessHandle>>;
pub mod native;
