use crate::data::{matrix_data, vector_data};
use std140::*;
use crate::uniform_buffer::{IntoLayoutStd140, UniformLayout, UniformLayoutStd140};

#[repr_std140]
pub struct GPUTestStruct {
    data: vec3,
    other_data: vec3
}

#[repr_std140]
pub struct GPULayout {
    mvp: mat4x4,
    mv: mat4x4,
    test_arr: array<vec3, 2>,
    test_struct: GPUTestStruct
}

impl UniformLayout for GPULayout {}
impl UniformLayoutStd140 for GPULayout {}

pub struct TestStruct {
    data: vector_data::f32_f32_f32,
    other_data: vector_data::f32_f32_f32
}

impl IntoLayoutStd140 for TestStruct {
    fn into_std140(self) -> GPUTestStruct {
        GPUTestStruct {
            data: vec3(self.data.d0, self.data.d1, self.data.d2),
            other_data: vec3(self.data.d0, self.data.d1, self.data.d2)
        }
    }
}

pub struct Std140DefaultLayout {
    mvp: matrix_data::mat4,
    mv: matrix_data::mat4,
    test_arr: [vector_data::f32_f32_f32; 2],
    test_struct: TestStruct,
}

impl IntoLayoutStd140 for Std140DefaultLayout {
    fn into_std140(self) -> GPULayout {
        GPULayout {
            mvp: mat4x4(
                vec4(self.mvp.m11, self.mvp.m21, self.mvp.m31, self.mvp.m41),
                vec4(self.mvp.m12, self.mvp.m22, self.mvp.m32, self.mvp.m42),
                vec4(self.mvp.m13, self.mvp.m23, self.mvp.m33, self.mvp.m43),
                vec4(self.mvp.m14, self.mvp.m24, self.mvp.m34, self.mvp.m44)
            ),
            mv: mat4x4(
                vec4(self.mv.m11, self.mv.m21, self.mv.m31, self.mv.m41),
                vec4(self.mv.m12, self.mv.m22, self.mv.m32, self.mv.m42),
                vec4(self.mv.m13, self.mv.m23, self.mv.m33, self.mv.m43),
                vec4(self.mv.m14, self.mv.m24, self.mv.m34, self.mv.m44)
            ),
            test_arr: array![
                vec3(self.test_arr[0].d0, self.test_arr[0].d1, self.test_arr[0].d2),
                vec3(self.test_arr[1].d0, self.test_arr[1].d1, self.test_arr[1].d2)
            ],
            test_struct: self.test_struct.into_std140()
        }
    }
}