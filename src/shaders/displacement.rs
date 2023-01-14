use glam::{vec3, Mat3, Vec2, Vec3};
use rgb::alt::BGRA8;

use crate::color::to_bgra;

use super::{light, Light, Payload, Shader};

pub struct DisplacementShader {
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

impl DisplacementShader {
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

impl Shader for DisplacementShader {
    fn shading(&self, payload: Payload) -> BGRA8 {
        let kh = 0.2;
        let kn = 0.1;
        let Vec3 { x, y, z } = payload.normal;

        let xz = (x * x + z * z).sqrt();
        let t = vec3(x * y / xz, xz, z * y / xz);
        let b = payload.normal.cross(t);
        let tbn = Mat3::from_cols(t, b, payload.normal);
        let Vec2 { x: u, y: v } = payload.tex_coords;
        let texture = payload.texture;
        #[rustfmt::skip]
        let d_u = kh * kn * (
            texture.pixel(u + 1. / texture.width(), v).length()
            - texture.pixel(u, v).length()
        ) * 255.;
        #[rustfmt::skip]
        let d_v = kh * kn * (
            texture.pixel(u, v + 1. / texture.height()).length()
            - texture.pixel(u, v).length()
        ) * 255.;
        let point = payload.point + kn * payload.normal * texture.pixel(u, v).length() * 255.;
        let ln = vec3(-d_u, -d_v, 1.);
        let normal = (tbn * ln).normalize();
        let mut result_color = vec3(0., 0., 0.);
        for &Light { source, intensity } in &self.lights {
            let r = source - point;
            let l = r.normalize();
            let r2w = 1. / r.length_squared();
            let l_diffuse = payload.color * intensity * r2w * normal.dot(l).max(0.);
            let v = (self.eye_pos - point).normalize();
            let h = (l + v).normalize();
            let l_spec =
                self.spec_coeff * intensity * r2w * normal.dot(h).max(0.).powi(self.spec_exp);
            result_color += l_diffuse + l_spec + self.amb_coeff * self.amb_intensity;
        }
        to_bgra(result_color)
    }
}
