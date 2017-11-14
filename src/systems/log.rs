use std::io::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use core::engine_support_systems::system_management::system_types::{VSystem, SystemType, VSystemBuilder};
use core::engine_support_systems::system_management::systems::loggers::VLog;
use core::engine_support_systems::system_management::system_builders::logger_builders::VLoggerBuilder;

use core::engine_support_systems::error_handling::error::{GameResult, GameError};

use core::engine_support_systems::data_structures::system_context::{SystemContext};
//TODO: Rewrite Logger completely

#[derive(Debug)]
pub struct LoggerBuilder {
    system_log_paths: HashMap<SystemType, PathBuf>,
    main_log_path: PathBuf,
}

impl VSystemBuilder for LoggerBuilder {
    fn system_builder_type(&self) -> SystemType {
        SystemType::Log
    }
}

impl VLoggerBuilder for LoggerBuilder {
    fn start_up(&self, context: &SystemContext) -> GameResult<Box<VLog>> {
        let mut logger = Logger {
            system_log_paths: self.system_log_paths,
            main_log_path: self.main_log_path,
        };

        //Ask the filesystem to create the hierarchy needed
        for (system_type, system_pathbuf) in logger.system_log_paths.iter_mut() {

            context.use_file_system()?.mkdir(system_pathbuf.as_path())?;
            if !context.use_file_system()?.metadata(system_pathbuf.as_path())?.is_dir() {
                return Err(GameError::FileSystemError(format!("The path {:?} should be a directory, not a file ! The logger system takes care of creating the log files.", system_pathbuf.as_path())));
            }

            match *system_type {
                SystemType::Physic => {
                    system_pathbuf.set_file_name("physic.log");
                },
                SystemType::Audio => {
                    system_pathbuf.set_file_name("audio.log");
                },
                SystemType::Rendering => {
                    system_pathbuf.set_file_name("rendering.log");
                },
                SystemType::Resource => {
                    system_pathbuf.set_file_name("resource.log");
                },
                SystemType::Input => {
                    system_pathbuf.set_file_name("input.log");
                },
                SystemType::Filesystem => {
                    system_pathbuf.set_file_name("filesystem.log");
                },
                SystemType::Log => {
                    system_pathbuf.set_file_name("logger.log");
                },
            }
        }

        for (system_type, system_path) in logger.system_log_paths.iter() {
            logger.write_to_dedicated_log(context, *system_type, format!("The {} log has been created at path {:?}", system_type, system_path.as_path()).as_str())?;
        }

        logger.main_log_path.set_file_name("main.log");
        logger.write_to_main_log(context, format!("The main log has been created at path {:?}", logger.main_log_path.as_path()).as_str())?;
        //Success !
        logger.write_to_dedicated_log(context,SystemType::Log, format!("The logger has been successfully created !").as_str())?;
        Ok(Box::new(logger))
    }
}

impl LoggerBuilder {
    pub fn new(main_log_path: &Path) -> LoggerBuilder {
        LoggerBuilder {
            system_log_paths: HashMap::new(),
            main_log_path: main_log_path.to_path_buf(),
        }
    }

    pub fn add_path_for_system_log(mut self, system: SystemType, path: &Path) -> LoggerBuilder {
        self.system_log_paths.insert(system, path.to_path_buf());
        self
    }
}


#[derive(Debug)]
pub struct Logger {
    system_log_paths: HashMap<SystemType, PathBuf>,
    main_log_path: PathBuf,
}

impl VSystem for Logger {

    fn shut_down(&mut self, context: &SystemContext) -> GameResult<()> {
        //TODO: finish this func
        self.write_to_main_log(context, "The log system is shutting down")?;
        self.write_to_dedicated_log(context, SystemType::Log, "Shutting down...")?;
        Ok(())
    }

    fn system_type(&self) -> SystemType {
        SystemType::Log
    }
}

impl VLog for Logger {
    fn write_to_dedicated_log(&self, context: &SystemContext, system_type: SystemType, message: &str) -> GameResult<()> {
        match self.system_log_paths.get(&system_type) {
            Some(path) => {
                context.use_file_system()?.append(path.as_path())?.write_all(message.as_bytes())?;
            },
            None => {
                return Err(GameError::FileSystemError(format!("Could not find the {} log to write to it !", system_type)));
            },
        }

        Ok(())
    }

    fn write_to_main_log(&self, context: &SystemContext, message: &str) -> GameResult<()> {
        context.use_file_system()?.append(self.main_log_path.as_path())?.write_all(message.as_bytes())?;
        Ok(())
    }
}