use gl_renderer::{buffer, VertexAttribPointers, IndexedVertArray, Program};
use gl_renderer::{GlUniform, data::matrix_data, data::matrix_data::GlMat};

pub struct IMeshDefaults {
    mvp: matrix_data::mat4,
    mv: matrix_data::mat4,
    m: matrix_data::mat4,
    v: matrix_data::mat4,
    p: matrix_data::mat4
}

impl GlUniform for IMeshDefaults {
    fn gl_uniform(&self) {
        unsafe {
            self.mvp.gl_uniform(0);
            self.mv.gl_uniform(1);
            self.m.gl_uniform(2);
            self.v.gl_uniform(3);
            self.p.gl_uniform(4);
        }
    }

    fn from_uniform(program: &Program) -> Self {
        let mvp = unsafe {
            matrix_data::mat4::from_gl_uniform(&program, 0)
        };

        let mv = unsafe {
            matrix_data::mat4::from_gl_uniform(&program, 1)
        };

        let m = unsafe {
            matrix_data::mat4::from_gl_uniform(&program, 2)
        };

        let v = unsafe {
            matrix_data::mat4::from_gl_uniform(&program, 3)
        };

        let p = unsafe {
            matrix_data::mat4::from_gl_uniform(&program, 4)
        };


        IMeshDefaults { mvp, mv, m, v, p }
    }
}

impl IMeshDefaults {
    pub fn new(
        mvp: matrix_data::mat4,
        mv: matrix_data::mat4,
        m: matrix_data::mat4,
        v: matrix_data::mat4,
        p: matrix_data::mat4
    ) -> Self {
        IMeshDefaults { mvp, mv, m, v, p }
    }

    pub fn zero() -> Self {
        let zero = matrix_data::mat4::new(
            0., 0., 0., 0.,
            0., 0., 0., 0.,
            0., 0., 0., 0.,
            0., 0., 0., 0.
        );

        Self::new(zero, zero, zero, zero, zero)
    }

    pub fn set(
        &mut self,
        mvp: matrix_data::mat4,
        mv: matrix_data::mat4,
        m: matrix_data::mat4,
        v: matrix_data::mat4,
        p: matrix_data::mat4
    ) {
        self.mvp = mvp;
        self.mv = mv;
        self.m = m;
        self.v = v;
        self.p = p;
    }
}

pub struct IndexedMesh {
    _index_vbo: buffer::ElementArrayBuffer,
    _vertex_vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
    n_indices: usize,
}

impl IndexedMesh {
    pub fn new<V: VertexAttribPointers>(verts: &IndexedVertArray<V>) -> IndexedMesh {
        let vertex_vbo = buffer::ArrayBuffer::new();

        vertex_vbo.bind();
        vertex_vbo.static_draw_data(&verts.vertices);
        vertex_vbo.unbind();

        let index_vbo = buffer::ElementArrayBuffer::new();
        index_vbo.bind();
        index_vbo.static_draw_data(&verts.indices);
        index_vbo.unbind();

        let vao = buffer::VertexArray::new();
        vao.bind();
        vertex_vbo.bind();
        V::vertex_attrib_pointers();
        vertex_vbo.unbind();
        vao.unbind();

        IndexedMesh {
            _index_vbo: index_vbo,
            _vertex_vbo: vertex_vbo,
            vao,
            n_indices: verts.indices.len()
        }
    }

    pub fn render(&self) {
        self.vao.bind();
        self._index_vbo.bind();

        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.n_indices as gl::types::GLsizei,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid
            );
        }
    }
}