use crate::data::{matrix_data, vector_data};
use crate::data::matrix_data::{AsColSlices, GlMat};
use std::ffi::{CString};
use std::mem::size_of;
use std::slice::from_raw_parts;
use c_str_macro::c_str;
use crate::uniform_buffer::{UniformBlock};
use crate::Program;
use crate::data::vector_data::f32_f32_f32;
use failure::_core::marker::PhantomData;

pub struct TestStruct {
    pub data: vector_data::f32_f32_f32,
    pub other_data: vector_data::f32_f32_f32,
}

pub struct ShaderDefaultLayout {
    pub mvp: matrix_data::mat4,
    pub mv: matrix_data::mat4,
    pub test_arr: [vector_data::f32_f32_f32; 2],
    pub test_struct: TestStruct,
    pub test_struct_arr: [TestStruct; 3]
}

pub trait GPUVariant<'a> {
    type Variant;
}

impl<'a> GPUVariant<'a> for vector_data::f32_f32_f32 {
    type Variant = GPUBasic<'a, vector_data::f32_f32_f32>;
}

impl<'a, T: AsColSlices> GPUVariant<'a> for T {
    type Variant = GPUMatrix<'a, T>;
}

impl<'a> GPUVariant<'a> for ShaderDefaultLayout {
    type Variant = GPUShaderDefaultLayout<'a>;
}

impl<'a> GPUVariant<'a> for TestStruct {
    type Variant = GPUTestStruct<'a>;
}

pub struct GPUMatrix<'a, M: AsColSlices> {
    ub: &'a UniformBlock,
    offset: gl::types::GLint,
    stride: gl::types::GLint,
    _marker: PhantomData<M>
}

impl<'a, M: AsColSlices> GPUMatrix<'a, M> {
    fn from_name(program: &Program, name: &str, ub: &'a UniformBlock) -> Self {
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

    fn from_params(offset: gl::types::GLint, stride: gl::types::GLint, ub: &'a UniformBlock) -> Self {
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
pub struct GPUBasic<'a, T> {
    ub: &'a UniformBlock,
    offset: gl::types::GLint,
    _marker: PhantomData<T>
}

impl<'a, T> GPUBasic<'a, T> {
    fn from_name(program: &Program, name: &str, ub: &'a UniformBlock) -> Self {
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

    fn from_params(offset: gl::types::GLint, ub: &'a UniformBlock) -> Self {
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

pub trait GPUAggregate<'a> {
    type Input;
    fn from_name(program: &Program, name: &str, ub: &'a UniformBlock) -> Self;

    fn set(&self, data: &Self::Input);
}

/// Struct containing the necessary handles to manage an array of basic types (excluding matrices) on the GPU with T
/// being a type on the CPU side that implements the GPUVariant trait.
pub struct GPUBasicArray<'a, T>
    where T: GPUVariant<'a, Variant = GPUBasic<'a, T>>
{
    ub: &'a UniformBlock,
    pub elems: Vec<T::Variant>,
    stride: gl::types::GLint,
    offset: gl::types::GLint
}

impl<'a, T> GPUBasicArray<'a, T>
    where T: GPUVariant<'a, Variant = GPUBasic<'a, T>>
{
    /// constructs the array from the name of the variable on the GPU by querying all the parameters at runtime.
    fn from_name(program: &Program, name: &str, len: usize, ub: &'a UniformBlock) -> Self {
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
            elems.push(GPUBasic::from_params(offset + stride * i as gl::types::GLint, ub))
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
pub struct GPUMatrixArray<'a, T>
    where T: GPUVariant<'a, Variant = GPUMatrix<'a, T>> + AsColSlices
{
    ub: &'a UniformBlock,
    pub elems: Vec<T::Variant>,
    stride: gl::types::GLint,
    offset: gl::types::GLint
}

impl<'a, T> GPUMatrixArray<'a, T>
    where T: GPUVariant<'a, Variant = GPUMatrix<'a, T>> + AsColSlices
{
    /// constructs the array from the name of the variable on the GPU by querying all the parameters at runtime.
    fn from_name(program: &Program, name: &str, len: usize, ub: &'a UniformBlock) -> Self {
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
            elems.push(GPUMatrix::from_params(offset + stride * i as i32, mat_stride, ub))
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

pub struct GPUAggregateArray<'a, T>
    where
        T: GPUVariant<'a>,
        T::Variant: GPUAggregate<'a>
{
    ub: &'a UniformBlock,
    pub elems: Vec<T::Variant>,
}

impl<'a, T> GPUAggregateArray<'a, T>
    where
        T: GPUVariant<'a>,
        T::Variant: GPUAggregate<'a>
{
    fn from_name(program: &Program, name: &str, len: usize, ub: &'a UniformBlock) -> Self {
        let mut elems = Vec::with_capacity(len);
        for i in 0..len {
            elems.push(T::Variant::from_name(program, &format!("{}[{}]", name, i), ub))
        }

        GPUAggregateArray {
            ub,
            elems,
        }
    }

    fn set(&self, data: &[<<T as GPUVariant<'a>>::Variant as GPUAggregate<'a>>::Input]) {
        for (i, elem) in data.iter().enumerate() {
            self.elems[i].set(elem);
        }
    }
}

pub struct GPUTestStruct<'a> {
    pub data: <vector_data::f32_f32_f32 as GPUVariant<'a>>::Variant,
    pub other_data: <vector_data::f32_f32_f32 as GPUVariant<'a>>::Variant,
}

impl<'a> GPUAggregate<'a> for GPUTestStruct<'a> {
    type Input = TestStruct;

    fn from_name(program: &Program, name: &str, ub: &'a UniformBlock) -> Self {
        GPUTestStruct {
            data: <vector_data::f32_f32_f32 as GPUVariant<'a>>::Variant::from_name(program, &format!("{}{}", name, ".data"), ub),
            other_data: <vector_data::f32_f32_f32 as GPUVariant<'a>>::Variant::from_name(program, &format!("{}{}", name, ".other_data"), ub)
        }
    }

    fn set(&self, data: &TestStruct) {
        self.data.set(&data.data);
        self.other_data.set(&data.other_data);
    }
}

pub struct GPUShaderDefaultLayout<'a> {
    pub mvp: <matrix_data::mat4 as GPUVariant<'a>>::Variant,
    pub mv: <matrix_data::mat4 as GPUVariant<'a>>::Variant,
    pub test_arr: GPUBasicArray<'a, vector_data::f32_f32_f32>,
    pub test_struct: <TestStruct as GPUVariant<'a>>::Variant,
    pub test_struct_arr: GPUAggregateArray<'a, TestStruct>
}

impl<'a> GPUAggregate<'a> for GPUShaderDefaultLayout<'a> {
    type Input = ShaderDefaultLayout;

    fn from_name(program: &Program, name: &str, ub: &'a UniformBlock) -> Self {
        GPUShaderDefaultLayout {
            mvp: <matrix_data::mat4 as GPUVariant<'a>>::Variant::from_name(program, &format!("{}{}", name, ".mvp"), ub),
            mv: <matrix_data::mat4 as GPUVariant<'a>>::Variant::from_name(program, &format!("{}{}", name, ".mv"), ub),
            test_arr: GPUBasicArray::<'a, vector_data::f32_f32_f32>::from_name(program, &format!("{}{}", name, ".test_arr"), 2, ub),
            test_struct: <TestStruct as GPUVariant<'a>>::Variant::from_name(program, &format!("{}{}", name, ".test_struct"), ub),
            test_struct_arr: GPUAggregateArray::<'a, TestStruct>::from_name(program, &format!("{}{}", name, ".test_struct_arr"), 3, ub)
        }
    }

    fn set(&self, data: &ShaderDefaultLayout) {
        self.mvp.set(&data.mvp);
        self.mv.set(&data.mv);
        self.test_arr.set(&data.test_arr);
        self.test_struct.set(&data.test_struct);
        self.test_struct_arr.set(&data.test_struct_arr);
    }
}