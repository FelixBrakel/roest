use std::path::{Path, PathBuf};


pub mod synchronous;

fn resource_name_to_path<P: AsRef<Path>>(&name: &P) -> PathBuf {
    return Resource::ROOT_PATH.join(name);
}