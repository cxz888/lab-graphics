use glam::{Mat4, Vec2, Vec3};
use rgb::alt::BGRA8;
use std::{fmt, path::Path};

#[derive(Debug)]
pub struct Object {
    pub vertices: Vec<Vec3>,
    pub vertex_color: Vec<BGRA8>,
    pub normals: Vec<Vec3>,
    pub texcoords: Vec<Vec2>,
    pub indices: Vec<[usize; 3]>,
    pub normal_indices: Vec<[usize; 3]>,
    pub texcoord_indices: Vec<[usize; 2]>,
    pub model: Mat4,
}

use anyhow::Result;

impl Object {
    /// 从 .obj 文件中读取对象模型
    pub fn load_obj(path: impl AsRef<Path> + fmt::Debug) -> Result<Object> {
        let (models, _) = tobj::load_obj(path, &tobj::LoadOptions::default())?;
        let mesh = &models[0].mesh;
        let mut vertices = Vec::with_capacity(mesh.positions.len() / 3);
        let mut vertex_color = Vec::with_capacity(mesh.vertex_color.len() / 3);
        let mut normals = Vec::with_capacity(mesh.normals.len() / 3);
        let mut texcoords = Vec::with_capacity(mesh.texcoords.len() / 2);
        let mut indices = Vec::with_capacity(mesh.indices.len() / 3);
        let mut normal_indices = Vec::with_capacity(mesh.normal_indices.len() / 3);
        let mut texcoord_indices = Vec::with_capacity(mesh.texcoord_indices.len() / 2);
        for i in 0..mesh.positions.len() / 3 {
            vertices.push(Vec3::new(
                mesh.positions[3 * i],
                mesh.positions[3 * i + 1],
                mesh.positions[3 * i + 2],
            ));
            vertex_color.push(if 3 * i + 2 < mesh.vertex_color.len() {
                BGRA8 {
                    b: mesh.vertex_color[3 * i] as u8,
                    g: mesh.vertex_color[3 * i + 1] as u8,
                    r: mesh.vertex_color[3 * i + 2] as u8,
                    a: 0,
                }
            } else {
                BGRA8 {
                    b: 92,
                    g: 121,
                    r: 148,
                    a: 0,
                }
            });
        }
        for i in 0..mesh.normals.len() / 3 {
            normals.push(
                Vec3::new(
                    mesh.normals[3 * i],
                    mesh.normals[3 * i + 1],
                    mesh.normals[3 * i + 2],
                )
                .normalize(),
            );
        }
        for i in 0..mesh.texcoords.len() / 2 {
            texcoords.push(Vec2::new(mesh.texcoords[2 * i], mesh.texcoords[2 * i + 1]));
        }
        for i in 0..mesh.indices.len() / 3 {
            indices.push([
                mesh.indices[3 * i] as usize,
                mesh.indices[3 * i + 1] as usize,
                mesh.indices[3 * i + 2] as usize,
            ]);
        }
        for i in 0..mesh.normal_indices.len() / 3 {
            normal_indices.push([
                mesh.normal_indices[3 * i] as usize,
                mesh.normal_indices[3 * i + 1] as usize,
                mesh.normal_indices[3 * i + 2] as usize,
            ]);
        }
        for i in 0..mesh.texcoord_indices.len() / 2 {
            texcoord_indices.push([
                mesh.texcoord_indices[2 * i] as usize,
                mesh.texcoord_indices[2 * i + 1] as usize,
            ]);
        }
        if normals.len() == 0 {
            for [i, j, k] in indices.iter().copied() {
                let va = vertices[i] - vertices[j];
                let vb = vertices[k] - vertices[j];
                normal_indices.push([normals.len(), normals.len(), normals.len()]);
                normals.push(va.cross(vb).normalize());
            }
        }
        return Ok(Object {
            vertices,
            vertex_color,
            normals,
            texcoords,
            indices,
            normal_indices,
            texcoord_indices,
            model: Default::default(),
        });
    }
    pub fn model(mut self, model: Mat4) -> Self {
        self.model = model;
        self
    }
}
