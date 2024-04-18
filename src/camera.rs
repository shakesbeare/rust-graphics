use crate::transform::Transform;

pub struct Camera<P: Projection> {
    #[allow(unused)]
    proj: P,
    pub transform: Transform,
    pub pitch: f32,
    pub yaw: f32,
    pub aspect_ratio: f32,
    fov: f32,
    z_near: f32,
    z_far: f32,
}

impl<P: Projection> Camera<P> {
    pub fn new(
        fov_degrees: f32,
        aspect_ratio: f32,
        projection_type: P,
        transform: Transform,
    ) -> Self {
        Camera {
            proj: projection_type,
            transform,
            aspect_ratio,
            pitch: 0.0,
            yaw: 0.0,
            fov: fov_degrees.to_radians(),
            z_near: 0.1,
            z_far: 1000.0,
        }
    }

    pub fn forward(&self) -> glam::Vec3 {
        glam::Vec3::new(
            self.pitch.cos() * self.yaw.sin(),
            self.pitch.sin(),
            self.pitch.cos() * self.yaw.cos(),
        )
        .normalize()
    }

    pub fn backward(&self) -> glam::Vec3 {
        -self.forward()
    }

    pub fn right(&self) -> glam::Vec3 {
        glam::Vec3::Y.cross(self.forward()).normalize()
    }

    pub fn left(&self) -> glam::Vec3 {
        -self.right()
    }

    pub fn up(&self) -> glam::Vec3 {
        glam::Vec3::Y
    }

    pub fn down(&self) -> glam::Vec3 {
        glam::Vec3::NEG_Y
    }

    pub fn translate(&mut self, translation: glam::Vec3) {
        self.transform.translation += translation;
    }

    pub fn projection_matrix(&self) -> glam::Mat4 {
        P::generate_view_projection_matrix(
            self.aspect_ratio,
            self.transform.translation,
            glam::Vec3::Y,
            self.fov,
            self.forward(),
            self.z_near,
            self.z_far,
        )
    }
}

pub trait Projection {
    fn generate_view_projection_matrix(
        aspect_ratio: f32,
        eye: glam::Vec3,
        up: glam::Vec3,
        fov: f32,
        target: glam::Vec3,
        z_near: f32,
        z_far: f32,
    ) -> glam::Mat4;
}

pub struct Orthographic;

impl Projection for Orthographic {
    fn generate_view_projection_matrix(
        _aspect_ratio: f32,
        eye: glam::Vec3,
        up: glam::Vec3,
        fov: f32,
        target: glam::Vec3,
        z_near: f32,
        z_far: f32,
    ) -> glam::Mat4 {
        let projection =
            glam::Mat4::orthographic_rh(-fov, fov, -fov, fov, z_near, z_far);
        let view = glam::Mat4::look_to_rh(eye, target, up);
        return projection * view;
    }
}

pub struct Perspective;

impl Projection for Perspective {
    fn generate_view_projection_matrix(
        aspect_ratio: f32,
        eye: glam::Vec3,
        up: glam::Vec3,
        fov: f32,
        target: glam::Vec3,
        z_near: f32,
        z_far: f32,
    ) -> glam::Mat4 {
        let projection = glam::Mat4::perspective_rh(fov, aspect_ratio, z_near, z_far);
        let view = glam::Mat4::look_to_rh(eye, target, up);
        return projection * view;
    }
}
