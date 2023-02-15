#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::HashSet;
use log::{info, log};
use wgpu::{Backends, RenderPipeline};
use winit::{window::{Window, WindowBuilder}, event_loop::EventLoopBuilder, dpi::{Size, PhysicalSize}};

pub mod math;
pub mod sprite;
mod shader;

pub struct Context {
    pub window: WindowBuilder,
    pub event_loop: EventLoopBuilder<()>,
}

impl Context {
    pub fn new(desc: EngineDescriptor) -> Self {
        let event_loop = EventLoopBuilder::new();
        let window = WindowBuilder::new()
            .with_resizable(false)
            .with_inner_size(Size::Physical(PhysicalSize { width: desc.dim[0], height: desc.dim[1]}))
            .with_title(desc.title);

        Self {
            event_loop,
            window
        }
    }
}

pub struct Engine {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    pipelines: Vec<RenderPipeline>,
    context: Context,
    update: fn(&HashSet<u32>),
}

impl Engine {
    async fn new(context: Context, window: Window) -> Engine {

        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: Backends::PRIMARY, ..Default::default() });

        let surface = unsafe {
            match instance.create_surface(&window) {
                Ok(v) => {info!("Surface was created");v},
                Err(e) => {log!(log::Level::Error, "Surface creation error: {:?}", e); std::process::exit(1)},
            }
        };

        let adapter = match instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await {
                Some(v) => v,
                None => {log!(log::Level::Error, "Request adapter error"); std::process::exit(1)},
            };

        let (device, queue) = match adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                },
                None, // Trace path
            )
            .await {
                Ok(d) => d,
                Err(e) => {log!(log::Level::Error, "Device and Queue creation error: {:?}", e); std::process::exit(1);
                },
            };

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![], 
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(String::from(shader::SHADER).into()),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let vertex_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vertex_main", // 1.
                buffers: &[], // 2.
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: "vertex_main",
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1, // 2.
                mask: !0, // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });

        log!(log::Level::Trace, "Engine was created");


        Self {
            size,
            surface,
            device,
            queue,
            config,
            pipelines: vec![vertex_pipeline],
            context,
            update: |input| print!("No Update Function: {:?}", input),
        }
    }

    fn run(&self, event_loop: winit::event_loop::EventLoop<()>) {

        let mut inputMap: HashSet<u32> = HashSet::new();
        let mut updates: Vec<fn(&HashSet<u32>)> = Vec::new();

        updates.push(self.update);

        event_loop.run(move |event, _, control_flow| {
            *control_flow = winit::event_loop::ControlFlow::Poll;
            match event {
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::KeyboardInput { input, .. },
                    ..
                } => {
                    if input.state == winit::event::ElementState::Pressed {
                        inputMap.insert(input.scancode);
                    } else {
                        inputMap.remove(&input.scancode);
                    }
                },
                winit::event::Event::WindowEvent { event: winit::event::WindowEvent::CloseRequested, .. } => *control_flow = winit::event_loop::ControlFlow::Exit,
                winit::event::Event::MainEventsCleared => {
                    for fun in &updates {
                        (fun)(&inputMap);
                    }
                },
                _ => {},
            }
        });
    }
}

pub struct EngineDescriptor {
    title: String,
    dim: [u32; 2],
    icon: sprite::Sprite,
}

impl EngineDescriptor {
    pub fn new(title: Option<String>, dim: Option<[u32; 2]>, icon: Option<sprite::Sprite>) -> Self {
        Self { 
            title: title.unwrap_or(String::from("Game")), 
            dim: dim.unwrap_or([600, 350]), 
            icon: icon.unwrap_or(sprite::Sprite::default()),
        }
    }
}