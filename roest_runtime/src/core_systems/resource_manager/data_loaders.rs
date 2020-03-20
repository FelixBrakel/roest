mod shader_loader;
mod program_loader;

pub use shader_loader::{ShaderLoader, Error as ShLoaderError};
pub use program_loader::{ProgramLoader, Error as PrLoaderError};
