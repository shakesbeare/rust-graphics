pub mod texture;

use std::borrow::Cow;
use wgpu::util::DeviceExt;
use winit::{event::WindowEvent, window::Window};

const VERTICES: &[Vertex] = &[
    // top (0, 0, 1)
    Vertex {
        position: [-1.0, -1.0, 1.0, 1.0],
        color: [0.0, 1.0, 0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, 1.0, 1.0],
        color: [0.0, 1.0, 0.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 1.0, 1.0],
        color: [0.0, 1.0, 0.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, 1.0, 1.0],
        color: [0.0, 1.0, 0.0, 1.0],
    },
    // bottom (0, 0, -1)
    Vertex {
        position: [-1.0, 1.0, -1.0, 1.0],
        color: [0.0, 1.0, 0.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, -1.0, 1.0],
        color: [0.0, 1.0, 0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, -1.0, 1.0],
        color: [0.0, 1.0, 0.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0, -1.0, 1.0],
        color: [0.0, 1.0, 0.0, 1.0],
    },
    // right (1.0, 0, 0)
    Vertex {
        position: [1.0, -1.0, -1.0, 1.0],
        color: [0.0, 0.0, 1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, -1.0, 1.0],
        color: [0.0, 0.0, 1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 1.0, 1.0],
        color: [0.0, 0.0, 1.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, 1.0, 1.0],
        color: [0.0, 0.0, 1.0, 1.0],
    },
    // left (-1.0, 0, 0)
    Vertex {
        position: [-1.0, -1.0, 1.0, 1.0],
        color: [0.0, 0.0, 1.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, 1.0, 1.0],
        color: [0.0, 0.0, 1.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, -1.0, 1.0],
        color: [0.0, 0.0, 1.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0, -1.0, 1.0],
        color: [0.0, 0.0, 1.0, 1.0],
    },
    // front (0, 1.0, 0)
    Vertex {
        position: [1.0, 1.0, -1.0, 1.0],
        color: [1.0, 0.0, 0.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, -1.0, 1.0],
        color: [1.0, 0.0, 0.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, 1.0, 1.0],
        color: [1.0, 0.0, 0.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 1.0, 1.0],
        color: [1.0, 0.0, 0.0, 1.0],
    },
    // back (0, -1.0, 0)
    Vertex {
        position: [1.0, -1.0, 1.0, 1.0],
        color: [1.0, 0.0, 0.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0, 1.0, 1.0],
        color: [1.0, 0.0, 0.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0, -1.0, 1.0],
        color: [1.0, 0.0, 0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, -1.0, 1.0],
        color: [1.0, 0.0, 0.0, 1.0],
    },
];

#[rustfmt::skip]
const INDICES: &[u16] = &[
        0, 1, 2, 2, 3, 0, // top
        4, 5, 6, 6, 7, 4, // bottom
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back];
];

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: [f32; 4],
    pub color: [f32; 4],
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x4];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub struct Graphics<'a, P: Projection> {
    instance: wgpu::Instance,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'a>,
    pub camera: Camera<P>,
    camera_uniform_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    window: Window,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    depth_texture: texture::Texture,
}

impl<'a, P: Projection> Graphics<'a, P> {
    pub async fn new(window: Window, projection_type: P) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::default();
        let surface = {
            let window_ptr = &window as *const Window;
            // SAFETY:
            //   Self owns both window and surface
            instance.create_surface(unsafe { &*window_ptr }).unwrap()
        };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Adapter should be appropriate for the given surface");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                "shader.wgsl"
            ))),
        });

        let vertex_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            });

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();

        let camera = Camera::new(
            45.0,
            config.width as f32,
            config.height as f32,
            projection_type,
        );
        {
            let cam = &camera;
            let proj = cam.projection_matrix();
            for v in VERTICES {
                let vec = glam::Vec4::from(v.position);
                log::info!("{:?} x {:?} = ", vec, proj);
            }
            log::info!("{proj:?}");
        }

        let mx_total = camera.projection_matrix();
        let mx_ref: &[f32; 16] = mx_total.as_ref();

        let camera_uniform_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(mx_ref),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera_bind_group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_uniform_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let depth_texture =
            texture::create_depth_texture(&device, &config, "depth_texture");
        let render_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(swapchain_format.into())],
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: texture::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });

        Self {
            instance,
            device,
            queue,
            config,
            surface,
            camera,
            camera_uniform_buffer,
            camera_bind_group,
            size,
            window,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            depth_texture,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.depth_texture =
            texture::create_depth_texture(&self.device, &self.config, "depth_texture");

        self.camera.aspect_ratio = self.size.width as f32 / self.size.height as f32;

        let mx_total = self.camera.projection_matrix();
        let mx_ref: &[f32; 16] = mx_total.as_ref();

        self.queue.write_buffer(
            &self.camera_uniform_buffer,
            0,
            bytemuck::cast_slice(mx_ref),
        );

        self.surface.configure(&self.device, &self.config);
        self.window.request_redraw();
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
        let mx_total = self.camera.projection_matrix();
        let mx_ref: &[f32; 16] = mx_total.as_ref();

        self.queue.write_buffer(
            &self.camera_uniform_buffer,
            0,
            bytemuck::cast_slice(mx_ref),
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self
            .surface
            .get_current_texture()
            .expect("failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut render_pass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(
                        wgpu::RenderPassDepthStencilAttachment {
                            view: &self.depth_texture.view,
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Clear(1.0),
                                store: wgpu::StoreOp::Store,
                            }),
                            stencil_ops: None,
                        },
                    ),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                self.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
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

pub struct Camera<P: Projection> {
    #[allow(unused)]
    proj: P,
    pub eye: glam::Vec3,
    pub up: glam::Vec3,
    aspect_ratio: f32,
    pub target: glam::Vec3,
    fov: f32,
    z_near: f32,
    z_far: f32,
}

impl<P: Projection> Camera<P> {
    pub fn new(fov_degrees: f32, width: f32, height: f32, projection_type: P) -> Self {
        let eye = glam::Vec3::new(1.35f32, -5.0, 10.0);
        let target = glam::Vec3::new(0.0, 0.0, 0.0);
        let forward = target - eye.normalize();
        let a = 1.0;
        let b = 1.0;
        let c = (eye.x + eye.y) / 10.0;

        let v = glam::Vec3::new(a, b, c);
        let v_n = v.normalize();
        let v_unit = v / v_n;

        Camera {
            proj: projection_type,
            eye: eye,
            target: forward,
            up: v_unit,
            aspect_ratio: width / height,
            fov: fov_degrees.to_radians(),
            z_near: 0.1,
            z_far: 1000.0,
        }
    }
    pub fn projection_matrix(&self) -> glam::Mat4 {
        P::generate_view_projection_matrix(
            self.aspect_ratio,
            self.eye,
            self.up,
            self.fov,
            self.target,
            self.z_near,
            self.z_far,
        )
    }
}
