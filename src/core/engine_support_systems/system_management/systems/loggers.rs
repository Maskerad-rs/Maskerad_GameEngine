use core::engine_support_systems::error_handling::error::GameResult;
use core::engine_support_systems::system_management::SystemType;
use std::fmt;

//TODO: Rewrite the Log trait

pub trait VLog : fmt::Debug {
    fn system_type(&self) -> SystemType {SystemType::Log}
    fn start_up(&self) -> GameResult<Box<VLog>>;
    fn shut_down(&self) -> GameResult<()>;
    fn write_to_dedicated_log(&self, subsystem_type: SystemType, message: &str) -> GameResult<()>; //The dedicated log file of a system.
    fn write_to_main_log(&self, message: &str) -> GameResult<()>;
}