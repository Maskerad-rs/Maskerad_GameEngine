use core::engine_support_systems::system_management::systems::filesystems::VFilesystem;
use core::engine_support_systems::data_structures::threadpools::systems::filesystem::filesystem_threadpool_messages::FilesystemMessage;

use std::sync::Arc;

//TODO: Problem with Send + Sync. See this -> https://users.rust-lang.org/t/sending-trait-objects-between-threads/2374
pub struct FilesystemWorkerMessage(pub FilesystemMessage, pub Arc<VFilesystem + Send + Sync>);