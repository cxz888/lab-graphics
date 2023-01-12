use glam::{Vec2, Vec3};
use rgb::alt::BGRA8;

use crate::color;

pub struct Payload {
    pub color: Vec3,
    pub normal: Vec3,
    pub point: Vec3,
    pub tex_coords: Vec2,
}

pub trait Shader {
    fn shading(&self, payload: &Payload) -> BGRA8;
}

#[derive(Clone, Copy)]
pub struct Light {
    pub source: Vec3,
    pub intensity: Vec3,
}

#[inline]
pub const fn light(source: Vec3, intensity: Vec3) -> Light {
    Light { source, intensity }
}

pub struct EmptyShader;
impl Shader for EmptyShader {
    fn shading(&self, payload: &Payload) -> BGRA8 {
        BGRA8 {
            b: payload.color.x as u8,
            g: payload.color.y as u8,
            r: payload.color.z as u8,
            a: 0,
        }
    }
}

pub struct PhongShader {
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

impl PhongShader {
    pub fn new(
        eye_pos: Vec3,
        lights: Vec<Light>,
        amb_coeff: Vec3,
        amb_intensity: Vec3,
        spec_coeff: Vec3,
        spec_exp: i32,
    ) -> PhongShader {
        PhongShader {
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

impl Shader for PhongShader {
    fn shading(&self, payload: &Payload) -> BGRA8 {
        let mut result_color = Vec3::new(0., 0., 0.);
        for &Light { source, intensity } in &self.lights {
            let r = source - payload.point;
            let l = r.normalize();
            let r2w = 1. / r.length_squared();
            let l_diffuse = payload.color * intensity * r2w * payload.normal.dot(l).max(0.);
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

pub struct NormalShader;

impl Shader for NormalShader {
    fn shading(&self, payload: &Payload) -> BGRA8 {
        BGRA8 {
            b: ((payload.normal.x + 1.) * 0.5 * 255.) as u8,
            g: ((payload.normal.y + 1.) * 0.5 * 255.) as u8,
            r: ((payload.normal.z + 1.) * 0.5 * 255.) as u8,
            a: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use glam::{vec2, vec3};

    use super::{light, Payload, PhongShader, Shader};

    #[test]
    fn test_phong_shader() {
        // 环境
        let amb_coeff = vec3(0.005, 0.005, 0.005);
        let amb_intensity = vec3(10., 10., 10.);
        let spec_coeff = vec3(0.7937, 0.7937, 0.7937);
        let spec_exp = 150;
        let shader = PhongShader::new(
            vec3(10., 10., 0.),
            vec![light(vec3(10., -10., 0.), vec3(100., 100., 100.))],
            amb_coeff,
            amb_intensity,
            spec_coeff,
            spec_exp,
        );
        let payload = Payload {
            color: vec3(0.6, 0.6, 0.6),
            normal: vec3(1., 0., 0.),
            point: vec3(0., 0., 0.),
            tex_coords: vec2(0., 0.),
        };
        dbg!(shader.shading(&payload));
    }
}
