use rust_graphics::camera::{Perspective, Projection};
use rust_graphics::State;
use winit::{
    dpi::{LogicalSize, Size},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

async fn run(event_loop: EventLoop<()>, window: Window) {
    let mut state = State::new(window, Perspective).await;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run(move |event, target| {
            // main event loop
            event_handler(event, target, &mut state);
            update(&mut state);
        })
        .unwrap();
}

fn update<P>(state: &mut State<P>)
where
    P: Projection,
{
    state.update();

    let input = &state.input;
    let mut move_vec = glam::Vec3::splat(0.0);
    let move_speed = 5.0;
    if input.pressed(KeyCode::ArrowLeft) {
        move_vec.x -= move_speed;
    }

    if input.pressed(KeyCode::ArrowRight) {
        move_vec.x += move_speed;
    }

    if input.pressed(KeyCode::ArrowUp) {
        move_vec.z -= move_speed;
    }

    if input.pressed(KeyCode::ArrowDown) {
        move_vec.z += move_speed;
    }

    if input.pressed(KeyCode::KeyQ) {
        move_vec.y += move_speed;
    }

    if input.pressed(KeyCode::KeyE) {
        move_vec.y -= move_speed;
    }

    let local_x_axis = state.camera.dir.cross(state.camera.up).normalize();
    let local_z_axis = -state.camera.dir.normalize();
    let move_vec = move_vec.x * local_x_axis
        + move_vec.z * local_z_axis
        + move_vec.y * state.camera.up.normalize();

    let new_location = state.camera.location + move_vec * state.delta_time;
    state.camera.move_camera(new_location);

    let rot = 30.0_f32.to_radians() * state.delta_time;
    let cube_rotate = glam::Quat::from_euler(glam::EulerRot::XYZ, rot, rot, 0.0);
    state.mesh.transform.rotation = state.mesh.transform.rotation.mul_quat(cube_rotate);


    state.window().request_redraw();
}

fn event_handler<P>(
    event: Event<()>,
    target: &EventLoopWindowTarget<()>,
    state: &mut State<P>,
) where
    P: Projection,
{
    if let Event::WindowEvent {
        window_id: _,
        event,
    } = event
    {
        match event {
            WindowEvent::Resized(new_size) => {
                log::debug!("Resized to: {:?}", new_size);
                state.resize(new_size);
            }
            WindowEvent::RedrawRequested => {
                // point camera at origin
                // state.camera.point_at(glam::Vec3::splat(0.0));
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => {
                        log::error!("surface error: lost");
                        state.resize(state.size)
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
                        state.input.press(key);
                    }
                    winit::event::ElementState::Released => {
                        let PhysicalKey::Code(key) = event.physical_key else {
                            unreachable!();
                        };
                        state.input.release(key);
                    }
                }
                state.window().request_redraw();
            }
            WindowEvent::CloseRequested => target.exit(),
            _ => {}
        }
    }
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
