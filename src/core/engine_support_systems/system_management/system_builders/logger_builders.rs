use core::engine_support_systems::system_management::systems::loggers::VLog;
use core::engine_support_systems::data_structures::system_context::SystemContext;
use core::engine_support_systems::error_handling::error::GameResult;
use core::engine_support_systems::system_management::system_types::VSystemBuilder;

use std::fmt;

pub trait VLoggerBuilder : VSystemBuilder {
    fn start_up(&self, context: &SystemContext) -> GameResult<Box<VLog>>;
}