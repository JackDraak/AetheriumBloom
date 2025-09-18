pub mod ecs;
pub mod memory;
pub mod time;
pub mod events;
pub mod safety;
pub mod warning;

use anyhow::Result;
use winit::window::Window;
use wgpu::*;

use crate::consciousness::LlamaManager;
use crate::mathematics::BeatEngine;
use crate::reality::PsychedelicRenderer;
use crate::input::ResonanceEffect;

pub struct ChaosEngine<'a> {
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    surface: Surface<'a>,
    size: winit::dpi::PhysicalSize<u32>,

    // Core systems
    llama_manager: LlamaManager,
    beat_engine: BeatEngine,
    renderer: PsychedelicRenderer,

    // Reality state
    reality_distortion: f32,
    cosmic_time: f64,
    consciousness_level: f32,
}

impl<'a> ChaosEngine<'a> {
    pub async fn new(window: &'a Window) -> Result<Self> {
        let size = window.inner_size();

        // Initialize GPU madness
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            flags: InstanceFlags::default(),
            dx12_shader_compiler: Default::default(),
            gles_minor_version: Gles3MinorVersion::Automatic,
        });

        let surface = unsafe { instance.create_surface(&window) }?;

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance, // Maximum chaos requires power
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| anyhow::anyhow!("Failed to find suitable GPU for reality distortion"))?;

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    required_features: Features::empty(),
                    required_limits: Limits::default(),
                    label: None,
                },
                None,
            )
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo, // Vsync for smooth chaos
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let llama_manager = LlamaManager::new();
        let beat_engine = BeatEngine::new();
        let renderer = PsychedelicRenderer::new(&device, &config)?;

        Ok(Self {
            device,
            queue,
            config,
            surface,
            size,
            llama_manager,
            beat_engine,
            renderer,
            reality_distortion: 0.0,
            cosmic_time: 0.0,
            consciousness_level: 1.0,
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.renderer.resize(&self.device, &self.config);
        }
    }

    pub fn reconfigure_surface(&mut self) {
        self.surface.configure(&self.device, &self.config);
    }

    pub fn apply_resonance_effect(&mut self, effect: ResonanceEffect) {
        match effect {
            ResonanceEffect::Spawn { count, intensity } => {
                self.llama_manager.spawn_llamas(count, intensity);
                self.reality_distortion += intensity * 0.1;
            }
            ResonanceEffect::RealityTear { strength } => {
                self.reality_distortion += strength;
                self.consciousness_level *= 1.1;
            }
            ResonanceEffect::Ripple { amplitude } => {
                self.reality_distortion += amplitude * 0.05;
            }
        }
    }

    pub fn update(&mut self) {
        self.cosmic_time += 1.0 / 60.0; // Assume 60 FPS for now

        // Update mathematical beat engine
        let beat_state = self.beat_engine.update(self.cosmic_time);

        // Update llama consciousness
        self.llama_manager.update(self.cosmic_time, &beat_state);

        // Apply reality decay
        self.reality_distortion *= 0.995;
        self.reality_distortion = self.reality_distortion.max(0.0);

        // Update consciousness level based on llama count
        let llama_count = self.llama_manager.llama_count() as f32;
        self.consciousness_level = (llama_count * 0.1 + 1.0).min(10.0);
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let render_data = crate::reality::RenderData {
            llamas: self.llama_manager.get_render_data(),
            reality_distortion: self.reality_distortion,
            consciousness_level: self.consciousness_level,
            cosmic_time: self.cosmic_time,
            beat_intensity: self.beat_engine.current_intensity(),
        };

        self.renderer.render(&self.device, &self.queue, &view, render_data)?;
        output.present();

        Ok(())
    }
}