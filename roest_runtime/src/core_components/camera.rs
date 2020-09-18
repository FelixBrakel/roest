use gl_renderer::data::matrix_data::{mat4};

#[derive(Copy, Clone)]
pub struct Camera {
    // pub view: mat4,
    perspective: mat4,
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32
}

// impl Component for Camera {
//     type Storage = HashMapStorage<Self>;
// }

#[allow(dead_code)]
impl Camera {
    pub fn from_fov(
        fov: f32,
        aspect: f32,
        near: f32,
        far: f32
    ) -> Self {
        // let view = mat4::new(
        //     rotation[(0, 0)], rotation[(0, 1)], rotation[(0, 2)], location.d0,
        //     rotation[(1, 0)], rotation[(1, 1)], rotation[(1, 2)], location.d1,
        //     rotation[(2, 0)], rotation[(2, 1)], rotation[(2, 2)], location.d2,
        //     0., 0., 0., 1.
        // ).try_inverse();

        // let cam_view_mat: mat4 = match view {
        //     Some(mat) => mat.into(),
        //     None => panic!("Matrix is not invertible!")
        // };
        // TODO Use radians here
        let half_height = near * (fov.to_radians() / 2.).tan();
        let half_width = half_height * aspect;
        let depth = far - near;
        let perspective = mat4::new(
            near / half_width, 0., 0., 0.,
            0., near / half_height, 0., 0.,
            0., 0., -(far + near) / depth, -2. * far * near / depth,
            0., 0., -1., 0.
        );

        Camera { perspective, fov, aspect, near, far }
    }

    pub fn update_perspective(&mut self, fov: f32, aspect: f32, near: f32, far: f32) {
        self.fov = fov;
        self.aspect = aspect;
        self.near = near;
        self.far = far;

        self.recalculate_perspective();
    }

    fn recalculate_perspective(&mut self) {
        let half_height = self.near * (self.fov.to_radians() / 2.).tan();
        let half_width = half_height * self.aspect;
        let depth = self.far - self.near;

        self.perspective = mat4::new(
            self.near / half_width, 0., 0., 0.,
            0., self.near / half_height, 0., 0.,
            0., 0., -(self.far + self.near) / depth, -2. * self.far * self.near / depth,
            0., 0., -1., 0.
        );
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.recalculate_perspective();
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
        self.recalculate_perspective();
    }

    pub fn set_near(&mut self, near: f32) {
        self.near = near;
        self.recalculate_perspective();
    }

    pub fn set_far(&mut self, far: f32) {
        self.far = far;
        self.recalculate_perspective();
    }

    pub fn fov(&self) -> f32 {
        self.fov
    }

    pub fn aspect(&self) -> f32 {
        self.aspect
    }

    pub fn near(&self) -> f32 {
        self.near
    }

    pub fn far(&self) -> f32 {
        self.far
    }

    pub fn perspective(&self) -> &mat4 {
        &self.perspective
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::from_fov(
            65.,
            16. / 9.,
            0.1,
            100.
        )
    }
}