use std::marker::PhantomData;
use crate::data::vector_data::f32_f32_f32_f32;
use failure::_core::ffi::c_void;
use crate::GPUVariant;
use crate::buffered_uniform_struct_shared::{GPUBasic, GPUBasicArray, GPUTexture, GPUTextureArray};

pub enum TexWrapMode {
    ClampToBorder = gl::CLAMP_TO_BORDER as isize,
    ClampToEdge = gl::CLAMP_TO_EDGE as isize,
    MirroredRepeat = gl::MIRRORED_REPEAT as isize,
    MirrorClampToEdge = gl::MIRROR_CLAMP_TO_EDGE as isize,
    Repeat = gl::REPEAT as isize,
}

pub enum TexMinFilterMode {
    Nearest = gl::NEAREST as isize,
    Linear = gl::LINEAR as isize,
    NearestMipmapNearest = gl::NEAREST_MIPMAP_NEAREST as isize,
    LinearMipmapNearest = gl::LINEAR_MIPMAP_NEAREST as isize,
    NearestMipmapLinear = gl::NEAREST_MIPMAP_LINEAR as isize,
    LinearMipmapLinear = gl::LINEAR_MIPMAP_LINEAR as isize,
}

pub enum TexMagFilterMode {
    Nearest = gl::NEAREST as isize,
    Linear = gl::LINEAR as isize,
}


pub trait TextureType {
    type Tex;
    const TEXTURE_TYPE: gl::types::GLenum;
}

pub struct Texture2D;

impl TextureType for Texture2D {
    type Tex = Self;
    const TEXTURE_TYPE: gl::types::GLenum = gl::TEXTURE_2D;
}

pub enum BindlessTexture<T> {
    Resident(ResidentBindlessTexture<T>),
    NonResident(NonResidentBindlessTexture<T>),
}

impl<T> BindlessTexture<T> {
    pub fn switch(&mut self) {
        *self = match self {
            BindlessTexture::Resident(t) => {
                BindlessTexture::NonResident(t.into())
            },
            BindlessTexture::NonResident(t) => {
                BindlessTexture::Resident(t.into())
            }
        }
    }
}

pub struct ResidentBindlessTexture<T> {
    handle: gl::types::GLuint64,
    tex: Texture<T>
}

impl<T: TextureType> ResidentBindlessTexture<T> {
    pub fn get_handle(&self) -> gl::types::GLuint64 {
        self.handle
    }
}

impl<T> From<NonResidentBindlessTexture<T>> for ResidentBindlessTexture<T> {
    fn from(non_resident_tex: NonResidentBindlessTexture<T>) -> Self {
        let resident_tex = ResidentBindlessTexture { handle: non_resident_tex.handle, tex: non_resident_tex.tex };

        unsafe {
            gl::MakeTextureHandleResidentARB(resident_tex.handle);
        }

        resident_tex
    }
}

pub struct NonResidentBindlessTexture<T> {
    handle: gl::types::GLuint64,
    tex: Texture<T>
}

impl<T: TextureType> NonResidentBindlessTexture<T> {
    pub fn new(tex: Texture<T>) -> Self {
        let handle = unsafe {
            gl::GetTextureHandleARB(tex.to)
        };

        NonResidentBindlessTexture { handle, tex }
    }
}

impl<T> From<ResidentBindlessTexture<T>> for NonResidentBindlessTexture<T> {
    fn from(resident_tex: ResidentBindlessTexture<T>) -> Self {
        let non_resident_tex = NonResidentBindlessTexture { handle: resident_tex.handle, tex: resident_tex.tex };
        unsafe {
            gl::MakeTextureHandleNonResidentARB(non_resident_tex.handle);
        }

        non_resident_tex
    }
}

impl<T> From<Texture<T>> for NonResidentBindlessTexture<T> {
    fn from(tex: Texture<T>) -> Self {
        NonResidentBindlessTexture::new(tex)
    }
}

pub struct Texture<T> {
    to: gl::types::GLuint,
    _marker: PhantomData<T>
}

impl<T> Texture<T> where T: TextureType {
    pub fn new(wrap_s: TexWrapMode,
               wrap_t: TexWrapMode,
               min: TexMinFilterMode,
               mag: TexMagFilterMode) -> Self {
        let mut to = 0;

        unsafe {
            gl::CreateTextures(T::TEXTURE_TYPE,  1, &mut to);
        }

        let tex = Texture {
            to,
            _marker: PhantomData
        };

        tex.set_wrap_s(wrap_s);
        tex.set_wrap_t(wrap_t);
        tex.set_min_filter(min);
        tex.set_mag_filter(mag);

        tex
   }

    pub fn set_wrap_s(&self, wrap_mode: TexWrapMode) {
        unsafe {
            gl::TextureParameteri(self.to, gl::TEXTURE_WRAP_S, wrap_mode as i32);
        }
    }

    pub fn set_wrap_t(&self, wrap_mode: TexWrapMode) {
        unsafe {
            gl::TextureParameteri(self.to, gl::TEXTURE_WRAP_T, wrap_mode as i32);
        }
    }

    pub fn set_border_color(&self, color: f32_f32_f32_f32) {
        unsafe {
            gl::TextureParameterfv(self.to, gl::TEXTURE_BORDER_COLOR, &color as *const f32_f32_f32_f32 as *const f32);
        }
    }

    pub fn set_min_filter(&self, filter_mode: TexMinFilterMode) {
        unsafe {
            gl::TextureParameteri(self.to, gl::TEXTURE_MIN_FILTER, filter_mode as i32);
        }
    }

    pub fn set_mag_filter(&self, filter_mode: TexMagFilterMode) {
        unsafe {
            gl::TextureParameteri(self.to, gl::TEXTURE_MAG_FILTER, filter_mode as i32);
        }
    }

    pub fn gen_mipmaps(&self) {
        unsafe {
            gl::GenerateTextureMipmap(self.to);
        }
    }
}

impl<T> Texture<T>
    where T: TextureType<Tex = Texture2D>
{
    pub fn storage_2d(&self, width: i32, height: i32) {
        unsafe {
            gl::TextureStorage2D(self.to, 1, gl::RGBA8, width, height);
        }
    }

    pub fn sub_image_2d(&self, width: i32, height: i32, pixels: &[u8]) {
        unsafe {
            gl::TextureSubImage2D(
                self.to,
                0,
                0,
                0,
                width,
                height,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                pixels as *const _ as *const c_void
            )
        }
    }
}

impl<T> GPUVariant for Texture<T> {
    type Variant = GPUTexture;
    type ArrayVariant = GPUTextureArray;
}
