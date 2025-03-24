use crate::contexts::aggregator::AggregatorContext;
use crate::contexts::x_square::EigenSquareContext;
use blueprint_sdk::macros::context::KeystoreContext;
use blueprint_sdk::runner::config::BlueprintEnvironment;

/// Combined context that includes both the EigenSquareContext and AggregatorContext
/// This allows both jobs to share the same context in the router
#[derive(Clone, KeystoreContext)]
pub struct CombinedContext {
    pub eigen_context: EigenSquareContext,
    pub aggregator_context: Option<AggregatorContext>,
    #[config]
    pub std_config: BlueprintEnvironment,
}

impl CombinedContext {
    pub fn new(
        eigen_context: EigenSquareContext,
        aggregator_context: Option<AggregatorContext>,
        std_config: BlueprintEnvironment,
    ) -> Self {
        Self {
            eigen_context,
            aggregator_context,
            std_config,
        }
    }
}
