use std::io::Result;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

use core::engine_support_systems::system_management::{SystemType, System, Log};


//The logger create 2 files on start_up() : the general log file, and the logger dedicated log file.
//The logger check that the filesystem structure has been generated by the Resource subsystem.
pub struct Logger {
    paths: HashMap<SystemType, Box<Path>>,
    files: HashMap<SystemType, File>,
}

impl Logger {
    pub fn new(paths: HashMap<SystemType, Box<Path>>) -> Self {
        Logger {
            paths,
            files: HashMap::new(),
        }
    }

    pub fn add_path(&mut self, system_type: SystemType, the_path: Box<Path>) {
        self.paths.insert(system_type, the_path);
    }
}

impl Default for Logger {
    fn default() -> Logger {
        Logger {
            paths: HashMap::new(),
            files: HashMap::new(),
        }
    }
}

impl System for Logger {
    fn start_up(&mut self) {
        for (key, value) in self.paths.iter() {
            let file = File::create(value)
        }
    }

    fn shut_down(&mut self) {

    }
}

impl Log for Logger {
    fn write_to_general_log(&mut self) {

    }

    fn write_to_dedicated_log(&mut self, system_type: SystemType) {

    }
}