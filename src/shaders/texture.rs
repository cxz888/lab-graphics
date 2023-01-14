use glam::{vec3, Vec3};
use rgb::alt::BGRA8;

use crate::color;

use super::{light, Light, Payload, Shader};

pub struct TextureShader {
    eye_pos: Vec3,
    pub lights: Vec<Light>,
    /// 环境光系数
    amb_coeff: Vec3,
    /// 环境光强
    amb_intensity: Vec3,
    /// 高光系数
    spec_coeff: Vec3,
    /// 高光指数
    spec_exp: i32,
}

impl TextureShader {
    pub fn new(
        eye_pos: Vec3,
        lights: Vec<Light>,
        amb_coeff: Vec3,
        amb_intensity: Vec3,
        spec_coeff: Vec3,
        spec_exp: i32,
    ) -> Self {
        Self {
            eye_pos,
            lights,
            amb_coeff,
            amb_intensity,
            spec_coeff,
            spec_exp,
        }
    }
    pub fn example(eye_pos: Vec3) -> Self {
        let lights = vec![
            light(vec3(20., 20., 20.), vec3(500., 500., 500.)),
            light(vec3(-20., 20., 0.), vec3(500., 500., 500.)),
        ];
        // 环境
        let amb_coeff = vec3(0.005, 0.005, 0.005);
        let amb_intensity = vec3(10., 10., 10.);
        let spec_coeff = vec3(0.7937, 0.7937, 0.7937);
        let spec_exp = 150;
        Self {
            eye_pos,
            lights,
            amb_coeff,
            amb_intensity,
            spec_coeff,
            spec_exp,
        }
    }
    pub fn eye_pos(&mut self, eye_pos: Vec3) -> &mut Self {
        self.eye_pos = eye_pos;
        self
    }
}

impl Shader for TextureShader {
    fn shading(&self, payload: Payload) -> BGRA8 {
        let mut result_color = Vec3::new(0., 0., 0.);
        let diffuse_coeff = payload
            .texture
            .pixel(payload.tex_coords.x, payload.tex_coords.y);
        for &Light { source, intensity } in &self.lights {
            let r = source - payload.point;
            let l = r.normalize();
            let r2w = 1. / r.length_squared();
            let l_diffuse = diffuse_coeff * intensity * r2w * payload.normal.dot(l).max(0.);
            let v = (self.eye_pos - payload.point).normalize();
            let h = (l + v).normalize();
            let l_spec = self.spec_coeff
                * intensity
                * r2w
                * payload.normal.dot(h).max(0.).powi(self.spec_exp);
            result_color += l_diffuse + l_spec;
        }
        result_color += self.amb_coeff * self.amb_intensity;
        color::to_bgra(result_color)
    }
}
