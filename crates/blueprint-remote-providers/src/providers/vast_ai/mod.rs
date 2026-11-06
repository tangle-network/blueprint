//! Vast native GPU inventory, explicit bids and managed container recovery.
//! Live API: <https://docs.vast.ai/api-reference/search/search-offers>.
//! Workload environment strings and unsupported volume policies fail closed.

mod adapter;

pub use adapter::VastAiAdapter;
