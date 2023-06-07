#![allow(non_snake_case)]
use std::{ops::Deref, ffi::c_char};
use std::ffi::*;

use winit::dpi::{Size, PhysicalSize};
use winit::event::{ElementState, VirtualKeyCode};
use winit::window::Window;
use winit::{window::WindowBuilder, event_loop::{EventLoopBuilder, ControlFlow}};

mod shader;

static mut RENDERER: Option<Renderer> = None;

struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    render_pipelines: Vec<wgpu::RenderPipeline>,
}

#[no_mangle]
extern "C" fn run(
        name: *const c_char,
        size: *const (c_uint, c_uint),
        upFunc: extern fn(Keys) -> ()
    ) {
    let eventLoop = EventLoopBuilder::new().build();
    
    let window = unsafe {
        WindowBuilder::new()
            .with_title(CStr::from_ptr(
                name
            ).to_str().expect("Could not read title"))
            .with_inner_size(Size::Physical(PhysicalSize {
                width: (*size).0 as u32,
                height: (*size).1 as u32,
            }))
            .build(eventLoop.deref()).expect("couldn't create the window")
    };

    {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = pollster::block_on(instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        )).unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        )).unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())            
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(shader::SHAPES)),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let shape_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[], // 2.
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: "fs_main",
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

        unsafe { RENDERER = Some(Renderer {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipelines: vec![shape_render_pipeline]
        }) };
    }

    eventLoop.run(move |event,_,controlFlow| {
        *controlFlow = ControlFlow::Wait;

        let mut keys: Keys = Keys::Null;

        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *controlFlow = ControlFlow::Exit;
                    return;
                },
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == ElementState::Pressed {
                        match &input.virtual_keycode.unwrap() {
                            VirtualKeyCode::A => keys = Keys::A,
                            VirtualKeyCode::B => keys = Keys::B,
                            VirtualKeyCode::C => keys = Keys::C,
                            VirtualKeyCode::D => keys = Keys::D,
                            VirtualKeyCode::E => keys = Keys::E,
                            VirtualKeyCode::F => keys = Keys::F,
                            VirtualKeyCode::G => keys = Keys::G,
                            VirtualKeyCode::H => keys = Keys::H,
                            VirtualKeyCode::I => keys = Keys::I,
                            VirtualKeyCode::J => keys = Keys::J,
                            VirtualKeyCode::K => keys = Keys::K,
                            VirtualKeyCode::L => keys = Keys::L,
                            VirtualKeyCode::M => keys = Keys::M,
                            VirtualKeyCode::N => keys = Keys::N,
                            VirtualKeyCode::O => keys = Keys::O,
                            VirtualKeyCode::P => keys = Keys::P,
                            VirtualKeyCode::Q => keys = Keys::Q,
                            VirtualKeyCode::R => keys = Keys::R,
                            VirtualKeyCode::S => keys = Keys::S,
                            VirtualKeyCode::T => keys = Keys::T,
                            VirtualKeyCode::U => keys = Keys::U,
                            VirtualKeyCode::V => keys = Keys::V,
                            VirtualKeyCode::W => keys = Keys::W,
                            VirtualKeyCode::X => keys = Keys::X,
                            VirtualKeyCode::Y => keys = Keys::Y,
                            VirtualKeyCode::Z => keys = Keys::Z,
                            VirtualKeyCode::Key1 => keys = Keys::One,
                            VirtualKeyCode::Key2 => keys = Keys::Two,
                            VirtualKeyCode::Key3 => keys = Keys::Three,
                            VirtualKeyCode::Key4 => keys = Keys::Four,
                            VirtualKeyCode::Key5 => keys = Keys::Five,
                            VirtualKeyCode::Key6 => keys = Keys::Six,
                            VirtualKeyCode::Key7 => keys = Keys::Seven,
                            VirtualKeyCode::Key8 => keys = Keys::Eight,
                            VirtualKeyCode::Key9 => keys = Keys::Nine,
                            VirtualKeyCode::Key0 => keys = Keys::Zero,
                            _ => (),
                        }
                    }
                },
                _ => return,
            },
            _ => ()
        }

        (upFunc)(keys);
    })
}

#[no_mangle]
extern "C" fn draw() {
    match unsafe { &RENDERER } {
        Some(renderer) => {
            let output = match renderer.surface.get_current_texture() {
                Ok(v) => v,
                Err(er) => panic!("{}", er),
            };
            let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder = renderer.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    // This is what @location(0) in the fragment shader targets
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(
                                wgpu::Color {
                                    r: 0.1,
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                }
                            ),
                            store: true,
                        }
                    })
                ],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&renderer.render_pipelines[0]);
            render_pass.draw(0..3, 0..1); // 3.
        },
        &None => println!("Window is not running"),
    }
}

#[repr(C)]
enum Keys {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Null,
}