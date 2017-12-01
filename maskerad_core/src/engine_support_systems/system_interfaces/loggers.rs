// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use engine_support_systems::error_handling::error::GameResult;
use engine_support_systems::system_interfaces::SystemType;
use std::fmt;

//TODO: Rewrite the Log trait

pub trait VLog : fmt::Debug {
    fn system_type(&self) -> SystemType {SystemType::Log}
    fn start_up(&self) -> GameResult<Box<VLog>>;
    fn shut_down(&self) -> GameResult<()>;
    fn write_to_dedicated_log(&self, subsystem_type: SystemType, message: &str) -> GameResult<()>; //The dedicated log file of a system.
    fn write_to_main_log(&self, message: &str) -> GameResult<()>;
}