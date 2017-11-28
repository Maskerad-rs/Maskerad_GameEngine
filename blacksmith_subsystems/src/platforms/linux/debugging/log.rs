
/*

use std::path::PathBuf;
use std::collections::HashMap;

use core::engine_support_systems::system_interfaces::system_types::{VSystem, SystemType};
use core::engine_support_systems::system_interfaces::systems::loggers::VLog;

use core::engine_support_systems::error_handling::error::{GameResult, GameError};

//TODO: Rewrite Logger completely

#[derive(Debug)]
pub struct Logger {
    system_log_paths: HashMap<SystemType, PathBuf>,
    main_log_path: PathBuf,
}

impl VSystem for Logger {



    fn shut_down(&mut self) -> GameResult<()> {
        //TODO: finish this func
        self.write_to_main_log("The log system is shutting down")?;
        self.write_to_dedicated_log(SystemType::Log, "Shutting down...")?;
        Ok(())
    }

    fn system_type(&self) -> SystemType {
        SystemType::Log
    }
}

impl VLog for Logger {
    fn start_up(&self) -> GameResult<Box<Logger>> {
        let mut logger = Logger {
            system_log_paths: self.system_log_paths.clone(),
            main_log_path: self.main_log_path.clone(),
        };

        //Ask the filesystem to create the hierarchy needed
        for (system_type, system_pathbuf) in logger.system_log_paths.iter_mut() {
            /*
            context.use_file_system()?.mkdir(system_pathbuf.as_path())?;
            if !context.use_file_system()?.metadata(system_pathbuf.as_path())?.is_dir() {
                return Err(GameError::FileSystemError(format!("The path {:?} should be a directory, not a file ! The logger system takes care of creating the log files.", system_pathbuf.as_path())));
            }
            */

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
            logger.write_to_dedicated_log(*system_type, format!("The {} log has been created at path {:?}", system_type, system_path.as_path()).as_str())?;
        }

        logger.main_log_path.set_file_name("main.log");
        logger.write_to_main_log(format!("The main log has been created at path {:?}", logger.main_log_path.as_path()).as_str())?;
        //Success !
        logger.write_to_dedicated_log(SystemType::Log, format!("The logger has been successfully created !").as_str())?;
        Ok(Box::new(logger))
    }

    fn write_to_dedicated_log(&self, system_type: SystemType, message: &str) -> GameResult<()> {
        match self.system_log_paths.get(&system_type) {
            Some(path) => {
                //context.use_file_system()?.append(path.as_path())?.write_all(message.as_bytes())?;
            },
            None => {
                return Err(GameError::FileSystemError(format!("Could not find the {} log to write to it !", system_type)));
            },
        }

        Ok(())
    }

    fn write_to_main_log(&self, message: &str) -> GameResult<()> {
        //context.use_file_system()?.append(self.main_log_path.as_path())?.write_all(message.as_bytes())?;
        Ok(())
    }
}

*/