use rgb::alt::BGRA8;

use super::{Payload, Shader};

pub struct NormalShader;

impl Shader for NormalShader {
    fn shading(&self, payload: Payload) -> BGRA8 {
        BGRA8 {
            b: ((payload.normal.x + 1.) * 0.5 * 255.) as u8,
            g: ((payload.normal.y + 1.) * 0.5 * 255.) as u8,
            r: ((payload.normal.z + 1.) * 0.5 * 255.) as u8,
            a: 0,
        }
    }
}
