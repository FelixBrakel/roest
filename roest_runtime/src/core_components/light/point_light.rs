use nalgebra as na;

pub struct PointLight {
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,

    pub ambient: na::Vector3<f32>,
    pub diffuse: na::Vector3<f32>,
    pub specular: na::Vector3<f32>,
}

impl Default for PointLight {
    fn default() -> Self {
        PointLight {
            constant: 1.,
            linear: 0.35,
            quadratic: 0.44,
            ambient: na::Vector3::new(0.1, 0.1, 0.1),
            diffuse: na::Vector3::new(0.5, 0.5, 0.5),
            specular: na::Vector3::new(1.0, 1.0, 1.0)
        }
    }
}