mod blinn_phong;
mod bump;
mod displacement;
mod empty;
mod normal;
mod texture;

use glam::{Vec2, Vec3};
use rgb::alt::BGRA8;

use crate::texture::Texture;

pub use blinn_phong::BlinnPhongShader;
pub use bump::BumpShader;
pub use displacement::DisplacementShader;
pub use empty::EmptyShader;
pub use normal::NormalShader;
pub use texture::TextureShader;

pub struct Payload<'a> {
    pub color: Vec3,
    pub normal: Vec3,
    pub point: Vec3,
    pub tex_coords: Vec2,
    pub texture: &'a Texture,
}

pub trait Shader {
    fn shading(&self, payload: Payload) -> BGRA8;
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
