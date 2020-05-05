use crate::buffer::{UniformBuffer};
use crate::{Program};
use std::ffi::CString;
use crate::uniform_struct_shared::{GPUAggregate, GPUVariant};
use std::os::raw::c_char;
use std::sync::Arc;

pub struct InterfaceBlock<U: GPUVariant> {
    // uniform_block: Arc<UniformBlock>,
    pub uniform_struct: U::Variant,
}

impl<U: GPUVariant> InterfaceBlock<U>
    where
        U: GPUVariant,
        <U as GPUVariant>::Variant: GPUAggregate
{
    pub fn new(program: &Program, name: &str,) -> Self {
        let ub = Arc::new(UniformBlock::new(program, name));
        let uniform_struct = U::Variant::from_name(program, name, ub.clone());
        InterfaceBlock {
            // uniform_block: ub,
            uniform_struct
        }
    }
}

pub struct UniformBlock {
    ubo: UniformBuffer,
}

impl UniformBlock {
    //TODO: make this method absolutely safe by checking if the program is safe at runtime.
    pub fn new(program: &Program, name: &str) -> Self {
        let ubo = UniformBuffer::new();
        let block_index = unsafe {
            Self::get_block_index(program, name)
        };

        let block_size = unsafe {
            Self::get_block_size(program, block_index)
        } as usize;

        ubo.bind();
        ubo.dynamic_draw_alloc(block_size);
        ubo.unbind();

        unsafe {
            gl::UniformBlockBinding(program.get_id(), block_index, 1);
        }

        ubo.bind_base(1);

        UniformBlock {
            ubo,
        }
    }

    unsafe fn get_block_index(program: &Program, name: &str) -> gl::types::GLuint {
        let name = CString::new(name).unwrap();
        let block_index = gl::GetUniformBlockIndex(
            program.get_id(),
            name.as_ptr() as *const gl::types::GLchar
        );

        block_index
    }
    unsafe fn get_block_size(program: &Program, index: gl::types::GLuint) -> gl::types::GLint {
        let mut blocksize: gl::types::GLint = 0;

        gl::GetActiveUniformBlockiv(
            program.get_id(),
            index,
            gl::UNIFORM_BLOCK_DATA_SIZE,
            &mut blocksize as *mut gl::types::GLint
        );

        blocksize
    }

    pub unsafe fn get_elem_indices(program: &Program, names: &[CString]) -> Vec<gl::types::GLuint> {
        let num_fields = names.len();
        let mut indices: Vec<gl::types::GLuint> = Vec::with_capacity(num_fields);
        indices.resize(num_fields, 0);
        // let mut indices: [gl::types::GLuint; Self::NUM_FIELDS] = [0; 2];
        let mut gl_names: Vec<*const c_char> = Vec::with_capacity(num_fields);
        for string in names {
            gl_names.push(string.as_ptr());
        }

        gl::GetUniformIndices(
            program.get_id(),
            num_fields as gl::types::GLsizei,
            gl_names.as_ptr() as *const *const c_char,
            indices.as_mut_ptr()
        );

        return indices;
    }

    pub unsafe fn get_elem_offsets(program: &Program, indices: &[gl::types::GLuint]) -> Vec<gl::types::GLint> {
        let num_fields = indices.len();
        let mut offsets: Vec<gl::types::GLint> = Vec::with_capacity(num_fields);
        offsets.resize(num_fields, 0);

        gl::GetActiveUniformsiv(
            program.get_id(),
            num_fields as gl::types::GLsizei,
            indices.as_ptr(),
            gl::UNIFORM_OFFSET,
            offsets.as_mut_ptr()
        );

        offsets
    }

    pub unsafe fn get_elem_matrix_strides(program: &Program, indices: &[gl::types::GLuint]) -> Vec<gl::types::GLint> {
        let num_fields = indices.len();
        let mut offsets: Vec<gl::types::GLint> = Vec::with_capacity(num_fields);
        offsets.resize(num_fields, 0);

        gl::GetActiveUniformsiv(
            program.get_id(),
            num_fields as gl::types::GLsizei,
            indices.as_ptr(),
            gl::UNIFORM_MATRIX_STRIDE,
            offsets.as_mut_ptr()
        );

        offsets
    }

    pub unsafe fn get_elem_array_strides(program: &Program, indices: &[gl::types::GLuint]) -> Vec<gl::types::GLint> {
        let num_fields = indices.len();
        let mut offsets: Vec<gl::types::GLint> = Vec::with_capacity(num_fields);
        offsets.resize(num_fields, 0);

        gl::GetActiveUniformsiv(
            program.get_id(),
            num_fields as gl::types::GLsizei,
            indices.as_ptr(),
            gl::UNIFORM_ARRAY_STRIDE,
            offsets.as_mut_ptr()
        );

        offsets
    }


    pub fn set_subset<T>(&self, data: &[T], offset: usize) {
        self.ubo.bind();
        unsafe {
            self.ubo.draw_subdata(data, offset as gl::types::GLintptr);
        }
        self.ubo.unbind();
    }
}
