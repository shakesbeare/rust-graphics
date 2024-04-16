use crate::{transform::Transform, Vertex};

pub struct Mesh<'a> {
    pub vertices: &'a [Vertex],
    pub indices: &'a [u16],
    pub transform: Transform,
}

impl<'a> Mesh<'a> {
    pub fn vertices_transformed(&self) -> Vec<Vertex> {
        self.vertices.iter().map(|v| {
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
        }).collect()
    }

    pub fn new(vertices: &'a [Vertex], indices: &'a [u16]) -> Self {
        let transform = Transform { 
            translation: glam::Vec3::ZERO,
            rotation: glam::Quat::default(),
            scale: glam::Vec3::splat(1.0),
        };

        Self {
            vertices,
            indices,
            transform,
        }
    }
}

