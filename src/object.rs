use nalgebra_glm::{Mat4, Vec3};
use rgb::alt::BGRA8;
use scanner_rust::ScannerStr;
use std::path::Path;

#[derive(Debug)]
pub struct Object {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<[usize; 3]>,
    pub colors: Vec<BGRA8>,
    pub model: Mat4,
}

use anyhow::{anyhow, Result};

impl Object {
    pub fn load_obj(path: impl AsRef<Path>) -> Result<Object> {
        let obj = std::fs::read_to_string(path).unwrap();
        let mut scanner = ScannerStr::new(&obj);
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut colors = Vec::new();

        loop {
            match scanner.next()? {
                Some(header) => match header {
                    "v" => {
                        let x = scanner.next_f32().unwrap().unwrap();
                        let y = scanner.next_f32().unwrap().unwrap();
                        let z = scanner.next_f32().unwrap().unwrap();
                        vertices.push(Vec3::new(x, y, z));
                    }
                    "c" => {
                        let b = scanner.next_u8().unwrap().unwrap();
                        let g = scanner.next_u8().unwrap().unwrap();
                        let r = scanner.next_u8().unwrap().unwrap();
                        colors.push(BGRA8 { b, g, r, a: 0 });
                    }
                    "f" => {
                        let i1 = scanner.next_usize().unwrap().unwrap();
                        let i2 = scanner.next_usize().unwrap().unwrap();
                        let i3 = scanner.next_usize().unwrap().unwrap();
                        indices.push([i1 - 1, i2 - 1, i3 - 1]);
                    }
                    _ => {
                        return Err(anyhow!("Error header{}", header));
                    }
                },
                None => {
                    return Ok(Object {
                        vertices,
                        indices,
                        colors,
                        model: Mat4::identity(),
                    });
                }
            }
        }
    }
    pub fn model(mut self, model: Mat4) -> Self {
        self.model = model;
        self
    }
}
