//! Prime Intellect availability offers and pod lifecycle.
//! <https://docs.primeintellect.ai/api-reference/check-gpu-availability>
//! Quotes choose the cheapest priced offer with the requested class/count/VRAM.
//! Credentials may be supplied by the caller or PRIME_INTELLECT_API_KEY.
//! PRIME_INTELLECT_REGION is the optional availability region filter.

mod adapter;
mod instance_mapper;

pub use adapter::PrimeIntellectAdapter;
pub use instance_mapper::PrimeIntellectInstanceMapper;
