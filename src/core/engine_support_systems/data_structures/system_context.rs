//It's a HashMap<SubsystemType, Subsystem>
//We should try to use closures to use those systems

use core::engine_support_systems::system_management::systems::loggers::VLog;
use core::engine_support_systems::system_management::systems::filesystems::VFilesystem;
use core::engine_support_systems::error_handling::error::{GameResult, GameError};


//The 'Game' struct (the main one), when created, will create the various
//systems needed. If system initialization successful, then the SystemContext
//globally accessible struct will take ownership of this system as Box<>, Rc<>, RefCell<>
//or Rc<RefCell<>>, i don't know.
lazy_static! {
    static ref CONTEXT: SystemContext = {
        let mut system_context = SystemContext::new();
        system_context
    };
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