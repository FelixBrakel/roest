use std::marker::PhantomData;
use gl_renderer::texture::{TextureType, Texture, TexWrapMode, TexMinFilterMode, TexMagFilterMode, Texture2D};
use std::path::Path;
use crate::core_systems::resource_manager::{Loader, file_name_to_path, Error as ResError};
use image::{DynamicImage, ImageError};
use failure::Fail;

pub struct ImageLoader {
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "ImageError")]
    Image{#[cause] inner: ImageError},
    #[fail(display = "Failed to load resource {}", name)]
    ResourceLoad{name: String, #[cause] inner: ResError}
}

impl ImageLoader {
    pub fn new() -> Self {
        ImageLoader {
        }
    }
}

impl Loader for ImageLoader {
    type E = Error;
    type R = DynamicImage;

    fn load(&self, name: impl AsRef<Path>) -> Result<DynamicImage, Error> {
        let path = file_name_to_path(&name).map_err(|e| Error::ResourceLoad { name: name.as_ref().to_string_lossy().into_owned(), inner: e })?;
        image::open(path).map_err(|e| Error::Image { inner: e })
    }
}