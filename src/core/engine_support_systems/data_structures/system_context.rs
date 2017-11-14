//It's a HashMap<SubsystemType, Subsystem>
//We should try to use closures to use those systems
use std::collections::HashMap;

use core::engine_support_systems::system_management::system_types::{SystemType, VSystem, VSystemBuilder};
use core::engine_support_systems::system_management::systems::loggers::VLog;
use core::engine_support_systems::system_management::systems::filesystems::{VFile, VMetadata, VFilesystem, OpenOptions};
use core::engine_support_systems::error_handling::error::{GameResult, GameError};

use core::engine_support_systems::system_management::system_builders::logger_builders::VLoggerBuilder;
use core::engine_support_systems::system_management::system_builders::filesystem_builders::VFilesystemBuilder;

use systems::filesystem::{Metadata, Filesystem, FilesystemBuilder};
use systems::log::{LoggerBuilder, Logger};

pub struct SystemContextBuilder {
    file_system: Option<Box<VFilesystemBuilder>>,
    log_system: Option<Box<VLoggerBuilder>>,
}

impl SystemContextBuilder {
    pub fn new() -> Self {
        SystemContextBuilder {
            file_system: None,
            log_system: None,
        }
    }

    pub fn with_file_system(mut self, file_system_builder: Box<VFilesystemBuilder>) -> Self {
        self.file_system = Some(file_system_builder);
        self
    }

    pub fn with_log_system(mut self, log_system_builder: Box<VLoggerBuilder>) -> Self {
        self.log_system = Some(log_system_builder);
        self
    }

    pub fn start_up_systems(&self) -> GameResult<SystemContext> {

        let mut system_context = SystemContext {
            file_system: None,
            log_system: None,
        };

        //WARNING: There's an order of initialization !

        //First : the filesystem
        //Second : the logger
        system_context.add_file_system(match self.file_system {
            Some(ref file_system) => {
                file_system.start_up(&system_context)?
            },
            None => {
                return Err(GameError::ContextError(format!("The system context builder did not have any file system attached to it !")));
            }
        });

        system_context.add_log_system(match self.log_system {
            Some(ref log_system) => {
                log_system.start_up(&system_context)?
            },
            None => {
                return Err(GameError::ContextError(format!("The system context builder did not have any log system attached to it !")));
            }
        });

        Ok(system_context)
    }
}

pub struct SystemContext {
    file_system: Option<Box<VFilesystem>>,
    log_system: Option<Box<VLog>>,
    //...
}

impl SystemContext {

    fn new() -> Self {
        SystemContext {
            file_system: None,
            log_system: None,
        }
    }
    fn add_file_system(&mut self, file_system: Box<VFilesystem>) {
        self.file_system = Some(file_system);
    }

    fn add_log_system(&mut self, log_system: Box<VLog>) {
        self.log_system = Some(log_system);
    }


    pub fn use_log_system(&self) -> GameResult<&Box<VLog>> {
        match self.log_system {
            Some(ref log_system) => Ok(&log_system),
            None => {
                Err(GameError::ContextError(format!("The context doesn't have any log system !")))
            },
        }
    }


    pub fn use_file_system(&self) -> GameResult<&Box<VFilesystem>> {
        match self.file_system {
            Some(ref file_system) => Ok(&file_system),
            None => {
                Err(GameError::ContextError(format!("The context doesn't have any file system !")))
            },
        }
    }

    //TODO: use_rendering_input_physic_audio_resource_system...
}