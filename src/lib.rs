//! # Fish
//! this is the Fish game engine documentation. It is a 2d game engine that is meant to make pc development with rust easier for Keycap Studios.
//! ## Getting Started
//! to 

#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{collections::HashSet, ops::Deref};
use log::{log, Level};
use wgpu::{Backends, RenderPipeline};
use winit::{window::{Window, WindowBuilder},event_loop::{EventLoopBuilder, EventLoop}, dpi::{Size, PhysicalSize}};

pub mod math;
pub mod sprite;
mod shader;

/// gives the window and event loop to the engine. This does allow you to have access to the window and customize it to your liking
pub struct Context {
    pub window: WindowBuilder,
    pub event_loop: EventLoopBuilder<()>,
}

impl Context {
    /// Creates a new context.
    pub fn new() -> Self {
        let event_loop: EventLoopBuilder<()> = EventLoopBuilder::new();
        let window: WindowBuilder = WindowBuilder::new()
            .with_resizable(false);

        Self {
            event_loop,
            window
        }
    }

    /// Changes the size of the window.
    pub fn withSize(self, width: u32, height: u32) -> Self {
        Self { 
            window: self.window.with_inner_size(Size::Physical(PhysicalSize { width, height})), 
            event_loop: self.event_loop
        }
    }

    /// Changes the title of the window.
    pub fn withTitle(self, title: &str) -> Self {
        Self {
            window: self.window.with_title(title),
            event_loop: self.event_loop
        }
    }
}

/// This is the main Engine. This holds all of the backend variables that are required when rendering to a screen.
pub struct Engine {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    pipelines: Vec<RenderPipeline>,
    update: fn(&HashSet<u32>),
    ctx: (Window, EventLoop<()>)
}

impl Engine {
    /// Creates a engine. Give Context to the engine.
    async fn new(mut context: Context) -> Engine {
        let eventLoop: EventLoop<()> = context.event_loop.build();
        let window: Window = context.window.build(eventLoop.deref()).unwrap();


        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: Backends::PRIMARY, ..Default::default() });

        let surface = unsafe {
            match instance.create_surface(&window) {
                Ok(v) => {log!(Level::Trace, "Surface was created");v},
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
                Some(v) => {log!(Level::Trace, "Adapter was created");v},
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
            update: |input| print!("No Update Function. Input: {:?}", input),
            ctx: (window, eventLoop)
        }
    }

    /// Runs the Engine.
    fn run(self) {
        let mut inputMap: HashSet<u32> = HashSet::new();
        let mut updates: Vec<fn(&HashSet<u32>)> = Vec::new();

        updates.push(self.update);

        self.ctx.1.run(move |event, _, control_flow| {
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

    /// Inserts the given update function into the Engine. The update replaces the old one and is the global one.
    fn update(mut self, func: fn(&HashSet<u32>)) {
        self.update = func;
    }
}