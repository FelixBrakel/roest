use std::path::{Path, PathBuf};
use core_systems::resource_manager::Resource;

pub mod synchronous;

struct FileSystem {
    root_path: AsRef<Path>
}

impl FileSystem {
    fn new() -> Self {

    }
}

pub fn initialize() -> FileSystem {
    FileSystem::
}
pub fn resource_name_to_path<P: Resource>(&name: &P) -> PathBuf {
    return .join(name.get_name());
}