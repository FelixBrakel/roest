use nalgebra as na;

#[derive(Copy, Clone)]
pub struct Transform {
    scale: na::Vector3<f32>,
    location: na::Vector3<f32>,
    rotation: na::UnitQuaternion<f32>,
}

fn to_degrees(rads: f32) -> f32 {
    rads * 57.295779513
}

fn to_radians(degrees: f32) -> f32 {
    degrees * 0.01745329252
}

#[allow(dead_code)]
impl Transform {
    pub fn new(scale: f32, location: na::Vector3<f32>, rotation: na::UnitQuaternion<f32>) -> Self {
        Transform { scale: na::Vector3::new(scale, scale, scale), location, rotation}
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

    pub fn translate_x(&mut self, translation: f32) {
        self.location.x += translation;
    }

    pub fn translate_y(&mut self, translation: f32) {
        self.location.y += translation;
    }

    pub fn translate_z(&mut self, translation: f32) {
        self.location.z += translation;
    }

    pub fn rotate_by(&mut self, x: f32, y: f32, z: f32) {
        let new_rot = na::UnitQuaternion::from_euler_angles(x, y, z);
        self.rotation = self.rotation * new_rot;
        //TODO make this not renormalize every single time.
        self.rotation.renormalize_fast();
    }

    pub fn rotation_x(&mut self, rotation: f32) {
        let (x, y, z) = self.rotation.euler_angles();
        self.rotation = na::UnitQuaternion::from_euler_angles(to_radians(rotation), y, z);
    }

    pub fn rotation_y(&mut self, rotation: f32) {
        let (x, y, z) = self.rotation.euler_angles();
        self.rotation = na::UnitQuaternion::from_euler_angles(x, to_radians(rotation), z);
    }

    pub fn rotation_z(&mut self, rotation: f32) {
        let (x, y, z) = self.rotation.euler_angles();
        self.rotation = na::UnitQuaternion::from_euler_angles(x, y, to_radians(rotation));
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