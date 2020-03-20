pub mod data;
pub mod buffer;

mod viewport;
mod shader;
mod program;
mod color_buffer;
mod vertex_attrib_pointers;

pub use viewport::Viewport;
pub use shader::{Shader, Error as ShaderError};
pub use program::{Program, Error as ProgramError};
pub use color_buffer::ColorBuffer;
pub use vertex_attrib_pointers::VertexAttribPointers;

use std::ffi::{CString};

fn create_initialized_cstring(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}
