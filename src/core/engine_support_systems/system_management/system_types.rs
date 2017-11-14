use std::fmt::Display;
use std::fmt;

use core::engine_support_systems::error_handling::error::GameResult;
use core::engine_support_systems::data_structures::system_context::{SystemContext};

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

pub trait VSystemBuilder : fmt::Debug {
    fn system_builder_type(&self) -> SystemType;
}

pub trait VSystem : fmt::Debug {
    fn shut_down(&mut self, context: &SystemContext) -> GameResult<()>;
    fn system_type(&self) -> SystemType;
}