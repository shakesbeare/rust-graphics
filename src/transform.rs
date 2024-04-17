#[derive(Debug)]
pub struct Transform {
    pub translation: glam::Vec3,
    pub rotation: glam::Quat,
    pub scale: glam::Vec3,
}

impl Transform {
    pub fn new(translation: glam::Vec3, rotation: glam::Quat, scale: glam::Vec3) -> Self {
        Transform {
            translation,
            rotation,
            scale,
        }
    }

    pub fn from_translation(translation: glam::Vec3) -> Self {
        Transform {
            translation,
            rotation: glam::Quat::IDENTITY,
            scale: glam::Vec3::splat(1.0),
        }
    }
}
