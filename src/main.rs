use rust_graphics::{Graphics, Orthographic, Perspective};
use winit::{
    dpi::{LogicalSize, Size},
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

async fn run(event_loop: EventLoop<()>, window: Window) {
    let mut graphics = Graphics::new(window, Perspective).await;
    event_loop
        .run(move |event, target| {
            let _ = &mut graphics;
            if let Event::WindowEvent {
                window_id: _,
                event,
            } = event
            {
                match event {
                    WindowEvent::Resized(new_size) => {
                        log::debug!("Resized to: {:?}", new_size);
                        graphics.resize(new_size);
                    }
                    WindowEvent::RedrawRequested => {
                        graphics.update();
                        match graphics.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => {
                                log::error!("surface error: lost");
                                graphics.resize(graphics.size)
                            }
                            Err(wgpu::SurfaceError::OutOfMemory) => target.exit(),
                            Err(e) => eprintln!("{e:?}"),
                        }
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        let key = event.physical_key;
                        let cam_speed = 1.0;
                        match event.state {
                            winit::event::ElementState::Pressed => {
                                let forward =
                                    graphics.camera.target - graphics.camera.eye;
                                let forward_norm = forward.normalize();
                                let forward_mag = forward.length();
                                if key == PhysicalKey::Code(KeyCode::Escape) {
                                    target.exit();
                                } else if key == PhysicalKey::Code(KeyCode::ArrowUp) {
                                    graphics.camera.eye += forward_norm * cam_speed;
                                } else if key == PhysicalKey::Code(KeyCode::ArrowDown) {
                                    graphics.camera.eye -= forward_norm * cam_speed;
                                } else if key == PhysicalKey::Code(KeyCode::ArrowRight)
                                {
                                    let right = forward_norm.cross(graphics.camera.up);
                                    graphics.camera.eye = graphics.camera.target
                                        - (forward + right * cam_speed).normalize()
                                            * forward_mag;
                                } else if key == PhysicalKey::Code(KeyCode::ArrowLeft) {
                                    let right = forward_norm.cross(graphics.camera.up);
                                    graphics.camera.eye = graphics.camera.target
                                        - (forward - right * cam_speed).normalize()
                                            * forward_mag;
                                }
                            }
                            winit::event::ElementState::Released => {}
                        }
                        graphics.window().request_redraw();
                    }
                    WindowEvent::CloseRequested => target.exit(),
                    _ => {}
                }
            }
        })
        .unwrap();
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    #[allow(unused_mut)]
    let mut builder = winit::window::WindowBuilder::new();
    let window = builder
        // .with_resizable(false)
        .with_inner_size(Size::Logical(LogicalSize {
            width: 800.0,
            height: 600.0,
        }))
        .build(&event_loop)
        .unwrap();

    env_logger::init();
    pollster::block_on(run(event_loop, window));
}
