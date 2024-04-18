use std::path::PathBuf;

use crate::{transform::Transform, vertex::Vertex};

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub transform: Transform,
}

impl Mesh {
    pub fn vertices_transformed(&self) -> Vec<Vertex> {
        self.vertices
            .iter()
            .map(|v| {
                let pos = v.position;
                let color = v.color;

                let transformation_matrix = glam::Mat4::from_scale_rotation_translation(
                    self.transform.scale,
                    self.transform.rotation,
                    self.transform.translation,
                );

                let transformed = transformation_matrix.mul_vec4(pos.into());
                Vertex {
                    position: transformed.into(),
                    color,
                }
            })
            .collect()
    }

    pub fn new(vertices: &[Vertex], indices: &[u16]) -> Self {
        let transform = Transform {
            translation: glam::Vec3::ZERO,
            rotation: glam::Quat::default(),
            scale: glam::Vec3::splat(1.0),
        };

        Self {
            vertices: vertices.to_vec(),
            indices: indices.to_vec(),
            transform,
        }
    }
}

impl From<obj::Obj> for Mesh {
    fn from(value: obj::Obj) -> Self {
        let vertices = value
            .vertices
            .iter()
            .map(|v| Vertex {
                position: [v.position[0], v.position[1], v.position[2], 1.0],
                color: [0.33, 0.33, 0.33, 1.0],
            })
            .collect::<Vec<Vertex>>();

        let transform = Transform::from_translation(glam::Vec3::new(0.0, 0.0, 0.0));

        Mesh {
            vertices,
            indices: value.indices,
            transform,
        }
    }
}

impl From<PathBuf> for Mesh {
    fn from(value: PathBuf) -> Self {
        let contents = std::fs::read_to_string(value).unwrap();
        let obj = obj::load_obj(contents.as_bytes()).unwrap();
        Mesh::from(obj)
    }
}
