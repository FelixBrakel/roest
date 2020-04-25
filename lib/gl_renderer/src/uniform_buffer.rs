use crate::buffer::{UniformBuffer};
use crate::{Program};
use std::ffi::CString;
use std140::Std140Struct;

pub trait LayoutEnum {}

pub trait UniformLayoutShared: UniformLayout {
    type LayoutElem: LayoutEnum;
    // const STRUCT_NAME: CString;
    // const NUM_FIELDS: usize;
    // const FIELD_NAMES: [CString; Self::NUM_FIELDS];
    fn struct_name() -> CString;

    fn field_names() -> Vec<CString>;

    unsafe fn get_elem_indices(program: &Program) -> Vec<gl::types::GLuint> {
        let num_fields = Self::field_names().len();
        let mut indices: Vec<gl::types::GLuint> = Vec::with_capacity(num_fields);
        indices.resize(num_fields, 0);
        // let mut indices: [gl::types::GLuint; Self::NUM_FIELDS] = [0; 2];
        let mut names: Vec<*const gl::types::GLchar> = Vec::with_capacity(num_fields);
        for string in Self::field_names() {
            names.push(string.as_ptr());
        }

        gl::GetUniformIndices(
            program.get_id(),
            num_fields as gl::types::GLsizei,
            names.as_ptr(),
            indices.as_mut_ptr()
        );

        return indices;
    }

    unsafe fn get_elem_offsets(program: &Program, indices: &[gl::types::GLuint]) -> Vec<gl::types::GLint> {
        let num_fields = Self::field_names().len();
        let mut offsets: Vec<gl::types::GLint> = Vec::with_capacity(num_fields);
        offsets.resize(num_fields, 0);

        gl::GetActiveUniformsiv(
            program.get_id(),
            num_fields as gl::types::GLsizei,
            indices.as_ptr(),
            gl::UNIFORM_OFFSET,
            offsets.as_mut_ptr()
        );

        offsets
    }

    fn get_field_types(program: &Program, indices: &[gl::types::GLuint]) -> Vec<FieldType>;

    fn match_elem(elem: &Self::LayoutElem) -> (usize, &[&[u8]]);

    fn set_elem(&mut self, elem: Self::LayoutElem);

    fn data_slices(&self) -> Vec<&[&[u8]]>;
}

pub trait UniformLayout {
    unsafe fn get_block_index(program: &Program) -> gl::types::GLuint {
        let block_index = gl::GetUniformBlockIndex(
            program.get_id(),
            Self::struct_name().as_ptr() as *const gl::types::GLchar
        );

        block_index
    }
    unsafe fn get_block_size(program: &Program, index: gl::types::GLuint) -> gl::types::GLint {
        let mut blocksize: gl::types::GLint = 0;

        gl::GetActiveUniformBlockiv(
            program.get_id(),
            index,
            gl::UNIFORM_BLOCK_DATA_SIZE,
            &mut blocksize as *mut gl::types::GLint
        );

        blocksize
    }
}

pub trait UniformLayoutStd140: UniformLayout + Std140Struct {}

pub trait IntoLayoutStd140 {
    fn into_std140<S: UniformLayoutStd140>(self) -> S;
}

pub enum FieldType {
    //NOTE: Structs act like primitives
    Primitive,
    Matrix(gl::types::GLint),
    Array(gl::types::GLint, FieldType),
}

struct UniformBlockField {
    index: gl::types::GLuint,
    offset: gl::types::GLint,
    field_type: FieldType
}

pub struct UniformBlock<U: UniformLayout> {
    ubo: UniformBuffer,
    layout: U,
    block_index: gl::types::GLuint,
    size: usize,
    fields: Vec<UniformBlockField>,
}

impl<U: UniformLayoutShared> UniformBlock<U> {
    //TODO: make this method absolutely safe by checking if the program is safe at runtime.
    pub fn new(program: &Program, layout: U) -> Self {
        let ubo = UniformBuffer::new();
        let block_index = unsafe {
            U::get_block_index(program)
        };

        let block_size = unsafe {
            U::get_block_size(program, block_index)
        } as usize;

        let indices = unsafe {
            U::get_elem_indices(program)
        };
        ubo.static_draw_alloc(block_size);

        let offsets = unsafe {
            U::get_elem_offsets(program, &indices)
        };

        let mut field_types = U::get_field_types(program, &indices);

        let mut fields: Vec<UniformBlockField> = Vec::with_capacity(indices.len());
        //NOTE: counting backwards might be more efficient here because of how the Vec's 'remove()' function works
        for i in 0..(indices.len() - 1) {
            fields.push(
                UniformBlockField {
                    index: indices[i],
                    offset: offsets[i],
                    field_type: field_types.remove(i)
                }
            )
        }

        UniformBlock {
            ubo,
            layout,
            block_index,
            size: block_size,
            fields,
            // indices,
            // offsets
        }
    }

    pub fn set_subset<T>(&self, data: &[T], offset: usize) {
        self.ubo.bind();
        unsafe {
            self.ubo.static_draw_subdata(data, offset as gl::types::GLintptr);
        }
        self.ubo.unbind();
    }

    fn buffer_with_stride(data: &[&[u8]], stride: gl::types::GLint) -> Vec<u8> {
        //NOTE: this should make it so no reallocs are needed?
        let mut buf: Vec<u8> = Vec::with_capacity(data.len() * data[0].len() + (data.len() - 1) * stride);
        for item in data {
            buf.extend_from_slice(*item);
            buf.resize_with(buf.len() + stride, 0);
        }

        // self.set_subset(&buf, id);
        buf
    }

    pub fn set(&mut self, elem: U::LayoutElem) {
        let (id, data) = U::match_elem(&elem);
        match &self.fields[id].field_type {
            FieldType::Primitive => {
                self.set_subset(data[0], id);
            },
            FieldType::Array(stride, elem_type) => {
                let buf = Self::buffer_with_stride(data, *stride);
                for elem in data {

                }
                self.set_subset(&buf, id);
            },
            FieldType::Matrix(stride) => {
                let buf = Self::buffer_with_stride(data, *stride);
                self.set_subset(&buf, id);
            },
        }

        self.layout.set_elem(elem);
    }

    pub fn set_all(&mut self, layout: U) {
        self.layout = layout;
        let slices = self.layout.data_slices();

        let mut buf: Vec<u8> = Vec::with_capacity(self.size);

        for (i, slice) in slices.iter().enumerate() {
            let mut tmp = Vec::new();
            match self.fields[i].field_type {
                FieldType::Primitive => {
                    tmp.extend_from_slice(slice[0])
                },
                FieldType::Array(stride, elem_type) => {
                    tmp.extend(Self::buffer_with_stride(*slice, stride));

                },
                FieldType::Matrix(stride) => {
                    tmp.extend(Self::buffer_with_stride(*slice, stride));
                }
            }

            let buf_ptr = &mut buf[self.fields[i].offset as usize] as *mut u8;
            unsafe {
                std::ptr::copy_nonoverlapping(tmp.as_ptr(), buf_ptr, tmp.len());
            }
        }

        self.ubo.bind();
        self.ubo.static_draw_data(&buf);
        self.ubo.unbind();
    }
}
