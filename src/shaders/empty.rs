use rgb::alt::BGRA8;

use super::{Payload, Shader};

pub struct EmptyShader;

impl Shader for EmptyShader {
    fn shading(&self, payload: Payload) -> BGRA8 {
        BGRA8 {
            b: payload.color.x as u8,
            g: payload.color.y as u8,
            r: payload.color.z as u8,
            a: 0,
        }
    }
}
