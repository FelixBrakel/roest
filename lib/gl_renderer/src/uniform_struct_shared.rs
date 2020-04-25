use crate::data::{matrix_data, vector_data};
use crate::data::matrix_data::AsColSlices;
use std::ffi::{CString};
use std::mem::size_of;
use std::slice::from_raw_parts;
use c_str_macro::c_str;
use crate::uniform_buffer::{UniformLayoutShared, LayoutEnum, FieldType, UniformBlock};
use crate::Program;
use crate::data::vector_data::f32_f32_f32;
use crate::buffer::UniformBuffer;
use failure::_core::marker::PhantomData;

pub struct TestStruct {
    data: vector_data::f32_f32_f32,
    other_data: vector_data::f32_f32_f32,
}

pub struct ShaderDefaultLayout {
    mvp: matrix_data::mat4,
    mv: matrix_data::mat4,
    test_arr: [vector_data::f32_f32_f32; 2],
    test_struct: TestStruct,
    test_struct_arr: [TestStruct; 3]
}

pub trait GPUSettable {
    fn set<T>(&self, data: &T);

    fn buf<T>(&self, data: &T) -> &[u8];
}

struct GPUTestStruct<'a> {
    pub data: GPUBasic<'a, vector_data::f32_f32_f32>,
    pub other_data: GPUBasic<'a, vector_data::f32_f32_f32>
}

impl GPUSettable for GPUTestStruct {
    fn set(&self, data: &TestStruct) {
        self.data.set(&data.data);
        self.other_data.set(&data.other_data);
    }

    fn buf(&self, data: &TestStruct) -> &[u8] {
        unimplemented!("Aggregate types don't exist as a buf")
    }
}

struct GPUShaderDefaultLayout<'a> {
    pub mvp: GPUMatrix<'a, matrix_data::mat4>,
    pub mv: GPUMatrix<'a, matrix_data::mat4>,
    pub test_arr: GPUArray<'a, GPUBasic<'a, vector_data::f32_f32_f32>>,
    pub test_struct: GPUTestStruct<'a>,
    pub test_struct_arr: GPUArray<'a, GPUTestStruct<'a>>
}

impl GPUSettable for GPUShaderDefaultLayout {
    fn set(&self, data: &ShaderDefaultLayout) {
        self.mvp.set(&data.mvp);
        self.mv.set(&data.mv);
        self.test_arr.set(&data.test_arr);
        self.test_struct.set(&data.test_struct);
        self.test_struct_arr.set(&data.test_struct_arr);
    }

    fn buf(&self, data: &ShaderDefaultLayout) -> &[u8] {
        unimplemented!("Aggregate types don't exist as a buf")
    }
}

struct GPUArray<'a, S: GPUSettable> {
    ub: &'a UniformBlock<ShaderDefaultLayout>,
    pub elems: Vec<S>,
    stride: gl::types::GLint,
    offset: gl::types::GLint
}

impl<S: GPUSettable> GPUSettable for GPUArray<S> {
    fn set<T>(&self, data: &Vec<T>) {
        self.ub.set_subset(self.buf(data), self.offset as usize);
    }

    fn buf<T>(&self, data: &Vec<T>) -> &[u8] {
        let mut buf = Vec::new();
        for (i, elem) in data.iter().enumerate() {
            buf.extend_from_slice(self.elems[i].buf(elem));
        }

        &buf
    }
}

struct GPUMatrix<'a, M: AsColSlices> {
    ub: &'a UniformBlock<ShaderDefaultLayout>,
    offset: gl::types::GLint,
    stride: gl::types::GLint,
    _marker: PhantomData<M>
}

impl<M: AsColSlices> GPUSettable for GPUMatrix<M> {
    fn set<M>(&self, data: &M) {
        self.ub.set_subset(self.buf(data), self.offset as usize);
    }

    fn buf<M>(&self, data: &M) -> &[u8] {
        let slices: &[&[u8]] = data.as_col_slices;
        let mut buf: Vec<u8> = Vec::with_capacity(slices.len() * slices[0].len() + slices.len() & self.stride);
        buf.extend_from_slice(slices[0]);

        for i in 1..(slices.len() - 1) {
            buf.resize(buf.len() + self.stride, 0);
            buf.extend_from_slice(slices[i]);
        }

        &buf
    }
}

struct GPUBasic<'a, T> {
    ub: &'a UniformBlock<ShaderDefaultLayout>,
    offset: gl::types::GLint,
    _marker: PhantomData<T>
}

impl<T> GPUSettable for GPUBasic<T> {
    fn set<T>(&self, data: &T) {
        self.ub.set_subset(self.buf(), self.offset as usize);
    }

    fn buf<T>(&self, data: &T) -> &[u8] {
        unsafe {
            from_raw_parts(&data as *const u8, size_of::<T>())
        }
    }
}

// pub enum ShaderDefaultLayoutEnum {
//     All(ShaderDefaultLayout),
//     MVP(matrix_data::mat4),
//     MV(matrix_data::mat4),
//     TestArr(usize, vector_data::f32_f32_f32),
//     TestStruct(TestStruct),
// }
//
// impl LayoutEnum for ShaderDefaultLayoutEnum {}
//
// impl UniformLayoutShared for ShaderDefaultLayout {
//     type LayoutElem = ShaderDefaultLayoutEnum;
//     // const STRUCT_NAME: &'static CStr = c_str!("Defaults");
//     // const NUM_FIELDS: usize = 2;
//     // const FIELD_NAMES: [&'static CStr; Self::NUM_FIELDS] = [c_str!("mvp"), c_str!("mv")];
//
//     fn struct_name() -> CString {
//         CString::new("Default").unwrap()
//     }
//
//     fn field_names() -> Vec<CString> {
//         Vec::from(&[
//             CString::new("mvp").unwrap(),
//             CString::new("mv").unwrap(),
//             CString::new("test_arr").unwrap()
//         ][..])
//     }
//         // CString::new("mvp").expect("Field name 'mvp' is not a valid CString"),
//         // CString::new("mv").expect("Field name 'mv' is not a valid CString")
//     // ];
//
//     fn get_field_types(program: &Program, indices: &[gl::types::GLuint]) -> Vec<FieldType> {
//         let mut field_types: Vec<FieldType> = Vec::with_capacity(3);
//         let mut stride = 0;
//
//         unsafe {
//             gl::GetActiveUniformsiv(
//                 program.get_id(),
//                 1 as gl::types::GLsizei,
//                 &indices[0] as *const gl::types::GLuint,
//                 gl::UNIFORM_MATRIX_STRIDE,
//                 &mut stride as *mut gl::types::GLint
//             )
//         }
//         field_types.push(FieldType::Matrix(stride));
//
//         unsafe {
//             gl::GetActiveUniformsiv(
//                 program.get_id(),
//                 1 as gl::types::GLsizei,
//                 &indices[1] as *const gl::types::GLuint,
//                 gl::UNIFORM_MATRIX_STRIDE,
//                 &mut stride as *mut gl::types::GLint
//             )
//         }
//         field_types.push(FieldType::Matrix(stride));
//
//         unsafe {
//             gl::GetActiveUniformsiv(
//                 program.get_id(),
//                 1 as gl::types::GLsizei,
//                 &indices[2] as *const gl::types::GLuint,
//                 gl::UNIFORM_ARRAY_STRIDE,
//                 &mut stride as *mut gl::types::GLint
//             )
//         }
//         field_types.push(FieldType::Array(stride, FieldType::Primitive));
//
//         field_types
//     }
//
//     fn match_elem(elem: &ShaderDefaultLayoutEnum) -> (usize, &[&[u8]]) {
//         match elem {
//             ShaderDefaultLayoutEnum::All(s) {
//                 (
//                     0,
//                     s.data_slices()[0]
//                 )
//             },
//             ShaderDefaultLayoutEnum::MVP(mat) => {
//                 // self.mvp = mat;
//                 let mvp_col1 = mat.columns(0, 1).as_slice();
//                 let mvp_col2 = mat.columns(1, 1).as_slice();
//                 let mvp_col3 = mat.columns(2, 1).as_slice();
//                 let mvp_col4 = mat.columns(3, 1).as_slice();
//                 (
//                     0,
//                     unsafe {
//                         &[
//                             from_raw_parts(mvp_col1.as_ptr() as *const u8, mvp_col1.len() * size_of::<f32>()),
//                             from_raw_parts(mvp_col2.as_ptr() as *const u8, mvp_col2.len() * size_of::<f32>()),
//                             from_raw_parts(mvp_col3.as_ptr() as *const u8, mvp_col3.len() * size_of::<f32>()),
//                             from_raw_parts(mvp_col4.as_ptr() as *const u8, mvp_col4.len() * size_of::<f32>()),
//                         ]
//                     }
//                 )
//             },
//             ShaderDefaultLayoutEnum::MV(mat) => {
//                 // self.mv = mat;
//                 let mv_col1 = mat.columns(0, 1).as_slice();
//                 let mv_col2 = mat.columns(1, 1).as_slice();
//                 let mv_col3 = mat.columns(2, 1).as_slice();
//                 let mv_col4 = mat.columns(3, 1).as_slice();
//
//                 (
//                     1,
//                     unsafe {
//                         &[
//                             from_raw_parts(mv_col1.as_ptr() as *const u8, mv_col1.len() * size_of::<f32>()),
//                             from_raw_parts(mv_col2.as_ptr() as *const u8, mv_col2.len() * size_of::<f32>()),
//                             from_raw_parts(mv_col3.as_ptr() as *const u8, mv_col3.len() * size_of::<f32>()),
//                             from_raw_parts(mv_col4.as_ptr() as *const u8, mv_col4.len() * size_of::<f32>()),
//                         ]
//                     }
//                 )
//             },
//             ShaderDefaultLayoutEnum::TestArr(idx, vec) => {
//                 let test_arr_slice = vec;
//                 (
//                     2,
//                     unsafe {
//                         &[
//                             from_raw_parts(&test_arr_slice[0] as *const u8, size_of::<f32_f32_f32>()),
//                             from_raw_parts(&test_arr_slice[1] as *const u8, size_of::<f32_f32_f32>())
//                         ]
//                         // from_raw_parts(test_arr_slice.as_ptr() as *const u8, test_arr_slice.len() * size_of::<f32_f32_f32>())
//                     }
//                 )
//             },
//             ShaderDefaultLayoutEnum::TestStruct(test_struct) => {
//                 test_struct.data_slices()
//             }
//         }
//     }
//
//     fn set_elem(&mut self, elem: ShaderDefaultLayoutEnum) {
//         match elem {
//             ShaderDefaultLayoutEnum::MVP(mat) => self.mvp = mat,
//             ShaderDefaultLayoutEnum::MV(mat) => self.mv = mat,
//             ShaderDefaultLayoutEnum::TestArr(arr) => self.test_arr = arr,
//             ShaderDefaultLayoutEnum::TestStruct(test_struct) => self.test_struct = test_struct,
//         }
//     }
//
//     fn data_slices(&self) -> Vec<&[&[u8]]> {
//         let num_fields = Self::field_names().len();
//         let mut buf: Vec<&[&[u8]]> = Vec::with_capacity(num_fields);
//         let mv_slice = self.mv.as_slice();
//
//         let mvp_col1 = self.mvp.columns(0, 1).as_slice();
//         let mvp_col2 = self.mvp.columns(1, 1).as_slice();
//         let mvp_col3 = self.mvp.columns(2, 1).as_slice();
//         let mvp_col4 = self.mvp.columns(3, 1).as_slice();
//         buf.push(unsafe {
//             &[
//                 from_raw_parts(mvp_col1.as_ptr() as *const u8, mvp_col1.len() * size_of::<f32>()),
//                 from_raw_parts(mvp_col2.as_ptr() as *const u8, mvp_col2.len() * size_of::<f32>()),
//                 from_raw_parts(mvp_col3.as_ptr() as *const u8, mvp_col3.len() * size_of::<f32>()),
//                 from_raw_parts(mvp_col4.as_ptr() as *const u8, mvp_col4.len() * size_of::<f32>()),
//             ]
//         });
//
//         let mv_col1 = mat.columns(0, 1).as_slice();
//         let mv_col2 = mat.columns(1, 1).as_slice();
//         let mv_col3 = mat.columns(2, 1).as_slice();
//         let mv_col4 = mat.columns(3, 1).as_slice();
//         buf.push(unsafe {
//             &[
//                 from_raw_parts(mv_col1.as_ptr() as *const u8, mv_col1.len() * size_of::<f32>()),
//                 from_raw_parts(mv_col2.as_ptr() as *const u8, mv_col2.len() * size_of::<f32>()),
//                 from_raw_parts(mv_col3.as_ptr() as *const u8, mv_col3.len() * size_of::<f32>()),
//                 from_raw_parts(mv_col4.as_ptr() as *const u8, mv_col4.len() * size_of::<f32>()),
//             ]
//         });
//
//         buf.push(unsafe{
//             &[
//                 from_raw_parts(&self.test_arr[0] as *const u8, size_of::<f32_f32_f32>()),
//                 from_raw_parts(&self.test_arr[1] as *const u8, size_of::<f32_f32_f32>())
//             ]
//         });
//
//         buf
//     }
// }