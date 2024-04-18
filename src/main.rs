#![allow(clippy::collapsible_match)]
use glam::Vec3;
use rust_graphics::camera::{Camera, Perspective, Projection};
use rust_graphics::mesh::Mesh;
use rust_graphics::time;
use rust_graphics::transform::Transform;
use rust_graphics::Entity;
use rust_graphics::{render::Render, Input};
use winit::event::DeviceEvent;
use winit::{
    dpi::{LogicalSize, Size},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    keyboard::PhysicalKey,
    window::Window,
};

use ::anyhow::Result;

async fn run(event_loop: EventLoop<()>, window: Window) {
    time::startup();
    window
        .set_cursor_grab(winit::window::CursorGrabMode::Locked)
        .unwrap();
    window.set_cursor_visible(false);

    let mut render = Render::new(window, Perspective).await;
    let mut input = rust_graphics::Input::default();
    let mut camera = Camera::new(
        90.0,
        render.size.width as f32 / render.size.height as f32,
        Perspective,
        Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)),
    );
    let mut meshes: Vec<Mesh> = vec![Mesh::from(
        std::env::current_dir().unwrap().join("assets/teapot.obj"),
    )];

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run(move |event, target| {
            // Pre Update
            event_handler(event, target, &mut render, &mut input, &mut camera, &meshes);

            // Update
            update(&mut meshes);

            // Post Update
            time::update();
            render.window().request_redraw();
        })
        .unwrap();
}

fn event_handler<P>(
    event: Event<()>,
    target: &EventLoopWindowTarget<()>,
    state: &mut Render,
    input: &mut Input,
    camera: &mut Camera<P>,
    meshes: &[Mesh],
) where
    P: Projection,
{
    if let Event::DeviceEvent { event, .. } = &event {
        if let DeviceEvent::MouseMotion { delta } = event {
            input.mouse_motion = (delta.0 as f32, delta.1 as f32).into();
        }
    }
    if let Event::WindowEvent { event, .. } = event {
        match event {
            WindowEvent::Resized(new_size) => {
                log::debug!("Resized to: {:?}", new_size);
                state.resize(new_size, camera);
            }
            WindowEvent::RedrawRequested => {
                // point camera at origin
                // state.camera.point_at(glam::Vec3::splat(0.0));
                match state.render(meshes) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => {
                        log::error!("surface error: lost");
                        state.resize(state.size, camera)
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => target.exit(),
                    Err(e) => eprintln!("{e:?}"),
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                match event.state {
                    winit::event::ElementState::Pressed => {
                        let PhysicalKey::Code(key) = event.physical_key else {
                            unreachable!();
                        };
                        input.keyboard.press(key);
                    }
                    winit::event::ElementState::Released => {
                        let PhysicalKey::Code(key) = event.physical_key else {
                            unreachable!();
                        };
                        input.keyboard.release(key);
                    }
                }
                state.window().request_redraw();
            }
            WindowEvent::CloseRequested => target.exit(),
            _ => {}
        }
    }
}

fn update(entities: &mut [Mesh]) {
    for entity in entities.iter_mut() {
        entity.update();
    }
}

fn main() -> Result<()> {
    let event_loop = EventLoop::new().unwrap();
    #[allow(unused_mut)]
    let mut builder = winit::window::WindowBuilder::new();
    let window = builder
        .with_resizable(false)
        .with_inner_size(Size::Logical(LogicalSize {
            width: 800.0,
            height: 600.0,
        }))
        .build(&event_loop)
        .unwrap();

    env_logger::init();
    pollster::block_on(run(event_loop, window));

    Ok(())
}
