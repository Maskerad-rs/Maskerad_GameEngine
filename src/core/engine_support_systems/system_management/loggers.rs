use core::engine_support_systems::error_handling::error::GameResult;
use core::engine_support_systems::system_management::system_types::{SystemType, VSystem};

//TODO: Rewrite the Log trait

pub trait VLog : VSystem {
    fn write_to_dedicated_log(&mut self, subsystem_type: SystemType, message: &str) -> GameResult<()>; //The dedicated log file of a system.
}