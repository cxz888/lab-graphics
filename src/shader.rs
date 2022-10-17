use nalgebra_glm::{Vec2, Vec3};
use rgb::alt::BGRA8;

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
    lights: Vec<Light>,
    /// 漫反射系数
    diffuse_coeff: Vec3,
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
        diffuse_coeff: Vec3,
        amb_coeff: Vec3,
        amb_intensity: Vec3,
        spec_coeff: Vec3,
        spec_exp: i32,
    ) -> PhongShader {
        PhongShader {
            eye_pos,
            lights,
            diffuse_coeff,
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
        for Light { source, intensity } in &self.lights {
            let r = source - payload.point;
            let l = r.normalize();
            let r2w = 1. / r.norm_squared();
            let l_diffuse = self
                .diffuse_coeff
                .component_mul(&payload.color)
                .component_mul(intensity)
                * r2w
                * payload.normal.dot(&l).max(0.);
            // if l_diffuse.norm() > 0. {
            //     println!("l_d: {l_diffuse}");
            //     println!("diffuse_coeff: {diffuse_coeff}");
            //     println!("intensity: {intensity}");
            //     println!("dc * it: {}", diffuse_coeff.component_mul(intensity));
            //     println!("r2w: {r2w}");
            //     println!("normal: {}", payload.normal);
            //     println!("l: {l}");
            //     println!("n*l: {}", payload.normal.dot(&l));
            // }
            let v = (self.eye_pos - payload.point).normalize();
            let h = (l + v).normalize();
            let l_spec = self.spec_coeff.component_mul(intensity)
                * r2w
                * payload.normal.dot(&h).max(0.).powi(self.spec_exp);
            result_color += l_diffuse + l_spec;
        }
        result_color += self.amb_coeff.component_mul(&self.amb_intensity);
        BGRA8 {
            b: (result_color.x * 255.).clamp(0., 255.) as u8,
            g: (result_color.y * 255.).clamp(0., 255.) as u8,
            r: (result_color.z * 255.).clamp(0., 255.) as u8,
            a: 0,
        }
    }
}

pub struct NormalShader;

impl Shader for NormalShader {
    fn shading(&self, payload: &Payload) -> BGRA8 {
        // result_color += self.amb_coeff.component_mul(&self.amb_intensity);
        BGRA8 {
            b: ((payload.normal.x + 1.) * 0.5 * 255.).clamp(0., 255.) as u8,
            g: ((payload.normal.y + 1.) * 0.5 * 255.).clamp(0., 255.) as u8,
            r: ((payload.normal.z + 1.) * 0.5 * 255.).clamp(0., 255.) as u8,
            a: 0,
        }
    }
}
