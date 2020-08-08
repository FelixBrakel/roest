use std::path::Path;
use crate::core_systems::resource_manager::{Loader, file_name_to_path, Error as ResError};
use image::{DynamicImage, ImageError};
use thiserror::Error;

pub struct ImageLoader {
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("ImageError")]
    Image{#[source] inner: ImageError},
    #[error("Failed to load resource {}", name)]
    ResourceLoad{name: String, #[source] inner: ResError}
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