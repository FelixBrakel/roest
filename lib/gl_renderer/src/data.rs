mod i8_inner;
mod i8_float_inner;
mod i16_inner;
mod i16_float_inner;
mod i32_inner;
mod i32_float_inner;
mod u8_inner;
mod u8_float_inner;
mod u16_inner;
mod u16_float_inner;
mod u32_inner;
mod u32_float_inner;
mod f16_inner;
mod f32_inner;
mod f64_inner;
mod misc_inner;


pub use i8_inner::{i8_, i8_i8, i8_i8_i8, i8_i8_i8_i8};
pub use i8_float_inner::{i8_float, i8_i8_float, i8_i8_i8_float, i8_i8_i8_i8_float};
pub use i16_inner::{i16_, i16_i16, i16_i16_i16, i16_i16_i16_i16};
pub use i16_float_inner::{i16_float, i16_i16_float, i16_i16_i16_float, i16_i16_i16_i16_float};
pub use i32_inner::{i32_, i32_i32, i32_i32_i32, i32_i32_i32_i32};
pub use i32_float_inner::{i32_float, i32_i32_float, i32_i32_i32_float, i32_i32_i32_i32_float};
pub use u8_inner::{u8_, u8_u8, u8_u8_u8, u8_u8_u8_u8};
pub use u8_float_inner::{u8_float, u8_u8_float, u8_u8_u8_float, u8_u8_u8_u8_float};
pub use u16_inner::{u16_, u16_u16, u16_u16_u16, u16_u16_u16_u16};
pub use u16_float_inner::{u16_float, u16_u16_float, u16_u16_u16_float, u16_u16_u16_u16_float};
pub use u32_inner::{u32_, u32_u32, u32_u32_u32, u32_u32_u32_u32};
pub use u32_float_inner::{u32_float, u32_u32_float, u32_u32_u32_float, u32_u32_u32_u32_float};
pub use f16_inner::{f16_, f16_f16, f16_f16_f16, f16_f16_f16_f16};
pub use f32_inner::{f32_, f32_f32, f32_f32_f32, f32_f32_f32_f32};
pub use f64_inner::{f64_, f64_f64, f64_f64_f64, f64_f64_f64_f64};
pub use misc_inner::{i2_i10_i10_i10_rev, i2_i10_i10_i10_rev_float, u2_u10_u10_u10_rev_float, u2_u10_u10_u10_rev,
                     u10_u11_u11_rev, u10_u11_u11_rev_float};

pub trait VertexData {
    unsafe fn vertex_attrib_pointer(gl: &gl::Gl, stride: usize, location: usize, offset: usize);
}