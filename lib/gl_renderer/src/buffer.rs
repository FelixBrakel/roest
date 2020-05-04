use gl;
use std::mem;

pub type ArrayBuffer = Buffer<BufferTypeArray>;
pub type ElementArrayBuffer = Buffer<BufferTypeElementsArray>;
pub type UniformBuffer = Buffer<BufferTypeUniform>;

pub struct BufferTypeArray;
impl BufferType for BufferTypeArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ARRAY_BUFFER;
}

pub struct BufferTypeElementsArray;
impl BufferType for BufferTypeElementsArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ELEMENT_ARRAY_BUFFER;
}

pub struct BufferTypeUniform;
impl BufferType for BufferTypeUniform {
    const BUFFER_TYPE: gl::types::GLuint = gl::UNIFORM_BUFFER;
}

pub struct Buffer<B> where B: BufferType {
    vbo: gl::types::GLuint,
    _marker: ::std::marker::PhantomData<B>
}

pub trait BufferType {
    const BUFFER_TYPE: gl::types::GLuint;
}

impl<B> Buffer<B> where B: BufferType {
    pub fn new() -> Buffer<B> {
        let mut vbo: gl::types::GLuint = 0;

        unsafe {
            gl::GenBuffers(1, &mut vbo);
        };

        Buffer {
            vbo,
            _marker: ::std::marker::PhantomData,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(B::BUFFER_TYPE, self.vbo);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(B::BUFFER_TYPE, 0);
        }
    }

    pub fn bind_base(&self, index: gl::types::GLuint) {
        unsafe {
            gl::BindBufferBase(gl::UNIFORM_BUFFER, index, self.vbo);
        }
    }

    pub fn static_draw_data<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData(
                B::BUFFER_TYPE,
                (data.len() * mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }

    pub fn static_draw_data_struct<T>(&self, data: &T, len: usize) {
        let tmp = data as *const T;
        unsafe {
            gl::BufferData(
                B::BUFFER_TYPE,
                len as gl::types::GLsizeiptr,
                tmp as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }

    pub fn dynamic_draw_alloc(&self, size: usize) {
        unsafe {
            gl::BufferData(
                B::BUFFER_TYPE,
                size as gl::types::GLsizeiptr,
                std::ptr::null() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW
            );
        }
    }

    pub unsafe fn draw_subdata<T>(&self, data: &[T], offset: gl::types::GLintptr) {
        gl::BufferSubData(
            B::BUFFER_TYPE,
            offset,
            (data.len() * mem::size_of::<T>()) as gl::types::GLsizeiptr,
            data.as_ptr() as *const gl::types::GLvoid
        )
    }
}

impl<B> Drop for Buffer<B> where B: BufferType{
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.vbo)
        }
    }
}

pub struct VertexArray {
    vao: gl::types::GLuint,
}

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        VertexArray {
            vao
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.vao);
        }
    }
}