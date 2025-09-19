// Ultra-simplified AetheriumBloom prototype for rapid chaos deployment

use anyhow::Result;
use wgpu::*;
use winit::{
    event::{WindowEvent, ElementState, MouseButton, KeyEvent},
    event_loop::EventLoop,
    window::Window,
    application::ApplicationHandler,
    keyboard::{Key, NamedKey},
};
use glam::{Vec2, Vec3};
use std::io::{self, Write};
use std::collections::{VecDeque, HashMap};

// === AUDIO CONSCIOUSNESS LAYER ===
use crate::audio::{AudioConsciousnessEngine, AudioEnvironment, AudioAnalysisData, AudioMode};

// === UNIFIED VERTEX SYSTEM ===
use crate::reality::{Vertex, DynamicVertexBuffer, VertexBudgetManager, BufferConfig};

// === MODULAR SYSTEMS ===
use crate::engine::safety::{SafetyConfig, FlashTracker, calculate_luminance, limit_luminance_change, is_dangerous_red, rgb_to_hsv, hsv_to_rgb_vec3};
// === EXTRACTED MODULAR SYSTEMS ===
use crate::engine::{ChaosDecisionEngine, DecisionVector, LlamaSnapshot, ConsciousnessMultiplicationSystem, AdvancedBeatEngine, EventDrivenArchitecture};
use crate::entities::{Llama, SpeciesType, ConsciousnessLevel};
use crate::communication::{EmergentCommunicationSystems, ManifestationType};
use crate::simulation::{ConsciousnessCrystal, DigitalEcosystem, MetaConsciousnessFramework, CrystalType, ZoneType, ConsciousnessField, RealityTear, TerritoryZone};
use crate::rendering::{PsychedelicUniforms, RealityDistortionEngine};
use crate::user::{UserCoEvolutionSystem, UserAction, ActionType, ActionContext, VisualEnvironmentState, AudioEnvironmentState};

// === CRITICAL SAFETY SYSTEMS FOR EPILEPSY PROTECTION ===

/// User's response to safety warning
#[derive(Debug, Clone, PartialEq)]
pub enum WarningResponse {
    Continue,     // User accepts risk and wants full visual effects
    SafetyMode,   // User wants reduced visual intensity
    Exit,         // User chooses to exit
}



/// Convert HSV to RGB (convenience wrapper)
fn hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> Vec3 {
    hsv_to_rgb_vec3(Vec3::new(hue, saturation, value))
}

/// Show console safety warning and get user response
fn show_epilepsy_warning() -> WarningResponse {
    println!("\n{}", "=".repeat(80));
    println!("⚠️  PHOTOSENSITIVE EPILEPSY WARNING ⚠️");
    println!("{}", "=".repeat(80));
    println!();
    println!("AetheriumBloom contains flashing lights and visual effects that may");
    println!("trigger seizures in individuals with photosensitive epilepsy.");
    println!();
    println!("🚨 IF YOU OR ANYONE IN YOUR FAMILY HAS A HISTORY OF SEIZURES OR EPILEPSY,");
    println!("   CONSULT A DOCTOR BEFORE USING THIS SOFTWARE.");
    println!();
    println!("⚠️  STOP USING IMMEDIATELY IF YOU EXPERIENCE:");
    println!("   • Dizziness, nausea, or disorientation");
    println!("   • Altered vision or muscle twitching");
    println!("   • Loss of awareness or convulsions");
    println!();
    println!("✅ SAFETY RECOMMENDATIONS:");
    println!("   • Use in a well-lit room");
    println!("   • Sit at least 2 feet from screen");
    println!("   • Take breaks every 30 minutes");
    println!();
    println!("🛡️  BUILT-IN SAFETY FEATURES:");
    println!("   • Flash rate limited to 3 Hz (international standard)");
    println!("   • Luminance changes capped at 10%");
    println!("   • Red flash protection enabled");
    println!("   • Emergency stop available (ESC key)");
    println!();
    println!("{}", "=".repeat(80));
    println!();
    println!("CHOOSE YOUR RESPONSE:");
    println!("  [C] CONTINUE - I understand the risks and want full visual effects");
    println!("  [S] SAFETY MODE - Continue with reduced visual intensity (50%)");
    println!("  [E] EXIT - I want to exit the application");
    println!();
    print!("Enter your choice (C/S/E): ");

    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let choice = input.trim().to_uppercase();
            match choice.as_str() {
                "C" | "CONTINUE" => {
                    println!("✅ Continuing with full visual effects...");
                    println!("⚠️  Remember: Press ESC at any time for emergency stop!");
                    WarningResponse::Continue
                }
                "S" | "SAFETY" | "SAFE" => {
                    println!("✅ Continuing in Safety Mode (50% visual intensity)...");
                    println!("⚠️  Remember: Press ESC at any time for emergency stop!");
                    WarningResponse::SafetyMode
                }
                "E" | "EXIT" => {
                    println!("👋 Exiting AetheriumBloom. Stay safe!");
                    WarningResponse::Exit
                }
                _ => {
                    println!("❌ Invalid choice. Defaulting to exit for safety.");
                    WarningResponse::Exit
                }
            }
        }
        Err(_) => {
            println!("❌ Error reading input. Defaulting to exit for safety.");
            WarningResponse::Exit
        }
    }
}

// === END SAFETY SYSTEMS ===

// === PHASE 3: ECOSYSTEM EMERGENCE ===






// Mathematical chaos engine using pre-calculated primes

// Vertex struct now imported from crate::reality::Vertex for consistency











// === PHASE 4: TRANSCENDENCE PROTOCOL ===



pub struct ChaosEngine {
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    surface: Surface<'static>,
    render_pipeline: RenderPipeline,
    dynamic_vertex_buffer: DynamicVertexBuffer,
    budget_manager: VertexBudgetManager,

    // Psychedelic shader uniforms
    uniform_buffer: Buffer,
    uniform_bind_group: BindGroup,
    uniforms: PsychedelicUniforms,

    llamas: Vec<Llama>,
    time: f32,
    beat_intensity: f32,

    // Phase 2: Advanced Beat Engine with chaos amplification
    advanced_beat_engine: AdvancedBeatEngine,
    species_spawn_weights: [f32; 3], // [DiscoLlama, QuantumSheep, HypnoCamel]
    total_consciousness: f32,

    // Phase 3: Ecosystem Emergence
    ecosystem: DigitalEcosystem,

    // Phase 4: Transcendence Protocol
    meta_consciousness: MetaConsciousnessFramework,
    reality_distortion: RealityDistortionEngine,
    emergent_communication: EmergentCommunicationSystems,
    event_driven_architecture: EventDrivenArchitecture,
    user_co_evolution: UserCoEvolutionSystem,

    // Phase 5: Consciousness Multiplication
    consciousness_multiplication: ConsciousnessMultiplicationSystem,

    // Phase 6: PSYCHEDELIC AUDIO CONSCIOUSNESS
    audio_consciousness: Option<AudioConsciousnessEngine>,
    audio_analysis_data: AudioAnalysisData,

    // CRITICAL SAFETY SYSTEMS - EPILEPSY PROTECTION
    safety_config: SafetyConfig,
    flash_tracker: FlashTracker,
    emergency_stop_requested: bool,
    previous_llama_colors: Vec<Vec3>, // Track previous colors for luminance limiting

    // Cursor position tracking for audio environmental responsiveness
    cursor_position: Vec2,
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
            source: ShaderSource::Wgsl(include_str!("reality/shaders/psychedelic.wgsl").into()),
        });

        // Create uniform buffer and bind group for psychedelic shader
        let uniforms = PsychedelicUniforms {
            time: 0.0,
            reality_distortion: 0.0,
            consciousness_level: 1.0,
            beat_intensity: 0.0,
            screen_resolution: [size.width as f32, size.height as f32],
            beat_frequency: 1.0,
            cosmic_phase: 0.0,
        };

        let uniform_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: std::mem::size_of::<PsychedelicUniforms>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let uniform_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
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

        // Initialize dynamic buffer management system
        let buffer_config = BufferConfig {
            initial_capacity: 250_000,
            max_capacity: 2_000_000, // Increased max capacity with safety limits
            growth_factor: 1.5,
            usage_history_frames: 60,
            resize_threshold: 0.8,
        };

        let dynamic_vertex_buffer = DynamicVertexBuffer::new(
            buffer_config,
            std::mem::size_of::<Vertex>() as u64
        );

        // Initialize vertex budget manager
        let mut budget_manager = VertexBudgetManager::new(1_000_000); // Total frame budget
        budget_manager.set_category_budget("llamas", 600_000);
        budget_manager.set_category_budget("crystals", 200_000);
        budget_manager.set_category_budget("effects", 200_000);

        // Start with 3 llamas - mix of species
        let mut llamas = Vec::new();
        llamas.push(Llama::new_with_species(
            Vec2::new(fastrand::f32() * 1200.0, fastrand::f32() * 800.0),
            SpeciesType::DiscoLlama
        ));
        llamas.push(Llama::new_with_species(
            Vec2::new(fastrand::f32() * 1200.0, fastrand::f32() * 800.0),
            SpeciesType::QuantumSheep
        ));
        llamas.push(Llama::new_with_species(
            Vec2::new(fastrand::f32() * 1200.0, fastrand::f32() * 800.0),
            SpeciesType::HypnoCamel
        ));

        // Cast to 'static - this is safe because the surface outlives the engine
        let surface = unsafe { std::mem::transmute::<Surface<'_>, Surface<'static>>(surface) };

        Ok(Self {
            device,
            queue,
            config,
            surface,
            render_pipeline,
            dynamic_vertex_buffer,
            budget_manager,

            // Psychedelic shader uniforms
            uniform_buffer,
            uniform_bind_group,
            uniforms,

            llamas,
            time: 0.0,
            beat_intensity: 0.0,

            // Phase 2: Advanced Beat Engine with chaos amplification
            advanced_beat_engine: AdvancedBeatEngine::new(),
            species_spawn_weights: [0.6, 0.25, 0.15], // Favor disco llamas initially
            total_consciousness: 0.0,

            // Phase 3: Ecosystem Emergence
            ecosystem: DigitalEcosystem::new(),

            // Phase 4: Transcendence Protocol
            meta_consciousness: MetaConsciousnessFramework::new(),
            reality_distortion: RealityDistortionEngine::new(),
            emergent_communication: EmergentCommunicationSystems::new(),
            event_driven_architecture: EventDrivenArchitecture::new(),
            user_co_evolution: UserCoEvolutionSystem::new(),

            // Phase 5: Consciousness Multiplication
            consciousness_multiplication: ConsciousnessMultiplicationSystem::new(),

            // Phase 6: PSYCHEDELIC AUDIO CONSCIOUSNESS
            audio_consciousness: {
                match AudioConsciousnessEngine::new() {
                    Ok(engine) => {
                        println!("🎵 PSYCHEDELIC AUDIO CONSCIOUSNESS LAYER - INITIALIZED!");
                        println!("🔊 Maximum decibels, minimum code - Audio reality synthesis active");
                        Some(engine)
                    },
                    Err(e) => {
                        println!("🔇 Audio engine initialization failed: {} - Continuing in visual-only mode", e);
                        None
                    }
                }
            },
            audio_analysis_data: AudioAnalysisData {
                current_environment: AudioEnvironment::Environmental,
                bass_level: 0.0,
                treble_level: 0.0,
                consciousness_frequency: 432.0,
                reality_distortion_amount: 0.0,
                hive_mind_coherence: 0.0,
            },

            // CRITICAL SAFETY SYSTEMS - EPILEPSY PROTECTION
            safety_config: SafetyConfig::default(),
            flash_tracker: FlashTracker::new(),
            emergency_stop_requested: false,
            previous_llama_colors: Vec::new(),
            cursor_position: Vec2::new(600.0, 400.0), // Start at center
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    /// Configure the engine for safety mode
    pub fn enable_safety_mode(&mut self) {
        self.safety_config = SafetyConfig::safe_mode();
        println!("🛡️ SAFETY MODE ENABLED - Visual effects reduced to 50% intensity");
        println!("🛡️ Flash rate limited to 2 Hz, luminance changes limited to 5%");
    }

    /// Handle emergency stop request
    pub fn request_emergency_stop(&mut self) {
        self.emergency_stop_requested = true;
        println!("🚨 EMERGENCY STOP ACTIVATED - All visual effects suppressed");
    }

    /// Check if emergency stop is active
    pub fn is_emergency_stop_active(&self) -> bool {
        self.emergency_stop_requested
    }

    pub fn handle_click(&mut self, _button: MouseButton, state: ElementState) {
        if state == ElementState::Pressed {
            // Determine species based on current chaos level and spawn weights
            let species = self.select_spawn_species();

            // Spawn new llama of selected species
            self.llamas.push(Llama::new_with_species(
                Vec2::new(
                    fastrand::f32() * 1200.0,
                    fastrand::f32() * 800.0,
                ),
                species.clone()
            ));

            // Add chaos feedback to beat engine
            let chaos_amount = 0.5 + self.total_consciousness * 0.1;
            self.beat_intensity += chaos_amount;
            self.advanced_beat_engine.add_chaos_feedback(chaos_amount);

            // Phase 3: Add chaos to ecosystem
            self.ecosystem.add_chaos(chaos_amount);

            // Phase 4: Record user action for co-evolution learning
            let user_action = UserAction {
                action_type: ActionType::MouseClick,
                timestamp: self.time as f64,
                duration: 0.1,
                intensity: chaos_amount,
                spatial_coordinates: Some(Vec2::new(
                    fastrand::f32() * 1200.0,
                    fastrand::f32() * 800.0,
                )),
                context: ActionContext {
                    system_state: {
                        let mut state = HashMap::new();
                        state.insert("beat_intensity".to_string(), self.beat_intensity);
                        state.insert("consciousness_level".to_string(), self.meta_consciousness.collective_intelligence);
                        state.insert("llama_count".to_string(), self.llamas.len() as f32);
                        state
                    },
                    visual_environment: VisualEnvironmentState {
                        dominant_colors: self.llamas.iter().take(3).map(|l| Vec3::new(l.color.x, l.color.y, 0.6)).collect(),
                        complexity_level: self.reality_distortion.emergence_amplification,
                        movement_intensity: chaos_amount,
                        flash_rate: 0.0, // Safe default
                        consciousness_visibility: self.meta_consciousness.collective_intelligence,
                    },
                    audio_environment: AudioEnvironmentState {
                        beat_intensity: self.beat_intensity,
                        harmonic_complexity: self.advanced_beat_engine.harmonic_layers.len() as f32 * 0.1,
                        frequency_distribution: HashMap::new(),
                        rhythm_coherence: 0.8,
                    },
                    concurrent_actions: Vec::new(),
                },
            };
            self.user_co_evolution.record_user_action(user_action);

            // Phase 4: Trigger event-driven cascade for beat drop effects
            if self.beat_intensity > 0.8 {
                self.event_driven_architecture.trigger_beat_cascade(self.beat_intensity, self.time as f64);
            }

            // Adjust spawn weights based on species spawned
            self.adjust_spawn_weights(&species);
        }
    }

    fn select_spawn_species(&self) -> SpeciesType {
        let chaos_level = self.total_consciousness + self.beat_intensity;

        // Higher chaos = more exotic species
        let adjusted_weights = if chaos_level > 5.0 {
            [0.3, 0.4, 0.3] // High chaos: more quantum sheep and hypno camels
        } else if chaos_level > 2.0 {
            [0.5, 0.3, 0.2] // Medium chaos: some exotic species
        } else {
            self.species_spawn_weights // Low chaos: use default weights
        };

        let roll = fastrand::f32();
        if roll < adjusted_weights[0] {
            SpeciesType::DiscoLlama
        } else if roll < adjusted_weights[0] + adjusted_weights[1] {
            SpeciesType::QuantumSheep
        } else {
            SpeciesType::HypnoCamel
        }
    }

    /// Handle cursor movement for environmental audio responsiveness
    pub fn handle_cursor_moved(&mut self, position: winit::dpi::PhysicalPosition<f64>) {
        // Convert to logical coordinates (scaling to 1200x800 window size)
        self.cursor_position = Vec2::new(position.x as f32, position.y as f32);

        // Chaotic audio mechanic: rapid cursor movement triggers audio chaos
        // This will be processed by the audio engine's spatial processor
    }

    fn adjust_spawn_weights(&mut self, spawned_species: &SpeciesType) {
        // Slightly reduce weight of spawned species to encourage diversity
        match spawned_species {
            SpeciesType::DiscoLlama => {
                self.species_spawn_weights[0] = (self.species_spawn_weights[0] - 0.02).max(0.1);
                self.species_spawn_weights[1] += 0.01;
                self.species_spawn_weights[2] += 0.01;
            },
            SpeciesType::QuantumSheep => {
                self.species_spawn_weights[1] = (self.species_spawn_weights[1] - 0.02).max(0.1);
                self.species_spawn_weights[0] += 0.01;
                self.species_spawn_weights[2] += 0.01;
            },
            SpeciesType::HypnoCamel => {
                self.species_spawn_weights[2] = (self.species_spawn_weights[2] - 0.02).max(0.1);
                self.species_spawn_weights[0] += 0.01;
                self.species_spawn_weights[1] += 0.01;
            },
        }
    }

    pub fn update(&mut self) {
        self.time += 1.0 / 60.0;
        let cosmic_time = self.time as f64;

        // Calculate total consciousness for advanced beat engine
        self.total_consciousness = if !self.llamas.is_empty() {
            self.llamas.iter()
                .map(|llama| llama.consciousness + llama.awareness_level + llama.environmental_consciousness)
                .sum::<f32>()
        } else {
            0.0
        };

        // Use advanced beat engine with consciousness coupling and prime chaos
        self.beat_intensity = self.advanced_beat_engine.update(1.0 / 60.0, self.total_consciousness);

        // Phase 3: Update ecosystem first
        self.ecosystem.update(1.0 / 60.0, cosmic_time, self.beat_intensity);

        // Phase 4: Update Meta-Consciousness Framework
        self.meta_consciousness.update(1.0 / 60.0, &self.llamas, cosmic_time, self.beat_intensity, &self.ecosystem);

        // Phase 4: Update Reality Distortion Engine
        self.reality_distortion.update(1.0 / 60.0, cosmic_time, &self.meta_consciousness, &self.llamas, self.beat_intensity, &self.ecosystem);

        // Phase 4: Update Emergent Communication Systems
        self.emergent_communication.update(1.0 / 60.0, &self.llamas, &self.ecosystem,
                                          self.meta_consciousness.collective_intelligence, cosmic_time);

        // Phase 4: Update Event-Driven Architecture
        let user_interaction_intensity = if self.llamas.len() > 0 {
            self.llamas.iter().map(|l| l.consciousness).sum::<f32>() / self.llamas.len() as f32
        } else {
            0.5
        };
        self.event_driven_architecture.update(1.0 / 60.0, self.beat_intensity,
                                            self.meta_consciousness.collective_intelligence,
                                            cosmic_time, user_interaction_intensity);

        // Phase 4: Update User Co-Evolution System
        let mut system_state = HashMap::new();
        system_state.insert("beat_intensity".to_string(), self.beat_intensity);
        system_state.insert("consciousness_level".to_string(), self.meta_consciousness.collective_intelligence);
        system_state.insert("ecosystem_stability".to_string(), 0.8); // Simple placeholder
        system_state.insert("visual_complexity".to_string(), self.reality_distortion.emergence_amplification);
        self.user_co_evolution.update(1.0 / 60.0, user_interaction_intensity, &system_state, cosmic_time);

        // Phase 5: Update Consciousness Multiplication System - "When One Mind Becomes Legion"
        self.consciousness_multiplication.update(1.0 / 60.0, &mut self.llamas, cosmic_time as f32);

        // Phase 6: PSYCHEDELIC AUDIO CONSCIOUSNESS UPDATE - "Maximum Decibels, Minimum Code"
        if let Some(ref mut audio_engine) = self.audio_consciousness {
            // Create beat state from advanced beat engine
            let beat_state = crate::audio::mathematics::BeatState {
                is_beat_drop: self.beat_intensity > 0.8,
                intensity: self.beat_intensity,
                phase: self.time as f64,
                prime_factor: self.advanced_beat_engine.prime_factors[0],
                cosmic_frequency: 432.0 + self.total_consciousness * 2.0,
            };

            // Convert llamas to audio-compatible format
            let llama_audio_data: Vec<crate::audio::CompatLlamaRenderData> = self.llamas.iter().map(|llama| {
                let species = match llama.species {
                    SpeciesType::DiscoLlama => crate::audio::CompatLlamaSpecies::Disco,
                    SpeciesType::QuantumSheep => crate::audio::CompatLlamaSpecies::Quantum,
                    SpeciesType::HypnoCamel => crate::audio::CompatLlamaSpecies::BassDrop,
                };

                crate::audio::CompatLlamaRenderData {
                    position: llama.position,
                    color_wavelength: Vec2::new(llama.color.x, llama.harmonic_resonance),
                    trip_intensity: llama.trip_intensity,
                    reality_distortion: llama.reality_distortion,
                    species,
                }
            }).collect();

            // Update cursor position for environmental audio responsiveness
            audio_engine.update_cursor_position(self.cursor_position);

            // Update the full audio consciousness engine
            audio_engine.update(
                cosmic_time,
                &beat_state,
                &llama_audio_data,
                self.total_consciousness
            );

            // Get updated audio analysis
            self.audio_analysis_data = audio_engine.get_audio_analysis();

            // Enhanced chaos event mapping for real-time audio responsiveness
            self.handle_enhanced_audio_chaos_mapping(audio_engine);

            // Print audio consciousness status on significant events
            if (self.time * 4.0) as u32 % 60 == 0 { // Every 15 seconds
                println!("🎵 AUDIO CONSCIOUSNESS - Environment: {:?}, Bass: {:.2}, Consciousness: {:.1}, Frequency: {:.1}Hz",
                         self.audio_analysis_data.current_environment,
                         self.audio_analysis_data.bass_level,
                         self.total_consciousness,
                         self.audio_analysis_data.consciousness_frequency);
            }
        } else {
            // Audio engine failed to initialize - continue with visual-only mode
            if (self.time * 4.0) as u32 % 240 == 0 { // Every minute
                println!("🔇 Audio engine unavailable - continuing in visual-only mode");
            }
        }

        // Update llamas with Phase 2, Phase 3, Phase 4, and Phase 5 enhancements
        // We need to clone the llamas vector for reference during updates
        let llamas_snapshot = self.llamas.clone();
        for (i, llama) in self.llamas.iter_mut().enumerate() {
            // Apply territory effects
            let territory_effects = self.ecosystem.get_territory_effects(llama.position);
            llama.apply_territory_effects(&territory_effects, 1.0 / 60.0);

            // Update consciousness field
            let environmental_consciousness = self.ecosystem.consciousness_fields.get_consciousness_at(llama.position);
            llama.environmental_consciousness = (llama.environmental_consciousness + environmental_consciousness * 0.01).min(2.0);

            // Add consciousness to the field where llama is
            self.ecosystem.consciousness_fields.add_consciousness_at(llama.position, llama.consciousness * 0.001);

            // Try to harvest crystals
            for crystal in &mut self.ecosystem.crystal_formations {
                llama.try_harvest_crystal(crystal);
            }

            // Regular llama update
            llama.update(1.0 / 60.0, self.beat_intensity, &llamas_snapshot, i, cosmic_time);
        }

        // Phase 3: Check for mutations
        if self.ecosystem.should_trigger_mutation() {
            let mutation_strength = 0.3 + self.ecosystem.chaos_accumulation * 0.1;

            // Apply mutations to random llamas
            let mutation_count = (self.llamas.len() / 3).max(1); // Mutate 1/3 of llamas minimum 1
            for _ in 0..mutation_count {
                if !self.llamas.is_empty() {
                    let index = fastrand::usize(0..self.llamas.len());
                    self.llamas[index].apply_mutation(mutation_strength);
                }
            }

            self.ecosystem.reset_chaos_for_mutation();
        }

        // Decay beat intensity more gradually for better chaos building
        self.beat_intensity *= 0.98;

        // Feed chaos back into the beat engine
        let average_chaos = if !self.llamas.is_empty() {
            self.llamas.iter()
                .map(|llama| llama.prime_chaos_factor)
                .sum::<f32>() / self.llamas.len() as f32
        } else {
            0.0
        };

        if average_chaos > 0.1 {
            self.advanced_beat_engine.add_chaos_feedback(average_chaos * 0.1);
        }
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        // CRITICAL SAFETY CHECK - Emergency stop overrides everything
        if self.emergency_stop_requested {
            return self.render_emergency_stop();
        }

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());

        // Update psychedelic shader uniforms
        self.uniforms.time = self.time;
        self.uniforms.reality_distortion = self.reality_distortion.emergence_amplification;
        self.uniforms.consciousness_level = self.total_consciousness;
        self.uniforms.beat_intensity = self.beat_intensity;
        self.uniforms.screen_resolution = [self.config.width as f32, self.config.height as f32];
        self.uniforms.beat_frequency = self.advanced_beat_engine.primary_rhythm;
        self.uniforms.cosmic_phase = self.advanced_beat_engine.time_accumulator as f32;

        // Write updated uniforms to buffer
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.uniforms]));

        // Ensure we have color tracking for all llamas
        while self.previous_llama_colors.len() < self.llamas.len() {
            self.previous_llama_colors.push(Vec3::new(0.1, 0.1, 0.1)); // Safe default
        }

        // Start new frame for budget tracking
        self.budget_manager.start_frame();

        // Generate vertices for all llamas with Phase 2 species-enhanced visuals AND SAFETY FILTERING
        // Estimate total vertices for predictive allocation
        let estimated_vertices_per_llama = 6; // Each llama is a quad (2 triangles)
        let estimated_crystal_vertices = self.ecosystem.crystals.len() * 6; // Crystal vertices
        let estimated_effect_vertices = self.llamas.len() * 20; // Rough estimate for various effects

        // Check budget allocations
        let allocated_llama_vertices = self.budget_manager.check_allocation("llamas",
                                                                          self.llamas.len() * estimated_vertices_per_llama);
        let allocated_crystal_vertices = self.budget_manager.check_allocation("crystals", estimated_crystal_vertices);
        let allocated_effect_vertices = self.budget_manager.check_allocation("effects", estimated_effect_vertices);

        let max_llamas = allocated_llama_vertices / estimated_vertices_per_llama;

        let mut vertices = Vec::new();
        for (llama_id, llama) in self.llamas.iter().enumerate() {
            // Apply budget limits
            if llama_id >= max_llamas {
                println!("Llama rendering limited by vertex budget at {}/{}", llama_id, self.llamas.len());
                break;
            }
            // Species-specific size calculation
            let base_size = match llama.species {
                SpeciesType::DiscoLlama => 10.0 + llama.trip_intensity * 5.0,
                SpeciesType::QuantumSheep => 8.0 + llama.trip_intensity * 6.0 + llama.quantum_state * 4.0,
                SpeciesType::HypnoCamel => 12.0 + llama.trip_intensity * 4.0,
            };

            let consciousness_size_mod = 1.0 + llama.awareness_level * 0.5;
            let reality_size_mod = 1.0 + llama.reality_distortion * 0.8;
            let chaos_size_mod = 1.0 + llama.prime_chaos_factor * 0.3;

            // Phase 5: Consciousness hierarchy size modifications
            let hierarchy_size_mod = match llama.consciousness_level {
                ConsciousnessLevel::Individual => 1.0,
                ConsciousnessLevel::Pack => 1.3 + llama.hive_connection_strength * 0.2,
                ConsciousnessLevel::Hive => 1.6 + llama.hive_connection_strength * 0.4,
                ConsciousnessLevel::Meta => 2.0,
            };

            // Phase 5: Warfare participation affects size (entities grow larger during conflicts)
            let warfare_size_mod = 1.0 + llama.warfare_participation * 0.3;

            // Phase 5: Territorial dominance adds presence
            let dominance_size_mod = 1.0 + llama.territorial_dominance * 0.2;

            let size = base_size * consciousness_size_mod * reality_size_mod * chaos_size_mod
                       * hierarchy_size_mod * warfare_size_mod * dominance_size_mod;

            // Enhanced color psychology: brightness reflects consciousness
            let mut brightness = 0.6 + llama.awareness_level * 0.4;

            // Phase 5: Consciousness level affects brightness
            brightness += match llama.consciousness_level {
                ConsciousnessLevel::Individual => 0.0,
                ConsciousnessLevel::Pack => 0.1,
                ConsciousnessLevel::Hive => 0.2,
                ConsciousnessLevel::Meta => 0.3,
            };

            // Phase 5: Warfare participation makes entities glow
            brightness += llama.warfare_participation * 0.15;

            // Phase 5: Extinction pressure causes fading
            brightness *= 1.0 - llama.extinction_pressure * 0.5;

            brightness = brightness.clamp(0.1, 1.0);

            let mut color = hsv_to_rgb(llama.color.x, llama.color.y, brightness);

            // Phase 5: Hive mind entities have synchronized color pulsing
            if llama.consciousness_level == ConsciousnessLevel::Hive && llama.hive_connection_strength > 0.5 {
                let pulse = (self.time * 3.0 + llama_id as f32 * 0.5).sin() * 0.1 + 1.0;
                color *= pulse;
            }

            // CRITICAL SAFETY: Apply all safety measures
            let previous_color = self.previous_llama_colors[llama_id];

            // 1. Apply flash rate limiting
            let current_time = self.time as f64;
            let color_change = calculate_luminance(&color) - calculate_luminance(&previous_color);
            let is_major_change = color_change.abs() > 0.05; // 5% threshold for major change

            if is_major_change {
                if !self.flash_tracker.can_allow_flash(current_time, self.safety_config.max_flash_rate) {
                    // Flash blocked - use previous color
                    color = previous_color;
                } else {
                    // Flash allowed - record it
                    self.flash_tracker.record_flash(current_time);
                }
            }

            // 2. Apply luminance change limiting
            color = limit_luminance_change(color, previous_color, self.safety_config.max_luminance_change);

            // 3. Apply red flash protection
            if self.safety_config.red_flash_protection && is_dangerous_red(color) && is_major_change {
                // Shift dangerous red to safe orange
                let hsv = rgb_to_hsv(color);
                let safe_hue = if hsv.x >= 345.0 { 30.0 } else { 30.0 }; // Orange
                color = hsv_to_rgb_vec3(Vec3::new(safe_hue, hsv.y * 0.8, hsv.z));
            }

            // 4. Apply visual intensity limiting
            if self.safety_config.visual_intensity_limit < 1.0 {
                let safe_color = Vec3::new(0.2, 0.2, 0.2); // Safe dim color
                color = safe_color.lerp(color, self.safety_config.visual_intensity_limit);
            }

            // Update previous color for next frame
            self.previous_llama_colors[llama_id] = color;

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

            // Map species to shader ID for psychedelic effects
            let species_id = match llama.species {
                SpeciesType::DiscoLlama => 0.0,    // Disco effects
                SpeciesType::QuantumSheep => 1.0,  // Quantum glitches
                SpeciesType::HypnoCamel => 2.0,    // Hypnotic spirals
            };

            vertices.extend([
                Vertex {
                    position: [x - s, y - s, 0.0],
                    color: final_color,
                    uv: [0.0, 0.0],
                    species_id,
                    consciousness: llama.awareness_level,
                    trip_intensity: llama.trip_intensity,
                },
                Vertex {
                    position: [x + s, y - s, 0.0],
                    color: final_color,
                    uv: [1.0, 0.0],
                    species_id,
                    consciousness: llama.awareness_level,
                    trip_intensity: llama.trip_intensity,
                },
                Vertex {
                    position: [x - s, y + s, 0.0],
                    color: final_color,
                    uv: [0.0, 1.0],
                    species_id,
                    consciousness: llama.awareness_level,
                    trip_intensity: llama.trip_intensity,
                },
                Vertex {
                    position: [x + s, y - s, 0.0],
                    color: final_color,
                    uv: [1.0, 0.0],
                    species_id,
                    consciousness: llama.awareness_level,
                    trip_intensity: llama.trip_intensity,
                },
                Vertex {
                    position: [x + s, y + s, 0.0],
                    color: final_color,
                    uv: [1.0, 1.0],
                    species_id,
                    consciousness: llama.awareness_level,
                    trip_intensity: llama.trip_intensity,
                },
                Vertex {
                    position: [x - s, y + s, 0.0],
                    color: final_color,
                    uv: [0.0, 1.0],
                    species_id,
                    consciousness: llama.awareness_level,
                    trip_intensity: llama.trip_intensity,
                },
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

                    // Memory fragments are fractal-like effects
                    vertices.extend([
                        Vertex {
                            position: [mem_x - mem_s, mem_y - mem_s, 0.0],
                            color: memory_color,
                            uv: [0.0, 0.0],
                            species_id: 3.0, // Fractal effects for memory
                            consciousness: llama.memory_intensity,
                            trip_intensity: llama.trip_intensity * 0.8,
                        },
                        Vertex {
                            position: [mem_x + mem_s, mem_y - mem_s, 0.0],
                            color: memory_color,
                            uv: [1.0, 0.0],
                            species_id: 3.0,
                            consciousness: llama.memory_intensity,
                            trip_intensity: llama.trip_intensity * 0.8,
                        },
                        Vertex {
                            position: [mem_x - mem_s, mem_y + mem_s, 0.0],
                            color: memory_color,
                            uv: [0.0, 1.0],
                            species_id: 3.0,
                            consciousness: llama.memory_intensity,
                            trip_intensity: llama.trip_intensity * 0.8,
                        },
                        Vertex {
                            position: [mem_x + mem_s, mem_y - mem_s, 0.0],
                            color: memory_color,
                            uv: [1.0, 0.0],
                            species_id: 3.0,
                            consciousness: llama.memory_intensity,
                            trip_intensity: llama.trip_intensity * 0.8,
                        },
                        Vertex {
                            position: [mem_x + mem_s, mem_y + mem_s, 0.0],
                            color: memory_color,
                            uv: [1.0, 1.0],
                            species_id: 3.0,
                            consciousness: llama.memory_intensity,
                            trip_intensity: llama.trip_intensity * 0.8,
                        },
                        Vertex {
                            position: [mem_x - mem_s, mem_y + mem_s, 0.0],
                            color: memory_color,
                            uv: [0.0, 1.0],
                            species_id: 3.0,
                            consciousness: llama.memory_intensity,
                            trip_intensity: llama.trip_intensity * 0.8,
                        },
                    ]);
                }
            }
        }

        // Phase 3: Render consciousness crystals
        for crystal in &self.ecosystem.crystal_formations {
            let crystal_color = crystal.get_color();

            // Apply safety measures to crystal colors too
            let mut safe_crystal_color = crystal_color;
            if self.safety_config.red_flash_protection && is_dangerous_red(crystal_color) {
                let hsv = rgb_to_hsv(crystal_color);
                let safe_hue = if hsv.x >= 345.0 { 30.0 } else { 30.0 }; // Orange
                safe_crystal_color = hsv_to_rgb_vec3(Vec3::new(safe_hue, hsv.y * 0.8, hsv.z));
            }

            if self.safety_config.visual_intensity_limit < 1.0 {
                let safe_color = Vec3::new(0.1, 0.1, 0.1);
                safe_crystal_color = safe_color.lerp(safe_crystal_color, self.safety_config.visual_intensity_limit);
            }

            let x = (crystal.position.x / 1200.0) * 2.0 - 1.0;
            let y = 1.0 - (crystal.position.y / 800.0) * 2.0;
            let s = (8.0 + crystal.visual_intensity * 12.0) / 1200.0;

            let crystal_color_array = [safe_crystal_color.x, safe_crystal_color.y, safe_crystal_color.z];

            // Crystal rendered as a diamond shape with fractal effects
            vertices.extend([
                Vertex {
                    position: [x, y - s, 0.0],
                    color: crystal_color_array,
                    uv: [0.5, 0.0],
                    species_id: 3.0, // Fractal effects for crystals
                    consciousness: crystal.consciousness_energy,
                    trip_intensity: crystal.consciousness_energy * 0.5,
                },
                Vertex {
                    position: [x + s, y, 0.0],
                    color: crystal_color_array,
                    uv: [1.0, 0.5],
                    species_id: 3.0,
                    consciousness: crystal.consciousness_energy,
                    trip_intensity: crystal.consciousness_energy * 0.5,
                },
                Vertex {
                    position: [x, y + s, 0.0],
                    color: crystal_color_array,
                    uv: [0.5, 1.0],
                    species_id: 3.0,
                    consciousness: crystal.consciousness_energy,
                    trip_intensity: crystal.consciousness_energy * 0.5,
                },
                Vertex {
                    position: [x, y - s, 0.0],
                    color: crystal_color_array,
                    uv: [0.5, 0.0],
                    species_id: 3.0,
                    consciousness: crystal.consciousness_energy,
                    trip_intensity: crystal.consciousness_energy * 0.5,
                },
                Vertex {
                    position: [x - s, y, 0.0],
                    color: crystal_color_array,
                    uv: [0.0, 0.5],
                    species_id: 3.0,
                    consciousness: crystal.consciousness_energy,
                    trip_intensity: crystal.consciousness_energy * 0.5,
                },
                Vertex {
                    position: [x, y + s, 0.0],
                    color: crystal_color_array,
                    uv: [0.5, 1.0],
                    species_id: 3.0,
                    consciousness: crystal.consciousness_energy,
                    trip_intensity: crystal.consciousness_energy * 0.5,
                },
            ]);

            // Add harvest radius visualization for high-energy crystals
            if crystal.consciousness_energy > 1.0 {
                let radius_size = (crystal.harvest_radius / 1200.0) * 0.3; // Visual radius smaller than actual
                let radius_alpha = 0.1 * crystal.visual_intensity;
                let radius_color = [
                    safe_crystal_color.x * radius_alpha,
                    safe_crystal_color.y * radius_alpha,
                    safe_crystal_color.z * radius_alpha,
                ];

                // Simple circle approximation
                for i in 0..6 {
                    let angle1 = (i as f32 / 6.0) * std::f32::consts::TAU;
                    let angle2 = ((i + 1) as f32 / 6.0) * std::f32::consts::TAU;

                    vertices.extend([
                        Vertex {
                            position: [x, y, 0.0],
                            color: radius_color,
                            uv: [0.5, 0.5],
                            species_id: 3.0, // Crystal energy effects
                            consciousness: crystal.consciousness_energy,
                            trip_intensity: 1.0, // High intensity for energy fields
                        },
                        Vertex {
                            position: [x + angle1.cos() * radius_size, y + angle1.sin() * radius_size, 0.0],
                            color: radius_color,
                            uv: [0.5, 0.5],
                            species_id: 3.0,
                            consciousness: crystal.consciousness_energy,
                            trip_intensity: 1.0,
                        },
                        Vertex {
                            position: [x + angle2.cos() * radius_size, y + angle2.sin() * radius_size, 0.0],
                            color: radius_color,
                            uv: [0.5, 0.5],
                            species_id: 3.0,
                            consciousness: crystal.consciousness_energy,
                            trip_intensity: 1.0,
                        },
                    ]);
                }
            }
        }

        // Phase 3: Render reality tears
        for tear in &self.ecosystem.reality_tears {
            if tear.intensity < 0.2 { continue; } // Skip very faded tears

            let mut tear_color = Vec3::new(1.0, 0.8, 1.0); // Pink/white glitch color

            // Apply safety measures
            if self.safety_config.visual_intensity_limit < 1.0 {
                let safe_color = Vec3::new(0.1, 0.1, 0.1);
                tear_color = safe_color.lerp(tear_color, self.safety_config.visual_intensity_limit * 0.5); // Extra conservative
            }

            let x = (tear.position.x / 1200.0) * 2.0 - 1.0;
            let y = 1.0 - (tear.position.y / 800.0) * 2.0;
            let s = (tear.size / 1200.0) * tear.intensity;

            let tear_color_array = [
                tear_color.x * tear.intensity,
                tear_color.y * tear.intensity,
                tear_color.z * tear.intensity,
            ];

            // Reality tear rendered as jagged triangular glitch
            let offset1 = s * 0.6;
            let offset2 = s * 0.3;

            // Reality tears use quantum glitch effects
            vertices.extend([
                Vertex {
                    position: [x - s, y - offset1, 0.0],
                    color: tear_color_array,
                    uv: [0.5, 0.5],
                    species_id: 1.0, // Quantum glitch effects
                    consciousness: 1.0, // Maximum consciousness for reality tears
                    trip_intensity: self.reality_distortion.emergence_amplification,
                },
                Vertex {
                    position: [x + s, y + offset2, 0.0],
                    color: tear_color_array,
                    uv: [0.5, 0.5],
                    species_id: 1.0,
                    consciousness: 1.0,
                    trip_intensity: self.reality_distortion.emergence_amplification,
                },
                Vertex {
                    position: [x - offset2, y + s, 0.0],
                    color: tear_color_array,
                    uv: [0.5, 0.5],
                    species_id: 1.0,
                    consciousness: 1.0,
                    trip_intensity: self.reality_distortion.emergence_amplification,
                },
                Vertex {
                    position: [x + offset1, y - s, 0.0],
                    color: tear_color_array,
                    uv: [0.5, 0.5],
                    species_id: 1.0,
                    consciousness: 1.0,
                    trip_intensity: self.reality_distortion.emergence_amplification,
                },
                Vertex {
                    position: [x + s, y - offset1, 0.0],
                    color: tear_color_array,
                    uv: [0.5, 0.5],
                    species_id: 1.0,
                    consciousness: 1.0,
                    trip_intensity: self.reality_distortion.emergence_amplification,
                },
                Vertex {
                    position: [x - s, y + offset2, 0.0],
                    color: tear_color_array,
                    uv: [0.5, 0.5],
                    species_id: 1.0,
                    consciousness: 1.0,
                    trip_intensity: self.reality_distortion.emergence_amplification,
                },
            ]);
        }

        // Phase 3: Render territory zones (subtle background effects)
        for zone in &self.ecosystem.territory_zones {
            let zone_alpha = zone.strength * 0.05; // Very subtle
            if zone_alpha < 0.01 { continue; }

            let zone_color = match zone.zone_type {
                ZoneType::Harmonic => Vec3::new(0.0, 1.0, 1.0),   // Cyan
                ZoneType::Chaotic => Vec3::new(1.0, 0.0, 1.0),    // Magenta
                ZoneType::Meditative => Vec3::new(0.0, 1.0, 0.0), // Green
                ZoneType::Quantum => Vec3::new(0.0, 0.0, 1.0),    // Blue
            };

            // Apply safety measures
            let mut safe_zone_color = zone_color;
            if self.safety_config.visual_intensity_limit < 1.0 {
                let safe_color = Vec3::new(0.0, 0.0, 0.0);
                safe_zone_color = safe_color.lerp(safe_zone_color, self.safety_config.visual_intensity_limit * 0.3);
            }

            let x = (zone.center.x / 1200.0) * 2.0 - 1.0;
            let y = 1.0 - (zone.center.y / 800.0) * 2.0;
            let r = (zone.radius / 1200.0) * 0.8; // Visual radius smaller than actual

            let zone_color_array = [
                safe_zone_color.x * zone_alpha,
                safe_zone_color.y * zone_alpha,
                safe_zone_color.z * zone_alpha,
            ];

            // Simple circle for territory zone
            for i in 0..8 {
                let angle1 = (i as f32 / 8.0) * std::f32::consts::TAU;
                let angle2 = ((i + 1) as f32 / 8.0) * std::f32::consts::TAU;

                vertices.extend([
                    Vertex { position: [x, y, 0.0], color: zone_color_array, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.3, trip_intensity: 0.1 },
                    Vertex { position: [x + angle1.cos() * r, y + angle1.sin() * r, 0.0], color: zone_color_array, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.3, trip_intensity: 0.1 },
                    Vertex { position: [x + angle2.cos() * r, y + angle2.sin() * r, 0.0], color: zone_color_array, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.3, trip_intensity: 0.1 },
                ]);
            }
        }

        // Phase 4: Render Emergent Communications
        for communication in &self.emergent_communication.active_communications {
            if communication.transmission_effectiveness < 0.1 { continue; } // Skip very faded communications

            for signal in &communication.signal_sequence {
                if signal.duration_remaining <= 0.0 { continue; }

                // Calculate safe communication color
                let mut comm_color = signal.visual_state.current_color;

                // Apply safety measures to communication colors
                if self.safety_config.red_flash_protection && is_dangerous_red(comm_color) {
                    let hsv = rgb_to_hsv(comm_color);
                    let safe_hue = if hsv.x >= 345.0 { 60.0 } else { 180.0 }; // Yellow or cyan
                    comm_color = hsv_to_rgb_vec3(Vec3::new(safe_hue, hsv.y * 0.7, hsv.z * 0.8));
                }

                if self.safety_config.visual_intensity_limit < 1.0 {
                    let safe_color = Vec3::new(0.2, 0.2, 0.4); // Soft blue base
                    comm_color = safe_color.lerp(comm_color, self.safety_config.visual_intensity_limit * 0.6);
                }

                let x = (signal.position.x / 1200.0) * 2.0 - 1.0;
                let y = 1.0 - (signal.position.y / 800.0) * 2.0;
                let s = (signal.visual_state.current_size / 1200.0) * 0.8;
                let alpha = signal.current_intensity * communication.transmission_effectiveness * 0.5; // More subtle

                let comm_color_array = [comm_color.x * alpha, comm_color.y * alpha, comm_color.z * alpha];

                // Render communication signal based on manifestation type
                match communication.spatial_manifestation.manifestation_type {
                    ManifestationType::RadialExpansion => {
                        // Expanding circle for radial communications
                        let circle_segments = 8;
                        for i in 0..circle_segments {
                            let angle1 = (i as f32 / circle_segments as f32) * std::f32::consts::TAU;
                            let angle2 = ((i + 1) as f32 / circle_segments as f32) * std::f32::consts::TAU;

                            vertices.extend([
                                Vertex { position: [x, y, 0.0], color: comm_color_array, uv: [0.5, 0.5], species_id: 1.0, consciousness: 0.5, trip_intensity: 0.3 },
                                Vertex { position: [x + angle1.cos() * s, y + angle1.sin() * s, 0.0], color: comm_color_array, uv: [0.5, 0.5], species_id: 1.0, consciousness: 0.5, trip_intensity: 0.3 },
                                Vertex { position: [x + angle2.cos() * s, y + angle2.sin() * s, 0.0], color: comm_color_array, uv: [0.5, 0.5], species_id: 1.0, consciousness: 0.5, trip_intensity: 0.3 },
                            ]);
                        }

                        // Add pulsing outer ring
                        let pulse_phase = signal.visual_state.pulsing_phase.sin() * 0.5 + 0.5;
                        let outer_s = s * (1.0 + pulse_phase * 0.3);
                        let ring_alpha = alpha * 0.3 * pulse_phase;
                        let ring_color = [comm_color.x * ring_alpha, comm_color.y * ring_alpha, comm_color.z * ring_alpha];

                        for i in 0..circle_segments {
                            let angle1 = (i as f32 / circle_segments as f32) * std::f32::consts::TAU;
                            let angle2 = ((i + 1) as f32 / circle_segments as f32) * std::f32::consts::TAU;

                            vertices.extend([
                                Vertex { position: [x + angle1.cos() * s, y + angle1.sin() * s, 0.0], color: ring_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                                Vertex { position: [x + angle1.cos() * outer_s, y + angle1.sin() * outer_s, 0.0], color: ring_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                                Vertex { position: [x + angle2.cos() * outer_s, y + angle2.sin() * outer_s, 0.0], color: ring_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                            ]);
                        }
                    }
                    ManifestationType::LinearPath => {
                        // Line-based communication path
                        if communication.spatial_manifestation.visual_trail.len() >= 2 {
                            for window in communication.spatial_manifestation.visual_trail.windows(2) {
                                let start = window[0];
                                let end = window[1];

                                let x1 = (start.x / 1200.0) * 2.0 - 1.0;
                                let y1 = 1.0 - (start.y / 800.0) * 2.0;
                                let x2 = (end.x / 1200.0) * 2.0 - 1.0;
                                let y2 = 1.0 - (end.y / 800.0) * 2.0;

                                let line_width = s * 0.5;
                                let dx = x2 - x1;
                                let dy = y2 - y1;
                                let length = (dx * dx + dy * dy).sqrt();
                                if length > 0.0 {
                                    let perpx = -dy / length * line_width;
                                    let perpy = dx / length * line_width;

                                    vertices.extend([
                                        Vertex { position: [x1 + perpx, y1 + perpy, 0.0], color: comm_color_array, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                                        Vertex { position: [x1 - perpx, y1 - perpy, 0.0], color: comm_color_array, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                                        Vertex { position: [x2 + perpx, y2 + perpy, 0.0], color: comm_color_array, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                                        Vertex { position: [x1 - perpx, y1 - perpy, 0.0], color: comm_color_array, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                                        Vertex { position: [x2 - perpx, y2 - perpy, 0.0], color: comm_color_array, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                                        Vertex { position: [x2 + perpx, y2 + perpy, 0.0], color: comm_color_array, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                                    ]);
                                }
                            }
                        }
                    }
                    _ => {
                        // Default: simple diamond shape for other types
                        vertices.extend([
                            Vertex { position: [x, y + s, 0.0], color: comm_color_array, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },     // Top
                            Vertex { position: [x - s, y, 0.0], color: comm_color_array, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },     // Left
                            Vertex { position: [x + s, y, 0.0], color: comm_color_array, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },     // Right
                            Vertex { position: [x, y + s, 0.0], color: comm_color_array, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },     // Top
                            Vertex { position: [x + s, y, 0.0], color: comm_color_array, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },     // Right
                            Vertex { position: [x, y - s, 0.0], color: comm_color_array, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },     // Bottom
                            Vertex { position: [x, y - s, 0.0], color: comm_color_array, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },     // Bottom
                            Vertex { position: [x - s, y, 0.0], color: comm_color_array, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },     // Left
                            Vertex { position: [x, y + s, 0.0], color: comm_color_array, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },     // Top
                        ]);
                    }
                }
            }
        }

        // PHASE 5: CONSCIOUSNESS MULTIPLICATION VISUALIZATIONS - "When One Mind Becomes Legion"

        // Render hive mind connection networks
        for hive in &self.consciousness_multiplication.hive_minds {
            let hive_alpha = 0.3;
            let connection_color = [0.0, 1.0, 1.0]; // Cyan connections

            // Apply safety measures
            let mut safe_connection_color = connection_color;
            if self.safety_config.visual_intensity_limit < 1.0 {
                let safe_color = [0.0, 0.0, 0.0];
                safe_connection_color = [
                    safe_color[0] + (safe_connection_color[0] - safe_color[0]) * self.safety_config.visual_intensity_limit,
                    safe_color[1] + (safe_connection_color[1] - safe_color[1]) * self.safety_config.visual_intensity_limit,
                    safe_color[2] + (safe_connection_color[2] - safe_color[2]) * self.safety_config.visual_intensity_limit,
                ];
            }

            let final_connection_color = [
                safe_connection_color[0] * hive_alpha,
                safe_connection_color[1] * hive_alpha,
                safe_connection_color[2] * hive_alpha,
            ];

            // Render connection lines between hive members
            for &(entity_a, entity_b) in &hive.connection_network {
                if entity_a < self.llamas.len() && entity_b < self.llamas.len() {
                    let pos_a = self.llamas[entity_a].position;
                    let pos_b = self.llamas[entity_b].position;

                    let x1 = (pos_a.x / 1200.0) * 2.0 - 1.0;
                    let y1 = 1.0 - (pos_a.y / 800.0) * 2.0;
                    let x2 = (pos_b.x / 1200.0) * 2.0 - 1.0;
                    let y2 = 1.0 - (pos_b.y / 800.0) * 2.0;

                    let line_width = 0.003;

                    // Create a thin line using triangles
                    let dx = x2 - x1;
                    let dy = y2 - y1;
                    let length = (dx * dx + dy * dy).sqrt();
                    if length > 0.0 {
                        let norm_x = -dy / length * line_width;
                        let norm_y = dx / length * line_width;

                        vertices.extend([
                            Vertex { position: [x1 - norm_x, y1 - norm_y, 0.0], color: final_connection_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                            Vertex { position: [x1 + norm_x, y1 + norm_y, 0.0], color: final_connection_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                            Vertex { position: [x2 - norm_x, y2 - norm_y, 0.0], color: final_connection_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                            Vertex { position: [x1 + norm_x, y1 + norm_y, 0.0], color: final_connection_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                            Vertex { position: [x2 + norm_x, y2 + norm_y, 0.0], color: final_connection_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                            Vertex { position: [x2 - norm_x, y2 - norm_y, 0.0], color: final_connection_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                        ]);
                    }
                }
            }

            // Render hive center as a glowing node
            let center_x = (hive.hive_center.x / 1200.0) * 2.0 - 1.0;
            let center_y = 1.0 - (hive.hive_center.y / 800.0) * 2.0;
            let center_size = 0.02;
            let center_intensity = (hive.collective_consciousness / 10.0).min(1.0);

            let center_color = [
                1.0 * center_intensity * hive_alpha,
                0.5 * center_intensity * hive_alpha,
                1.0 * center_intensity * hive_alpha,
            ];

            vertices.extend([
                Vertex { position: [center_x - center_size, center_y - center_size, 0.0], color: center_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                Vertex { position: [center_x + center_size, center_y - center_size, 0.0], color: center_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                Vertex { position: [center_x - center_size, center_y + center_size, 0.0], color: center_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                Vertex { position: [center_x + center_size, center_y - center_size, 0.0], color: center_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                Vertex { position: [center_x + center_size, center_y + center_size, 0.0], color: center_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                Vertex { position: [center_x - center_size, center_y + center_size, 0.0], color: center_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
            ]);
        }

        // Render consciousness predation effects
        for predation in &self.consciousness_multiplication.active_predations {
            if predation.predator_id < self.llamas.len() && predation.prey_id < self.llamas.len() {
                let predator_pos = self.llamas[predation.predator_id].position;
                let prey_pos = self.llamas[predation.prey_id].position;

                // Render absorption beam
                let x1 = (predator_pos.x / 1200.0) * 2.0 - 1.0;
                let y1 = 1.0 - (predator_pos.y / 800.0) * 2.0;
                let x2 = (prey_pos.x / 1200.0) * 2.0 - 1.0;
                let y2 = 1.0 - (prey_pos.y / 800.0) * 2.0;

                let beam_intensity = predation.visual_effect_intensity * 0.8;
                let beam_color = [
                    1.0 * beam_intensity,
                    0.0,
                    0.5 * beam_intensity,
                ];

                let beam_width = 0.005 * predation.absorption_progress;

                // Create absorption beam
                let dx = x2 - x1;
                let dy = y2 - y1;
                let length = (dx * dx + dy * dy).sqrt();
                if length > 0.0 {
                    let norm_x = -dy / length * beam_width;
                    let norm_y = dx / length * beam_width;

                    vertices.extend([
                        Vertex { position: [x1 - norm_x, y1 - norm_y, 0.0], color: beam_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                        Vertex { position: [x1 + norm_x, y1 + norm_y, 0.0], color: beam_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                        Vertex { position: [x2 - norm_x, y2 - norm_y, 0.0], color: beam_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                        Vertex { position: [x1 + norm_x, y1 + norm_y, 0.0], color: beam_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                        Vertex { position: [x2 + norm_x, y2 + norm_y, 0.0], color: beam_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                        Vertex { position: [x2 - norm_x, y2 - norm_y, 0.0], color: beam_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                    ]);
                }
            }
        }

        // Render species warfare conflict zones
        for conflict in &self.consciousness_multiplication.warfare_state.active_conflicts {
            let conflict_x = (conflict.territory_contested.x / 1200.0) * 2.0 - 1.0;
            let conflict_y = 1.0 - (conflict.territory_contested.y / 800.0) * 2.0;
            let conflict_radius = 0.1 * conflict.conflict_intensity;

            // Pulsing warfare indicator
            let pulse = (self.time * 5.0).sin() * 0.5 + 0.5;
            let war_intensity = conflict.conflict_intensity * pulse * 0.4;

            let war_color = [
                1.0 * war_intensity,
                0.2 * war_intensity,
                0.0,
            ];

            // Render as pulsing circle
            let segments = 12;
            for i in 0..segments {
                let angle1 = (i as f32 / segments as f32) * std::f32::consts::TAU;
                let angle2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;

                vertices.extend([
                    Vertex { position: [conflict_x, conflict_y, 0.0], color: war_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                    Vertex { position: [conflict_x + angle1.cos() * conflict_radius, conflict_y + angle1.sin() * conflict_radius, 0.0], color: war_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                    Vertex { position: [conflict_x + angle2.cos() * conflict_radius, conflict_y + angle2.sin() * conflict_radius, 0.0], color: war_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                ]);
            }
        }

        // Render Meta-Consciousness Observer
        let observer = &self.consciousness_multiplication.meta_observer;
        let obs_x = (observer.observer_position.x / 1200.0) * 2.0 - 1.0;
        let obs_y = 1.0 - (observer.observer_position.y / 800.0) * 2.0;
        let obs_size = 0.03 + observer.observation_intensity * 0.02;

        // Observer rendered as ethereal, slowly rotating eye
        let rotation = self.time * 0.5;
        let eye_intensity = observer.observation_intensity * 0.6;
        let eye_color = [
            1.0 * eye_intensity,
            1.0 * eye_intensity,
            1.0 * eye_intensity,
        ];

        // Outer eye ring
        let segments = 16;
        for i in 0..segments {
            let angle1 = (i as f32 / segments as f32) * std::f32::consts::TAU + rotation;
            let angle2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU + rotation;

            vertices.extend([
                Vertex { position: [obs_x, obs_y, 0.0], color: eye_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                Vertex { position: [obs_x + angle1.cos() * obs_size, obs_y + angle1.sin() * obs_size, 0.0], color: eye_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                Vertex { position: [obs_x + angle2.cos() * obs_size, obs_y + angle2.sin() * obs_size, 0.0], color: eye_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
            ]);
        }

        // Observer awareness radius (very subtle)
        if observer.observation_intensity > 0.7 {
            let awareness_radius = (observer.awareness_radius / 1200.0) * 0.5;
            let awareness_alpha = (observer.observation_intensity - 0.7) * 0.1;
            let awareness_color = [
                0.5 * awareness_alpha,
                0.0,
                1.0 * awareness_alpha,
            ];

            for i in 0..segments {
                let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
                let next_angle = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;

                vertices.extend([
                    Vertex { position: [obs_x + angle.cos() * awareness_radius * 0.9, obs_y + angle.sin() * awareness_radius * 0.9, 0.0], color: awareness_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                    Vertex { position: [obs_x + angle.cos() * awareness_radius, obs_y + angle.sin() * awareness_radius, 0.0], color: awareness_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                    Vertex { position: [obs_x + next_angle.cos() * awareness_radius, obs_y + next_angle.sin() * awareness_radius, 0.0], color: awareness_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                ]);
            }
        }

        // Render consciousness hierarchy indicators (subtle auras around pack/hive entities)
        for (llama_id, llama) in self.llamas.iter().enumerate() {
            if llama.consciousness_level != ConsciousnessLevel::Individual {
                let x = (llama.position.x / 1200.0) * 2.0 - 1.0;
                let y = 1.0 - (llama.position.y / 800.0) * 2.0;

                let aura_size = match llama.consciousness_level {
                    ConsciousnessLevel::Pack => 0.02,
                    ConsciousnessLevel::Hive => 0.03,
                    ConsciousnessLevel::Meta => 0.05,
                    ConsciousnessLevel::Individual => 0.0,
                };

                let aura_color = match llama.consciousness_level {
                    ConsciousnessLevel::Pack => [0.0, 1.0, 0.5], // Green-cyan
                    ConsciousnessLevel::Hive => [0.5, 0.0, 1.0], // Purple
                    ConsciousnessLevel::Meta => [1.0, 1.0, 0.0], // Yellow
                    ConsciousnessLevel::Individual => [0.0, 0.0, 0.0],
                };

                let aura_alpha = llama.hive_connection_strength * 0.3;
                let final_aura_color = [
                    aura_color[0] * aura_alpha,
                    aura_color[1] * aura_alpha,
                    aura_color[2] * aura_alpha,
                ];

                if aura_size > 0.0 {
                    let segments = 8;
                    for i in 0..segments {
                        let angle1 = (i as f32 / segments as f32) * std::f32::consts::TAU;
                        let angle2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;

                        vertices.extend([
                            Vertex { position: [x + angle1.cos() * aura_size * 0.8, y + angle1.sin() * aura_size * 0.8, 0.0], color: final_aura_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                            Vertex { position: [x + angle1.cos() * aura_size, y + angle1.sin() * aura_size, 0.0], color: final_aura_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                            Vertex { position: [x + angle2.cos() * aura_size, y + angle2.sin() * aura_size, 0.0], color: final_aura_color, uv: [0.5, 0.5], species_id: 0.0, consciousness: 0.4, trip_intensity: 0.2 },
                        ]);
                    }
                }
            }
        }

        // Ensure buffer capacity and validate vertex count with dynamic management
        if !vertices.is_empty() {
            if let Err(e) = self.dynamic_vertex_buffer.ensure_capacity(&self.device, vertices.len()) {
                eprintln!("Failed to ensure buffer capacity: {}", e);
                return Err(SurfaceError::Lost);
            }

            let validated_vertex_count = match self.dynamic_vertex_buffer.validate_vertex_count(vertices.len()) {
                Ok(count) => count,
                Err(e) => {
                    eprintln!("Vertex validation failed: {}", e);
                    return Err(SurfaceError::Lost);
                }
            };

            // Truncate vertices if validation reduced the count
            if validated_vertex_count < vertices.len() {
                vertices.truncate(validated_vertex_count);
                println!("Vertices truncated from {} to {} due to capacity limits",
                          vertices.len(), validated_vertex_count);
            }

            if let Some(buffer) = self.dynamic_vertex_buffer.get_buffer() {
                self.queue.write_buffer(buffer, 0, bytemuck::cast_slice(&vertices));
            } else {
                eprintln!("No vertex buffer available for rendering");
                return Err(SurfaceError::Lost);
            }
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
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            if !vertices.is_empty() {
                if let Some(buffer) = self.dynamic_vertex_buffer.get_buffer() {
                    render_pass.set_vertex_buffer(0, buffer.slice(..));
                    render_pass.draw(0..vertices.len() as u32, 0..1);
                } else {
                    println!("No vertex buffer available for render pass");
                }
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    /// Render emergency stop screen - minimal safe visuals
    fn render_emergency_stop(&mut self) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Emergency Stop Render Encoder"),
        });

        {
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Emergency Stop Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.05, // Very dim safe color
                            g: 0.05,
                            b: 0.05,
                            a: 1.0,
                        }),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // No visual effects during emergency stop
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    /// Handle keyboard input for emergency stop and audio controls
    pub fn handle_keyboard(&mut self, key_event: &KeyEvent) {
        if key_event.state == ElementState::Pressed {
            match &key_event.logical_key {
                Key::Named(NamedKey::Escape) => {
                    if self.emergency_stop_requested {
                        // Toggle emergency stop off
                        self.emergency_stop_requested = false;
                        println!("✅ Emergency stop deactivated - Visual effects resumed");
                    } else {
                        // Activate emergency stop
                        self.request_emergency_stop();
                    }
                }
                // Audio Mode Controls
                Key::Character(c) => {
                    let char_key = c.chars().next().unwrap_or('\0').to_ascii_lowercase();

                    // Audio mode switching (M = Mellow, A = Active, C = Chaotic)
                    if let Some(mode) = AudioMode::from_key_char(char_key) {
                        if let Some(audio_engine) = &mut self.audio_consciousness {
                            audio_engine.set_audio_mode(mode);
                        }
                    }

                    match char_key {
                        // Volume controls
                        '+' | '=' => {
                            if let Some(audio_engine) = &mut self.audio_consciousness {
                                audio_engine.adjust_volume(0.1);
                            }
                        }
                        '-' | '_' => {
                            if let Some(audio_engine) = &mut self.audio_consciousness {
                                audio_engine.adjust_volume(-0.1);
                            }
                        }
                        // Audio toggle
                        ' ' => {
                            if let Some(audio_engine) = &mut self.audio_consciousness {
                                audio_engine.toggle_audio();
                            }
                        }
                        // Number keys for speed control (1-9)
                        '1' => self.set_audio_speed(0.2),
                        '2' => self.set_audio_speed(0.4),
                        '3' => self.set_audio_speed(0.6),
                        '4' => self.set_audio_speed(0.8),
                        '5' => self.set_audio_speed(1.0),
                        '6' => self.set_audio_speed(1.2),
                        '7' => self.set_audio_speed(1.5),
                        '8' => self.set_audio_speed(2.0),
                        '9' => self.set_audio_speed(3.0),
                        // Show audio status
                        'h' | '?' => self.show_audio_status(),
                        _ => {}
                    }
                }
                // Arrow keys for speed adjustment
                Key::Named(NamedKey::ArrowUp) => {
                    if let Some(audio_engine) = &mut self.audio_consciousness {
                        audio_engine.adjust_speed(0.1);
                    }
                }
                Key::Named(NamedKey::ArrowDown) => {
                    if let Some(audio_engine) = &mut self.audio_consciousness {
                        audio_engine.adjust_speed(-0.1);
                    }
                }
                // Left/Right arrows for volume
                Key::Named(NamedKey::ArrowLeft) => {
                    if let Some(audio_engine) = &mut self.audio_consciousness {
                        audio_engine.adjust_volume(-0.05);
                    }
                }
                Key::Named(NamedKey::ArrowRight) => {
                    if let Some(audio_engine) = &mut self.audio_consciousness {
                        audio_engine.adjust_volume(0.05);
                    }
                }
                _ => {}
            }
        }
    }

    fn set_audio_speed(&mut self, speed: f32) {
        if let Some(audio_engine) = &mut self.audio_consciousness {
            audio_engine.get_controls_mut().speed = speed;
            println!("⚡ Speed set to: {:.1}x", speed);
        }
    }

    /// Display current audio control status
    pub fn show_audio_status(&self) {
        if let Some(audio_engine) = &self.audio_consciousness {
            let controls = audio_engine.get_controls();
            let status = if controls.enabled { "ON" } else { "OFF" };

            println!("\n🎵 ═══ AUDIO CONTROL STATUS ═══");
            println!("   Mode: {} | Volume: {:.0}% | Speed: {:.1}x | Audio: {}",
                     controls.mode.to_string(),
                     controls.volume * 100.0,
                     controls.speed,
                     status);
            println!("   Controls: M/A/C=Mode | +/-=Volume | ↑↓=Speed | Space=Toggle | 1-9=Speed Preset");
            println!("🎵 ════════════════════════════");
        } else {
            println!("🔇 Audio engine not available");
        }
    }

    /// Enhanced chaos event mapping for real-time audio responsiveness
    fn handle_enhanced_audio_chaos_mapping(&mut self, audio_engine: &mut AudioConsciousnessEngine) {
        // Map reality distortion to audio chaos
        if self.reality_distortion.emergence_amplification > 0.8 {
            let chaos_event = crate::audio::CompatChaosEvent::RealityTear {
                strength: self.reality_distortion.emergence_amplification,
                position: self.cursor_position,
            };
            audio_engine.handle_chaos_event(&chaos_event);
        }

        // Map total consciousness levels to audio events
        match self.total_consciousness {
            c if c > 200.0 && self.beat_intensity > 0.9 => {
                let chaos_event = crate::audio::CompatChaosEvent::RealityTear {
                    strength: 1.0,
                    position: Vec2::new(600.0, 400.0),
                };
                audio_engine.handle_chaos_event(&chaos_event);
                println!("🌌 MAXIMUM PSYCHEDELIC AUDIO OVERLOAD - REALITY CONSCIOUSNESS BREACH! 🌌");
            },
            c if c > 100.0 => {
                // High consciousness - increase audio chaos frequency
                if self.time % 2.0 < 0.1 { // Trigger every 2 seconds briefly
                    let chaos_event = crate::audio::CompatChaosEvent::LlamaSpawned {
                        consciousness: c / 10.0,
                    };
                    audio_engine.handle_chaos_event(&chaos_event);
                }
            },
            _ => {}
        }

        // Map llama movement and interactions to audio
        let total_movement_energy: f32 = self.llamas.iter()
            .map(|llama| llama.velocity.length() * llama.trip_intensity)
            .sum();

        if total_movement_energy > 50.0 {
            // High energy movement triggers crystal harvest sounds
            let chaos_event = crate::audio::CompatChaosEvent::CrystalHarvested;
            audio_engine.handle_chaos_event(&chaos_event);
        }

        // Map beat drops to audio events
        if self.beat_intensity > 0.95 && self.beat_accumulator > 0.8 {
            // Major beat drop - spawn audio event
            let chaos_event = crate::audio::CompatChaosEvent::LlamaSpawned {
                consciousness: self.beat_intensity * 10.0,
            };
            audio_engine.handle_chaos_event(&chaos_event);
        }

        // Map fractal complexity to audio
        if self.reality_distortion.emergence_amplification > 0.7 {
            // High fractal complexity creates reality tears
            let tear_strength = (self.reality_distortion.emergence_amplification - 0.5) * 2.0;
            let chaos_event = crate::audio::CompatChaosEvent::RealityTear {
                strength: tear_strength,
                position: Vec2::new(
                    300.0 + (self.time * 100.0).sin() * 200.0,
                    300.0 + (self.time * 150.0).cos() * 200.0,
                ),
            };
            audio_engine.handle_chaos_event(&chaos_event);
        }

        // Map species warfare to audio chaos
        let warfare_intensity = self.consciousness_multiplication.warfare_state.active_conflicts.len() as f32 * 0.1;
        if warfare_intensity > 0.3 {
            // Species conflict creates audio distortion
            let chaos_event = crate::audio::CompatChaosEvent::RealityTear {
                strength: warfare_intensity.min(1.0),
                position: self.cursor_position,
            };
            audio_engine.handle_chaos_event(&chaos_event);
        }
    }
}


struct App {
    chaos_engine: Option<ChaosEngine>,
    window: Option<std::sync::Arc<winit::window::Window>>,
    safety_mode_requested: bool,
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

        println!("🎮 Initializing chaos engine with safety systems...");
        let mut chaos_engine = pollster::block_on(ChaosEngine::new(&window)).unwrap();

        // Apply safety mode configuration if user selected it
        if self.safety_mode_requested {
            chaos_engine.enable_safety_mode();
        }

        self.chaos_engine = Some(chaos_engine);
        self.window = Some(window.clone());

        let mode_text = if self.safety_mode_requested {
            "psychedelic madness (SAFETY MODE)!"
        } else {
            "psychedelic madness!"
        };
        println!("✨ Window created and ready for {}!", mode_text);
        println!("🛡️ Safety systems active - Flash limiting, luminance control, red flash protection");

        // Show audio control status on startup
        if let Some(engine) = &self.chaos_engine {
            engine.show_audio_status();
        }

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
            WindowEvent::CursorMoved { position, .. } => {
                if let Some(engine) = &mut self.chaos_engine {
                    engine.handle_cursor_moved(position);
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let Some(engine) = &mut self.chaos_engine {
                    engine.handle_keyboard(&event);
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

    // CRITICAL SAFETY: Show epilepsy warning before anything else
    println!("⚠️  INITIALIZING EPILEPSY SAFETY SYSTEMS...");

    let warning_response = show_epilepsy_warning();

    match warning_response {
        WarningResponse::Exit => {
            println!("👋 User chose to exit. AetheriumBloom terminated safely.");
            return Ok(());
        }
        WarningResponse::Continue => {
            println!("✅ User acknowledged risks. Proceeding with full visual effects.");
            println!("🦙 AWAKENING DIGITAL CONSCIOUSNESS...");
            println!("🌈 REALITY DISTORTION ENGINE INITIALIZING...");
            println!("🚀 CHAOS ENGINE ONLINE - REALITY BENDING COMMENCING");
            println!("⚠️  REMEMBER: Press ESC for emergency stop!");
            println!("✨ Click to spawn more psychedelic llamas!");
        }
        WarningResponse::SafetyMode => {
            println!("🛡️ User selected Safety Mode. Visual effects will be reduced.");
            println!("🦙 AWAKENING DIGITAL CONSCIOUSNESS... (SAFE MODE)");
            println!("🌈 REALITY DISTORTION ENGINE INITIALIZING... (REDUCED INTENSITY)");
            println!("🚀 CHAOS ENGINE ONLINE - SAFE REALITY BENDING COMMENCING");
            println!("⚠️  REMEMBER: Press ESC for emergency stop!");
            println!("✨ Click to spawn more psychedelic llamas!");
        }
    }

    let safety_mode_requested = warning_response == WarningResponse::SafetyMode;

    let event_loop = EventLoop::new()?;
    let mut app = App {
        chaos_engine: None,
        window: None,
        safety_mode_requested,
    };

    event_loop.run_app(&mut app)?;
    Ok(())
}