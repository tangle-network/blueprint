//! Lambda Cloud native SKU inventory and managed instance lifecycle.
//! Caller credentials may select an existing SSH key and per-instance firewall rulesets.
//! API contract: <https://docs.lambda.ai/public-cloud/cloud-api/>.

mod adapter;

pub use adapter::LambdaLabsAdapter;
