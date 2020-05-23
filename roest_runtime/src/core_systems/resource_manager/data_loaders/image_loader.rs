use std::marker::PhantomData;
use gl_renderer::texture::{TextureType, Texture, TexWrapMode, TexMinFilterMode, TexMagFilterMode, Texture2D};
use std::path::Path;
use crate::core_systems::resource_manager::{Loader, file_name_to_path};
use image::{DynamicImage, ImageError};

pub struct ImageLoader {
}

impl ImageLoader {
    pub fn new() -> Self {
        ImageLoader {
        }
    }
}

impl Loader for ImageLoader {
    type E = ImageError;
    type R = DynamicImage;

    fn load(&self, name: impl AsRef<Path>) -> Result<DynamicImage, ImageError> {
        image::open(file_name_to_path(name)?)
    }
}