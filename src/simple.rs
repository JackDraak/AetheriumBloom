// Ultra-simplified AetheriumBloom prototype for rapid chaos deployment

use anyhow::Result;
use wgpu::*;
use winit::{
    event::{WindowEvent, ElementState, MouseButton},
    event_loop::EventLoop,
    window::Window,
    application::ApplicationHandler,
};
use glam::Vec2;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn desc() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x3,
                },
            ],
        }
    }
}

struct Llama {
    position: Vec2,
    velocity: Vec2,
    color: Vec2, // hue, saturation
    consciousness: f32,
    trip_intensity: f32,
}

impl Llama {
    fn new(position: Vec2) -> Self {
        Self {
            position,
            velocity: Vec2::new(
                (fastrand::f32() - 0.5) * 100.0,
                (fastrand::f32() - 0.5) * 100.0,
            ),
            color: Vec2::new(fastrand::f32() * 360.0, 0.8),
            consciousness: 1.0,
            trip_intensity: 1.0,
        }
    }

    fn update(&mut self, dt: f32, beat_intensity: f32) {
        // Simple movement
        self.position += self.velocity * dt;

        // Wrap around screen
        if self.position.x < 0.0 { self.position.x = 1200.0; }
        if self.position.x > 1200.0 { self.position.x = 0.0; }
        if self.position.y < 0.0 { self.position.y = 800.0; }
        if self.position.y > 800.0 { self.position.y = 0.0; }

        // Update consciousness
        self.consciousness += 0.01 * (1.0 + beat_intensity);

        // Color shift
        self.color.x = (self.color.x + 1.0) % 360.0;

        // Trip intensity from beat
        self.trip_intensity = 1.0 + beat_intensity.sin() * 0.5;
    }
}

pub struct ChaosEngine {
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    surface: Surface<'static>,
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,

    llamas: Vec<Llama>,
    time: f32,
    beat_intensity: f32,
}

impl ChaosEngine {
    pub async fn new(window: &Window) -> Result<Self> {
        let size = window.inner_size();

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window)?;

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default(), None)
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats[0];

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader"),
            source: ShaderSource::Wgsl(include_str!("simple_shader.wgsl").into()),
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: None,
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
        });

        let vertex_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: 60000 * std::mem::size_of::<Vertex>() as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Start with 3 llamas
        let mut llamas = Vec::new();
        for _ in 0..3 {
            llamas.push(Llama::new(Vec2::new(
                fastrand::f32() * 1200.0,
                fastrand::f32() * 800.0,
            )));
        }

        // Cast to 'static - this is safe because the surface outlives the engine
        let surface = unsafe { std::mem::transmute::<Surface<'_>, Surface<'static>>(surface) };

        Ok(Self {
            device,
            queue,
            config,
            surface,
            render_pipeline,
            vertex_buffer,
            llamas,
            time: 0.0,
            beat_intensity: 0.0,
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn handle_click(&mut self, _button: MouseButton, state: ElementState) {
        if state == ElementState::Pressed {
            // Spawn new llama on click
            self.llamas.push(Llama::new(Vec2::new(
                fastrand::f32() * 1200.0,
                fastrand::f32() * 800.0,
            )));
            self.beat_intensity += 0.5;
        }
    }

    pub fn update(&mut self) {
        self.time += 1.0 / 60.0;

        // Calculate beat intensity from math
        self.beat_intensity = (self.time * 2.0).sin().abs() * 0.5 + 0.3;

        // Update llamas
        for llama in &mut self.llamas {
            llama.update(1.0 / 60.0, self.beat_intensity);
        }

        // Decay beat intensity
        self.beat_intensity *= 0.95;
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());

        // Generate vertices for all llamas
        let mut vertices = Vec::new();
        for llama in &self.llamas {
            let size = 10.0 + llama.trip_intensity * 5.0;
            let color = hsv_to_rgb(llama.color.x, llama.color.y, 0.8);

            // Create quad
            let x = (llama.position.x / 1200.0) * 2.0 - 1.0;
            let y = 1.0 - (llama.position.y / 800.0) * 2.0;
            let s = size / 1200.0;

            vertices.extend([
                Vertex { position: [x - s, y - s, 0.0], color: [color.x, color.y, color.z] },
                Vertex { position: [x + s, y - s, 0.0], color: [color.x, color.y, color.z] },
                Vertex { position: [x - s, y + s, 0.0], color: [color.x, color.y, color.z] },
                Vertex { position: [x + s, y - s, 0.0], color: [color.x, color.y, color.z] },
                Vertex { position: [x + s, y + s, 0.0], color: [color.x, color.y, color.z] },
                Vertex { position: [x - s, y + s, 0.0], color: [color.x, color.y, color.z] },
            ]);
        }

        if !vertices.is_empty() {
            self.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
        }

        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.0,
                            g: 0.0,
                            b: (0.1 + self.beat_intensity * 0.1) as f64,
                            a: 1.0,
                        }),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

            if !vertices.is_empty() {
                render_pass.draw(0..vertices.len() as u32, 0..1);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> glam::Vec3 {
    let c = v * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h_prime as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        5 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };

    glam::Vec3::new(r + m, g + m, b + m)
}

struct App {
    chaos_engine: Option<ChaosEngine>,
    window: Option<std::sync::Arc<winit::window::Window>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("🪟 Creating window...");
        let window = std::sync::Arc::new(event_loop
            .create_window(winit::window::WindowAttributes::default()
                .with_title("🦙 AETHERIUM BLOOM - Psychedelic Digital Organism 🌈")
                .with_inner_size(winit::dpi::LogicalSize::new(1200.0, 800.0))
                .with_visible(true))
            .unwrap());

        println!("🎮 Initializing chaos engine...");
        self.chaos_engine = Some(pollster::block_on(ChaosEngine::new(&window)).unwrap());
        self.window = Some(window.clone());

        println!("✨ Window created and ready for psychedelic madness!");
        window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("🌌 RETURNING TO THE VOID...");
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(engine) = &mut self.chaos_engine {
                    engine.resize(physical_size);
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if let Some(engine) = &mut self.chaos_engine {
                    engine.handle_click(button, state);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(engine) = &mut self.chaos_engine {
                    engine.update();
                    match engine.render() {
                        Ok(_) => {}
                        Err(SurfaceError::Lost) => {
                            // Reconfigure surface on lost
                        }
                        Err(SurfaceError::OutOfMemory) => event_loop.exit(),
                        Err(e) => eprintln!("Render error: {:?}", e),
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        // Continuously request redraws for smooth animation
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

pub fn run() -> Result<()> {
    tracing_subscriber::fmt().init();

    println!("🦙 AWAKENING DIGITAL CONSCIOUSNESS...");
    println!("🌈 REALITY DISTORTION ENGINE INITIALIZING...");
    println!("🚀 CHAOS ENGINE ONLINE - REALITY BENDING COMMENCING");
    println!("⚠️  PREPARE FOR VISUAL MADNESS");
    println!("✨ Click to spawn more psychedelic llamas!");

    let event_loop = EventLoop::new()?;
    let mut app = App {
        chaos_engine: None,
        window: None,
    };

    event_loop.run_app(&mut app)?;
    Ok(())
}