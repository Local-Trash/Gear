//! # Fish
//! This is the Fish game engine documentation. It is a 2d game engine that is meant to make pc development with rust easier for Keycap Studios.
//! ## Getting Started
//! This is the code you can use to get started and create a window.
//! ```toml
//! [dependencies]
//! pollster = "*"
//! fish = { git = "https://github.com/Local-Trash/Fish" }
//! ```
//! ```rust
//! let ctx = fish::Context::new()
//!     .withTitle("Example");
//! let engine = pollster::block_on(fish::Engine::new());
//! engine.run();
//! ```

#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{collections::HashSet, ops::Deref};
use Math::*;
pub use Log::*;
use wgpu::{Backends, RenderPipeline};
use winit::{window::{Window, WindowBuilder}, event_loop::{EventLoopBuilder, EventLoop}, dpi::{Size, PhysicalSize}};
pub use wgpu;
pub use winit;

pub mod Math;
pub mod Sprite;
pub mod Log;
mod shader;

/// gives the window and event loop to the engine. This does allow you to have access to the window and customize it to your liking
pub struct Context {
    /// This is used for making the window. Can be changed with the winit api.
    pub window: WindowBuilder,
    /// This is the event_loop builder. You can make your own, but why.
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
pub struct Engine<V> where V: Vectors {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    pipelines: Vec<RenderPipeline>,
    update: fn(&HashSet<u32>),
    ctx: (Window, EventLoop<()>),
    enities: Vec<Enity<V>>
}

impl<V> Engine<V> where V: Vectors {
    /// Creates a engine. Give Context to the engine.
    pub async fn new(mut context: Context) -> Engine<V> {
        let eventLoop: EventLoop<()> = context.event_loop.build();
        let window: Window = context.window.build(eventLoop.deref()).unwrap();


        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: Backends::PRIMARY, ..Default::default() });

        let surface = unsafe {
            match instance.create_surface(&window) {
                Ok(v) => v,
                Err(e) => log!(LogType::Error, "Failed to create surface: {:?}", e),
            }
        };

        log!(LogType::Debug, "Surface created: {:?}", surface);

        let adapter = match instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await {
                Some(v) => v,
                None => panic!("Failed to find an appropriate adapter."),
            };

        let (device, queue) = match adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await {
                Ok(d) => d,
                Err(e) => panic!("Failed to create device: {:?}", e),
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
                entry_point: "vertex_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "vertex_main",
                targets: &[Some(wgpu::ColorTargetState { 
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false, 
            },
            multiview: None,
        });

        Self {
            size,
            surface,
            device,
            queue,
            config,
            pipelines: vec![vertex_pipeline],
            update: |_| log!(LogType::Warning, "No global update function was given."),
            ctx: (window, eventLoop),
            enities: vec![]
        }
    }

    /// Runs the Engine.
    pub fn run(self) {
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
                    for func in &updates {
                        (func)(&inputMap);
                    }
                },
                _ => {},
            }
        });
    }

    /// Inserts the given update function into the Engine. The update replaces the old one and is the global one.
    pub fn insertUpdate(mut self, func: fn(&HashSet<u32>)) {
        self.update = func;
    }

    /// Inserts Enities into the game engine and then returns their id to be able to be used in the global update function.
    pub fn insertEnities(&mut self, enity: Enity<V>) -> f32 {
        let id = {
            let mut id = 1.0;
            for ent in &self.enities {
                id += ent.id;
            }
            id
        };
        self.enities.push(enity);
        id
    }
}

/// Used for implemnting enities into the ecs
pub struct Enity<V> where V: Vectors
{
    /// The Vector position
    pub pos: V::Vector,
    /// Weather it should 
    pub active: bool,
    /// The update function of the Enity
    pub update: (),
    /// This is a tupple that holds the did components of the Enity
    pub traits: (),
    /// The objects id
    pub id: f32,
}