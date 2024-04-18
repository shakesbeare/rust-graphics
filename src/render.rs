use crate::camera::{Camera, Projection};
use crate::mesh::Mesh;
use crate::texture;
use crate::transform::Transform;
use crate::vertex::Vertex;
use std::borrow::Cow;
use wgpu::util::DeviceExt;
use winit::window::Window;

pub struct Buffers {
    vertex: wgpu::Buffer,
    index: wgpu::Buffer,
    uniform: wgpu::Buffer,
}

pub struct RenderTextures {
    depth_texture: texture::Texture,
}

pub struct BindGroups {
    camera_bind_group: wgpu::BindGroup,
}

pub struct Render<'a> {
    queue: wgpu::Queue,
    buffers: Buffers,
    _instance: wgpu::Instance,
    device: wgpu::Device,
    config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface<'a>,
    pub size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    render_pipeline: wgpu::RenderPipeline,
    render_textures: RenderTextures,
    bind_groups: BindGroups,
}

impl<'a> Render<'a> {
    pub async fn new<P: Projection>(window: Window, projection_type: P) -> Self {
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

        let path = std::env::current_dir().unwrap().join("assets/teapot.obj");
        let mesh = Mesh::from(path);

        let vertex_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&mesh.vertices_transformed()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });

        let index_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&mesh.indices),
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            });

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();

        let camera = Camera::new(
            90.0,
            config.width as f32 / config.height as f32,
            projection_type,
            Transform::from_translation(glam::Vec3::new(0.0, 0.0, -10.0)),
        );

        let mx_total = camera.projection_matrix();
        let mx_ref: &[f32; 16] = mx_total.as_ref();

        let uniform_buffer =
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
                resource: uniform_buffer.as_entire_binding(),
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
            _instance: instance,
            device,
            queue,
            config,
            surface,
            size,
            window,
            render_pipeline,
            bind_groups: BindGroups { camera_bind_group },
            buffers: Buffers {
                vertex: vertex_buffer,
                index: index_buffer,
                uniform: uniform_buffer,
            },
            render_textures: RenderTextures { depth_texture },
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize<P: Projection>(
        &mut self,
        new_size: winit::dpi::PhysicalSize<u32>,
        camera: &mut Camera<P>,
    ) {
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.render_textures.depth_texture =
            texture::create_depth_texture(&self.device, &self.config, "depth_texture");

        camera.aspect_ratio = self.size.width as f32 / self.size.height as f32;

        let mx_total = camera.projection_matrix();
        let mx_ref: &[f32; 16] = mx_total.as_ref();

        self.queue
            .write_buffer(&self.buffers.uniform, 0, bytemuck::cast_slice(mx_ref));

        self.surface.configure(&self.device, &self.config);
        self.window.request_redraw();
    }

    pub fn render(&mut self, meshes: &[Mesh]) -> Result<(), wgpu::SurfaceError> {
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
                            view: &self.render_textures.depth_texture.view,
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
            render_pass.set_bind_group(0, &self.bind_groups.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.buffers.vertex.slice(..));
            render_pass.set_index_buffer(
                self.buffers.index.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            for mesh in meshes {
                self.queue.write_buffer(
                    &self.buffers.vertex,
                    0,
                    bytemuck::cast_slice(&mesh.vertices_transformed()),
                );

                self.queue.write_buffer(
                    &self.buffers.index,
                    0,
                    bytemuck::cast_slice(&mesh.indices),
                );
                render_pass.draw_indexed(0..mesh.indices.len() as u32, 0, 0..1);
            }
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}
