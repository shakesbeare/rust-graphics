pub struct Camera<P: Projection> {
    #[allow(unused)]
    proj: P,
    pub location: glam::Vec3,
    pub up: glam::Vec3,
    pub aspect_ratio: f32,
    pub dir: glam::Vec3,
    fov: f32,
    z_near: f32,
    z_far: f32,
}

impl<P: Projection> Camera<P> {
    pub fn new(fov_degrees: f32, aspect_ratio: f32, projection_type: P) -> Self {
        let eye = glam::Vec3::new(3.75, 1.675, 3.75);
        Camera {
            proj: projection_type,
            location: eye,
            dir: -eye, // always look at origin
            up: glam::Vec3::Y,
            aspect_ratio,
            fov: fov_degrees.to_radians(),
            z_near: 0.1,
            z_far: 1000.0,
        }
    }

    pub fn move_camera(&mut self, location: glam::Vec3) {
        self.location = location;
    }

    pub fn rotate_camera(&mut self, direction: glam::Vec3) {
        self.dir = direction;
    }

    pub fn point_at(&mut self, target: glam::Vec3) {
        let direction = target - self.location;
        self.rotate_camera(direction);
    }

    pub fn projection_matrix(&self) -> glam::Mat4 {
        P::generate_view_projection_matrix(
            self.aspect_ratio,
            self.location,
            self.up,
            self.fov,
            self.dir,
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
        aspect_ratio: f32,
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

