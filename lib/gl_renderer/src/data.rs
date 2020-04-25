pub mod vector_data;
pub mod vector_data_zst;
pub mod matrix_data;
pub mod matrix_data_zst;

pub trait ZSTVariant {
    type Original;
}