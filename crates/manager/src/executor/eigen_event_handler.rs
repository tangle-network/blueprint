use crate::error::Result;
use crate::executor::BlueprintEnvironment;
use crate::executor::BlueprintManagerContext;
use blueprint_core::info;
use alloy_rpc_types::Log;

pub(crate) async fn handle_eigen_event(
    event: &Log,
    _blueprint_config: &BlueprintEnvironment,
    _ctx: &BlueprintManagerContext,
) -> Result<()> {
    info!("Received notification {:?}", event);

    Ok(())
}