mod u2_u10_u10_u10_rev_float_inner;
mod f32_f32_f32_inner;
mod i8_inner;
mod i8_float_inner;

pub use u2_u10_u10_u10_rev_float_inner::u2_u10_u10_u10_rev_float;
pub use f32_f32_f32_inner::f32_f32_f32;
pub use i8_inner::i8_;
pub use i8_float_inner::i8_float;

pub trait VertexData {
    unsafe fn vertex_attrib_pointer(gl: &gl::Gl, stride: usize, location: usize, offset: usize);
}