use gl_renderer::data::matrix_data::{mat4, mat3};
use gl_renderer::data::vector_data::{f32_f32_f32};

#[derive(Copy, Clone)]
pub struct Camera {
    pub view: mat4,
    pub perspective: mat4
}

// impl Component for Camera {
//     type Storage = HashMapStorage<Self>;
// }

#[allow(dead_code)]
impl Camera {
    pub fn from_fov(
        location: f32_f32_f32,
        rotation: mat3,
        fov: f32,
        aspect: f32,
        near: f32,
        far: f32
    ) -> Self {
        let view = mat4::new(
            rotation[(0, 0)], rotation[(0, 1)], rotation[(0, 2)], location.d0,
            rotation[(1, 0)], rotation[(1, 1)], rotation[(1, 2)], location.d1,
            rotation[(2, 0)], rotation[(2, 1)], rotation[(2, 2)], location.d2,
            0., 0., 0., 1.
        ).try_inverse();

        let cam_view_mat: mat4 = match view {
            Some(mat) => mat.into(),
            None => panic!("Matrix is not invertible!")
        };

        let half_height = near * (fov.to_radians() / 2.).tan();
        let half_width = half_height * aspect;
        let depth = far - near;
        let perspective = mat4::new(
            near / half_width, 0., 0., 0.,
            0., near / half_height, 0., 0.,
            0., 0., -(far + near) / depth, -2. * far * near / depth,
            0., 0., -1., 0.
        );

        Camera { view: cam_view_mat, perspective }
    }

    pub fn set_location(&mut self, location: f32_f32_f32) {
        self.view[(0, 3)]= location.d0;
        self.view[(1, 3)] = location.d1;
        self.view[(2, 3)] = location.d2;
    }

    pub fn set_rotation(&mut self, rotation: mat3) {
        self.view[(0, 0)] = rotation[(0, 0)];
        self.view[(0, 1)] = rotation[(0, 1)];
        self.view[(0, 2)] = rotation[(0, 2)];
        self.view[(1, 0)] = rotation[(1, 0)];
        self.view[(1, 1)] = rotation[(1, 1)];
        self.view[(1, 2)] = rotation[(1, 2)];
        self.view[(2, 0)] = rotation[(2, 0)];
        self.view[(2, 1)] = rotation[(2, 1)];
        self.view[(2, 2)] = rotation[(2, 2)];
    }

    pub fn update_perspective(&mut self, fov: f32, aspect: f32, near: f32, far: f32) {
        let half_height = near * (fov.to_radians() / 2.).tan();
        let half_width = half_height * aspect;
        let depth = far - near;

        self.perspective = mat4::new(
            near / half_width, 0., 0., 0.,
            0., near / half_height, 0., 0.,
            0., 0., -(far + near) / depth, -2. * far * near / depth,
            0., 0., -1., 0.
        );
    }

    // pub fn rotate(&self, rotation: mat3) {
    //
    // }

    // pub fn translate(&self, translation: f32_f32_f32) {
    //
    // }
}