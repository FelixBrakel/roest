use crate::data::matrix_data::{AsColSlices};
use std::ffi::{CString};
use std::mem::size_of;
use std::slice::from_raw_parts;
use crate::uniform_buffer::{UniformBlock};
use crate::Program;
use failure::_core::marker::PhantomData;
use std::sync::Arc;
use crate::texture::{Texture, TextureType, ResidentBindlessTexture};

pub trait GPUVariant {
    type Variant;
    type ArrayVariant;
}

impl<T: AsColSlices> GPUVariant for T {
    type Variant = GPUMatrix<T>;
    type ArrayVariant = GPUMatrixArray<T>;
}

pub struct GPUMatrix<M: AsColSlices> {
    ub: Arc<UniformBlock>,
    offset: gl::types::GLint,
    stride: gl::types::GLint,
    _marker: PhantomData<M>
}

impl<M: AsColSlices> GPUMatrix<M> {
    pub fn from_name(program: &Program, name: &str, ub: Arc<UniformBlock>) -> Self {
        let idx = unsafe {
            UniformBlock::get_elem_indices(program, &[CString::new(name).unwrap()])[0]
        };

        let offset = unsafe {
            UniformBlock::get_elem_offsets(program, &[idx])[0]
        };

        let stride = unsafe {
            UniformBlock::get_elem_matrix_strides(program, &[idx])[0]
        };

        GPUMatrix {
            ub,
            offset,
            stride,
            _marker: PhantomData
        }
    }

    pub fn from_params(offset: gl::types::GLint, stride: gl::types::GLint, ub: Arc<UniformBlock>) -> Self {
        GPUMatrix {
            ub,
            offset,
            stride,
            _marker: PhantomData
        }
    }

    pub fn set(&self, data: &M) {
        self.ub.set_subset(&self.buf(data), self.offset as usize);
    }

    pub fn buf(&self, data: &M) -> Vec<u8> {
        let slices = data.as_col_slices();
        let mut buf: Vec<u8> = Vec::with_capacity(slices.len() * self.stride as usize);
        buf.extend_from_slice(slices[0]);
        let stride = self.stride as usize - slices[0].len();

        for i in 1..slices.len() {
            buf.resize(buf.len() + stride, 0);
            buf.extend_from_slice(slices[i]);
        }

        buf
    }
}

//NOTE: The generic parameter T here is the CPU Variant
pub struct GPUBasic<T> {
    ub: Arc<UniformBlock>,
    offset: gl::types::GLint,
    _marker: PhantomData<T>
}

impl<T> GPUBasic<T> {
    pub fn from_name(program: &Program, name: &str, ub: Arc<UniformBlock>) -> Self {
        let idx = unsafe {
            UniformBlock::get_elem_indices(program, &[CString::new(name).unwrap()])[0]
        };

        let offset = unsafe {
            UniformBlock::get_elem_offsets(program, &[idx])[0]
        };

        GPUBasic {
            ub,
            offset,
            _marker: PhantomData
        }
    }

    pub fn from_params(offset: gl::types::GLint, ub: Arc<UniformBlock>) -> Self {
        GPUBasic {
            ub,
            offset,
            _marker: PhantomData
        }
    }

    pub fn set(&self, data: &T) {
        self.ub.set_subset(self.buf(data), self.offset as usize);
    }

    pub fn buf(&self, data: &T) -> &[u8] {
        unsafe {
            let tmp: *const T = data;
            from_raw_parts(tmp as *const _, size_of::<T>())
        }
    }
}

pub trait GPUAggregate {
    type Input;
    fn from_name(program: &Program, name: &str, ub: Arc<UniformBlock>) -> Self;

    fn set(&self, data: &Self::Input);
}

/// Struct containing the necessary handles to manage an array of basic types (excluding matrices) on the GPU with T
/// being a type on the CPU side that implements the GPUVariant trait.
pub struct GPUBasicArray<T>
    where T: GPUVariant<Variant = GPUBasic<T>>
{
    ub: Arc<UniformBlock>,
    pub elems: Vec<T::Variant>,
    stride: gl::types::GLint,
    offset: gl::types::GLint
}

impl<T> GPUBasicArray<T>
    where T: GPUVariant<Variant = GPUBasic<T>>
{
    /// constructs the array from the name of the variable on the GPU by querying all the parameters at runtime.
    pub fn from_name(program: &Program, name: &str, len: usize, ub: Arc<UniformBlock>) -> Self {
        let idx = unsafe {
            UniformBlock::get_elem_indices(program, &[CString::new(name).unwrap()])[0]
        };

        let offset = unsafe {
            UniformBlock::get_elem_offsets(program, &[idx])[0]
        };

        let stride = unsafe {
            UniformBlock::get_elem_array_strides(program, &[idx])[0]
        };

        let mut elems = Vec::with_capacity(len);
        for i in 0..len {
            elems.push(GPUBasic::from_params(offset + stride * i as gl::types::GLint, ub.clone()))
        }

        GPUBasicArray {
            ub,
            elems,
            stride,
            offset,
        }
    }

    /// Setter method for the entire array, calls buf so it doesn't have to make an API call for every element in the
    /// buffer separately
    pub fn set(&self, data: &[T]) {
        self.ub.set_subset(&self.buf(data), self.offset as usize);
    }

    /// Used to create a buffer with the correct stride which can then be uploaded to the GPU in one go.
    pub fn buf(&self, data: &[T]) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.stride as usize * self.elems.len());
        for (i, elem) in data.iter().enumerate() {
            let slice = self.elems[i].buf(elem);
            let slice_len = slice.len();
            buf.extend(slice);
            buf.resize(buf.len() + self.stride as usize - slice_len, 0);
        }

        buf
    }
}

/// Struct containing the necessary handles to manage an array of matrices on the GPU with T
/// being a matrix type on the CPU side that implements the GPUVariant trait as wel as the AsColSlices trait.
/// The reason this does not fall under GPUBasicArray even though a matrix is a basic type in GLSL is that it
/// needs some special logic to set up the buffer to account for matrix stride.,
pub struct GPUMatrixArray<T>
    where T: GPUVariant<Variant = GPUMatrix<T>> + AsColSlices
{
    ub: Arc<UniformBlock>,
    pub elems: Vec<T::Variant>,
    stride: gl::types::GLint,
    offset: gl::types::GLint
}

impl<T> GPUMatrixArray<T>
    where T: GPUVariant<Variant = GPUMatrix<T>> + AsColSlices
{
    /// constructs the array from the name of the variable on the GPU by querying all the parameters at runtime.
    pub fn from_name(program: &Program, name: &str, len: usize, ub: Arc<UniformBlock>) -> Self {
        let idx = unsafe {
            UniformBlock::get_elem_indices(program, &[CString::new(name).unwrap()])[0]
        };

        let offset = unsafe {
            UniformBlock::get_elem_offsets(program, &[idx])[0]
        };

        let stride = unsafe {
            UniformBlock::get_elem_array_strides(program, &[idx])[0]
        };

        let mat_stride = unsafe {
            UniformBlock::get_elem_matrix_strides(program, &[idx])[0]
        };


        let mut elems = Vec::with_capacity(len);
        for i in 0..len {
            elems.push(GPUMatrix::from_params(offset + stride * i as i32, mat_stride, ub.clone()))
        }

        GPUMatrixArray {
            ub,
            elems,
            stride,
            offset,
        }
    }
    /// Setter method for the entire array, calls buf so it doesn't have to make an API call for every element in the
    /// buffer separately
    pub fn set(&self, data: &[T]) {
        self.ub.set_subset(&self.buf(data), self.offset as usize);
    }

    /// Used to create a buffer with the correct stride which can then be uploaded to the GPU in one go.
    pub fn buf(&self, data: &[T]) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.stride as usize * self.elems.len());
        for (i, elem) in data.iter().enumerate() {
            let slice = self.elems[i].buf(elem);
            let slice_len = slice.len();
            buf.extend(slice);
            buf.resize(buf.len() + self.stride as usize - slice_len, 0);
        }

        buf
    }
}

pub struct GPUAggregateArray<T>
    where
        T: GPUVariant,
        T::Variant: GPUAggregate
{
    pub elems: Vec<T::Variant>,
}

impl<T> GPUAggregateArray<T>
    where
        T: GPUVariant,
        T::Variant: GPUAggregate
{
    pub fn from_name(program: &Program, name: &str, len: usize, ub: Arc<UniformBlock>) -> Self {
        let mut elems = Vec::with_capacity(len);
        for i in 0..len {
            elems.push(T::Variant::from_name(program, &format!("{}[{}]", name, i), ub.clone()))
        }

        GPUAggregateArray {
            elems,
        }
    }

    pub fn set(&self, data: &[<<T as GPUVariant>::Variant as GPUAggregate>::Input]) {
        for (i, elem) in data.iter().enumerate() {
            self.elems[i].set(elem);
        }
    }
}

pub struct GPUTexture {
    ub: Arc<UniformBlock>,
    offset: gl::types::GLint,
}

impl GPUTexture {
    pub fn from_name(program: &Program, name: &str, ub: Arc<UniformBlock>) -> Self {
        let idx = unsafe {
            UniformBlock::get_elem_indices(program, &[CString::new(name).unwrap()])[0]
        };

        let offset = unsafe {
            UniformBlock::get_elem_offsets(program, &[idx])[0]
        };

        GPUTexture {
            ub,
            offset,
        }
    }

    pub fn from_params(offset: gl::types::GLint, ub: Arc<UniformBlock>) -> Self {
        GPUTexture {
            ub,
            offset,
        }
    }

    pub fn set<T: TextureType>(&self, tex: &ResidentBindlessTexture<T>) {
        self.ub.set_subset(self.buf(tex), self.offset as usize);
    }

    pub fn buf<T: TextureType>(&self, tex: &ResidentBindlessTexture<T>) -> &[u8] {
        unsafe {
            let tmp: *const gl::types::GLuint64 = &tex.get_handle();
            from_raw_parts(tmp as *const u8, size_of::<gl::types::GLuint64>())
        }
    }
}

pub struct GPUTextureArray {
    ub: Arc<UniformBlock>,
    pub elems: Vec<GPUTexture>,
    stride: gl::types::GLint,
    offset: gl::types::GLint
}

impl GPUTextureArray {
    /// constructs the array from the name of the variable on the GPU by querying all the parameters at runtime.
    pub fn from_name(program: &Program, name: &str, len: usize, ub: Arc<UniformBlock>) -> Self {
        let idx = unsafe {
            UniformBlock::get_elem_indices(program, &[CString::new(name).unwrap()])[0]
        };

        let offset = unsafe {
            UniformBlock::get_elem_offsets(program, &[idx])[0]
        };

        let stride = unsafe {
            UniformBlock::get_elem_array_strides(program, &[idx])[0]
        };

        let mut elems = Vec::with_capacity(len);
        for i in 0..len {
            elems.push(GPUTexture::from_params(offset + stride * i as gl::types::GLint, ub.clone()))
        }

        GPUTextureArray {
            ub,
            elems,
            stride,
            offset,
        }
    }

    /// Setter method for the entire array, calls buf so it doesn't have to make an API call for every element in the
    /// buffer separately
    pub fn set<T>(&self, data: &[ResidentBindlessTexture<T>]) {
        self.ub.set_subset(&self.buf(data), self.offset as usize);
    }

    /// Used to create a buffer with the correct stride which can then be uploaded to the GPU in one go.
    pub fn buf<T>(&self, data: &[ResidentBindlessTexture<T>]) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.stride as usize * self.elems.len());
        for (i, tex) in data.iter().enumerate() {
            let slice = self.elems[i].buf(tex);
            let slice_len = slice.len();
            buf.extend(slice);
            buf.resize(buf.len() + self.stride as usize - slice_len, 0);
        }

        buf
    }
}

