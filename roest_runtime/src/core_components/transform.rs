use nalgebra as na;

#[derive(Copy, Clone)]
pub struct Transform {
    scale: f32,
    location: na::Vector3<f32>,
    rotation: na::UnitQuaternion<f32>,
}

#[allow(dead_code)]
impl Transform {
    pub fn new(scale: f32, location: na::Vector3<f32>, rotation: na::UnitQuaternion<f32>) -> Self {
        Transform { scale, location, rotation}
    }

    pub fn from_defaults() -> Self {
        Self::new(
            1.,
            na::Vector3::new(0., 0., 0.),
            na::UnitQuaternion::from_euler_angles(0., 0., 0.),
        )
    }

    pub fn translate(&mut self, translation: na::Vector3<f32>) {
        self.location += translation;
    }

    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
        let new_rot = na::UnitQuaternion::from_euler_angles(x, y, z);
        self.rotation = self.rotation * new_rot;
        //TODO make this not renormalize every single time.
        self.rotation.renormalize_fast();
    }

    pub fn scale_mat(&self) -> na::Matrix4<f32> {
        na::Matrix4::new(
            self.scale, 0., 0., 0.,
            0., self.scale, 0., 0.,
            0., 0., self.scale, 0.,
            0., 0., 0., 1.
        )
    }

    pub fn rotation_mat(&self) -> na::Matrix4<f32> {
        self.rotation.to_homogeneous()
    }

    pub fn translation_mat(&self) -> na::Matrix4<f32>{
        na::Matrix4::new(
            1., 0., 0., self.location[0],
            0., 1., 0., self.location[1],
            0., 0., 1., self.location[2],
            0., 0., 0., 1.,
        )
    }

    pub fn model(&self) -> na::Matrix4<f32> {
        // println!("{}", self.translation_mat());
        // println!("{}", self.rotation_mat());
        // println!("{}", self.scale_mat());

        return self.translation_mat() * self.rotation_mat() * self.scale_mat()
    }
}