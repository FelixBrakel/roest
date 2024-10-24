pub mod data;
pub mod buffer;
pub mod vertex;
pub mod buffered_uniform_struct_shared;
pub mod uniform_buffer;
pub mod light;
pub mod texture;

mod viewport;
mod shader;
mod program;
mod indexed_vert_array;
mod color_buffer;

pub use viewport::Viewport;
pub use shader::{Shader, Error as ShaderError};
pub use program::{Program, Error as ProgramError, GlUniform};
pub use indexed_vert_array::{IndexedVertArray};
pub use color_buffer::ColorBuffer;
pub use renderer_derive::{VertexAttribPointers, gl_getters, gl_setters, GPUVariant};
pub use buffered_uniform_struct_shared::{GPUVariant, GPUAggregate, GPUAggregateArray};

use std::ffi::{CString};

fn create_initialized_cstring(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

pub trait VertexAttribPointers {
    fn vertex_attrib_pointers();
}
