// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//We have the trait : Subsystem.
//We have the traits : Audio, Log, Rendering... bound to the Subsystem trait.
//We have SubsystemType : An enum indicating the subsystem_type.
//We have (in the Systems_layer), the actual systems implementing the traits.
//We have a subsystem_context: an hashmap owing the subsystems, passed as argument to the KindredEngine struct

//Subsystem trait : start_up(), shut_down() and builder pattern.
//We can try to use closures to use subsystems in the subsystem_locator.
pub mod filesystems;
pub mod loggers;

use engine_support_systems::error_handling::error::GameResult;

use std::fmt::Display;
use std::fmt;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum SystemType {
    Audio,
    Rendering,
    Physic,
    Log,
    Resource,
    Input,
    Filesystem,
}

impl Display for SystemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &SystemType::Audio => {
                write!(f, "Audio system")
            },
            &SystemType::Log => {
                write!(f, "Log system")
            },
            &SystemType::Rendering => {
                write!(f, "Rendering system")
            },
            &SystemType::Resource => {
                write!(f, "Resource system")
            },
            &SystemType::Input => {
                write!(f, "Input system")
            },
            &SystemType::Physic => {
                write!(f, "Physic system")
            },
            &SystemType::Filesystem => {
                write!(f, "File system")
            }
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum PlatformType {
    Linux,
}

//TODO: Send and Sync are unsafe traits, we must be sure our systems, traits and trait objects work as expected.
pub trait System : fmt::Debug + Send + Sync {
    fn system_type(&self) -> SystemType;
    fn platform(&self) -> PlatformType;
    fn shut_down(&self) -> GameResult<()>;
}