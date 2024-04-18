use bevy_input::ButtonInput;
use glam::Vec2;
use winit::keyboard::KeyCode;

pub mod camera;
pub mod mesh;
pub mod render;
pub mod texture;
pub mod time;
pub mod transform;
pub mod vertex;

pub struct Input {
    pub keyboard: ButtonInput<KeyCode>,
    pub mouse_motion: Vec2,
}

impl Input {
    pub fn new() -> Self {
        Self {
            keyboard: ButtonInput::default(),
            mouse_motion: Vec2::default(),
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Entity {
    fn start(&mut self);
    fn update(&mut self);
}
