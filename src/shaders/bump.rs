use glam::{vec3, Mat3, Vec2, Vec3};
use rgb::alt::BGRA8;

use crate::color::to_bgra;

use super::{Payload, Shader};

pub struct BumpShader {
    eye_pos: Vec3,
}

impl BumpShader {
    pub fn new(eye_pos: Vec3) -> Self {
        Self { eye_pos }
    }
    pub fn eye_pos(&mut self, eye_pos: Vec3) -> &mut Self {
        self.eye_pos = eye_pos;
        self
    }
}

impl Shader for BumpShader {
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
        let ln = vec3(-d_u, -d_v, 1.);
        let normal = (tbn * ln).normalize();
        to_bgra(normal)
    }
}
