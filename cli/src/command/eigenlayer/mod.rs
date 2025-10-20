pub mod deregister;
pub mod list;
/// EigenLayer AVS registration management commands
pub mod register;
pub mod rewards;
pub mod sync;

pub use deregister::deregister_avs;
pub use list::list_avs_registrations;
pub use register::register_avs;
pub use rewards::{claim_rewards, set_claimer, show_rewards};
pub use sync::sync_avs_registrations;
