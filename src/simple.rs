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

#[derive(Clone)]
struct Llama {
    // Core properties
    position: Vec2,
    velocity: Vec2,
    color: Vec2, // hue, saturation
    consciousness: f32,
    trip_intensity: f32,

    // Phase 1: Consciousness Depth Enhancement
    awareness_level: f32,           // 0.0-1.0 consciousness depth
    memory_fragments: Vec<Vec2>,    // Historical position memories (interesting locations)
    social_bonds: Vec<usize>,       // Connections to other llamas (indices)
    personality_matrix: [f32; 7],   // 7-dimensional personality traits
    reality_distortion: f32,        // Local space-time manipulation factor

    // Internal consciousness state
    emotional_state: f32,           // Current emotional resonance
    memory_intensity: f32,          // How strongly memories are forming
    social_attraction: f32,         // Desire to be near other llamas
    exploration_drive: f32,         // Tendency to seek new areas
}

impl Llama {
    fn new(position: Vec2) -> Self {
        // Generate unique personality matrix (7 traits)
        let personality_matrix = [
            fastrand::f32(),        // Curiosity
            fastrand::f32(),        // Sociability
            fastrand::f32(),        // Chaos affinity
            fastrand::f32(),        // Memory strength
            fastrand::f32(),        // Emotional volatility
            fastrand::f32(),        // Reality sensitivity
            fastrand::f32(),        // Exploration drive
        ];

        Self {
            // Core properties
            position,
            velocity: Vec2::new(
                (fastrand::f32() - 0.5) * 100.0,
                (fastrand::f32() - 0.5) * 100.0,
            ),
            color: Vec2::new(fastrand::f32() * 360.0, 0.8),
            consciousness: 1.0,
            trip_intensity: 1.0,

            // Phase 1: Consciousness Depth Enhancement
            awareness_level: fastrand::f32() * 0.3 + 0.1, // Start with low awareness
            memory_fragments: Vec::new(),
            social_bonds: Vec::new(),
            personality_matrix,
            reality_distortion: 0.0,

            // Internal consciousness state
            emotional_state: fastrand::f32(),
            memory_intensity: personality_matrix[3] * 0.5, // Based on memory strength trait
            social_attraction: personality_matrix[1] * 0.7, // Based on sociability trait
            exploration_drive: personality_matrix[6] * 0.8, // Based on exploration trait
        }
    }

    fn update(&mut self, dt: f32, beat_intensity: f32, all_llamas: &[Llama], my_index: usize) {
        // === Consciousness Evolution ===
        // Awareness grows with time and beat intensity, modulated by personality
        let consciousness_growth = 0.01 * (1.0 + beat_intensity) * self.personality_matrix[0]; // Curiosity trait
        self.awareness_level = (self.awareness_level + consciousness_growth).min(1.0);

        // Consciousness affects overall awareness
        self.consciousness += consciousness_growth;

        // === Memory Formation ===
        // Form memories of interesting locations (high beat intensity or social interactions)
        let memory_threshold = 0.7 + self.personality_matrix[3] * 0.3; // Memory strength affects threshold
        if beat_intensity > memory_threshold || self.social_attraction > 0.8 {
            // Only store memory if this location is significantly different from existing memories
            let should_remember = self.memory_fragments.is_empty() ||
                self.memory_fragments.iter().all(|mem| mem.distance(self.position) > 50.0);

            if should_remember && self.memory_fragments.len() < 10 {
                self.memory_fragments.push(self.position);
                self.memory_intensity = (self.memory_intensity + 0.1).min(1.0);
            }
        }

        // === Memory-Driven Movement ===
        let mut memory_influence = Vec2::ZERO;
        if !self.memory_fragments.is_empty() && fastrand::f32() < self.memory_intensity * 0.1 {
            // Sometimes return to interesting memories
            let target_memory = &self.memory_fragments[fastrand::usize(0..self.memory_fragments.len())];
            let to_memory = *target_memory - self.position;
            if to_memory.length() > 10.0 {
                memory_influence = to_memory.normalize() * 30.0 * self.personality_matrix[3]; // Memory strength
            }
        }

        // === Exploration Drive ===
        let exploration_force = Vec2::new(
            (fastrand::f32() - 0.5) * self.exploration_drive * 50.0,
            (fastrand::f32() - 0.5) * self.exploration_drive * 50.0,
        );

        // === Social Consciousness & Basic Flocking ===
        let mut social_force = Vec2::ZERO;
        let mut nearby_count = 0;
        let social_range = 100.0 + self.personality_matrix[1] * 50.0; // Sociability affects range

        for (i, other) in all_llamas.iter().enumerate() {
            if i == my_index { continue; }

            let distance = self.position.distance(other.position);
            if distance < social_range && distance > 0.1 {
                let to_other = other.position - self.position;
                nearby_count += 1;

                // Attraction force (modified by personality)
                if distance > 30.0 {
                    social_force += to_other.normalize() * self.social_attraction * 20.0;
                } else {
                    // Slight repulsion when too close
                    social_force -= to_other.normalize() * 10.0;
                }

                // Form social bonds
                if distance < 60.0 && !self.social_bonds.contains(&i) && self.social_bonds.len() < 5 {
                    self.social_bonds.push(i);
                }
            }
        }

        // Update social attraction based on nearby llamas
        if nearby_count > 0 {
            self.social_attraction = (self.social_attraction + 0.02).min(1.0);
        } else {
            self.social_attraction *= 0.99; // Slowly decay when alone
        }

        // === Reality Distortion ===
        // Affects local space-time based on consciousness level and chaos affinity
        self.reality_distortion = self.awareness_level * self.personality_matrix[2] * beat_intensity;

        // === Emotional State Evolution ===
        let emotional_volatility = self.personality_matrix[4];
        let emotion_change = (beat_intensity.sin() * emotional_volatility * 0.1) +
                           (social_force.length() * 0.01) +
                           (memory_influence.length() * 0.005);
        self.emotional_state = ((self.emotional_state + emotion_change).sin() + 1.0) * 0.5; // Keep in 0-1 range

        // === Movement Integration ===
        let personality_velocity_mod = 1.0 + self.personality_matrix[6] * 0.5; // Exploration drive affects speed
        let total_force = social_force + memory_influence + exploration_force;
        self.velocity += total_force * dt * personality_velocity_mod;

        // Apply reality distortion to movement
        if self.reality_distortion > 0.1 {
            let distortion_angle = self.reality_distortion * beat_intensity * 10.0;
            let cos_d = distortion_angle.cos();
            let sin_d = distortion_angle.sin();
            let distorted_vel = Vec2::new(
                self.velocity.x * cos_d - self.velocity.y * sin_d,
                self.velocity.x * sin_d + self.velocity.y * cos_d,
            );
            self.velocity = self.velocity.lerp(distorted_vel, self.reality_distortion * 0.3);
        }

        // Apply velocity damping
        self.velocity *= 0.98;

        // Update position
        self.position += self.velocity * dt;

        // Wrap around screen with reality distortion effects
        let wrap_margin = if self.reality_distortion > 0.3 { 50.0 } else { 0.0 };
        if self.position.x < -wrap_margin { self.position.x = 1200.0 + wrap_margin; }
        if self.position.x > 1200.0 + wrap_margin { self.position.x = -wrap_margin; }
        if self.position.y < -wrap_margin { self.position.y = 800.0 + wrap_margin; }
        if self.position.y > 800.0 + wrap_margin { self.position.y = -wrap_margin; }

        // === Advanced Color Psychology ===
        // Color reflects emotional state, consciousness, and personality
        let base_hue_shift = 1.0 + self.personality_matrix[0] * 2.0; // Curiosity affects color change speed
        let emotional_hue_offset = self.emotional_state * 60.0; // Emotion affects hue
        let consciousness_saturation = 0.5 + self.awareness_level * 0.5; // Consciousness affects saturation

        self.color.x = (self.color.x + base_hue_shift + emotional_hue_offset * dt) % 360.0;
        self.color.y = consciousness_saturation;

        // === Trip Intensity Evolution ===
        // Combines beat, consciousness, and reality distortion
        let base_trip = 1.0 + beat_intensity.sin() * 0.5;
        let consciousness_amplification = 1.0 + self.awareness_level * 0.7;
        let reality_amplification = 1.0 + self.reality_distortion * 0.5;
        self.trip_intensity = base_trip * consciousness_amplification * reality_amplification;
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

        // Calculate beat intensity from math - enhanced with consciousness feedback
        let base_beat = (self.time * 2.0).sin().abs() * 0.5 + 0.3;

        // Consciousness feedback: higher awareness levels amplify the beat
        let consciousness_feedback = if !self.llamas.is_empty() {
            self.llamas.iter()
                .map(|llama| llama.awareness_level)
                .sum::<f32>() / self.llamas.len() as f32
        } else {
            0.0
        };

        self.beat_intensity = base_beat * (1.0 + consciousness_feedback * 0.3);

        // Update llamas with social awareness
        // We need to clone the llamas vector for reference during updates
        let llamas_snapshot = self.llamas.clone();
        for (i, llama) in self.llamas.iter_mut().enumerate() {
            llama.update(1.0 / 60.0, self.beat_intensity, &llamas_snapshot, i);
        }

        // Decay beat intensity
        self.beat_intensity *= 0.95;
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());

        // Generate vertices for all llamas with consciousness-enhanced visuals
        let mut vertices = Vec::new();
        for llama in &self.llamas {
            // Size affected by trip intensity, consciousness, and reality distortion
            let base_size = 10.0 + llama.trip_intensity * 5.0;
            let consciousness_size_mod = 1.0 + llama.awareness_level * 0.5;
            let reality_size_mod = 1.0 + llama.reality_distortion * 0.8;
            let size = base_size * consciousness_size_mod * reality_size_mod;

            // Enhanced color psychology: brightness reflects consciousness
            let brightness = 0.6 + llama.awareness_level * 0.4;
            let color = hsv_to_rgb(llama.color.x, llama.color.y, brightness);

            // Reality distortion affects position rendering
            let mut render_x = llama.position.x;
            let mut render_y = llama.position.y;
            if llama.reality_distortion > 0.2 {
                let distortion_offset = llama.reality_distortion * 10.0;
                render_x += (self.time * 5.0 + llama.position.x * 0.01).sin() * distortion_offset;
                render_y += (self.time * 7.0 + llama.position.y * 0.01).cos() * distortion_offset;
            }

            // Create quad with enhanced visuals
            let x = (render_x / 1200.0) * 2.0 - 1.0;
            let y = 1.0 - (render_y / 800.0) * 2.0;
            let s = size / 1200.0;

            // Add slight color variation based on emotional state
            let emotional_tint = llama.emotional_state * 0.3;
            let final_color = [
                (color.x + emotional_tint).min(1.0),
                (color.y * (1.0 - emotional_tint * 0.5)).max(0.0),
                (color.z + emotional_tint * 0.5).min(1.0),
            ];

            vertices.extend([
                Vertex { position: [x - s, y - s, 0.0], color: final_color },
                Vertex { position: [x + s, y - s, 0.0], color: final_color },
                Vertex { position: [x - s, y + s, 0.0], color: final_color },
                Vertex { position: [x + s, y - s, 0.0], color: final_color },
                Vertex { position: [x + s, y + s, 0.0], color: final_color },
                Vertex { position: [x - s, y + s, 0.0], color: final_color },
            ]);

            // Add memory fragment visualization for high-consciousness llamas
            if llama.awareness_level > 0.6 && !llama.memory_fragments.is_empty() {
                for memory in &llama.memory_fragments {
                    let mem_x = (memory.x / 1200.0) * 2.0 - 1.0;
                    let mem_y = 1.0 - (memory.y / 800.0) * 2.0;
                    let mem_s = (2.0 + llama.memory_intensity * 3.0) / 1200.0;
                    let mem_alpha = llama.memory_intensity * 0.3;

                    let memory_color = [
                        color.x * mem_alpha,
                        color.y * mem_alpha,
                        color.z * mem_alpha,
                    ];

                    vertices.extend([
                        Vertex { position: [mem_x - mem_s, mem_y - mem_s, 0.0], color: memory_color },
                        Vertex { position: [mem_x + mem_s, mem_y - mem_s, 0.0], color: memory_color },
                        Vertex { position: [mem_x - mem_s, mem_y + mem_s, 0.0], color: memory_color },
                        Vertex { position: [mem_x + mem_s, mem_y - mem_s, 0.0], color: memory_color },
                        Vertex { position: [mem_x + mem_s, mem_y + mem_s, 0.0], color: memory_color },
                        Vertex { position: [mem_x - mem_s, mem_y + mem_s, 0.0], color: memory_color },
                    ]);
                }
            }
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