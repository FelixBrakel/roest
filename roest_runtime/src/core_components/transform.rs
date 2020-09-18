use nalgebra as na;
use nalgebra::{Vector3, Unit};

#[derive(Copy, Clone)]
pub struct Transform {
    scale: na::Vector3<f32>,
    location: na::Vector3<f32>,
    rotation: na::UnitQuaternion<f32>,
}

#[allow(dead_code)]
impl Transform {
    pub fn new(scale: f32, location: na::Vector3<f32>, rotation: na::UnitQuaternion<f32>) -> Self {
        Transform { scale: na::Vector3::new(scale, scale, scale), location, rotation}
    }

    pub fn translate(&mut self, translation: na::Vector3<f32>) {
        self.location += translation;
    }

    pub fn translate_x(&mut self, translation: f32) {
        self.location.x += translation;
    }

    pub fn translate_y(&mut self, translation: f32) {
        self.location.y += translation;
    }

    pub fn translate_z(&mut self, translation: f32) {
        self.location.z += translation;
    }

    pub fn position_x(&mut self, translation: f32) {
        self.location.x = translation;
    }

    pub fn position_y(&mut self, translation: f32) {
        self.location.y = translation;
    }

    pub fn position_z(&mut self, translation: f32) {
        self.location.z = translation;
    }

    pub fn rotate_by(&mut self, x: f32, y: f32, z: f32) {
        let new_rot = na::UnitQuaternion::from_euler_angles(x, y, z);
        self.rotation = self.rotation * new_rot;
        //TODO make this not renormalize every single time.
        self.rotation.renormalize_fast();
    }

    pub fn set_rotation(&mut self, x: f32, y: f32, z: f32) {
        self.rotation = na::UnitQuaternion::from_euler_angles(x, y, z);
    }

    /// Rotate around the x axis (in radians)
    pub fn rotation_x(&mut self, rotation: f32) {
        let (_x, y, z) = self.rotation.euler_angles();
        self.rotation = na::UnitQuaternion::from_euler_angles(rotation, y, z);
        self.rotation.renormalize_fast();
    }

    /// Rotate around the y axis (in radians)
    pub fn rotation_y(&mut self, rotation: f32) {
        let (x, _y, z) = self.rotation.euler_angles();
        let x_axis = Unit::new_normalize(Vector3::new(1., 0., 0.));
        let y_axis = Unit::new_normalize(Vector3::new(0., 1., 0.));
        let z_axis = Unit::new_normalize(Vector3::new(0., 0., 1.));

        let rot_x = na::Rotation3::from_axis_angle(&x_axis, x);
        let rot_y = na::Rotation3::from_axis_angle(&y_axis, rotation);
        let rot_z = na::Rotation3::from_axis_angle(&z_axis, z);

        self.rotation = na::UnitQuaternion::from_rotation_matrix(&(rot_z * rot_y * rot_x));
        self.rotation.renormalize_fast();
    }

    /// Rotate around the z axis (in radians)
    pub fn rotation_z(&mut self, rotation: f32) {
        let (x, y, _z) = self.rotation.euler_angles();
        self.rotation = na::UnitQuaternion::from_euler_angles(x, y, rotation);
        self.rotation.renormalize_fast();
    }

    pub fn scale_mat(&self) -> na::Matrix4<f32> {
        na::Matrix4::new(
            self.scale.x, 0., 0., 0.,
            0., self.scale.y, 0., 0.,
            0., 0., self.scale.z, 0.,
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

    pub fn translation(&self) -> na::Vector3<f32> {
        self.location.clone_owned()
    }

    pub fn rotation(&self) -> na::UnitQuaternion<f32> {
        self.rotation
    }

    pub fn scale(&self) -> na::Vector3<f32> {
        self.scale.clone_owned()
    }

    pub fn scale_x(&mut self, scale: f32) {
        self.scale.x = scale;
    }

    pub fn scale_y(&mut self, scale: f32) {
        self.scale.y = scale;
    }

    pub fn scale_z(&mut self, scale: f32) {
        self.scale.z = scale;
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new(
            1.,
            na::Vector3::new(0., 0., 0.),
            na::UnitQuaternion::from_euler_angles(0., 0., 0.),
        )
    }
}