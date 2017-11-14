use core::engine_support_systems::error_handling::error::GameResult;
use core::engine_support_systems::system_management::system_types::{SystemType, VSystem};
use core::engine_support_systems::data_structures::system_context::{SystemContext};

//TODO: Rewrite the Log trait

pub trait VLog : VSystem {
    fn write_to_dedicated_log(&self, context: &SystemContext, subsystem_type: SystemType, message: &str) -> GameResult<()>; //The dedicated log file of a system.
    fn write_to_main_log(&self, context: &SystemContext, message: &str) -> GameResult<()>;
}