//! # Fish
//! This is the Fish game engine documentation. It is a 2d game engine that is meant to make pc development with rust easier for Keycap Studios.

#![allow(non_snake_case)]
#![allow(dead_code)]

pub use Log::*;
pub use wgpu;
pub use winit;

use std::{collections::HashSet, ops::Deref};
use Math::*;
use wgpu::{Backends, RenderPipeline};
use winit::{window::{Window, WindowBuilder}, event_loop::{EventLoopBuilder, EventLoop}, dpi::{Size, PhysicalSize}};

pub mod Math;
pub mod Sprite;
pub mod Log;
mod shader;

/// Decides the Dimseion of the game.
pub enum Dimension {
    /// Will make the game 2D
    TwoD,
    /// Will make the game 3D
    ThreeD,
}

/// gives the window and event loop to the engine. This does allow you to have access to the window and customize it to your liking
pub struct Context {
    /// This is used for making the window. Can be changed with the winit api.
    pub window: WindowBuilder,
    /// This is the event_loop builder. You can make your own, but why.
    pub event_loop: EventLoopBuilder<()>,
    /// The Dimesion of a the game. You can possible create your own system to make your game a mix of both, but it will not be offically supported.
    pub dim: Dimension,
}

impl Context {
    /// Creates a new context.
    pub fn new() -> Self {
        // Defines the Builders
        let event_loop: EventLoopBuilder<()> = EventLoopBuilder::new();
        let window: WindowBuilder = WindowBuilder::new()
            .with_resizable(false);

        // Returns the builder and dim
        Self {
            event_loop,
            window,
            dim: Dimension::TwoD,
        }
    }

    /// Changes the size of the window
    pub fn withSize(self, width: u32, height: u32) -> Self {
        Self { 
            window: self.window.with_inner_size(Size::Physical(PhysicalSize { width, height })), 
            event_loop: self.event_loop,
            dim: self.dim,
        }
    }

    /// Changes the title of the window
    pub fn withTitle(self, title: &str) -> Self {
        Self {
            window: self.window.with_title(title),
            event_loop: self.event_loop,
            dim: self.dim,
        }
    }

    /// Changes the Dimension of the engine
    pub fn withDim(self, dim: Dimension) -> Self {
        Self {
            dim,
            event_loop: self.event_loop,
            window: self.window,
        }
    }
}

/// This is the main Engine. This holds all of the backend variables that are required when rendering to a screen.
pub struct Engine<'a> {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    pipelines: Vec<RenderPipeline>,
    update: fn(&HashSet<u32>, &mut Vec<Enity>),
    ctx: (Window, EventLoop<()>, Dimension),
    enities: Vec<Enity>,
    updateFuncs: Vec<(&'a fn(&HashSet<u32>, &mut Enity), &'a Enity)>,
}

impl<'a> Engine<'a> {
    /// Creates a engine. Give Context to the engine.
    pub async fn new(mut context: Context) -> Engine<'a> {
        // This builds the event_loop and window for the rest of the function and engine.
        let eventLoop: EventLoop<()> = context.event_loop.build();
        let window: Window = context.window.build(eventLoop.deref()).unwrap();

        // gets the size of the window.
        let size = window.inner_size();

        // Creates a new Instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: Backends::PRIMARY, ..Default::default() });

        // The Engine takes all in all errors.
        let surface = unsafe {
            match instance.create_surface(&window) {
                Ok(v) => v,
                Err(e) => {
                    log!(LogType::Error, "Failed to create surface: {:?}", e); 
                    panic!("Failed to create surface: {:?}", e) // This is to prevet the error that it doesn't return Surface.
                },
            }
        };
        log!(LogType::Debug, "Surface created: {:?}", surface);

        // Creates a adapter
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

        // Creates a device and queue
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

        // creates the config
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

        // takes the shader and makes a module
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(String::from(shader::SHADER).into()),
        });

        // Makes the render pipeline
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        // Makes the vertex render pipeline
        let vertex_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vertex_main",
                buffers: &[],
            }, // I'm not touching the rest of the struct
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

        log!(LogType::Debug, "Engine was created successfully");

        Self {
            size,
            surface,
            device,
            queue,
            config,
            // Will hold may pipelines for rendering images and particles
            pipelines: vec![vertex_pipeline],
            update: Self::update,
            ctx: (window, eventLoop, context.dim),
            enities: vec![],
            updateFuncs: vec![],
        }
    }

    // Simple eplace holder function for the new function
    fn update(_input: &HashSet<u32>, _entites: &mut Vec<Enity>) {
        log!(LogType::Warning, "No update Function was given");
    }

    /// Runs the Engine.
    pub fn run(self) {
        let mut inputMap: HashSet<u32> = HashSet::new();

        self.ctx.1.run(move |event, _, control_flow| {
            // Makes it able to hold multiply inputs
            *control_flow = winit::event_loop::ControlFlow::Poll; 
            match event {
                // Inserts the input into the inputMap
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
                // Checks for a close request
                winit::event::Event::WindowEvent { event: winit::event::WindowEvent::CloseRequested, .. } => *control_flow = winit::event_loop::ControlFlow::Exit,
                // This is each frame
                winit::event::Event::MainEventsCleared => {
                    self;
                },
                _ => {},
            }
        });
    }

    /// Inserts the given update function into the Engine. The update replaces the old one and is the global one.
    pub fn insertUpdate(&mut self, func: fn(&HashSet<u32>, &mut Vec<Enity>)) {
        self.update = func;
    }

    /// Inserts Enities into the game engine and then returns their id to be able to be used in the global update function.
    pub fn insertEnities(&mut self, enity: Enity) -> i32 {
        // Genorates the id (plzs dont import the rand crate)
        let id = {
            let mut id = 1;
            for ent in &self.enities {
                id += ent.id;
            }
            id
        };
        // pushes the input
        self.enities.push(enity);
        id
    }
}

/// Used for implemnting enities into the ecs
pub struct Enity {
    /// The Vector position. You can use what ever struct you want as long as it impl Vector
    pub pos: Box<dyn Vector>,
    /// Weather it should 
    pub active: bool,
    /// The update function of the Enity 
    pub update: fn(&HashSet<u32>, &mut Enity),
    /// This is a tupple that holds the did components of the Enity
    pub traits: (),
    /// The objects id
    pub id: i32,
}

impl Enity {
    /// Creates a Enity
    pub fn new() -> Self {
        Self {
            pos: Box::new(Vec2::new([0f32,0f32])),
            active: true,
            update: Enity::update,
            traits: (),
            id: 0, 
        }
    }

    // Place holder update function for the new function
    fn update(_: &HashSet<u32>, _: &mut Enity) {}

    /// Inserts a new update method for the enity
    pub fn insertUpdate(&mut self, func: fn(&HashSet<u32>, &mut Enity)) {
        self.update = func;
    }
}

// Tests
#[cfg(test)]
mod tests {

    // Tests for the log mod
    mod log {
        use crate::{log, LogType};

        #[test]
        fn success() {
            // This is testing all parameter cases
            log!(LogType::Debug, "test");
            log!(LogType::Debug, "test: {}", 1);
            log!(LogType::Warning, "test");
            log!(LogType::Warning, "test: {}", 2);
        }

        #[test]
        #[should_panic]
        fn failure() {
            // Same as the success, but with error
            log!(LogType::Error, "test");
            log!(LogType::Error, "test: {}", 1);
        }
    }

    // Tests for the enity struct
    mod enity {
        use crate::Enity;

        // tests the Creation of the struct
        #[test]
        fn creation() {
            Enity::new();
        }
    }
}