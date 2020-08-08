use gl_renderer::{
    light::*,
    data::{
        vector_data,
        matrix_data::{
            mat4,
            mat3
        },
    },
    GPUVariant
};


#[derive(GPUVariant, Default)]
pub struct Lights {
    directional: DirectionalLight,
    point_lights: [PointLight; 16],
    spot_lights: [SpotLight; 16],

    num_point_lights: vector_data::i32_,
    num_spot_lights: vector_data::i32_
}

#[derive(GPUVariant)]
pub struct Matrices {
    mvp: mat4,
    mv: mat4,
    m: mat4,
    v: mat4,
    p: mat4,
    n: mat3
}

impl Matrices {
    pub fn new(mvp: mat4, mv: mat4, m: mat4, v: mat4, p: mat4, n: mat3) -> Self {
        Matrices { mvp, mv, m, v, p, n }
    }
}

pub enum LightsError {
    MaxPointLightsReached,
    PointLightIdxOutOfBounds,
    MaxSpotLightsReached,
    SpotLightIdxOutOfBounds,
}

impl Lights {
    pub fn add_point_light(&mut self, light: PointLight) -> Result<(), LightsError> {
        if self.num_point_lights.d0 > 16 {
            return Err(LightsError::MaxPointLightsReached);
        }

        self.point_lights[self.num_point_lights.d0 as usize] = light;
        self.num_point_lights.d0 += 1;

        Ok(())
    }

    pub fn set_point_light(&mut self, idx: usize, light: PointLight) -> Result<(), LightsError> {
        if idx >= self.num_point_lights.d0 as usize {
            return Err(LightsError::PointLightIdxOutOfBounds);
        }

        self.point_lights[idx] = light;

        Ok(())
    }

    pub fn add_spot_light(&mut self, light: SpotLight) -> Result<(), LightsError> {
        if self.num_spot_lights.d0 > 16 {
            return Err(LightsError::MaxSpotLightsReached);
        }

        self.spot_lights[self.num_spot_lights.d0 as usize] = light;
        self.num_spot_lights.d0 += 1;

        Ok(())
    }

    pub fn set_spot_light(&mut self, idx: usize, light: SpotLight) -> Result<(), LightsError> {
        if idx >= self.num_spot_lights.d0 as usize {
            return Err(LightsError::SpotLightIdxOutOfBounds);
        }

        self.spot_lights[idx] = light;

        Ok(())
    }
}