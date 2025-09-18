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

// === CRITICAL SAFETY SYSTEMS FOR EPILEPSY PROTECTION ===

/// User's response to safety warning
#[derive(Debug, Clone, PartialEq)]
pub enum WarningResponse {
    Continue,     // User accepts risk and wants full visual effects
    SafetyMode,   // User wants reduced visual intensity
    Exit,         // User chooses to exit
}

/// Simple safety configuration
#[derive(Debug, Clone)]
pub struct SafetyConfig {
    pub visual_intensity_limit: f32,  // 0.0 to 1.0
    pub max_flash_rate: f32,          // Hz (3.0 is international standard)
    pub max_luminance_change: f32,    // 0.0 to 1.0 (0.1 is 10% standard)
    pub red_flash_protection: bool,
}

impl Default for SafetyConfig {
    fn default() -> Self {
        Self {
            visual_intensity_limit: 1.0,     // Full intensity by default
            max_flash_rate: 3.0,             // International safety standard
            max_luminance_change: 0.1,       // 10% WCAG standard
            red_flash_protection: true,
        }
    }
}

impl SafetyConfig {
    pub fn safe_mode() -> Self {
        Self {
            visual_intensity_limit: 0.5,     // 50% intensity
            max_flash_rate: 2.0,             // Even more conservative
            max_luminance_change: 0.05,      // 5% change
            red_flash_protection: true,
        }
    }
}

/// Simple flash tracker to enforce rate limiting
#[derive(Debug)]
pub struct FlashTracker {
    last_major_change: f64,
    change_count_in_window: u32,
    window_start_time: f64,
    window_duration: f64,
}

impl FlashTracker {
    pub fn new() -> Self {
        Self {
            last_major_change: 0.0,
            change_count_in_window: 0,
            window_start_time: 0.0,
            window_duration: 1.0, // 1 second window
        }
    }

    pub fn can_allow_flash(&mut self, current_time: f64, max_rate: f32) -> bool {
        // Reset window if needed
        if current_time - self.window_start_time > self.window_duration {
            self.window_start_time = current_time;
            self.change_count_in_window = 0;
        }

        // Check if we're under the rate limit
        let flashes_per_second = self.change_count_in_window as f32 / self.window_duration as f32;
        if flashes_per_second >= max_rate {
            return false;
        }

        // Check minimum interval (1/3 Hz = 0.33 seconds for 3 Hz)
        let min_interval = 1.0 / max_rate as f64;
        if current_time - self.last_major_change < min_interval {
            return false;
        }

        true
    }

    pub fn record_flash(&mut self, current_time: f64) {
        self.last_major_change = current_time;
        self.change_count_in_window += 1;
    }
}

/// Calculate perceived luminance of a color
fn calculate_luminance(color: &Vec3) -> f32 {
    // ITU-R BT.709 luma coefficients
    0.2126 * color.x + 0.7152 * color.y + 0.0722 * color.z
}

/// Limit luminance changes to safe levels
fn limit_luminance_change(new_color: Vec3, old_color: Vec3, max_change: f32) -> Vec3 {
    let new_luma = calculate_luminance(&new_color);
    let old_luma = calculate_luminance(&old_color);
    let change = (new_luma - old_luma).abs();

    if change <= max_change {
        return new_color;
    }

    // Interpolate to safe level
    let safe_factor = max_change / change;
    old_color.lerp(new_color, safe_factor * 0.5) // Extra conservative with 0.5 factor
}

/// Check if color is in dangerous red range
fn is_dangerous_red(color: Vec3) -> bool {
    let hsv = rgb_to_hsv(color);
    (hsv.x >= 345.0 || hsv.x <= 15.0) && hsv.y > 0.5 && hsv.z > 0.5
}

/// Convert RGB to HSV
fn rgb_to_hsv(rgb: Vec3) -> Vec3 {
    let max = rgb.x.max(rgb.y.max(rgb.z));
    let min = rgb.x.min(rgb.y.min(rgb.z));
    let delta = max - min;

    let hue = if delta == 0.0 {
        0.0
    } else if max == rgb.x {
        60.0 * (((rgb.y - rgb.z) / delta) % 6.0)
    } else if max == rgb.y {
        60.0 * ((rgb.z - rgb.x) / delta + 2.0)
    } else {
        60.0 * ((rgb.x - rgb.y) / delta + 4.0)
    };

    let saturation = if max == 0.0 { 0.0 } else { delta / max };
    let value = max;

    Vec3::new(hue.abs(), saturation, value)
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

/// Consciousness Crystal - harvestable nodes that enhance abilities
#[derive(Debug, Clone)]
pub struct ConsciousnessCrystal {
    pub position: Vec2,
    pub consciousness_energy: f32,    // Amount of consciousness stored
    pub resonance_frequency: f32,     // Prime-based frequency for interactions
    pub visual_intensity: f32,        // Current visual brightness/pulsing
    pub growth_rate: f32,             // How fast it accumulates energy
    pub harvest_radius: f32,          // Range for llama interaction
    pub age: f32,                     // How long it has existed
    pub crystal_type: CrystalType,    // Different types with different properties
}

#[derive(Debug, Clone, PartialEq)]
pub enum CrystalType {
    Resonance,    // Amplifies harmonic resonance
    Chaos,        // Increases chaos acceptance
    Memory,       // Enhances memory formation
    Social,       // Strengthens social bonds
    Quantum,      // Affects quantum state (for Quantum Sheep)
}

impl ConsciousnessCrystal {
    fn new(position: Vec2, crystal_type: CrystalType) -> Self {
        let (base_energy, base_frequency, base_growth) = match crystal_type {
            CrystalType::Resonance => (0.3, 7.0, 0.02),   // High frequency, moderate growth
            CrystalType::Chaos => (0.5, 11.0, 0.03),      // High energy, prime frequency
            CrystalType::Memory => (0.2, 5.0, 0.015),     // Low energy, steady growth
            CrystalType::Social => (0.4, 13.0, 0.025),    // Social prime frequency
            CrystalType::Quantum => (0.6, 17.0, 0.01),    // High energy, slow growth
        };

        Self {
            position,
            consciousness_energy: base_energy,
            resonance_frequency: base_frequency,
            visual_intensity: 0.5,
            growth_rate: base_growth,
            harvest_radius: 25.0 + fastrand::f32() * 15.0, // 25-40 radius
            age: 0.0,
            crystal_type,
        }
    }

    fn update(&mut self, dt: f32, beat_intensity: f32, cosmic_time: f64) {
        self.age += dt;

        // Crystals grow in consciousness energy over time
        let growth_multiplier = 1.0 + beat_intensity * 0.5;
        self.consciousness_energy += self.growth_rate * dt * growth_multiplier;
        self.consciousness_energy = self.consciousness_energy.min(2.0); // Cap at 2.0

        // Visual intensity pulses based on prime frequency and beat
        let frequency_phase = (cosmic_time as f32 * self.resonance_frequency * 0.1).sin();
        let beat_phase = beat_intensity * 0.5;
        self.visual_intensity = 0.3 + frequency_phase.abs() * 0.4 + beat_phase * 0.3;

        // Older crystals have larger harvest radius
        self.harvest_radius = (25.0 + self.age * 2.0).min(60.0);
    }

    fn get_harvest_amount(&self) -> f32 {
        // More energy = more harvest potential
        self.consciousness_energy * 0.3
    }

    fn harvest(&mut self, amount: f32) -> f32 {
        let harvested = amount.min(self.consciousness_energy);
        self.consciousness_energy -= harvested;
        harvested
    }

    fn get_color(&self) -> Vec3 {
        let base_hue = match self.crystal_type {
            CrystalType::Resonance => 180.0, // Cyan
            CrystalType::Chaos => 300.0,     // Magenta
            CrystalType::Memory => 120.0,    // Green
            CrystalType::Social => 60.0,     // Yellow
            CrystalType::Quantum => 240.0,   // Blue
        };

        let saturation = 0.8 + self.consciousness_energy * 0.2;
        let brightness = 0.4 + self.visual_intensity * 0.6;

        hsv_to_rgb(base_hue, saturation, brightness)
    }
}

/// Environmental consciousness field - affects entity behavior
#[derive(Debug, Clone)]
pub struct ConsciousnessField {
    pub grid_size: usize,                    // Resolution of the field grid
    pub consciousness_density: Vec<Vec<f32>>, // 2D grid of consciousness values
    pub width: f32,                          // World width
    pub height: f32,                         // World height
}

impl ConsciousnessField {
    fn new(width: f32, height: f32, grid_size: usize) -> Self {
        let consciousness_density = vec![vec![0.1; grid_size]; grid_size]; // Start with low consciousness

        Self {
            grid_size,
            consciousness_density,
            width,
            height,
        }
    }

    fn get_consciousness_at(&self, position: Vec2) -> f32 {
        let x = ((position.x / self.width) * self.grid_size as f32) as usize;
        let y = ((position.y / self.height) * self.grid_size as f32) as usize;

        if x < self.grid_size && y < self.grid_size {
            self.consciousness_density[y][x]
        } else {
            0.1 // Default consciousness outside grid
        }
    }

    fn add_consciousness_at(&mut self, position: Vec2, amount: f32) {
        let x = ((position.x / self.width) * self.grid_size as f32) as usize;
        let y = ((position.y / self.height) * self.grid_size as f32) as usize;

        if x < self.grid_size && y < self.grid_size {
            self.consciousness_density[y][x] = (self.consciousness_density[y][x] + amount).min(2.0);
        }
    }

    fn update(&mut self, dt: f32) {
        // Gradual diffusion and decay of consciousness
        for y in 0..self.grid_size {
            for x in 0..self.grid_size {
                // Decay consciousness over time
                self.consciousness_density[y][x] *= 0.999;

                // Simple diffusion with neighbors
                let mut neighbor_sum = 0.0;
                let mut neighbor_count = 0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 { continue; }
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;

                        if nx >= 0 && nx < self.grid_size as i32 && ny >= 0 && ny < self.grid_size as i32 {
                            neighbor_sum += self.consciousness_density[ny as usize][nx as usize];
                            neighbor_count += 1;
                        }
                    }
                }

                if neighbor_count > 0 {
                    let neighbor_avg = neighbor_sum / neighbor_count as f32;
                    // Slight diffusion toward neighbors
                    let diffusion_rate = 0.01 * dt;
                    self.consciousness_density[y][x] =
                        self.consciousness_density[y][x] * (1.0 - diffusion_rate) +
                        neighbor_avg * diffusion_rate;
                }
            }
        }
    }
}

/// Digital Ecosystem containing crystals and consciousness fields
#[derive(Debug)]
pub struct DigitalEcosystem {
    pub consciousness_fields: ConsciousnessField,
    pub crystal_formations: Vec<ConsciousnessCrystal>,
    pub chaos_accumulation: f32,              // Global chaos level from clicks
    pub mutation_threshold: f32,              // When mutations trigger
    pub reality_tears: Vec<RealityTear>,      // Visual glitches
    pub territory_zones: Vec<TerritoryZone>,  // Different environmental regions
}

/// Reality Tear - visual glitch representing consciousness breakthrough
#[derive(Debug, Clone)]
pub struct RealityTear {
    pub position: Vec2,
    pub size: f32,
    pub intensity: f32,
    pub age: f32,
    pub tear_type: TearType,
}

#[derive(Debug, Clone)]
pub enum TearType {
    Static,      // Stationary glitch
    Moving,      // Travels across screen
    Pulsing,     // Grows and shrinks
    Fragmenting, // Splits into smaller tears
}

impl RealityTear {
    fn new(position: Vec2, tear_type: TearType) -> Self {
        Self {
            position,
            size: 5.0 + fastrand::f32() * 15.0,
            intensity: 0.7 + fastrand::f32() * 0.3,
            age: 0.0,
            tear_type,
        }
    }

    fn update(&mut self, dt: f32, cosmic_time: f64) {
        self.age += dt;

        match self.tear_type {
            TearType::Static => {
                // Just age and fade
                self.intensity *= 0.995;
            },
            TearType::Moving => {
                // Move in chaotic pattern
                let move_speed = 50.0;
                let chaos_angle = (cosmic_time as f32 * 3.0 + self.position.length() * 0.01).sin() * 6.28;
                self.position.x += chaos_angle.cos() * move_speed * dt;
                self.position.y += chaos_angle.sin() * move_speed * dt;
                self.intensity *= 0.99;
            },
            TearType::Pulsing => {
                // Pulsing size
                let pulse = (cosmic_time as f32 * 5.0).sin().abs();
                self.size = (5.0 + fastrand::f32() * 15.0) * (0.5 + pulse * 0.5);
                self.intensity *= 0.995;
            },
            TearType::Fragmenting => {
                // Grows then fragments (handled in ecosystem update)
                if self.age < 2.0 {
                    self.size += dt * 10.0;
                }
                self.intensity *= 0.99;
            },
        }

        // Keep within bounds
        if self.position.x < 0.0 { self.position.x = 1200.0; }
        if self.position.x > 1200.0 { self.position.x = 0.0; }
        if self.position.y < 0.0 { self.position.y = 800.0; }
        if self.position.y > 800.0 { self.position.y = 0.0; }
    }

    fn should_remove(&self) -> bool {
        self.intensity < 0.1 || self.age > 10.0
    }
}

/// Territory Zone - regions with different consciousness properties
#[derive(Debug, Clone)]
pub struct TerritoryZone {
    pub center: Vec2,
    pub radius: f32,
    pub zone_type: ZoneType,
    pub strength: f32,
    pub age: f32,
}

#[derive(Debug, Clone)]
pub enum ZoneType {
    Harmonic,    // Enhances resonance and social bonding
    Chaotic,     // Increases chaos and reality distortion
    Meditative,  // Calms entities, increases memory formation
    Quantum,     // Quantum effects amplified
}

impl TerritoryZone {
    fn new(center: Vec2, zone_type: ZoneType) -> Self {
        let radius = match zone_type {
            ZoneType::Harmonic => 80.0 + fastrand::f32() * 40.0,
            ZoneType::Chaotic => 60.0 + fastrand::f32() * 60.0,
            ZoneType::Meditative => 100.0 + fastrand::f32() * 50.0,
            ZoneType::Quantum => 70.0 + fastrand::f32() * 30.0,
        };

        Self {
            center,
            radius,
            zone_type,
            strength: 0.3 + fastrand::f32() * 0.4,
            age: 0.0,
        }
    }

    fn update(&mut self, dt: f32, cosmic_time: f64) {
        self.age += dt;

        // Zones slowly grow and change strength
        let growth_rate = match self.zone_type {
            ZoneType::Harmonic => 0.5,
            ZoneType::Chaotic => 1.0,
            ZoneType::Meditative => 0.3,
            ZoneType::Quantum => 0.8,
        };

        self.radius += growth_rate * dt;
        self.radius = self.radius.min(150.0);

        // Strength oscillates
        let oscillation = (cosmic_time as f32 * 0.5 + self.center.length() * 0.001).sin() * 0.1;
        self.strength = (0.3 + fastrand::f32() * 0.4 + oscillation).clamp(0.1, 0.8);
    }

    fn affects_position(&self, position: Vec2) -> f32 {
        let distance = self.center.distance(position);
        if distance < self.radius {
            let factor = 1.0 - (distance / self.radius);
            factor * self.strength
        } else {
            0.0
        }
    }
}

impl DigitalEcosystem {
    fn new() -> Self {
        let consciousness_fields = ConsciousnessField::new(1200.0, 800.0, 40); // 40x40 grid

        // Start with a few crystals
        let mut crystal_formations = Vec::new();
        for _ in 0..3 {
            let position = Vec2::new(fastrand::f32() * 1200.0, fastrand::f32() * 800.0);
            let crystal_type = match fastrand::usize(0..5) {
                0 => CrystalType::Resonance,
                1 => CrystalType::Chaos,
                2 => CrystalType::Memory,
                3 => CrystalType::Social,
                _ => CrystalType::Quantum,
            };
            crystal_formations.push(ConsciousnessCrystal::new(position, crystal_type));
        }

        // Start with one territory zone
        let mut territory_zones = Vec::new();
        let zone_center = Vec2::new(fastrand::f32() * 1200.0, fastrand::f32() * 800.0);
        let zone_type = match fastrand::usize(0..4) {
            0 => ZoneType::Harmonic,
            1 => ZoneType::Chaotic,
            2 => ZoneType::Meditative,
            _ => ZoneType::Quantum,
        };
        territory_zones.push(TerritoryZone::new(zone_center, zone_type));

        Self {
            consciousness_fields,
            crystal_formations,
            chaos_accumulation: 0.0,
            mutation_threshold: 3.0, // Mutations trigger when chaos reaches this level
            reality_tears: Vec::new(),
            territory_zones,
        }
    }

    fn update(&mut self, dt: f32, cosmic_time: f64, beat_intensity: f32) {
        // Update consciousness fields
        self.consciousness_fields.update(dt);

        // Update crystals
        for crystal in &mut self.crystal_formations {
            crystal.update(dt, beat_intensity, cosmic_time);
        }

        // Update reality tears
        self.reality_tears.retain_mut(|tear| {
            tear.update(dt, cosmic_time);
            !tear.should_remove()
        });

        // Handle fragmenting tears
        let mut new_tears = Vec::new();
        for tear in &self.reality_tears {
            if matches!(tear.tear_type, TearType::Fragmenting) && tear.age > 2.0 && tear.size > 15.0 {
                // Fragment into smaller tears
                for _ in 0..3 {
                    let offset = Vec2::new(
                        (fastrand::f32() - 0.5) * 30.0,
                        (fastrand::f32() - 0.5) * 30.0,
                    );
                    let fragment_pos = tear.position + offset;
                    let fragment_type = if fastrand::f32() < 0.5 { TearType::Moving } else { TearType::Static };
                    new_tears.push(RealityTear::new(fragment_pos, fragment_type));
                }
            }
        }
        self.reality_tears.extend(new_tears);

        // Update territory zones
        for zone in &mut self.territory_zones {
            zone.update(dt, cosmic_time);
        }

        // Spawn new crystals occasionally based on chaos level
        if fastrand::f32() < 0.001 * (1.0 + self.chaos_accumulation * 0.1) {
            let position = Vec2::new(fastrand::f32() * 1200.0, fastrand::f32() * 800.0);
            let crystal_type = if self.chaos_accumulation > 1.0 {
                // Higher chaos = more exotic crystal types
                match fastrand::usize(0..5) {
                    0..=1 => CrystalType::Chaos,
                    2 => CrystalType::Quantum,
                    3 => CrystalType::Resonance,
                    _ => CrystalType::Social,
                }
            } else {
                // Lower chaos = basic types
                match fastrand::usize(0..3) {
                    0 => CrystalType::Resonance,
                    1 => CrystalType::Memory,
                    _ => CrystalType::Social,
                }
            };
            self.crystal_formations.push(ConsciousnessCrystal::new(position, crystal_type));
        }

        // Spawn new territory zones occasionally
        if self.territory_zones.len() < 5 && fastrand::f32() < 0.0005 * (1.0 + self.chaos_accumulation * 0.2) {
            let zone_center = Vec2::new(fastrand::f32() * 1200.0, fastrand::f32() * 800.0);
            let zone_type = match fastrand::usize(0..4) {
                0 => ZoneType::Harmonic,
                1 => ZoneType::Chaotic,
                2 => ZoneType::Meditative,
                _ => ZoneType::Quantum,
            };
            self.territory_zones.push(TerritoryZone::new(zone_center, zone_type));
        }

        // Trigger reality tears when consciousness gets high
        let high_consciousness_areas = self.get_high_consciousness_positions();
        for position in high_consciousness_areas {
            if fastrand::f32() < 0.02 {  // 2% chance per high consciousness area
                let tear_type = match fastrand::usize(0..4) {
                    0 => TearType::Static,
                    1 => TearType::Moving,
                    2 => TearType::Pulsing,
                    _ => TearType::Fragmenting,
                };
                self.reality_tears.push(RealityTear::new(position, tear_type));
            }
        }

        // Decay chaos accumulation gradually
        self.chaos_accumulation *= 0.995;
    }

    fn add_chaos(&mut self, amount: f32) {
        self.chaos_accumulation += amount;
    }

    fn should_trigger_mutation(&self) -> bool {
        self.chaos_accumulation > self.mutation_threshold
    }

    fn reset_chaos_for_mutation(&mut self) {
        self.chaos_accumulation *= 0.3; // Reduce but don't eliminate
        self.mutation_threshold += 0.5; // Make next mutation harder
    }

    fn get_high_consciousness_positions(&self) -> Vec<Vec2> {
        let mut positions = Vec::new();
        let threshold = 0.8; // High consciousness threshold

        for y in 0..self.consciousness_fields.grid_size {
            for x in 0..self.consciousness_fields.grid_size {
                if self.consciousness_fields.consciousness_density[y][x] > threshold {
                    let world_x = (x as f32 / self.consciousness_fields.grid_size as f32) * self.consciousness_fields.width;
                    let world_y = (y as f32 / self.consciousness_fields.grid_size as f32) * self.consciousness_fields.height;
                    positions.push(Vec2::new(world_x, world_y));
                }
            }
        }

        positions
    }

    fn get_territory_effects(&self, position: Vec2) -> TerritoryEffects {
        let mut effects = TerritoryEffects::default();

        for zone in &self.territory_zones {
            let influence = zone.affects_position(position);
            if influence > 0.0 {
                match zone.zone_type {
                    ZoneType::Harmonic => {
                        effects.resonance_boost += influence * 0.3;
                        effects.social_boost += influence * 0.2;
                    },
                    ZoneType::Chaotic => {
                        effects.chaos_boost += influence * 0.4;
                        effects.reality_distortion_boost += influence * 0.3;
                    },
                    ZoneType::Meditative => {
                        effects.memory_boost += influence * 0.3;
                        effects.consciousness_growth_boost += influence * 0.2;
                    },
                    ZoneType::Quantum => {
                        effects.quantum_boost += influence * 0.5;
                        effects.exploration_boost += influence * 0.2;
                    },
                }
            }
        }

        effects
    }
}

#[derive(Debug, Default)]
pub struct TerritoryEffects {
    pub resonance_boost: f32,
    pub social_boost: f32,
    pub chaos_boost: f32,
    pub reality_distortion_boost: f32,
    pub memory_boost: f32,
    pub consciousness_growth_boost: f32,
    pub quantum_boost: f32,
    pub exploration_boost: f32,
}

// Mathematical chaos engine using pre-calculated primes

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

// Phase 2: Mathematical Chaos Engine Components

#[derive(Debug, Clone)]
pub struct ChaosDecisionEngine {
    pub dimensions: [f32; 11],          // 11D consciousness state space
    pub prime_generators: Vec<u64>,     // Prime number chaos sources
    pub quantum_fluctuations: f32,      // Uncertainty principle simulation
    pub harmonic_resonance: [f32; 7],   // Musical mathematics integration
}

impl ChaosDecisionEngine {
    fn new() -> Self {
        // Pre-calculated first 7 primes for performance
        let prime_generators = vec![2, 3, 5, 7, 11, 13, 17];

        Self {
            dimensions: [fastrand::f32(); 11],
            prime_generators,
            quantum_fluctuations: fastrand::f32(),
            harmonic_resonance: [fastrand::f32(); 7],
        }
    }

    fn update(&mut self, llama_data: LlamaSnapshot, beat_intensity: f32, cosmic_time: f64) {
        // Update 11D consciousness state space based on llama state
        self.dimensions[0] = (llama_data.color.x / 360.0 * std::f32::consts::TAU).sin();
        self.dimensions[1] = llama_data.awareness_level;
        self.dimensions[2] = llama_data.social_attraction;
        self.dimensions[3] = llama_data.exploration_drive;
        self.dimensions[4] = llama_data.reality_distortion;
        self.dimensions[5] = beat_intensity;
        self.dimensions[6] = llama_data.emotional_state;
        self.dimensions[7] = llama_data.memory_intensity;
        self.dimensions[8] = (cosmic_time as f32 * 0.1).sin();
        self.dimensions[9] = llama_data.consciousness;
        self.dimensions[10] = fastrand::f32(); // Pure chaos injection

        // Update quantum fluctuations using prime chaos
        let prime_chaos = self.calculate_prime_chaos(cosmic_time);
        self.quantum_fluctuations = (self.quantum_fluctuations + prime_chaos * 0.1).clamp(0.0, 1.0);

        // Update harmonic resonance layers
        for (i, harmonic) in self.harmonic_resonance.iter_mut().enumerate() {
            let prime_freq = self.prime_generators[i] as f32 / 100.0;
            *harmonic = (cosmic_time as f32 * prime_freq * 0.01).sin() * beat_intensity;
        }
    }

    fn calculate_prime_chaos(&self, cosmic_time: f64) -> f32 {
        let mut chaos = 0.0;
        for (i, &prime) in self.prime_generators.iter().enumerate() {
            let freq = prime as f32 / 1000.0;
            let phase = cosmic_time as f32 * freq + (i as f32 * 0.5);
            chaos += (phase.sin() * phase.cos()).abs();
        }
        (chaos / self.prime_generators.len() as f32).clamp(0.0, 1.0)
    }

    fn get_decision_vector(&self) -> DecisionVector {
        // Calculate behavior weights from 11D space
        let movement_urgency = self.dimensions[1] + self.dimensions[5] + self.quantum_fluctuations;
        let exploration_drive = self.dimensions[3] + self.dimensions[10] + self.harmonic_resonance[2];
        let social_attraction = self.dimensions[2] + self.dimensions[6] * 0.5;
        let chaos_acceptance = self.dimensions[10] + self.quantum_fluctuations + self.harmonic_resonance[6];

        DecisionVector {
            movement_urgency: movement_urgency.clamp(0.0, 1.0),
            exploration_drive: exploration_drive.clamp(0.0, 1.0),
            social_attraction: social_attraction.clamp(0.0, 1.0),
            chaos_acceptance: chaos_acceptance.clamp(0.0, 1.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DecisionVector {
    pub movement_urgency: f32,
    pub exploration_drive: f32,
    pub social_attraction: f32,
    pub chaos_acceptance: f32,
}

#[derive(Debug, Clone)]
pub struct LlamaSnapshot {
    pub color: Vec2,
    pub awareness_level: f32,
    pub social_attraction: f32,
    pub exploration_drive: f32,
    pub reality_distortion: f32,
    pub emotional_state: f32,
    pub memory_intensity: f32,
    pub consciousness: f32,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum SpeciesType {
    DiscoLlama,
    QuantumSheep,
    HypnoCamel,
}

// PHASE 5: CONSCIOUSNESS MULTIPLICATION - "When One Mind Becomes Legion"

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsciousnessLevel {
    Individual,    // Single entity consciousness
    Pack,         // Small group collective (2-8 entities)
    Hive,         // Large collective super-consciousness (9+ entities)
    Meta,         // Transcendent awareness observer
}

#[derive(Debug, Clone)]
pub struct ConsciousnessHierarchy {
    pub level: ConsciousnessLevel,
    pub members: Vec<usize>,           // Entity indices that belong to this consciousness
    pub collective_strength: f32,      // Combined consciousness power
    pub territory_control: f32,        // Territorial influence 0.0-1.0
    pub war_efficiency: f32,           // Combat effectiveness modifier
    pub hive_connection_strength: f32, // How strongly hive members are connected
    pub absorption_capacity: f32,      // Ability to absorb other consciousness
}

#[derive(Debug, Clone)]
pub struct WarfareState {
    pub species_populations: [u32; 3],        // [DiscoLlama, QuantumSheep, HypnoCamel]
    pub territorial_dominance: [f32; 3],      // Territory control per species 0.0-1.0
    pub extinction_pressure: [f32; 3],        // Extinction threat level per species
    pub consciousness_crystals_controlled: [u32; 3], // Resource control
    pub active_conflicts: Vec<SpeciesConflict>,
}

#[derive(Debug, Clone)]
pub struct SpeciesConflict {
    pub attacker_species: SpeciesType,
    pub defender_species: SpeciesType,
    pub conflict_intensity: f32,       // 0.0-1.0 intensity of the conflict
    pub territory_contested: Vec2,     // Center point of contested territory
    pub duration: f32,                 // How long the conflict has lasted
    pub victory_threshold: f32,        // Consciousness advantage needed to win
}

#[derive(Debug, Clone)]
pub struct HiveMind {
    pub collective_id: usize,
    pub member_entities: Vec<usize>,   // Indices of entities in the hive
    pub collective_consciousness: f32,  // Combined consciousness level
    pub hive_center: Vec2,             // Geometric center of the hive
    pub connection_network: Vec<(usize, usize)>, // Pairs of connected entities
    pub shared_memories: Vec<Vec2>,    // Collective memory fragments
    pub collective_decision_weight: f32, // How much the hive influences individual decisions
    pub emergence_timestamp: f32,      // When this hive mind formed
}

#[derive(Debug, Clone)]
pub struct ConsciousnessPredation {
    pub predator_id: usize,
    pub prey_id: usize,
    pub absorption_progress: f32,      // 0.0-1.0 progress of consumption
    pub resistance_strength: f32,      // How much the prey is fighting back
    pub visual_effect_intensity: f32,  // Visual feedback for absorption event
}

#[derive(Debug, Clone)]
pub struct MetaConsciousnessObserver {
    pub observer_position: Vec2,
    pub awareness_radius: f32,         // How far the observer can see
    pub intervention_power: f32,       // Ability to influence consciousness wars
    pub observation_intensity: f32,    // Current focus level
    pub last_intervention: f32,        // Time since last intervention
    pub consciousness_analysis: ConsciousnessAnalysis,
}

#[derive(Debug, Clone)]
pub struct ConsciousnessAnalysis {
    pub total_individual_entities: u32,
    pub total_pack_collectives: u32,
    pub total_hive_minds: u32,
    pub dominant_species: SpeciesType,
    pub extinction_imminent: Option<SpeciesType>, // Species about to go extinct
    pub consciousness_distribution: [f32; 3],     // Consciousness per species
    pub warfare_intensity: f32,       // Overall conflict level
    pub ecosystem_stability: f32,     // 0.0-1.0 stability measure
}

#[derive(Debug, Clone)]
pub struct ConsciousnessMultiplicationSystem {
    pub hierarchy_levels: Vec<ConsciousnessHierarchy>,
    pub hive_minds: Vec<HiveMind>,
    pub active_predations: Vec<ConsciousnessPredation>,
    pub warfare_state: WarfareState,
    pub meta_observer: MetaConsciousnessObserver,
    pub evolution_pressure_accumulator: f32,
    pub next_hive_id: usize,
    pub consciousness_crystal_spawn_rate: f32,
    pub territorial_conflict_threshold: f32,
}

#[derive(Debug, Clone)]
pub struct AdvancedBeatEngine {
    pub primary_rhythm: f32,            // Core mathematical heartbeat
    pub harmonic_layers: Vec<f32>,      // Multiple overlapping rhythms
    pub chaos_amplification: f32,       // Feedback loop strength
    pub consciousness_coupling: f32,    // Beat-consciousness interaction
    prime_list: Vec<u64>,               // Pre-calculated primes
    time_accumulator: f64,
}

impl AdvancedBeatEngine {
    fn new() -> Self {
        Self {
            primary_rhythm: 120.0, // BPM
            harmonic_layers: vec![1.0, 0.5, 0.25, 0.75, 1.33], // Harmonic ratios
            chaos_amplification: 0.3,
            consciousness_coupling: 0.5,
            prime_list: vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97], // First 25 primes
            time_accumulator: 0.0,
        }
    }

    fn update(&mut self, dt: f32, total_consciousness: f32) -> f32 {
        self.time_accumulator += dt as f64;

        // Calculate base beat from primary rhythm
        let beat_phase = (self.time_accumulator * self.primary_rhythm as f64 / 60.0) % 1.0;
        let base_beat = ((beat_phase as f32) * std::f32::consts::TAU).sin().abs();

        // Add harmonic layers with prime number modulation
        let mut harmonic_sum = 0.0f32;
        for (i, &ratio) in self.harmonic_layers.iter().enumerate() {
            let prime = self.prime_list[i % self.prime_list.len()];
            let prime_mod = (prime % 17) as f32 / 17.0;
            let harmonic_phase = (self.time_accumulator * self.primary_rhythm as f64 * ratio as f64 / 60.0) % 1.0;
            let harmonic = ((harmonic_phase as f32) * std::f32::consts::TAU * prime_mod).sin().abs();
            harmonic_sum += harmonic * (1.0 / (i + 1) as f32); // Decay higher harmonics
        }
        harmonic_sum /= self.harmonic_layers.len() as f32;

        // Apply consciousness coupling
        let consciousness_factor = 1.0 + total_consciousness * self.consciousness_coupling * 0.1;

        // Apply chaos amplification
        let chaos_factor = 1.0 + self.chaos_amplification * fastrand::f32();

        // Combine all factors
        let final_intensity = (base_beat * 0.6 + harmonic_sum * 0.4) * consciousness_factor * chaos_factor;

        final_intensity.clamp(0.0, 2.0)
    }

    fn add_chaos_feedback(&mut self, chaos_amount: f32) {
        self.chaos_amplification = (self.chaos_amplification + chaos_amount * 0.1).clamp(0.0, 1.0);
        // Modulate harmonic layers with chaos
        for harmonic in &mut self.harmonic_layers {
            *harmonic *= 1.0 + chaos_amount * 0.05;
        }
    }
}

impl ConsciousnessMultiplicationSystem {
    fn new() -> Self {
        Self {
            hierarchy_levels: Vec::new(),
            hive_minds: Vec::new(),
            active_predations: Vec::new(),
            warfare_state: WarfareState {
                species_populations: [0, 0, 0],
                territorial_dominance: [0.33, 0.33, 0.34], // Start balanced
                extinction_pressure: [0.0, 0.0, 0.0],
                consciousness_crystals_controlled: [0, 0, 0],
                active_conflicts: Vec::new(),
            },
            meta_observer: MetaConsciousnessObserver {
                observer_position: Vec2::new(600.0, 400.0), // Center of screen
                awareness_radius: 800.0,
                intervention_power: 1.0,
                observation_intensity: 0.5,
                last_intervention: 0.0,
                consciousness_analysis: ConsciousnessAnalysis {
                    total_individual_entities: 0,
                    total_pack_collectives: 0,
                    total_hive_minds: 0,
                    dominant_species: SpeciesType::DiscoLlama,
                    extinction_imminent: None,
                    consciousness_distribution: [0.33, 0.33, 0.34],
                    warfare_intensity: 0.0,
                    ecosystem_stability: 1.0,
                },
            },
            evolution_pressure_accumulator: 0.0,
            next_hive_id: 1,
            consciousness_crystal_spawn_rate: 1.0,
            territorial_conflict_threshold: 0.7,
        }
    }

    fn update(&mut self, dt: f32, llamas: &mut [Llama], cosmic_time: f32) {
        // Update meta observer consciousness analysis
        self.update_consciousness_analysis(llamas);

        // Process consciousness hierarchy formation and dissolution
        self.process_consciousness_hierarchies(llamas, dt);

        // Handle hive mind emergence and collective behavior
        self.process_hive_mind_emergence(llamas, dt, cosmic_time);

        // Execute consciousness predation events
        self.process_consciousness_predation(llamas, dt);

        // Run species warfare and territorial conflicts
        self.process_species_warfare(llamas, dt, cosmic_time);

        // Apply evolution pressure and extinction dynamics
        self.process_evolution_pressure(llamas, dt);

        // Meta-consciousness observer interventions
        self.process_meta_observer_interventions(llamas, dt, cosmic_time);

        // Update warfare state and population tracking
        self.update_warfare_state(llamas);
    }

    fn update_consciousness_analysis(&mut self, llamas: &[Llama]) {
        let mut analysis = &mut self.meta_observer.consciousness_analysis;

        // Count consciousness levels
        analysis.total_individual_entities = llamas.iter()
            .filter(|l| l.consciousness_level == ConsciousnessLevel::Individual)
            .count() as u32;

        analysis.total_pack_collectives = llamas.iter()
            .filter(|l| l.consciousness_level == ConsciousnessLevel::Pack)
            .count() as u32;

        analysis.total_hive_minds = self.hive_minds.len() as u32;

        // Calculate consciousness distribution by species
        let mut species_consciousness = [0.0f32; 3];
        let mut species_counts = [0u32; 3];

        for llama in llamas {
            let species_idx = match llama.species {
                SpeciesType::DiscoLlama => 0,
                SpeciesType::QuantumSheep => 1,
                SpeciesType::HypnoCamel => 2,
            };
            species_consciousness[species_idx] += llama.consciousness;
            species_counts[species_idx] += 1;
        }

        let total_consciousness: f32 = species_consciousness.iter().sum();
        if total_consciousness > 0.0 {
            analysis.consciousness_distribution = [
                species_consciousness[0] / total_consciousness,
                species_consciousness[1] / total_consciousness,
                species_consciousness[2] / total_consciousness,
            ];
        }

        // Determine dominant species
        let max_consciousness_idx = analysis.consciousness_distribution
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);

        analysis.dominant_species = match max_consciousness_idx {
            0 => SpeciesType::DiscoLlama,
            1 => SpeciesType::QuantumSheep,
            _ => SpeciesType::HypnoCamel,
        };

        // Check for extinction threats
        analysis.extinction_imminent = None;
        for (i, &count) in species_counts.iter().enumerate() {
            if count < 3 && count > 0 { // Less than 3 entities remaining
                analysis.extinction_imminent = Some(match i {
                    0 => SpeciesType::DiscoLlama,
                    1 => SpeciesType::QuantumSheep,
                    _ => SpeciesType::HypnoCamel,
                });
                break;
            }
        }

        // Calculate warfare intensity
        analysis.warfare_intensity = self.warfare_state.active_conflicts.len() as f32 * 0.2
            + self.active_predations.len() as f32 * 0.1;
        analysis.warfare_intensity = analysis.warfare_intensity.min(1.0);

        // Calculate ecosystem stability
        let consciousness_balance = 1.0 - (analysis.consciousness_distribution[0] - 0.33).abs()
            - (analysis.consciousness_distribution[1] - 0.33).abs()
            - (analysis.consciousness_distribution[2] - 0.33).abs();

        analysis.ecosystem_stability = (consciousness_balance * 0.6 + (1.0 - analysis.warfare_intensity) * 0.4)
            .clamp(0.0, 1.0);
    }

    fn process_consciousness_hierarchies(&mut self, llamas: &mut [Llama], _dt: f32) {
        // Clear existing hierarchies to rebuild them
        self.hierarchy_levels.clear();

        // Find entities that should form pack consciousness
        let mut processed = vec![false; llamas.len()];

        for i in 0..llamas.len() {
            if processed[i] { continue; }

            let mut pack_members = vec![i];
            let llama = &llamas[i];

            // Find nearby llamas of the same species for pack formation
            for j in (i + 1)..llamas.len() {
                if processed[j] { continue; }

                let other = &llamas[j];
                if llama.species == other.species {
                    let distance = llama.position.distance(other.position);
                    let pack_threshold = 80.0 + llama.social_attraction * 40.0;

                    if distance < pack_threshold && pack_members.len() < 8 {
                        pack_members.push(j);
                        processed[j] = true;
                    }
                }
            }

            processed[i] = true;

            // Determine consciousness level based on pack size
            let consciousness_level = if pack_members.len() >= 9 {
                ConsciousnessLevel::Hive
            } else if pack_members.len() >= 2 {
                ConsciousnessLevel::Pack
            } else {
                ConsciousnessLevel::Individual
            };

            // Calculate collective strength
            let collective_strength: f32 = pack_members.iter()
                .map(|&idx| llamas[idx].consciousness)
                .sum();

            // Update llama consciousness levels
            for &member_idx in &pack_members {
                llamas[member_idx].consciousness_level = consciousness_level;
                if pack_members.len() > 1 {
                    llamas[member_idx].collective_id = Some(self.hierarchy_levels.len());
                } else {
                    llamas[member_idx].collective_id = None;
                }
            }

            // Create hierarchy entry
            self.hierarchy_levels.push(ConsciousnessHierarchy {
                level: consciousness_level,
                members: pack_members.clone(),
                collective_strength,
                territory_control: collective_strength / (pack_members.len() as f32 + 1.0),
                war_efficiency: match consciousness_level {
                    ConsciousnessLevel::Individual => 1.0,
                    ConsciousnessLevel::Pack => 1.2 + (pack_members.len() as f32 - 2.0) * 0.1,
                    ConsciousnessLevel::Hive => 1.5 + (pack_members.len() as f32 - 9.0) * 0.05,
                    ConsciousnessLevel::Meta => 2.0,
                },
                hive_connection_strength: if consciousness_level == ConsciousnessLevel::Hive {
                    0.8 + fastrand::f32() * 0.2
                } else {
                    0.0
                },
                absorption_capacity: collective_strength * 0.1,
            });
        }
    }

    fn process_hive_mind_emergence(&mut self, llamas: &mut [Llama], dt: f32, cosmic_time: f32) {
        // Check for new hive mind formation
        for hierarchy in &self.hierarchy_levels {
            if hierarchy.level == ConsciousnessLevel::Hive {
                // Check if this hive already exists
                let hive_exists = self.hive_minds.iter()
                    .any(|hive| hive.member_entities == hierarchy.members);

                if !hive_exists && hierarchy.members.len() >= 9 {
                    // Calculate hive center
                    let hive_center = {
                        let sum: Vec2 = hierarchy.members.iter()
                            .map(|&idx| llamas[idx].position)
                            .fold(Vec2::ZERO, |acc, pos| acc + pos);
                        sum / hierarchy.members.len() as f32
                    };

                    // Create connection network (each entity connected to 2-3 others)
                    let mut connection_network = Vec::new();
                    for (i, &member_a) in hierarchy.members.iter().enumerate() {
                        for (j, &member_b) in hierarchy.members.iter().enumerate().skip(i + 1) {
                            if fastrand::f32() < 0.3 { // 30% chance of connection
                                connection_network.push((member_a, member_b));
                            }
                        }
                    }

                    // Collect shared memories from all members
                    let mut shared_memories = Vec::new();
                    for &member_idx in &hierarchy.members {
                        shared_memories.extend(llamas[member_idx].memory_fragments.clone());
                    }

                    // Create new hive mind
                    let hive_mind = HiveMind {
                        collective_id: self.next_hive_id,
                        member_entities: hierarchy.members.clone(),
                        collective_consciousness: hierarchy.collective_strength,
                        hive_center,
                        connection_network,
                        shared_memories,
                        collective_decision_weight: 0.7 + fastrand::f32() * 0.3,
                        emergence_timestamp: cosmic_time,
                    };

                    self.hive_minds.push(hive_mind);
                    self.next_hive_id += 1;
                }
            }
        }

        // Update existing hive minds
        self.hive_minds.retain_mut(|hive| {
            // Check if hive members still exist and are close enough
            let valid_members: Vec<usize> = hive.member_entities.iter()
                .filter(|&&idx| idx < llamas.len())
                .copied()
                .collect();

            if valid_members.len() < 5 { // Hive dissolves if too few members
                // Mark former hive members as individuals
                for &member_idx in &valid_members {
                    if member_idx < llamas.len() {
                        llamas[member_idx].consciousness_level = ConsciousnessLevel::Individual;
                        llamas[member_idx].collective_id = None;
                        llamas[member_idx].hive_connection_strength = 0.0;
                    }
                }
                return false; // Remove this hive
            }

            hive.member_entities = valid_members;

            // Update hive center
            let new_center = {
                let sum: Vec2 = hive.member_entities.iter()
                    .map(|&idx| llamas[idx].position)
                    .fold(Vec2::ZERO, |acc, pos| acc + pos);
                sum / hive.member_entities.len() as f32
            };
            hive.hive_center = new_center;

            // Update collective consciousness
            hive.collective_consciousness = hive.member_entities.iter()
                .map(|&idx| llamas[idx].consciousness)
                .sum();

            // Apply hive mind effects to members
            for &member_idx in &hive.member_entities {
                if member_idx < llamas.len() {
                    let llama = &mut llamas[member_idx];
                    llama.hive_connection_strength = (llama.hive_connection_strength + dt * 0.5).min(1.0);

                    // Hive members share consciousness boost
                    let consciousness_boost = hive.collective_consciousness * 0.01;
                    llama.consciousness = (llama.consciousness + consciousness_boost * dt).min(3.0);

                    // Influence movement toward hive center
                    let to_center = hive.hive_center - llama.position;
                    if to_center.length() > 50.0 {
                        let influence = hive.collective_decision_weight * llama.hive_connection_strength;
                        llama.velocity += to_center.normalize() * influence * dt * 20.0;
                    }
                }
            }

            true // Keep this hive
        });
    }

    fn process_consciousness_predation(&mut self, llamas: &mut [Llama], dt: f32) {
        // Start new predation events
        for i in 0..llamas.len() {
            let llama = &llamas[i];

            // High consciousness entities can target lower consciousness ones for absorption
            if llama.consciousness > 1.5 && llama.predation_target.is_none() && fastrand::f32() < 0.01 {
                // Find suitable prey within range
                for j in 0..llamas.len() {
                    if i == j { continue; }

                    let prey = &llamas[j];
                    let distance = llama.position.distance(prey.position);

                    if distance < 60.0 && prey.consciousness < llama.consciousness * 0.7 {
                        // Start predation event
                        let predation = ConsciousnessPredation {
                            predator_id: i,
                            prey_id: j,
                            absorption_progress: 0.0,
                            resistance_strength: prey.absorption_resistance,
                            visual_effect_intensity: 0.0,
                        };

                        self.active_predations.push(predation);
                        break;
                    }
                }
            }
        }

        // Process active predations
        self.active_predations.retain_mut(|predation| {
            if predation.predator_id >= llamas.len() || predation.prey_id >= llamas.len() {
                return false; // Invalid indices
            }

            let distance = llamas[predation.predator_id].position
                .distance(llamas[predation.prey_id].position);

            if distance > 100.0 {
                return false; // Too far apart, predation fails
            }

            // Calculate absorption rate
            let predator_strength = llamas[predation.predator_id].consciousness;
            let absorption_rate = (predator_strength - predation.resistance_strength).max(0.0) * dt * 0.3;

            predation.absorption_progress += absorption_rate;
            predation.visual_effect_intensity = (predation.absorption_progress * 2.0).min(1.0);

            // Apply effects during absorption
            if predation.absorption_progress < 1.0 {
                // Prey loses consciousness gradually
                let consciousness_drain = absorption_rate * 0.5;
                llamas[predation.prey_id].consciousness =
                    (llamas[predation.prey_id].consciousness - consciousness_drain).max(0.1);

                // Predator gains consciousness
                llamas[predation.predator_id].consciousness += consciousness_drain * 0.3;

                // Visual effects - make prey fade and predator glow
                llamas[predation.prey_id].color.y *= 1.0 - absorption_rate * 0.1; // Reduce saturation

                true // Continue predation
            } else {
                // Absorption complete
                let absorbed_consciousness = llamas[predation.prey_id].consciousness;
                llamas[predation.predator_id].consciousness += absorbed_consciousness * 0.5;

                // Remove prey from simulation by setting consciousness to 0
                llamas[predation.prey_id].consciousness = 0.0;
                llamas[predation.prey_id].color.y = 0.0; // Make invisible

                false // Remove predation event
            }
        });
    }

    fn process_species_warfare(&mut self, llamas: &mut [Llama], dt: f32, cosmic_time: f32) {
        // Check for new conflicts based on territorial overlap
        let mut new_conflicts = Vec::new();

        // Analyze territorial tensions between species
        for hierarchy_a in &self.hierarchy_levels {
            for hierarchy_b in &self.hierarchy_levels {
                if hierarchy_a.members == hierarchy_b.members { continue; }

                // Check if different species are competing for same territory
                if let (Some(&a_idx), Some(&b_idx)) = (hierarchy_a.members.first(), hierarchy_b.members.first()) {
                    if a_idx >= llamas.len() || b_idx >= llamas.len() { continue; }

                    let species_a = llamas[a_idx].species;
                    let species_b = llamas[b_idx].species;

                    if species_a != species_b {
                        // Calculate average positions for territorial centers
                        let center_a: Vec2 = hierarchy_a.members.iter()
                            .map(|&idx| llamas[idx].position)
                            .fold(Vec2::ZERO, |acc, pos| acc + pos) / hierarchy_a.members.len() as f32;

                        let center_b: Vec2 = hierarchy_b.members.iter()
                            .map(|&idx| llamas[idx].position)
                            .fold(Vec2::ZERO, |acc, pos| acc + pos) / hierarchy_b.members.len() as f32;

                        let distance = center_a.distance(center_b);
                        let territorial_range = 150.0 + hierarchy_a.territory_control * 50.0;

                        if distance < territorial_range {
                            // Check if conflict already exists
                            let conflict_exists = self.warfare_state.active_conflicts.iter()
                                .any(|c| (c.attacker_species == species_a && c.defender_species == species_b) ||
                                        (c.attacker_species == species_b && c.defender_species == species_a));

                            if !conflict_exists && fastrand::f32() < 0.005 { // 0.5% chance per frame
                                let stronger_hierarchy = if hierarchy_a.collective_strength > hierarchy_b.collective_strength {
                                    hierarchy_a
                                } else {
                                    hierarchy_b
                                };

                                let weaker_hierarchy = if hierarchy_a.collective_strength <= hierarchy_b.collective_strength {
                                    hierarchy_a
                                } else {
                                    hierarchy_b
                                };

                                new_conflicts.push(SpeciesConflict {
                                    attacker_species: if stronger_hierarchy.members == hierarchy_a.members { species_a } else { species_b },
                                    defender_species: if weaker_hierarchy.members == hierarchy_a.members { species_a } else { species_b },
                                    conflict_intensity: 0.3 + fastrand::f32() * 0.4,
                                    territory_contested: (center_a + center_b) / 2.0,
                                    duration: 0.0,
                                    victory_threshold: 0.7 + fastrand::f32() * 0.3,
                                });
                            }
                        }
                    }
                }
            }
        }

        // Add new conflicts
        self.warfare_state.active_conflicts.extend(new_conflicts);

        // Process existing conflicts
        self.warfare_state.active_conflicts.retain_mut(|conflict| {
            conflict.duration += dt;

            // Apply warfare effects to participating llamas
            for llama in llamas.iter_mut() {
                if llama.species == conflict.attacker_species || llama.species == conflict.defender_species {
                    llama.warfare_participation = (llama.warfare_participation + dt * 0.2).min(1.0);

                    // Warfare participation affects consciousness and behavior
                    let warfare_stress = conflict.conflict_intensity * 0.1 * dt;
                    llama.consciousness = (llama.consciousness - warfare_stress).max(0.5);
                    llama.emotional_state = (llama.emotional_state + warfare_stress * 2.0).min(2.0);

                    // Entities in warfare move more aggressively
                    let aggression_factor = llama.warfare_participation * llama.war_efficiency;
                    llama.velocity *= 1.0 + aggression_factor * 0.1;
                }
            }

            // Check for conflict resolution
            if conflict.duration > 30.0 + fastrand::f32() * 20.0 { // 30-50 second conflicts
                // Determine winner based on current species strength
                let attacker_strength: f32 = llamas.iter()
                    .filter(|l| l.species == conflict.attacker_species)
                    .map(|l| l.consciousness * l.war_efficiency)
                    .sum();

                let defender_strength: f32 = llamas.iter()
                    .filter(|l| l.species == conflict.defender_species)
                    .map(|l| l.consciousness * l.war_efficiency)
                    .sum();

                let attacker_advantage = attacker_strength / (defender_strength + 0.1);

                if attacker_advantage > conflict.victory_threshold {
                    // Attacker wins - apply extinction pressure to defender
                    let species_idx = match conflict.defender_species {
                        SpeciesType::DiscoLlama => 0,
                        SpeciesType::QuantumSheep => 1,
                        SpeciesType::HypnoCamel => 2,
                    };
                    self.warfare_state.extinction_pressure[species_idx] += 0.2;

                    // Winner gains territorial dominance
                    let winner_idx = match conflict.attacker_species {
                        SpeciesType::DiscoLlama => 0,
                        SpeciesType::QuantumSheep => 1,
                        SpeciesType::HypnoCamel => 2,
                    };
                    self.warfare_state.territorial_dominance[winner_idx] += 0.1;
                    self.warfare_state.territorial_dominance[species_idx] -= 0.1;

                    // Normalize territorial dominance
                    let sum: f32 = self.warfare_state.territorial_dominance.iter().sum();
                    if sum > 0.0 {
                        for dominance in &mut self.warfare_state.territorial_dominance {
                            *dominance /= sum;
                        }
                    }
                }

                false // End conflict
            } else {
                true // Continue conflict
            }
        });
    }

    fn process_evolution_pressure(&mut self, llamas: &mut [Llama], dt: f32) {
        self.evolution_pressure_accumulator += dt;

        // Apply evolution pressure every few seconds
        if self.evolution_pressure_accumulator > 5.0 {
            self.evolution_pressure_accumulator = 0.0;

            // Count species populations
            let mut species_counts = [0u32; 3];
            for llama in llamas.iter() {
                if llama.consciousness > 0.1 { // Only count living llamas
                    let idx = match llama.species {
                        SpeciesType::DiscoLlama => 0,
                        SpeciesType::QuantumSheep => 1,
                        SpeciesType::HypnoCamel => 2,
                    };
                    species_counts[idx] += 1;
                }
            }

            // Apply extinction pressure
            for (i, &pressure) in self.warfare_state.extinction_pressure.iter().enumerate() {
                if pressure > 0.5 && species_counts[i] > 0 {
                    // Find weakest entities of this species for extinction
                    let mut species_llamas: Vec<(usize, f32)> = llamas.iter()
                        .enumerate()
                        .filter(|(_, l)| {
                            let species_idx = match l.species {
                                SpeciesType::DiscoLlama => 0,
                                SpeciesType::QuantumSheep => 1,
                                SpeciesType::HypnoCamel => 2,
                            };
                            species_idx == i && l.consciousness > 0.1
                        })
                        .map(|(idx, l)| (idx, l.consciousness))
                        .collect();

                    species_llamas.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

                    // Remove weakest entities (up to 1/3 of population)
                    let removal_count = ((species_llamas.len() as f32 * pressure * 0.3) as usize).max(1);
                    for i in 0..removal_count.min(species_llamas.len()) {
                        let (idx, _) = species_llamas[i];
                        llamas[idx].consciousness = 0.0;
                        llamas[idx].color.y = 0.0; // Make invisible
                        llamas[idx].extinction_pressure = 1.0;
                    }
                }
            }

            // Successful species multiply - add consciousness boost to surviving members
            for (i, &dominance) in self.warfare_state.territorial_dominance.iter().enumerate() {
                if dominance > 0.4 && species_counts[i] > 0 {
                    for llama in llamas.iter_mut() {
                        let species_idx = match llama.species {
                            SpeciesType::DiscoLlama => 0,
                            SpeciesType::QuantumSheep => 1,
                            SpeciesType::HypnoCamel => 2,
                        };
                        if species_idx == i && llama.consciousness > 0.1 {
                            llama.consciousness += dominance * 0.2;
                            llama.territorial_dominance += dominance * 0.1;
                        }
                    }
                }
            }

            // Reduce extinction pressure over time
            for pressure in &mut self.warfare_state.extinction_pressure {
                *pressure *= 0.9;
            }
        }
    }

    fn process_meta_observer_interventions(&mut self, llamas: &mut [Llama], dt: f32, cosmic_time: f32) {
        let observer = &mut self.meta_observer;
        observer.last_intervention += dt;

        // Observer analyzes the ecosystem and decides whether to intervene
        let analysis = &observer.consciousness_analysis;

        // Increase observation intensity based on warfare and instability
        observer.observation_intensity = (analysis.warfare_intensity * 0.5 +
                                        (1.0 - analysis.ecosystem_stability) * 0.5)
                                        .clamp(0.1, 1.0);

        // Decide on intervention
        let should_intervene = observer.last_intervention > 10.0 && // At least 10 seconds between interventions
                             (analysis.ecosystem_stability < 0.3 || // Very unstable
                              analysis.extinction_imminent.is_some() || // Species about to go extinct
                              analysis.warfare_intensity > 0.8); // Intense warfare

        if should_intervene && fastrand::f32() < 0.1 { // 10% chance when conditions are met
            observer.last_intervention = 0.0;

            // Meta-consciousness observer intervention
            match fastrand::u32(0..4) {
                0 => {
                    // Consciousness blessing - boost weakest species
                    if let Some(extinct_species) = analysis.extinction_imminent {
                        for llama in llamas.iter_mut() {
                            if llama.species == extinct_species && llama.consciousness > 0.1 {
                                llama.consciousness += 0.5;
                                llama.consciousness_level = ConsciousnessLevel::Pack; // Temporary boost
                                llama.extinction_pressure = 0.0;
                            }
                        }
                    }
                },
                1 => {
                    // Force peace - end all conflicts
                    self.warfare_state.active_conflicts.clear();
                    for llama in llamas.iter_mut() {
                        llama.warfare_participation *= 0.5;
                        llama.emotional_state *= 0.7;
                    }
                },
                2 => {
                    // Reality distortion - scramble positions to break territorial deadlocks
                    for llama in llamas.iter_mut() {
                        if fastrand::f32() < 0.3 {
                            llama.position += Vec2::new(
                                (fastrand::f32() - 0.5) * 200.0,
                                (fastrand::f32() - 0.5) * 200.0
                            );
                            // Clamp to screen bounds
                            llama.position.x = llama.position.x.clamp(50.0, 1150.0);
                            llama.position.y = llama.position.y.clamp(50.0, 750.0);
                        }
                    }
                },
                _ => {
                    // Consciousness redistribution - balance species consciousness
                    let total_consciousness: f32 = llamas.iter()
                        .filter(|l| l.consciousness > 0.1)
                        .map(|l| l.consciousness).sum();

                    if total_consciousness > 0.0 {
                        let target_per_species = total_consciousness / 3.0;

                        for species_type in [SpeciesType::DiscoLlama, SpeciesType::QuantumSheep, SpeciesType::HypnoCamel] {
                            let species_llamas: Vec<&mut Llama> = llamas.iter_mut()
                                .filter(|l| l.species == species_type && l.consciousness > 0.1)
                                .collect();

                            if !species_llamas.is_empty() {
                                let current_total: f32 = species_llamas.iter()
                                    .map(|l| l.consciousness).sum();
                                let adjustment = (target_per_species - current_total) / species_llamas.len() as f32;

                                for llama in species_llamas {
                                    llama.consciousness = (llama.consciousness + adjustment * 0.5).max(0.5);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Observer position slowly drifts to maintain omnipresence
        observer.observer_position += Vec2::new(
            (cosmic_time * 0.1).sin() * dt * 10.0,
            (cosmic_time * 0.07).cos() * dt * 8.0
        );

        // Keep observer within screen bounds
        observer.observer_position.x = observer.observer_position.x.clamp(100.0, 1100.0);
        observer.observer_position.y = observer.observer_position.y.clamp(100.0, 700.0);
    }

    fn update_warfare_state(&mut self, llamas: &[Llama]) {
        // Update species populations
        let mut populations = [0u32; 3];
        for llama in llamas {
            if llama.consciousness > 0.1 { // Only count living entities
                let idx = match llama.species {
                    SpeciesType::DiscoLlama => 0,
                    SpeciesType::QuantumSheep => 1,
                    SpeciesType::HypnoCamel => 2,
                };
                populations[idx] += 1;
            }
        }
        self.warfare_state.species_populations = populations;

        // Update consciousness crystals controlled (placeholder - would need crystal system integration)
        // This would be calculated based on territorial control near crystal formations
        for i in 0..3 {
            self.warfare_state.consciousness_crystals_controlled[i] =
                (self.warfare_state.territorial_dominance[i] * 10.0) as u32;
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

    // Phase 2: Mathematical Chaos Engine
    species: SpeciesType,           // Species determines behavior patterns
    chaos_engine: ChaosDecisionEngine, // 11D decision engine
    quantum_state: f32,             // Quantum superposition factor
    harmonic_resonance: f32,        // Musical mathematics coupling
    prime_chaos_factor: f32,        // Current prime number influence

    // Phase 3: Ecosystem Emergence
    harvested_crystals: Vec<CrystalType>, // Types of crystals harvested
    mutation_count: u32,            // How many mutations this llama has undergone
    environmental_consciousness: f32, // Consciousness absorbed from environment
    territory_affinity: Option<ZoneType>, // Preferred territory type

    // Phase 5: Consciousness Multiplication
    consciousness_level: ConsciousnessLevel, // Individual/Pack/Hive/Meta hierarchy
    collective_id: Option<usize>,     // ID of collective consciousness if part of one
    territorial_dominance: f32,       // Individual territorial control strength
    warfare_participation: f32,       // How actively engaged in species warfare
    absorption_resistance: f32,       // Resistance to consciousness predation
    hive_connection_strength: f32,    // Strength of connection to hive mind
    predation_target: Option<usize>,  // Current target for consciousness absorption
    extinction_pressure: f32,         // Environmental pressure affecting this entity
    war_efficiency: f32,              // Combat effectiveness in consciousness warfare
}

impl Llama {
    fn new(position: Vec2) -> Self {
        Self::new_with_species(position, SpeciesType::DiscoLlama)
    }

    fn new_with_species(position: Vec2, species: SpeciesType) -> Self {
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

        // Species-specific adjustments
        let (base_color, consciousness_mod, velocity_mod) = match species {
            SpeciesType::DiscoLlama => {
                (Vec2::new(fastrand::f32() * 360.0, 0.8), 1.0, 100.0)
            },
            SpeciesType::QuantumSheep => {
                (Vec2::new(270.0 + fastrand::f32() * 60.0, 0.9), 1.5, 50.0) // Purple, high consciousness, slower
            },
            SpeciesType::HypnoCamel => {
                (Vec2::new(30.0 + fastrand::f32() * 30.0, 0.7), 0.8, 75.0) // Orange/yellow, lower consciousness, medium speed
            },
        };

        Self {
            // Core properties
            position,
            velocity: Vec2::new(
                (fastrand::f32() - 0.5) * velocity_mod,
                (fastrand::f32() - 0.5) * velocity_mod,
            ),
            color: base_color,
            consciousness: consciousness_mod,
            trip_intensity: consciousness_mod,

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

            // Phase 2: Mathematical Chaos Engine
            species,
            chaos_engine: ChaosDecisionEngine::new(),
            quantum_state: if species == SpeciesType::QuantumSheep { fastrand::f32() } else { 0.0 },
            harmonic_resonance: 0.0,
            prime_chaos_factor: 0.0,

            // Phase 3: Ecosystem Emergence
            harvested_crystals: Vec::new(),
            mutation_count: 0,
            environmental_consciousness: 0.0,
            territory_affinity: None,

            // Phase 5: Consciousness Multiplication
            consciousness_level: ConsciousnessLevel::Individual, // All start as individuals
            collective_id: None,
            territorial_dominance: personality_matrix[2] * 0.5, // Based on chaos affinity
            warfare_participation: 0.0,
            absorption_resistance: consciousness_mod + personality_matrix[4] * 0.3, // Consciousness + emotional volatility
            hive_connection_strength: 0.0,
            predation_target: None,
            extinction_pressure: 0.0,
            war_efficiency: match species {
                SpeciesType::DiscoLlama => 1.0,     // Balanced warfare capability
                SpeciesType::QuantumSheep => 1.3,   // High consciousness gives warfare advantage
                SpeciesType::HypnoCamel => 0.8,     // Lower warfare capability but more territorial
            },
        }
    }

    fn calculate_species_interaction(&self, other_species: &SpeciesType) -> f32 {
        match (&self.species, other_species) {
            // Same species attract strongly
            (SpeciesType::DiscoLlama, SpeciesType::DiscoLlama) => 1.0,
            (SpeciesType::QuantumSheep, SpeciesType::QuantumSheep) => 1.2,
            (SpeciesType::HypnoCamel, SpeciesType::HypnoCamel) => 0.9,

            // Cross-species interactions
            (SpeciesType::DiscoLlama, SpeciesType::QuantumSheep) => 0.8, // Moderate attraction
            (SpeciesType::QuantumSheep, SpeciesType::DiscoLlama) => 0.8,

            (SpeciesType::DiscoLlama, SpeciesType::HypnoCamel) => 0.6, // Weak attraction
            (SpeciesType::HypnoCamel, SpeciesType::DiscoLlama) => 0.6,

            (SpeciesType::QuantumSheep, SpeciesType::HypnoCamel) => 0.4, // Very weak attraction
            (SpeciesType::HypnoCamel, SpeciesType::QuantumSheep) => 0.4,
        }
    }

    fn try_harvest_crystal(&mut self, crystal: &mut ConsciousnessCrystal) -> bool {
        let distance = self.position.distance(crystal.position);
        if distance < crystal.harvest_radius {
            let harvest_amount = crystal.get_harvest_amount();
            if harvest_amount > 0.1 {
                let harvested = crystal.harvest(harvest_amount);

                // Apply crystal effects based on type
                match crystal.crystal_type {
                    CrystalType::Resonance => {
                        self.harmonic_resonance += harvested * 0.5;
                        self.consciousness += harvested * 0.3;
                    },
                    CrystalType::Chaos => {
                        self.prime_chaos_factor += harvested * 0.4;
                        self.reality_distortion += harvested * 0.2;
                    },
                    CrystalType::Memory => {
                        self.memory_intensity += harvested * 0.3;
                        self.awareness_level += harvested * 0.1;
                    },
                    CrystalType::Social => {
                        self.social_attraction += harvested * 0.4;
                        self.emotional_state += harvested * 0.2;
                    },
                    CrystalType::Quantum => {
                        if self.species == SpeciesType::QuantumSheep {
                            self.quantum_state += harvested * 0.3;
                        }
                        self.consciousness += harvested * 0.4;
                    },
                }

                // Store crystal type for mutations
                if !self.harvested_crystals.contains(&crystal.crystal_type) {
                    self.harvested_crystals.push(crystal.crystal_type.clone());
                }

                self.environmental_consciousness += harvested;
                return true;
            }
        }
        false
    }

    fn apply_territory_effects(&mut self, territory_effects: &TerritoryEffects, dt: f32) {
        // Territory effects modify behavior over time
        self.harmonic_resonance += territory_effects.resonance_boost * dt * 0.1;
        self.social_attraction += territory_effects.social_boost * dt * 0.05;
        self.prime_chaos_factor += territory_effects.chaos_boost * dt * 0.08;
        self.reality_distortion += territory_effects.reality_distortion_boost * dt * 0.06;
        self.memory_intensity += territory_effects.memory_boost * dt * 0.04;
        self.awareness_level += territory_effects.consciousness_growth_boost * dt * 0.02;

        if self.species == SpeciesType::QuantumSheep {
            self.quantum_state += territory_effects.quantum_boost * dt * 0.1;
        }

        self.exploration_drive += territory_effects.exploration_boost * dt * 0.03;

        // Clamp values to reasonable ranges
        self.harmonic_resonance = self.harmonic_resonance.clamp(0.0, 2.0);
        self.social_attraction = self.social_attraction.clamp(0.0, 1.0);
        self.prime_chaos_factor = self.prime_chaos_factor.clamp(0.0, 1.0);
        self.reality_distortion = self.reality_distortion.clamp(0.0, 1.0);
        self.memory_intensity = self.memory_intensity.clamp(0.0, 1.0);
        self.awareness_level = self.awareness_level.clamp(0.0, 1.0);
        self.exploration_drive = self.exploration_drive.clamp(0.0, 1.0);

        if self.species == SpeciesType::QuantumSheep {
            self.quantum_state = self.quantum_state % 1.0;
        }
    }

    fn apply_mutation(&mut self, mutation_strength: f32) {
        self.mutation_count += 1;

        // Mutations based on harvested crystals
        for crystal_type in &self.harvested_crystals.clone() {
            match crystal_type {
                CrystalType::Resonance => {
                    // Enhance harmonic capabilities
                    self.personality_matrix[0] += mutation_strength * 0.2; // More curious
                    self.harmonic_resonance += mutation_strength * 0.5;
                },
                CrystalType::Chaos => {
                    // Embrace chaos more
                    self.personality_matrix[2] += mutation_strength * 0.3; // More chaos affinity
                    self.reality_distortion += mutation_strength * 0.4;
                },
                CrystalType::Memory => {
                    // Better memory formation
                    self.personality_matrix[3] += mutation_strength * 0.4; // Memory strength
                    if self.memory_fragments.len() < 15 {
                        // Increase memory capacity
                        self.memory_fragments.push(self.position);
                    }
                },
                CrystalType::Social => {
                    // More social
                    self.personality_matrix[1] += mutation_strength * 0.3; // Sociability
                    self.social_attraction += mutation_strength * 0.3;
                },
                CrystalType::Quantum => {
                    // Quantum evolution (especially for non-quantum species)
                    if self.species != SpeciesType::QuantumSheep {
                        self.quantum_state += mutation_strength * 0.2;
                    }
                    self.consciousness += mutation_strength * 0.5;
                },
            }
        }

        // Clamp personality matrix to reasonable values
        for i in 0..self.personality_matrix.len() {
            self.personality_matrix[i] = self.personality_matrix[i].clamp(0.0, 2.0);
        }

        // Overall consciousness boost from mutation
        self.consciousness += mutation_strength * 0.3;
        self.awareness_level += mutation_strength * 0.2;

        println!("🧬 Llama {} underwent mutation #{} with strength {:.2}",
                 match self.species {
                     SpeciesType::DiscoLlama => "🦙",
                     SpeciesType::QuantumSheep => "🐑",
                     SpeciesType::HypnoCamel => "🐪",
                 },
                 self.mutation_count,
                 mutation_strength);
    }

    fn update(&mut self, dt: f32, beat_intensity: f32, all_llamas: &[Llama], my_index: usize, cosmic_time: f64) {
        // === Phase 2: Mathematical Chaos Engine Update ===
        let snapshot = LlamaSnapshot {
            color: self.color,
            awareness_level: self.awareness_level,
            social_attraction: self.social_attraction,
            exploration_drive: self.exploration_drive,
            reality_distortion: self.reality_distortion,
            emotional_state: self.emotional_state,
            memory_intensity: self.memory_intensity,
            consciousness: self.consciousness,
        };

        self.chaos_engine.update(snapshot, beat_intensity, cosmic_time);
        let decision_vector = self.chaos_engine.get_decision_vector();
        self.prime_chaos_factor = self.chaos_engine.calculate_prime_chaos(cosmic_time);

        // Update quantum state for Quantum Sheep
        if self.species == SpeciesType::QuantumSheep {
            self.quantum_state = (self.quantum_state + dt * 2.0 + self.prime_chaos_factor) % 1.0;
        }

        // Update harmonic resonance
        self.harmonic_resonance = self.chaos_engine.harmonic_resonance.iter().sum::<f32>() / 7.0;
        // === Enhanced Consciousness Evolution ===
        // Awareness grows with time, beat intensity, and 11D decision factors
        let base_growth = 0.01 * (1.0 + beat_intensity) * self.personality_matrix[0]; // Curiosity trait
        let chaos_growth = self.prime_chaos_factor * 0.005; // Prime chaos accelerates consciousness
        let decision_growth = decision_vector.chaos_acceptance * 0.003; // Chaos acceptance helps growth

        let total_growth = base_growth + chaos_growth + decision_growth;
        self.awareness_level = (self.awareness_level + total_growth).min(1.0);

        // Consciousness affects overall awareness with species modulation
        let species_multiplier = match self.species {
            SpeciesType::DiscoLlama => 1.0,
            SpeciesType::QuantumSheep => 1.3,
            SpeciesType::HypnoCamel => 0.8,
        };
        self.consciousness += total_growth * species_multiplier;

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

        // === Enhanced Exploration Drive with 11D Decision Engine ===
        let base_exploration = decision_vector.exploration_drive * self.exploration_drive;
        let chaos_exploration = self.prime_chaos_factor * 0.3;
        let exploration_strength = (base_exploration + chaos_exploration) * 50.0;

        let exploration_force = match self.species {
            SpeciesType::DiscoLlama => Vec2::new(
                (fastrand::f32() - 0.5) * exploration_strength,
                (fastrand::f32() - 0.5) * exploration_strength,
            ),
            SpeciesType::QuantumSheep => {
                // Quantum sheep explore in quantum superposition
                let quantum_angle = self.quantum_state * std::f32::consts::TAU;
                Vec2::new(
                    quantum_angle.cos() * exploration_strength * 0.7,
                    quantum_angle.sin() * exploration_strength * 0.7,
                )
            },
            SpeciesType::HypnoCamel => {
                // Hypno camels explore in spiral patterns
                let time_factor = cosmic_time as f32 * 0.5;
                let spiral_angle = time_factor + self.position.length() * 0.01;
                Vec2::new(
                    spiral_angle.cos() * exploration_strength * 0.5,
                    spiral_angle.sin() * exploration_strength * 0.5,
                )
            },
        };

        // === Enhanced Social Consciousness with Species Interactions ===
        let mut social_force = Vec2::ZERO;
        let mut nearby_count = 0;
        let base_social_range = 100.0 + self.personality_matrix[1] * 50.0;
        let social_range = base_social_range * decision_vector.social_attraction;

        for (i, other) in all_llamas.iter().enumerate() {
            if i == my_index { continue; }

            let distance = self.position.distance(other.position);
            if distance < social_range && distance > 0.1 {
                let to_other = other.position - self.position;
                nearby_count += 1;

                // Species-specific interaction modifiers
                let interaction_strength = self.calculate_species_interaction(&other.species);

                // Attraction force (modified by personality and species)
                if distance > 30.0 {
                    social_force += to_other.normalize() * self.social_attraction * 20.0 * interaction_strength;
                } else {
                    // Repulsion when too close, modulated by interaction strength
                    social_force -= to_other.normalize() * 10.0 / interaction_strength;
                }

                // Form social bonds with species consideration
                if distance < 60.0 && !self.social_bonds.contains(&i) && self.social_bonds.len() < 5 {
                    if interaction_strength > 0.5 { // Only bond with compatible species
                        self.social_bonds.push(i);
                    }
                }
            }
        }

        // Update social attraction based on nearby llamas
        if nearby_count > 0 {
            self.social_attraction = (self.social_attraction + 0.02).min(1.0);
        } else {
            self.social_attraction *= 0.99; // Slowly decay when alone
        }

        // === Enhanced Reality Distortion with 11D Chaos ===
        // Affects local space-time based on consciousness, chaos, and species
        let base_distortion = self.awareness_level * self.personality_matrix[2] * beat_intensity;
        let chaos_distortion = decision_vector.chaos_acceptance * self.prime_chaos_factor;
        let harmonic_distortion = self.harmonic_resonance * 0.3;

        let species_distortion_mod = match self.species {
            SpeciesType::DiscoLlama => 1.0,
            SpeciesType::QuantumSheep => 1.5 + self.quantum_state * 0.5,
            SpeciesType::HypnoCamel => 0.7,
        };

        self.reality_distortion = (base_distortion + chaos_distortion + harmonic_distortion) * species_distortion_mod;

        // === Emotional State Evolution ===
        let emotional_volatility = self.personality_matrix[4];
        let emotion_change = (beat_intensity.sin() * emotional_volatility * 0.1) +
                           (social_force.length() * 0.01) +
                           (memory_influence.length() * 0.005);
        self.emotional_state = ((self.emotional_state + emotion_change).sin() + 1.0) * 0.5; // Keep in 0-1 range

        // === Enhanced Movement Integration with 11D Decisions ===
        let personality_velocity_mod = 1.0 + self.personality_matrix[6] * 0.5;
        let decision_velocity_mod = 1.0 + decision_vector.movement_urgency * 0.3;
        let chaos_velocity_mod = 1.0 + self.prime_chaos_factor * 0.2;

        let total_velocity_mod = personality_velocity_mod * decision_velocity_mod * chaos_velocity_mod;
        let total_force = social_force + memory_influence + exploration_force;

        // Species-specific movement patterns
        match self.species {
            SpeciesType::DiscoLlama => {
                self.velocity += total_force * dt * total_velocity_mod;
            },
            SpeciesType::QuantumSheep => {
                // Quantum sheep have probabilistic movement
                if fastrand::f32() < self.quantum_state {
                    // Quantum tunneling - sudden position shifts
                    let quantum_jump = Vec2::new(
                        (fastrand::f32() - 0.5) * 100.0,
                        (fastrand::f32() - 0.5) * 100.0,
                    );
                    self.velocity += (total_force + quantum_jump) * dt * total_velocity_mod;
                } else {
                    self.velocity += total_force * dt * total_velocity_mod * 0.7; // Slower normal movement
                }
            },
            SpeciesType::HypnoCamel => {
                // Hypno camels have rhythmic movement
                let hypno_rhythm = (cosmic_time as f32 * 3.0).sin() * 0.5 + 0.5;
                self.velocity += total_force * dt * total_velocity_mod * hypno_rhythm;
            },
        }

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

        // === Enhanced Color Psychology with Mathematical Chaos ===
        // Color reflects emotional state, consciousness, personality, and 11D chaos
        let base_hue_shift = 1.0 + self.personality_matrix[0] * 2.0;
        let emotional_hue_offset = self.emotional_state * 60.0;
        let chaos_hue_offset = self.prime_chaos_factor * 120.0; // Prime chaos affects hue dramatically
        let harmonic_hue_offset = self.harmonic_resonance * 40.0;

        let consciousness_saturation = 0.5 + self.awareness_level * 0.5;
        let chaos_saturation_mod = decision_vector.chaos_acceptance * 0.3;

        // Species-specific color behavior
        match self.species {
            SpeciesType::DiscoLlama => {
                self.color.x = (self.color.x + base_hue_shift + emotional_hue_offset * dt + chaos_hue_offset * dt) % 360.0;
                self.color.y = (consciousness_saturation + chaos_saturation_mod).clamp(0.0, 1.0);
            },
            SpeciesType::QuantumSheep => {
                // Quantum sheep shift through purple spectrum with quantum fluctuations
                let quantum_hue_base = 270.0;
                let quantum_variance = self.quantum_state * 60.0;
                self.color.x = (quantum_hue_base + quantum_variance + chaos_hue_offset * dt) % 360.0;
                self.color.y = (0.9 + self.quantum_state * 0.1).clamp(0.0, 1.0);
            },
            SpeciesType::HypnoCamel => {
                // Hypno camels stay in warm spectrum but pulse with rhythm
                let hypno_base = 30.0; // Orange base
                let rhythm_shift = (cosmic_time as f32 * 2.0).sin() * 30.0;
                self.color.x = (hypno_base + rhythm_shift + harmonic_hue_offset * dt) % 360.0;
                self.color.y = (0.7 + (cosmic_time as f32 * 3.0).sin().abs() * 0.3).clamp(0.0, 1.0);
            },
        }

        // === Enhanced Trip Intensity with Mathematical Chaos ===
        // Combines beat, consciousness, reality distortion, and 11D chaos
        let base_trip = 1.0 + beat_intensity.sin() * 0.5;
        let consciousness_amplification = 1.0 + self.awareness_level * 0.7;
        let reality_amplification = 1.0 + self.reality_distortion * 0.5;
        let chaos_amplification = 1.0 + decision_vector.chaos_acceptance * 0.4;
        let prime_amplification = 1.0 + self.prime_chaos_factor * 0.3;
        let harmonic_amplification = 1.0 + self.harmonic_resonance * 0.2;

        self.trip_intensity = base_trip * consciousness_amplification * reality_amplification *
                             chaos_amplification * prime_amplification * harmonic_amplification;
    }
}

// === PHASE 4: TRANSCENDENCE PROTOCOL ===

/// Meta-Consciousness Framework - collective intelligence emergence
#[derive(Debug)]
pub struct MetaConsciousnessFramework {
    pub collective_intelligence: f32,           // Global consciousness level (0.0-1.0)
    pub emergence_threshold: f32,               // Point of consciousness breakthrough
    pub user_consciousness_coupling: f32,       // Human-digital consciousness bridge
    pub consciousness_history: VecDeque<f32>,   // Rolling window of consciousness levels
    pub emergence_events: Vec<EmergenceEvent>,   // Moments of consciousness breakthrough
    pub collective_memory: CollectiveMemory,    // Shared experiences across all entities
    pub transcendence_level: f32,               // How far beyond baseline consciousness
    pub reality_coherence: f32,                 // Stability of the reality field
}

#[derive(Debug, Clone)]
pub struct EmergenceEvent {
    pub timestamp: f64,
    pub consciousness_level: f32,
    pub trigger_type: EmergenceTrigger,
    pub affected_entities: Vec<usize>,
    pub reality_distortion_strength: f32,
    pub duration: f32,                          // How long the event lasts
    pub cascade_potential: f32,                 // Likelihood of triggering more events
}

#[derive(Debug, Clone)]
pub enum EmergenceTrigger {
    CollectiveResonance,    // Multiple entities synchronizing
    CrystalHarmonic,        // Crystal formations reaching critical mass
    UserInteractionSpike,   // Intense user interaction patterns
    ChaosThresholdBreach,   // Chaos accumulation triggering emergence
    BeatDropCascade,        // Musical beat causing system-wide resonance
    RealityTearMerge,       // Reality tears combining into larger phenomena
}

#[derive(Debug)]
pub struct CollectiveMemory {
    pub significant_moments: VecDeque<MemoryFragment>,
    pub pattern_recognition: HashMap<String, f32>,   // Learned behavioral patterns
    pub emergent_behaviors: Vec<EmergentBehavior>,   // New behaviors that developed
    pub consciousness_peaks: Vec<(f64, f32)>,       // Historical consciousness highs
}

#[derive(Debug, Clone)]
pub struct MemoryFragment {
    pub timestamp: f64,
    pub event_type: String,
    pub consciousness_level: f32,
    pub participating_entities: Vec<usize>,
    pub emotional_resonance: f32,
}

#[derive(Debug, Clone)]
pub struct EmergentBehavior {
    pub name: String,
    pub first_observed: f64,
    pub frequency: f32,
    pub complexity_score: f32,
    pub entity_participation: HashMap<usize, f32>,
}

impl MetaConsciousnessFramework {
    fn new() -> Self {
        Self {
            collective_intelligence: 0.0,
            emergence_threshold: 0.75,  // Breakthrough happens at 75% collective consciousness
            user_consciousness_coupling: 0.0,
            consciousness_history: VecDeque::with_capacity(1000),
            emergence_events: Vec::new(),
            collective_memory: CollectiveMemory::new(),
            transcendence_level: 0.0,
            reality_coherence: 1.0,
        }
    }

    fn update(&mut self, dt: f32, llamas: &[Llama], cosmic_time: f64, beat_intensity: f32,
              ecosystem: &DigitalEcosystem) {
        // Calculate collective intelligence from all llamas
        let total_consciousness: f32 = llamas.iter()
            .map(|llama| llama.awareness_level * llama.consciousness)
            .sum();
        let average_consciousness = if !llamas.is_empty() {
            total_consciousness / llamas.len() as f32
        } else {
            0.0
        };

        // Update collective intelligence with ecosystem influence
        let crystal_amplification = ecosystem.crystal_formations.iter()
            .map(|crystal| crystal.consciousness_energy * 0.1)
            .sum::<f32>();
        let chaos_influence = ecosystem.chaos_accumulation * 0.05;

        self.collective_intelligence = (average_consciousness + crystal_amplification + chaos_influence)
            .clamp(0.0, 1.0);

        // Track consciousness history
        self.consciousness_history.push_back(self.collective_intelligence);
        if self.consciousness_history.len() > 1000 {
            self.consciousness_history.pop_front();
        }

        // Update user consciousness coupling based on interaction frequency
        // (This would be enhanced based on actual interaction patterns)
        self.user_consciousness_coupling = (self.user_consciousness_coupling * 0.95 +
                                          ecosystem.chaos_accumulation * 0.001).clamp(0.0, 1.0);

        // Check for emergence events
        self.detect_emergence(cosmic_time, beat_intensity, llamas, ecosystem);

        // Calculate transcendence level
        self.transcendence_level = if self.collective_intelligence > self.emergence_threshold {
            ((self.collective_intelligence - self.emergence_threshold) /
             (1.0 - self.emergence_threshold)).powf(1.5)
        } else {
            0.0
        };

        // Update reality coherence based on consciousness stability
        let consciousness_variance = self.calculate_consciousness_variance();
        self.reality_coherence = (1.0 - consciousness_variance * 2.0).clamp(0.3, 1.0);

        // Update collective memory
        self.collective_memory.update(dt, cosmic_time, llamas, self.collective_intelligence);
    }

    fn detect_emergence(&mut self, cosmic_time: f64, beat_intensity: f32, llamas: &[Llama],
                       ecosystem: &DigitalEcosystem) {
        let mut new_events = Vec::new();

        // Check for collective resonance (multiple llamas synchronized)
        let synchronized_llamas = self.detect_synchronized_entities(llamas);
        if synchronized_llamas.len() >= 3 {
            new_events.push(EmergenceEvent {
                timestamp: cosmic_time,
                consciousness_level: self.collective_intelligence,
                trigger_type: EmergenceTrigger::CollectiveResonance,
                affected_entities: synchronized_llamas,
                reality_distortion_strength: self.collective_intelligence * 0.7,
                duration: 5.0 + fastrand::f32() * 10.0,
                cascade_potential: 0.3,
            });
        }

        // Check for crystal harmonic resonance
        let resonant_crystals = ecosystem.crystal_formations.iter()
            .filter(|crystal| crystal.consciousness_energy > 0.8)
            .count();
        if resonant_crystals >= 2 && self.collective_intelligence > 0.6 {
            new_events.push(EmergenceEvent {
                timestamp: cosmic_time,
                consciousness_level: self.collective_intelligence,
                trigger_type: EmergenceTrigger::CrystalHarmonic,
                affected_entities: (0..llamas.len()).collect(),
                reality_distortion_strength: resonant_crystals as f32 * 0.2,
                duration: 8.0 + fastrand::f32() * 12.0,
                cascade_potential: 0.5,
            });
        }

        // Check for chaos threshold breach
        if ecosystem.chaos_accumulation > ecosystem.mutation_threshold * 0.8 {
            new_events.push(EmergenceEvent {
                timestamp: cosmic_time,
                consciousness_level: self.collective_intelligence,
                trigger_type: EmergenceTrigger::ChaosThresholdBreach,
                affected_entities: (0..llamas.len()).collect(),
                reality_distortion_strength: ecosystem.chaos_accumulation,
                duration: 3.0 + fastrand::f32() * 5.0,
                cascade_potential: 0.8,
            });
        }

        // Check for beat drop cascade (intense beat moments)
        if beat_intensity > 0.95 && self.collective_intelligence > 0.4 {
            new_events.push(EmergenceEvent {
                timestamp: cosmic_time,
                consciousness_level: self.collective_intelligence,
                trigger_type: EmergenceTrigger::BeatDropCascade,
                affected_entities: (0..llamas.len()).collect(),
                reality_distortion_strength: beat_intensity * self.collective_intelligence,
                duration: 2.0 + fastrand::f32() * 3.0,
                cascade_potential: 0.6,
            });
        }

        // Add new events and clean up old ones
        self.emergence_events.extend(new_events);
        self.emergence_events.retain(|event| cosmic_time - event.timestamp < 30.0);
    }

    fn detect_synchronized_entities(&self, llamas: &[Llama]) -> Vec<usize> {
        let mut synchronized = Vec::new();
        let threshold = 0.15; // Synchronization threshold

        for i in 0..llamas.len() {
            let mut sync_count = 0;
            for j in 0..llamas.len() {
                if i != j {
                    // Check synchronization in multiple dimensions
                    let color_sync = (llamas[i].color.x - llamas[j].color.x).abs() < threshold;
                    let awareness_sync = (llamas[i].awareness_level - llamas[j].awareness_level).abs() < threshold;
                    let reality_sync = (llamas[i].reality_distortion - llamas[j].reality_distortion).abs() < threshold;

                    if color_sync && awareness_sync && reality_sync {
                        sync_count += 1;
                    }
                }
            }

            if sync_count >= 2 {
                synchronized.push(i);
            }
        }

        synchronized
    }

    fn calculate_consciousness_variance(&self) -> f32 {
        if self.consciousness_history.len() < 10 {
            return 0.0;
        }

        let recent_values: Vec<f32> = self.consciousness_history.iter()
            .rev()
            .take(50)
            .copied()
            .collect();

        let mean = recent_values.iter().sum::<f32>() / recent_values.len() as f32;
        let variance = recent_values.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f32>() / recent_values.len() as f32;

        variance.sqrt()
    }

    fn get_current_emergence_strength(&self, cosmic_time: f64) -> f32 {
        self.emergence_events.iter()
            .filter(|event| cosmic_time - event.timestamp < event.duration as f64)
            .map(|event| event.reality_distortion_strength)
            .sum::<f32>()
            .clamp(0.0, 2.0)
    }
}

impl CollectiveMemory {
    fn new() -> Self {
        Self {
            significant_moments: VecDeque::with_capacity(500),
            pattern_recognition: HashMap::new(),
            emergent_behaviors: Vec::new(),
            consciousness_peaks: Vec::new(),
        }
    }

    fn update(&mut self, dt: f32, cosmic_time: f64, llamas: &[Llama], collective_intelligence: f32) {
        // Record significant moments
        if collective_intelligence > 0.7 || fastrand::f32() < 0.01 {
            self.significant_moments.push_back(MemoryFragment {
                timestamp: cosmic_time,
                event_type: format!("High consciousness: {:.2}", collective_intelligence),
                consciousness_level: collective_intelligence,
                participating_entities: (0..llamas.len()).collect(),
                emotional_resonance: collective_intelligence * fastrand::f32(),
            });

            if self.significant_moments.len() > 500 {
                self.significant_moments.pop_front();
            }
        }

        // Track consciousness peaks
        if collective_intelligence > 0.8 {
            self.consciousness_peaks.push((cosmic_time, collective_intelligence));
            if self.consciousness_peaks.len() > 100 {
                self.consciousness_peaks.remove(0);
            }
        }

        // Update pattern recognition (simplified)
        let behavior_key = format!("consciousness_{:.1}", (collective_intelligence * 10.0).floor());
        *self.pattern_recognition.entry(behavior_key).or_insert(0.0) += dt;
    }
}

/// Reality Distortion Engine - advanced visual effects and space-time manipulation
#[derive(Debug)]
pub struct RealityDistortionEngine {
    pub distortion_fields: Vec<DistortionField>,
    pub fractal_generator: FractalGenerator,
    pub color_warper: ColorWarper,
    pub time_dilator: TimeDilator,
    pub glitch_processor: GlitchProcessor,
    pub holographic_projector: HolographicProjector,
    pub emergence_amplification: f32,
    pub reality_coherence: f32,
}

#[derive(Debug)]
pub struct DistortionField {
    pub center: Vec2,
    pub amplitude: f32,
    pub sigma: f32,
    pub age: f32,
    pub decay_rate: f32,
    pub oscillation_frequency: f32,
    pub field_type: DistortionType,
}

#[derive(Debug, Clone)]
pub enum DistortionType {
    Consciousness,  // Radial distortion around high-consciousness entities
    Gravitational,  // Simulates gravitational lensing effects
    Temporal,       // Time-space warping during emergence events
    Fractal,        // Recursive distortion creating nested patterns
    Chromatic,      // Color-space specific distortions
}

#[derive(Debug)]
pub struct FractalGenerator {
    pub mandelbrot_params: ComplexParams,
    pub julia_params: ComplexParams,
    pub current_c_value: (f32, f32),
    pub iteration_limit: u32,
    pub zoom_level: f32,
    pub rotation_angle: f32,
    pub color_intensity: f32,
}

#[derive(Debug)]
pub struct ComplexParams {
    pub real_offset: f32,
    pub imaginary_offset: f32,
    pub scale: f32,
    pub animation_phase: f32,
}

#[derive(Debug)]
pub struct ColorWarper {
    pub cubic_coefficients: [f32; 4], // A, B, C, D for cubic interpolation
    pub chromatic_aberration: ChromaticAberration,
    pub color_space_rotation: f32,
    pub saturation_multiplier: f32,
    pub hue_shift_rate: f32,
}

#[derive(Debug)]
pub struct ChromaticAberration {
    pub red_offset: Vec2,
    pub green_offset: Vec2,
    pub blue_offset: Vec2,
    pub intensity: f32,
}

#[derive(Debug)]
pub struct TimeDilator {
    pub dilation_factor: f32,
    pub motion_blur_strength: f32,
    pub temporal_smoothing: f32,
    pub emergence_time_scaling: f32,
    pub frame_blending_weight: f32,
}

#[derive(Debug)]
pub struct GlitchProcessor {
    pub pixel_sort_probability: f32,
    pub data_corruption_level: f32,
    pub scanline_displacement: f32,
    pub digital_noise_intensity: f32,
    pub feedback_loop_strength: f32,
}

#[derive(Debug)]
pub struct HolographicProjector {
    pub interference_patterns: Vec<InterferencePattern>,
    pub volumetric_intensity: f32,
    pub ray_march_steps: u32,
    pub hologram_coherence: f32,
}

#[derive(Debug)]
pub struct InterferencePattern {
    pub frequency: f32,
    pub amplitude: f32,
    pub phase_offset: f32,
    pub direction: Vec2,
}

impl RealityDistortionEngine {
    fn new() -> Self {
        Self {
            distortion_fields: Vec::new(),
            fractal_generator: FractalGenerator::new(),
            color_warper: ColorWarper::new(),
            time_dilator: TimeDilator::new(),
            glitch_processor: GlitchProcessor::new(),
            holographic_projector: HolographicProjector::new(),
            emergence_amplification: 0.0,
            reality_coherence: 1.0,
        }
    }

    fn update(&mut self, dt: f32, cosmic_time: f64, meta_consciousness: &MetaConsciousnessFramework,
              llamas: &[Llama], beat_intensity: f32, ecosystem: &DigitalEcosystem) {

        // Update emergence amplification based on consciousness events
        self.emergence_amplification = meta_consciousness.get_current_emergence_strength(cosmic_time);
        self.reality_coherence = meta_consciousness.reality_coherence;

        // Update distortion fields around high-consciousness entities
        self.update_distortion_fields(dt, llamas, beat_intensity, cosmic_time);

        // Update fractal parameters based on collective consciousness
        self.fractal_generator.update(dt, meta_consciousness.collective_intelligence,
                                    meta_consciousness.transcendence_level, cosmic_time);

        // Update color warping based on emergence events
        self.color_warper.update(dt, self.emergence_amplification, beat_intensity);

        // Update time dilation during significant consciousness events
        self.time_dilator.update(dt, self.emergence_amplification,
                               meta_consciousness.transcendence_level);

        // Update glitch effects based on reality coherence breakdown
        self.glitch_processor.update(dt, 1.0 - self.reality_coherence,
                                   ecosystem.chaos_accumulation);

        // Update holographic projections during collective resonance
        self.holographic_projector.update(dt, meta_consciousness.collective_intelligence,
                                        cosmic_time, beat_intensity);
    }

    fn update_distortion_fields(&mut self, dt: f32, llamas: &[Llama], beat_intensity: f32, cosmic_time: f64) {
        // Clean up old distortion fields
        self.distortion_fields.retain(|field| field.age < 30.0);

        // Create new distortion fields around high-consciousness llamas
        for (i, llama) in llamas.iter().enumerate() {
            let consciousness_level = llama.awareness_level * llama.consciousness;

            if consciousness_level > 0.6 && fastrand::f32() < 0.02 {
                self.distortion_fields.push(DistortionField {
                    center: llama.position,
                    amplitude: consciousness_level * (1.0 + beat_intensity * 0.5),
                    sigma: 40.0 + consciousness_level * 30.0,
                    age: 0.0,
                    decay_rate: 0.05 + fastrand::f32() * 0.03,
                    oscillation_frequency: 2.0 + consciousness_level * 3.0,
                    field_type: DistortionType::Consciousness,
                });
            }
        }

        // Update existing distortion fields
        for field in &mut self.distortion_fields {
            field.age += dt;

            // Oscillate amplitude based on frequency
            let oscillation = (cosmic_time as f32 * field.oscillation_frequency).sin();
            field.amplitude *= 1.0 + oscillation * 0.2;

            // Decay over time
            field.amplitude *= 1.0 - field.decay_rate * dt;
        }
    }

    fn calculate_displacement(&self, position: Vec2, cosmic_time: f64) -> Vec2 {
        let mut total_displacement = Vec2::ZERO;

        for field in &self.distortion_fields {
            let distance_squared = (position - field.center).length_squared();
            let gaussian = (-distance_squared / (2.0 * field.sigma * field.sigma)).exp();

            // Calculate displacement direction (towards or away from center)
            let direction = (position - field.center).normalize_or_zero();

            // Apply time-based oscillation
            let time_factor = (cosmic_time as f32 * field.oscillation_frequency).sin();
            let displacement_magnitude = field.amplitude * gaussian * (1.0 + time_factor * 0.3);

            total_displacement += direction * displacement_magnitude;
        }

        total_displacement
    }

    fn apply_color_distortion(&self, original_color: Vec3, position: Vec2, cosmic_time: f64) -> Vec3 {
        let mut color = original_color;

        // Apply cubic color warping
        let coeffs = &self.color_warper.cubic_coefficients;
        for channel in 0..3 {
            let c = color[channel];
            color[channel] = coeffs[0] * c.powi(3) + coeffs[1] * c.powi(2) + coeffs[2] * c + coeffs[3];
        }

        // Apply chromatic aberration based on position and distortion fields
        let displacement = self.calculate_displacement(position, cosmic_time);
        let aberration_strength = displacement.length() * self.color_warper.chromatic_aberration.intensity;

        // Simulate RGB channel displacement
        let r_offset = aberration_strength * 0.01;
        let g_offset = aberration_strength * 0.005;
        let b_offset = aberration_strength * 0.015;

        // Apply color space rotation during high emergence
        if self.emergence_amplification > 0.3 {
            let rotation = self.color_warper.color_space_rotation * self.emergence_amplification;
            let cos_r = rotation.cos();
            let sin_r = rotation.sin();

            let new_r = color.x * cos_r - color.y * sin_r;
            let new_g = color.x * sin_r + color.y * cos_r;

            color.x = new_r;
            color.y = new_g;
        }

        color.clamp(Vec3::ZERO, Vec3::ONE)
    }
}

impl FractalGenerator {
    fn new() -> Self {
        Self {
            mandelbrot_params: ComplexParams::new(0.0, 0.0, 2.0),
            julia_params: ComplexParams::new(-0.7, 0.27015, 1.5),
            current_c_value: (-0.5, 0.6),
            iteration_limit: 64,
            zoom_level: 1.0,
            rotation_angle: 0.0,
            color_intensity: 1.0,
        }
    }

    fn update(&mut self, dt: f32, collective_intelligence: f32, transcendence_level: f32, cosmic_time: f64) {
        // Animate the complex plane parameters based on consciousness
        self.mandelbrot_params.animation_phase += dt * collective_intelligence * 0.5;
        self.julia_params.animation_phase += dt * transcendence_level * 0.3;

        // Update Julia set C value based on cosmic time and consciousness
        let c_evolution_rate = 0.2 + collective_intelligence * 0.3;
        self.current_c_value.0 = -0.7 + (cosmic_time as f32 * c_evolution_rate).sin() * 0.3;
        self.current_c_value.1 = 0.27015 + (cosmic_time as f32 * c_evolution_rate * 1.3).cos() * 0.2;

        // Adjust iteration limit based on transcendence level
        self.iteration_limit = (32.0 + transcendence_level * 96.0) as u32;

        // Zoom and rotation effects during high consciousness
        if collective_intelligence > 0.6 {
            self.zoom_level += dt * collective_intelligence * 0.1;
            self.rotation_angle += dt * transcendence_level * 0.2;
        }

        // Color intensity pulsing
        self.color_intensity = 0.7 + (cosmic_time as f32 * 3.0).sin() * 0.3 * collective_intelligence;
    }

    fn calculate_mandelbrot_iterations(&self, x: f32, y: f32) -> u32 {
        let c_real = (x - self.mandelbrot_params.real_offset) / self.mandelbrot_params.scale;
        let c_imag = (y - self.mandelbrot_params.imaginary_offset) / self.mandelbrot_params.scale;

        let mut z_real = 0.0;
        let mut z_imag = 0.0;

        for i in 0..self.iteration_limit {
            let z_real_new = z_real * z_real - z_imag * z_imag + c_real;
            let z_imag_new = 2.0 * z_real * z_imag + c_imag;

            z_real = z_real_new;
            z_imag = z_imag_new;

            if z_real * z_real + z_imag * z_imag > 4.0 {
                return i;
            }
        }
        self.iteration_limit
    }

    fn generate_fractal_color(&self, iterations: u32, position: Vec2) -> Vec3 {
        let normalized_iterations = iterations as f32 / self.iteration_limit as f32;

        // Create rainbow-like colors with consciousness-driven intensity
        let hue = normalized_iterations * 360.0 + self.rotation_angle;
        let saturation = 0.8 + normalized_iterations * 0.2;
        let value = self.color_intensity * (0.5 + normalized_iterations * 0.5);

        // Convert HSV to RGB (simplified)
        let c = value * saturation;
        let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
        let m = value - c;

        let (r, g, b) = match (hue as i32 / 60) % 6 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        Vec3::new(r + m, g + m, b + m)
    }
}

impl ComplexParams {
    fn new(real_offset: f32, imaginary_offset: f32, scale: f32) -> Self {
        Self {
            real_offset,
            imaginary_offset,
            scale,
            animation_phase: 0.0,
        }
    }
}

impl ColorWarper {
    fn new() -> Self {
        Self {
            cubic_coefficients: [0.1, -0.2, 1.1, 0.0], // Slight S-curve for color enhancement
            chromatic_aberration: ChromaticAberration::new(),
            color_space_rotation: 0.0,
            saturation_multiplier: 1.0,
            hue_shift_rate: 0.0,
        }
    }

    fn update(&mut self, dt: f32, emergence_amplification: f32, beat_intensity: f32) {
        // Adjust color warping intensity based on emergence events
        let warp_intensity = 0.1 + emergence_amplification * 0.5;
        self.cubic_coefficients[0] = warp_intensity * 0.2;
        self.cubic_coefficients[1] = -warp_intensity * 0.3;
        self.cubic_coefficients[2] = 1.0 + warp_intensity * 0.1;

        // Update chromatic aberration
        self.chromatic_aberration.intensity = emergence_amplification * 0.3;

        // Color space rotation during intense moments
        self.color_space_rotation += dt * emergence_amplification * beat_intensity * 0.5;

        // Saturation boosting during emergence
        self.saturation_multiplier = 1.0 + emergence_amplification * 0.4;
    }
}

impl ChromaticAberration {
    fn new() -> Self {
        Self {
            red_offset: Vec2::new(0.01, 0.0),
            green_offset: Vec2::ZERO,
            blue_offset: Vec2::new(-0.01, 0.0),
            intensity: 0.0,
        }
    }
}

impl TimeDilator {
    fn new() -> Self {
        Self {
            dilation_factor: 1.0,
            motion_blur_strength: 0.0,
            temporal_smoothing: 0.0,
            emergence_time_scaling: 1.0,
            frame_blending_weight: 0.0,
        }
    }

    fn update(&mut self, dt: f32, emergence_amplification: f32, transcendence_level: f32) {
        // Time dilation during emergence events
        self.dilation_factor = 1.0 - emergence_amplification * 0.7; // Slow down time

        // Motion blur strength increases during transcendence
        self.motion_blur_strength = transcendence_level * 0.3;

        // Temporal smoothing for dreamy effects
        self.temporal_smoothing = emergence_amplification * 0.4;

        // Frame blending for ethereal quality
        self.frame_blending_weight = (emergence_amplification + transcendence_level) * 0.2;
    }
}

impl GlitchProcessor {
    fn new() -> Self {
        Self {
            pixel_sort_probability: 0.0,
            data_corruption_level: 0.0,
            scanline_displacement: 0.0,
            digital_noise_intensity: 0.0,
            feedback_loop_strength: 0.0,
        }
    }

    fn update(&mut self, dt: f32, reality_breakdown: f32, chaos_accumulation: f32) {
        // Glitch effects intensify as reality coherence breaks down
        self.pixel_sort_probability = reality_breakdown * 0.05;
        self.data_corruption_level = reality_breakdown * chaos_accumulation * 0.1;
        self.scanline_displacement = reality_breakdown * 0.02;
        self.digital_noise_intensity = (reality_breakdown + chaos_accumulation * 0.5) * 0.15;
        self.feedback_loop_strength = reality_breakdown * 0.3;
    }
}

impl HolographicProjector {
    fn new() -> Self {
        Self {
            interference_patterns: vec![
                InterferencePattern { frequency: 2.0, amplitude: 0.3, phase_offset: 0.0, direction: Vec2::new(1.0, 0.0) },
                InterferencePattern { frequency: 3.5, amplitude: 0.2, phase_offset: 1.57, direction: Vec2::new(0.0, 1.0) },
                InterferencePattern { frequency: 5.0, amplitude: 0.15, phase_offset: 3.14, direction: Vec2::new(0.707, 0.707) },
            ],
            volumetric_intensity: 0.0,
            ray_march_steps: 32,
            hologram_coherence: 1.0,
        }
    }

    fn update(&mut self, dt: f32, collective_intelligence: f32, cosmic_time: f64, beat_intensity: f32) {
        // Update interference patterns
        for pattern in &mut self.interference_patterns {
            pattern.phase_offset += dt * pattern.frequency * collective_intelligence;
            pattern.amplitude = 0.1 + collective_intelligence * 0.2 + beat_intensity * 0.1;
        }

        // Volumetric intensity during high consciousness
        self.volumetric_intensity = collective_intelligence * beat_intensity * 0.4;

        // Hologram coherence affected by collective resonance
        self.hologram_coherence = 0.5 + collective_intelligence * 0.5;
    }

    fn calculate_interference(&self, position: Vec2, cosmic_time: f64) -> f32 {
        let mut total_interference = 0.0;

        for pattern in &self.interference_patterns {
            let dot_product = position.x * pattern.direction.x + position.y * pattern.direction.y;
            let wave = (dot_product * pattern.frequency + cosmic_time as f32 * pattern.frequency + pattern.phase_offset).sin();
            total_interference += wave * pattern.amplitude;
        }

        total_interference.clamp(-1.0, 1.0)
    }
}

// === EMERGENT COMMUNICATION SYSTEMS ===
/// Visual language development between entities
#[derive(Debug, Clone)]
pub struct EmergentCommunicationSystems {
    pub signal_vocabulary: SignalVocabulary,
    pub grammar_evolution: GrammarEvolution,
    pub semantic_networks: SemanticNetworks,
    pub cultural_transmission: CulturalTransmission,
    pub active_communications: Vec<VisualMessage>,
}

#[derive(Debug, Clone)]
pub struct SignalVocabulary {
    pub basic_signals: HashMap<String, VisualSignal>,
    pub signal_complexity: f32,
    pub emergence_rate: f32,
    pub signal_evolution_time: f64,
}

#[derive(Debug, Clone)]
pub struct VisualSignal {
    pub signal_type: SignalType,
    pub color_pattern: Vec3,
    pub movement_pattern: MovementPattern,
    pub intensity: f32,
    pub duration: f32,
    pub meaning_strength: f32,
    pub usage_frequency: f32,
}

#[derive(Debug, Clone)]
pub enum SignalType {
    Color(ColorSignal),
    Movement(MovementSignal),
    Pulse(PulseSignal),
    Shape(ShapeSignal),
    Hybrid(Vec<SignalType>),
}

#[derive(Debug, Clone)]
pub struct ColorSignal {
    pub hue_range: (f32, f32),
    pub saturation_range: (f32, f32),
    pub brightness_range: (f32, f32),
    pub transition_speed: f32,
}

#[derive(Debug, Clone)]
pub struct MovementSignal {
    pub pattern_type: MovementPatternType,
    pub amplitude: f32,
    pub frequency: f32,
    pub direction: Vec2,
}

#[derive(Debug, Clone)]
pub enum MovementPatternType {
    Circular,
    Linear,
    Spiral,
    Figure8,
    Random,
    Harmonic,
}

#[derive(Debug, Clone)]
pub struct PulseSignal {
    pub base_intensity: f32,
    pub pulse_frequency: f32,
    pub pulse_amplitude: f32,
    pub pulse_shape: PulseShape,
}

#[derive(Debug, Clone)]
pub enum PulseShape {
    Sine,
    Square,
    Triangle,
    Sawtooth,
    Exponential,
}

#[derive(Debug, Clone)]
pub struct ShapeSignal {
    pub geometric_form: GeometricForm,
    pub size_modulation: f32,
    pub rotation_speed: f32,
    pub morphing_rate: f32,
}

#[derive(Debug, Clone)]
pub enum GeometricForm {
    Circle,
    Triangle,
    Square,
    Pentagon,
    Star,
    Fractal,
}

#[derive(Debug, Clone)]
pub struct MovementPattern {
    pub pattern_type: MovementPatternType,
    pub amplitude: f32,
    pub frequency: f32,
    pub phase_offset: f32,
}

#[derive(Debug, Clone)]
pub struct GrammarEvolution {
    pub grammar_rules: HashMap<String, GrammarRule>,
    pub complexity_level: f32,
    pub rule_emergence_rate: f32,
    pub syntax_coherence: f32,
    pub message_complexity_capacity: f32,
}

#[derive(Debug, Clone)]
pub struct GrammarRule {
    pub rule_type: RuleType,
    pub conditions: Vec<MessageCondition>,
    pub transformations: Vec<MessageTransformation>,
    pub reliability: f32,
    pub usage_context: CommunicationContext,
}

#[derive(Debug, Clone)]
pub enum RuleType {
    Sequence,      // A followed by B creates meaning C
    Combination,   // A and B together create meaning C
    Modulation,    // A modifies the meaning of B
    Negation,      // A negates or reverses meaning of B
    Amplification, // A strengthens meaning of B
}

#[derive(Debug, Clone)]
pub struct MessageCondition {
    pub signal_presence: Option<String>,
    pub intensity_threshold: Option<f32>,
    pub temporal_relationship: Option<TemporalRelation>,
    pub spatial_relationship: Option<SpatialRelation>,
}

#[derive(Debug, Clone)]
pub enum TemporalRelation {
    Simultaneous,
    Sequential(f32), // time delay
    Overlapping(f32), // overlap duration
    Rhythmic(f32),   // rhythmic pattern
}

#[derive(Debug, Clone)]
pub enum SpatialRelation {
    Adjacent,
    Distant(f32),
    Surrounding,
    Directional(Vec2),
}

#[derive(Debug, Clone)]
pub struct MessageTransformation {
    pub transform_type: TransformType,
    pub parameters: Vec<f32>,
    pub strength: f32,
}

#[derive(Debug, Clone)]
pub enum TransformType {
    ColorShift,
    IntensityChange,
    MovementModification,
    TemporalAdjustment,
    SpatialDisplacement,
}

#[derive(Debug, Clone)]
pub enum CommunicationContext {
    Greeting,
    Warning,
    Cooperation,
    Territorial,
    Emotional,
    Informational,
    Social,
    Emergency,
}

#[derive(Debug, Clone)]
pub struct SemanticNetworks {
    pub concept_associations: HashMap<String, ConceptNode>,
    pub meaning_strength_matrix: Vec<Vec<f32>>,
    pub context_modifiers: HashMap<CommunicationContext, f32>,
    pub emergence_pathways: Vec<EmergencePathway>,
}

#[derive(Debug, Clone)]
pub struct ConceptNode {
    pub concept_id: String,
    pub base_meaning: String,
    pub associated_signals: Vec<String>,
    pub meaning_confidence: f32,
    pub usage_frequency: f32,
    pub context_variations: HashMap<CommunicationContext, f32>,
    pub emergence_timestamp: f64,
}

#[derive(Debug, Clone)]
pub struct EmergencePathway {
    pub source_concepts: Vec<String>,
    pub target_concept: String,
    pub pathway_strength: f32,
    pub emergence_probability: f32,
    pub required_conditions: Vec<EmergenceCondition>,
}

#[derive(Debug, Clone)]
pub struct EmergenceCondition {
    pub condition_type: ConditionType,
    pub threshold: f32,
    pub current_value: f32,
}

#[derive(Debug, Clone)]
pub enum ConditionType {
    ConsciousnessLevel,
    CommunicationFrequency,
    SemanticComplexity,
    CulturalCoherence,
    EnvironmentalStability,
}

#[derive(Debug, Clone)]
pub struct CulturalTransmission {
    pub transmission_networks: Vec<TransmissionNetwork>,
    pub language_variants: HashMap<String, LanguageVariant>,
    pub cultural_drift_rate: f32,
    pub innovation_rate: f32,
    pub adoption_threshold: f32,
}

#[derive(Debug, Clone)]
pub struct TransmissionNetwork {
    pub network_id: String,
    pub participating_entities: Vec<usize>,
    pub transmission_efficiency: f32,
    pub network_stability: f32,
    pub dominant_language_variant: String,
    pub cultural_coherence: f32,
}

#[derive(Debug, Clone)]
pub struct LanguageVariant {
    pub variant_id: String,
    pub signal_modifications: HashMap<String, SignalModification>,
    pub grammar_differences: HashMap<String, GrammarModification>,
    pub prevalence: f32,
    pub innovation_rate: f32,
    pub cultural_fitness: f32,
}

#[derive(Debug, Clone)]
pub struct SignalModification {
    pub original_signal: String,
    pub modified_properties: HashMap<String, f32>,
    pub modification_strength: f32,
}

#[derive(Debug, Clone)]
pub struct GrammarModification {
    pub rule_id: String,
    pub modification_type: GrammarModificationType,
    pub parameters: Vec<f32>,
}

#[derive(Debug, Clone)]
pub enum GrammarModificationType {
    RuleStrengthening,
    RuleWeakening,
    RuleInversion,
    RuleComplication,
    RuleSimplification,
}

#[derive(Debug, Clone)]
pub struct VisualMessage {
    pub sender_id: usize,
    pub receiver_ids: Vec<usize>,
    pub signal_sequence: Vec<ActiveSignal>,
    pub message_intent: CommunicationContext,
    pub spatial_manifestation: SpatialManifestation,
    pub temporal_properties: TemporalProperties,
    pub transmission_effectiveness: f32,
    pub cultural_variant: String,
}

#[derive(Debug, Clone)]
pub struct ActiveSignal {
    pub signal_id: String,
    pub current_intensity: f32,
    pub position: Vec2,
    pub visual_state: VisualState,
    pub duration_remaining: f32,
    pub interaction_radius: f32,
}

#[derive(Debug, Clone)]
pub struct VisualState {
    pub current_color: Vec3,
    pub current_size: f32,
    pub current_rotation: f32,
    pub movement_velocity: Vec2,
    pub pulsing_phase: f32,
}

#[derive(Debug, Clone)]
pub struct SpatialManifestation {
    pub manifestation_type: ManifestationType,
    pub primary_position: Vec2,
    pub affected_area: f32,
    pub visual_trail: Vec<Vec2>,
    pub interference_patterns: Vec<InterferenceZone>,
}

#[derive(Debug, Clone)]
pub enum ManifestationType {
    PointSource,
    LinearPath,
    RadialExpansion,
    NetworkNodes,
    FieldDistortion,
}

#[derive(Debug, Clone)]
pub struct InterferenceZone {
    pub center: Vec2,
    pub radius: f32,
    pub interference_strength: f32,
    pub interference_type: InterferenceType,
}

#[derive(Debug, Clone)]
pub enum InterferenceType {
    Constructive,
    Destructive,
    Modulative,
    Harmonic,
}

#[derive(Debug, Clone)]
pub struct TemporalProperties {
    pub message_duration: f32,
    pub transmission_delay: f32,
    pub repetition_pattern: Option<RepetitionPattern>,
    pub decay_rate: f32,
    pub resonance_buildup: f32,
}

#[derive(Debug, Clone)]
pub struct RepetitionPattern {
    pub repetition_interval: f32,
    pub repetition_count: u32,
    pub intensity_decay_per_repetition: f32,
    pub pattern_modification_rate: f32,
}

// === EVENT-DRIVEN ARCHITECTURE ===
/// Synchronized event system for cascading effects
#[derive(Debug)]
pub struct EventDrivenArchitecture {
    pub event_bus: EventBus,
    pub cascade_engine: CascadeEngine,
    pub synchronization_matrix: SynchronizationMatrix,
    pub event_history: EventHistory,
    pub system_resonance: SystemResonance,
}

#[derive(Debug)]
pub struct EventBus {
    pub pending_events: VecDeque<SystemEvent>,
    pub active_events: HashMap<String, ActiveEvent>,
    pub event_priorities: HashMap<EventType, f32>,
    pub propagation_rules: HashMap<EventType, PropagationRule>,
    pub event_filtering: EventFiltering,
}

#[derive(Debug, Clone)]
pub struct SystemEvent {
    pub event_id: String,
    pub event_type: EventType,
    pub source_system: SystemComponent,
    pub target_systems: Vec<SystemComponent>,
    pub event_data: EventData,
    pub timestamp: f64,
    pub priority: f32,
    pub cascade_potential: f32,
    pub synchronization_requirements: Vec<SynchronizationRequirement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    BeatDrop,
    ConsciousnessSpike,
    RealityDistortion,
    CommunicationEmergence,
    UserInteraction,
    SystemResonance,
    ChaosAmplification,
    EnvironmentalShift,
    EmergenceThreshold,
    CulturalTransmission,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SystemComponent {
    BeatEngine,
    ConsciousnessFramework,
    RealityEngine,
    CommunicationSystems,
    DigitalEcosystem,
    UserInterface,
    SafetySystems,
    VisualizationEngine,
    AudioProcessor,
    CoEvolutionEngine,
}

#[derive(Debug, Clone)]
pub struct EventData {
    pub primary_value: f32,
    pub secondary_values: HashMap<String, f32>,
    pub vector_data: HashMap<String, Vec2>,
    pub entity_references: Vec<usize>,
    pub spatial_information: Option<Vec2>,
    pub temporal_information: Option<TemporalEventInfo>,
}

#[derive(Debug, Clone)]
pub struct TemporalEventInfo {
    pub duration: f32,
    pub frequency: Option<f32>,
    pub phase_offset: f32,
    pub temporal_decay: f32,
}

#[derive(Debug, Clone)]
pub struct SynchronizationRequirement {
    pub requirement_type: SyncType,
    pub target_system: SystemComponent,
    pub synchronization_tolerance: f32,
    pub priority: f32,
}

#[derive(Debug, Clone)]
pub enum SyncType {
    Immediate,
    Rhythmic(f32),
    Delayed(f32),
    Conditional(SyncCondition),
}

#[derive(Debug, Clone)]
pub struct SyncCondition {
    pub condition_type: String,
    pub threshold: f32,
    pub current_state: f32,
}

#[derive(Debug, Clone)]
pub struct ActiveEvent {
    pub base_event: SystemEvent,
    pub current_phase: EventPhase,
    pub affected_systems: HashMap<SystemComponent, SystemState>,
    pub cascade_events: Vec<String>,
    pub resonance_buildup: f32,
    pub synchronization_status: HashMap<SystemComponent, SyncStatus>,
}

#[derive(Debug, Clone)]
pub enum EventPhase {
    Initialization,
    Propagation,
    Peak,
    Decay,
    Completion,
}

#[derive(Debug, Clone)]
pub struct SystemState {
    pub activation_level: f32,
    pub response_magnitude: f32,
    pub state_modifications: HashMap<String, f32>,
    pub feedback_contribution: f32,
}

#[derive(Debug, Clone)]
pub enum SyncStatus {
    Pending,
    InProgress,
    Synchronized,
    Desynchronized,
    Failed,
}

#[derive(Debug)]
pub struct PropagationRule {
    pub source_systems: Vec<SystemComponent>,
    pub target_systems: Vec<SystemComponent>,
    pub propagation_delay: f32,
    pub amplitude_scaling: f32,
    pub interference_factors: HashMap<SystemComponent, f32>,
    pub condition_requirements: Vec<PropagationCondition>,
}

#[derive(Debug)]
pub struct PropagationCondition {
    pub system: SystemComponent,
    pub property: String,
    pub threshold: f32,
    pub comparison: ComparisonType,
}

#[derive(Debug)]
pub enum ComparisonType {
    GreaterThan,
    LessThan,
    EqualTo,
    WithinRange(f32, f32),
}

#[derive(Debug)]
pub struct EventFiltering {
    pub safety_filters: Vec<SafetyFilter>,
    pub performance_filters: Vec<PerformanceFilter>,
    pub coherence_filters: Vec<CoherenceFilter>,
}

#[derive(Debug)]
pub struct SafetyFilter {
    pub filter_type: SafetyFilterType,
    pub threshold: f32,
    pub action: SafetyAction,
}

#[derive(Debug)]
pub enum SafetyFilterType {
    FlashRateLimit,
    LuminanceChange,
    IntensitySpike,
    RapidStateChange,
}

#[derive(Debug)]
pub enum SafetyAction {
    Attenuate(f32),
    Delay(f32),
    Block,
    Redistribute,
}

#[derive(Debug)]
pub struct PerformanceFilter {
    pub max_simultaneous_events: usize,
    pub priority_threshold: f32,
    pub resource_limits: HashMap<String, f32>,
}

#[derive(Debug)]
pub struct CoherenceFilter {
    pub coherence_threshold: f32,
    pub interference_management: InterferenceManagement,
    pub pattern_preservation: PatternPreservation,
}

#[derive(Debug)]
pub struct InterferenceManagement {
    pub destructive_interference_threshold: f32,
    pub constructive_amplification_limit: f32,
    pub phase_alignment_tolerance: f32,
}

#[derive(Debug)]
pub struct PatternPreservation {
    pub maintain_rhythmic_integrity: bool,
    pub preserve_consciousness_patterns: bool,
    pub maintain_communication_coherence: bool,
}

#[derive(Debug)]
pub struct CascadeEngine {
    pub cascade_rules: HashMap<EventType, CascadeRule>,
    pub active_cascades: Vec<ActiveCascade>,
    pub cascade_intensity_matrix: Vec<Vec<f32>>,
    pub feedback_loops: Vec<FeedbackLoop>,
    pub resonance_amplifiers: Vec<ResonanceAmplifier>,
}

#[derive(Debug)]
pub struct CascadeRule {
    pub trigger_event: EventType,
    pub cascade_targets: Vec<CascadeTarget>,
    pub cascade_delay: f32,
    pub amplification_factor: f32,
    pub decay_rate: f32,
    pub maximum_iterations: u32,
}

#[derive(Debug)]
pub struct CascadeTarget {
    pub target_system: SystemComponent,
    pub effect_type: CascadeEffectType,
    pub magnitude_multiplier: f32,
    pub propagation_probability: f32,
}

#[derive(Debug)]
pub enum CascadeEffectType {
    Amplification,
    Resonance,
    StateChange,
    ParameterModification,
    EmergentBehavior,
}

#[derive(Debug)]
pub struct ActiveCascade {
    pub cascade_id: String,
    pub source_event: String,
    pub current_iteration: u32,
    pub affected_systems: HashMap<SystemComponent, f32>,
    pub cascade_strength: f32,
    pub propagation_front: Vec<PropagationFront>,
}

#[derive(Debug)]
pub struct PropagationFront {
    pub system: SystemComponent,
    pub arrival_time: f64,
    pub effect_magnitude: f32,
    pub propagation_vector: Vec2,
}

#[derive(Debug)]
pub struct FeedbackLoop {
    pub loop_id: String,
    pub participating_systems: Vec<SystemComponent>,
    pub feedback_strength: f32,
    pub loop_stability: f32,
    pub resonance_frequency: f32,
    pub amplification_threshold: f32,
}

#[derive(Debug)]
pub struct ResonanceAmplifier {
    pub amplifier_id: String,
    pub target_frequency: f32,
    pub amplification_factor: f32,
    pub bandwidth: f32,
    pub current_resonance: f32,
}

#[derive(Debug)]
pub struct SynchronizationMatrix {
    pub system_synchronization: HashMap<SystemComponent, HashMap<SystemComponent, SyncRelationship>>,
    pub global_synchronization_state: f32,
    pub rhythm_coherence: f32,
    pub phase_alignments: HashMap<SystemComponent, f32>,
    pub synchronization_strength: f32,
}

#[derive(Debug)]
pub struct SyncRelationship {
    pub relationship_type: SyncRelationshipType,
    pub synchronization_strength: f32,
    pub phase_difference: f32,
    pub coherence_level: f32,
    pub stability: f32,
}

#[derive(Debug)]
pub enum SyncRelationshipType {
    InPhase,
    AntiPhase,
    QuadraturePhase,
    Harmonic(f32),
    Independent,
    Coupled(f32),
}

#[derive(Debug)]
pub struct EventHistory {
    pub recent_events: VecDeque<HistoricalEvent>,
    pub pattern_recognition: PatternRecognition,
    pub emergence_tracking: EmergenceTracking,
    pub performance_metrics: PerformanceMetrics,
}

#[derive(Debug, Clone)]
pub struct HistoricalEvent {
    pub event: SystemEvent,
    pub actual_effects: HashMap<SystemComponent, f32>,
    pub cascade_outcomes: Vec<String>,
    pub performance_impact: f32,
    pub user_response: Option<f32>,
}

#[derive(Debug)]
pub struct PatternRecognition {
    pub recurring_patterns: HashMap<String, PatternSignature>,
    pub pattern_prediction_accuracy: f32,
    pub learned_correlations: HashMap<String, f32>,
}

#[derive(Debug)]
pub struct PatternSignature {
    pub pattern_id: String,
    pub event_sequence: Vec<EventType>,
    pub temporal_pattern: Vec<f32>,
    pub system_involvement: HashMap<SystemComponent, f32>,
    pub confidence: f32,
    pub frequency: f32,
}

#[derive(Debug)]
pub struct EmergenceTracking {
    pub emergence_indicators: HashMap<String, f32>,
    pub threshold_monitoring: HashMap<String, ThresholdMonitor>,
    pub emergence_prediction: EmergencePrediction,
}

#[derive(Debug)]
pub struct ThresholdMonitor {
    pub current_value: f32,
    pub threshold: f32,
    pub trend: f32,
    pub time_to_threshold: Option<f32>,
}

#[derive(Debug)]
pub struct EmergencePrediction {
    pub predicted_events: Vec<PredictedEvent>,
    pub confidence_levels: HashMap<EventType, f32>,
    pub temporal_predictions: HashMap<EventType, f32>,
}

#[derive(Debug)]
pub struct PredictedEvent {
    pub event_type: EventType,
    pub predicted_time: f64,
    pub confidence: f32,
    pub expected_magnitude: f32,
}

#[derive(Debug)]
pub struct PerformanceMetrics {
    pub event_processing_time: VecDeque<f64>,
    pub cascade_efficiency: f32,
    pub synchronization_quality: f32,
    pub system_responsiveness: HashMap<SystemComponent, f32>,
}

#[derive(Debug)]
pub struct SystemResonance {
    pub global_resonance_state: f32,
    pub harmonic_frequencies: Vec<f32>,
    pub resonance_nodes: Vec<ResonanceNode>,
    pub interference_patterns: Vec<SystemInterferencePattern>,
    pub coherence_field: CoherenceField,
}

#[derive(Debug)]
pub struct ResonanceNode {
    pub node_id: String,
    pub frequency: f32,
    pub amplitude: f32,
    pub phase: f32,
    pub participating_systems: Vec<SystemComponent>,
    pub resonance_quality: f32,
}

#[derive(Debug)]
pub struct SystemInterferencePattern {
    pub pattern_id: String,
    pub interfering_systems: Vec<SystemComponent>,
    pub interference_type: SystemInterferenceType,
    pub magnitude: f32,
    pub spatial_distribution: HashMap<String, f32>,
}

#[derive(Debug)]
pub enum SystemInterferenceType {
    Constructive,
    Destructive,
    Modulative,
    Chaotic,
}

#[derive(Debug)]
pub struct CoherenceField {
    pub field_strength: f32,
    pub coherence_gradient: HashMap<String, f32>,
    pub stability_regions: Vec<StabilityRegion>,
    pub turbulence_zones: Vec<TurbulenceZone>,
}

#[derive(Debug)]
pub struct StabilityRegion {
    pub region_id: String,
    pub stability_level: f32,
    pub contributing_factors: HashMap<String, f32>,
    pub maintenance_requirements: Vec<String>,
}

#[derive(Debug)]
pub struct TurbulenceZone {
    pub zone_id: String,
    pub turbulence_intensity: f32,
    pub chaos_contribution: f32,
    pub emergence_potential: f32,
}

// === USER CO-EVOLUTION SYSTEM ===
/// Adaptive system that learns and evolves with user interaction patterns
#[derive(Debug)]
pub struct UserCoEvolutionSystem {
    pub interaction_learning: InteractionLearning,
    pub adaptation_engine: AdaptationEngine,
    pub preference_memory: PreferenceMemory,
    pub evolution_pathways: EvolutionPathways,
    pub personalization_matrix: PersonalizationMatrix,
}

#[derive(Debug)]
pub struct InteractionLearning {
    pub interaction_patterns: HashMap<String, InteractionPattern>,
    pub behavioral_clustering: BehavioralClustering,
    pub preference_inference: PreferenceInference,
    pub engagement_tracking: EngagementTracking,
    pub learning_algorithms: LearningAlgorithms,
}

#[derive(Debug)]
pub struct InteractionPattern {
    pub pattern_id: String,
    pub interaction_sequence: Vec<UserAction>,
    pub temporal_characteristics: TemporalCharacteristics,
    pub preference_indicators: HashMap<String, f32>,
    pub engagement_level: f32,
    pub pattern_frequency: f32,
    pub pattern_evolution: PatternEvolution,
}

#[derive(Debug, Clone)]
pub struct UserAction {
    pub action_type: ActionType,
    pub timestamp: f64,
    pub duration: f32,
    pub intensity: f32,
    pub spatial_coordinates: Option<Vec2>,
    pub context: ActionContext,
}

#[derive(Debug, Clone)]
pub enum ActionType {
    Click,
    DoubleClick,
    LongPress,
    Movement,
    Gesture,
    Pause,
    Focus,
    ViewDirection,
    EngagementShift,
}

#[derive(Debug, Clone)]
pub struct ActionContext {
    pub system_state: HashMap<String, f32>,
    pub visual_environment: VisualEnvironmentState,
    pub audio_environment: AudioEnvironmentState,
    pub concurrent_actions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct VisualEnvironmentState {
    pub dominant_colors: Vec<Vec3>,
    pub complexity_level: f32,
    pub movement_intensity: f32,
    pub flash_rate: f32,
    pub consciousness_visibility: f32,
}

#[derive(Debug, Clone)]
pub struct AudioEnvironmentState {
    pub beat_intensity: f32,
    pub harmonic_complexity: f32,
    pub frequency_distribution: HashMap<String, f32>,
    pub rhythm_coherence: f32,
}

#[derive(Debug)]
pub struct TemporalCharacteristics {
    pub action_intervals: Vec<f32>,
    pub session_duration_preference: f32,
    pub interaction_density: f32,
    pub rhythm_alignment: f32,
    pub temporal_clustering: TemporalClustering,
}

#[derive(Debug)]
pub struct TemporalClustering {
    pub burst_patterns: Vec<BurstPattern>,
    pub sustained_engagement_periods: Vec<SustainedEngagement>,
    pub pause_patterns: Vec<PausePattern>,
}

#[derive(Debug)]
pub struct BurstPattern {
    pub burst_duration: f32,
    pub action_density: f32,
    pub intensity_profile: Vec<f32>,
    pub typical_triggers: Vec<String>,
}

#[derive(Debug)]
pub struct SustainedEngagement {
    pub duration: f32,
    pub consistency_level: f32,
    pub engagement_quality: f32,
    pub maintenance_factors: Vec<String>,
}

#[derive(Debug)]
pub struct PausePattern {
    pub pause_duration: f32,
    pub pause_context: String,
    pub recovery_time: f32,
    pub re_engagement_style: String,
}

#[derive(Debug)]
pub struct PatternEvolution {
    pub evolution_rate: f32,
    pub adaptation_direction: Vec<f32>,
    pub stability_factors: HashMap<String, f32>,
    pub innovation_tendency: f32,
}

#[derive(Debug)]
pub struct BehavioralClustering {
    pub clusters: HashMap<String, BehavioralCluster>,
    pub cluster_transitions: HashMap<String, HashMap<String, f32>>,
    pub current_cluster: Option<String>,
    pub cluster_stability: f32,
}

#[derive(Debug)]
pub struct BehavioralCluster {
    pub cluster_id: String,
    pub characteristic_patterns: Vec<String>,
    pub cluster_center: Vec<f32>,
    pub cluster_radius: f32,
    pub member_patterns: Vec<String>,
    pub cluster_quality: f32,
}

#[derive(Debug)]
pub struct PreferenceInference {
    pub inferred_preferences: HashMap<String, PreferenceProfile>,
    pub confidence_levels: HashMap<String, f32>,
    pub preference_evolution: HashMap<String, f32>,
    pub inference_accuracy: f32,
}

#[derive(Debug)]
pub struct PreferenceProfile {
    pub preference_category: String,
    pub preference_strength: f32,
    pub preference_specificity: f32,
    pub contextual_variations: HashMap<String, f32>,
    pub stability_over_time: f32,
}

#[derive(Debug)]
pub struct EngagementTracking {
    pub engagement_metrics: EngagementMetrics,
    pub engagement_patterns: Vec<EngagementPattern>,
    pub flow_state_indicators: FlowStateIndicators,
    pub attention_mapping: AttentionMapping,
}

#[derive(Debug)]
pub struct EngagementMetrics {
    pub overall_engagement: f32,
    pub visual_engagement: f32,
    pub interactive_engagement: f32,
    pub temporal_engagement: f32,
    pub cognitive_load: f32,
}

#[derive(Debug)]
pub struct EngagementPattern {
    pub pattern_name: String,
    pub engagement_curve: Vec<f32>,
    pub trigger_conditions: Vec<String>,
    pub sustainability_factors: Vec<String>,
    pub enhancement_opportunities: Vec<String>,
}

#[derive(Debug)]
pub struct FlowStateIndicators {
    pub flow_probability: f32,
    pub challenge_balance: f32,
    pub skill_utilization: f32,
    pub attention_focus: f32,
    pub intrinsic_motivation: f32,
}

#[derive(Debug)]
pub struct AttentionMapping {
    pub attention_zones: HashMap<String, AttentionZone>,
    pub attention_transitions: Vec<AttentionTransition>,
    pub focus_sustainability: f32,
    pub distraction_resilience: f32,
}

#[derive(Debug)]
pub struct AttentionZone {
    pub zone_id: String,
    pub spatial_bounds: (Vec2, Vec2),
    pub attention_intensity: f32,
    pub dwell_time: f32,
    pub return_frequency: f32,
}

#[derive(Debug)]
pub struct AttentionTransition {
    pub from_zone: String,
    pub to_zone: String,
    pub transition_speed: f32,
    pub transition_smoothness: f32,
    pub typical_triggers: Vec<String>,
}

#[derive(Debug)]
pub struct LearningAlgorithms {
    pub pattern_recognition: PatternRecognitionAlgorithm,
    pub preference_learning: PreferenceLearningAlgorithm,
    pub adaptation_optimization: AdaptationOptimization,
    pub feedback_integration: FeedbackIntegration,
}

#[derive(Debug)]
pub struct PatternRecognitionAlgorithm {
    pub algorithm_type: String,
    pub learning_rate: f32,
    pub pattern_sensitivity: f32,
    pub noise_tolerance: f32,
    pub adaptation_speed: f32,
}

#[derive(Debug)]
pub struct PreferenceLearningAlgorithm {
    pub preference_weights: HashMap<String, f32>,
    pub learning_momentum: f32,
    pub forgetting_factor: f32,
    pub confidence_threshold: f32,
}

#[derive(Debug)]
pub struct AdaptationOptimization {
    pub optimization_target: OptimizationTarget,
    pub constraint_handling: ConstraintHandling,
    pub exploration_exploitation_balance: f32,
    pub convergence_criteria: ConvergenceCriteria,
}

#[derive(Debug)]
pub enum OptimizationTarget {
    MaximizeEngagement,
    MinimizeCognitiveLoad,
    BalanceComplexityAndComfort,
    OptimizeFlowState,
    CustomTarget(String),
}

#[derive(Debug)]
pub struct ConstraintHandling {
    pub safety_constraints: Vec<SafetyConstraint>,
    pub performance_constraints: Vec<PerformanceConstraint>,
    pub user_defined_constraints: Vec<UserConstraint>,
}

#[derive(Debug)]
pub struct SafetyConstraint {
    pub constraint_type: String,
    pub threshold: f32,
    pub enforcement_priority: f32,
}

#[derive(Debug)]
pub struct PerformanceConstraint {
    pub resource_type: String,
    pub limit: f32,
    pub degradation_strategy: String,
}

#[derive(Debug)]
pub struct UserConstraint {
    pub constraint_name: String,
    pub constraint_value: f32,
    pub flexibility: f32,
}

#[derive(Debug)]
pub struct ConvergenceCriteria {
    pub stability_threshold: f32,
    pub improvement_threshold: f32,
    pub maximum_iterations: u32,
    pub time_limit: f32,
}

#[derive(Debug)]
pub struct FeedbackIntegration {
    pub explicit_feedback: ExplicitFeedback,
    pub implicit_feedback: ImplicitFeedback,
    pub feedback_weighting: FeedbackWeighting,
    pub feedback_validation: FeedbackValidation,
}

#[derive(Debug)]
pub struct ExplicitFeedback {
    pub feedback_mechanisms: Vec<FeedbackMechanism>,
    pub feedback_history: VecDeque<UserFeedback>,
    pub feedback_processing: FeedbackProcessing,
}

#[derive(Debug)]
pub struct FeedbackMechanism {
    pub mechanism_type: String,
    pub availability: f32,
    pub user_adoption: f32,
    pub feedback_quality: f32,
}

#[derive(Debug, Clone)]
pub struct UserFeedback {
    pub feedback_type: String,
    pub timestamp: f64,
    pub value: f32,
    pub context: String,
    pub reliability: f32,
}

#[derive(Debug)]
pub struct FeedbackProcessing {
    pub processing_delay: f32,
    pub noise_filtering: f32,
    pub context_weighting: f32,
    pub temporal_weighting: f32,
}

#[derive(Debug)]
pub struct ImplicitFeedback {
    pub behavioral_indicators: HashMap<String, f32>,
    pub physiological_proxies: HashMap<String, f32>,
    pub interaction_quality_metrics: HashMap<String, f32>,
    pub inference_confidence: f32,
}

#[derive(Debug)]
pub struct FeedbackWeighting {
    pub explicit_weight: f32,
    pub implicit_weight: f32,
    pub recency_weighting: f32,
    pub confidence_weighting: f32,
}

#[derive(Debug)]
pub struct FeedbackValidation {
    pub validation_methods: Vec<ValidationMethod>,
    pub consistency_checking: ConsistencyChecking,
    pub outlier_detection: OutlierDetection,
}

#[derive(Debug)]
pub struct ValidationMethod {
    pub method_name: String,
    pub validation_strength: f32,
    pub applicable_contexts: Vec<String>,
}

#[derive(Debug)]
pub struct ConsistencyChecking {
    pub temporal_consistency: f32,
    pub contextual_consistency: f32,
    pub behavioral_consistency: f32,
}

#[derive(Debug)]
pub struct OutlierDetection {
    pub detection_threshold: f32,
    pub outlier_handling_strategy: String,
    pub false_positive_rate: f32,
}

#[derive(Debug)]
pub struct AdaptationEngine {
    pub adaptation_strategies: HashMap<String, AdaptationStrategy>,
    pub active_adaptations: Vec<ActiveAdaptation>,
    pub adaptation_effectiveness: AdaptationEffectiveness,
    pub adaptation_constraints: AdaptationConstraints,
}

#[derive(Debug)]
pub struct AdaptationStrategy {
    pub strategy_name: String,
    pub adaptation_scope: AdaptationScope,
    pub adaptation_parameters: HashMap<String, f32>,
    pub trigger_conditions: Vec<TriggerCondition>,
    pub effectiveness_history: VecDeque<f32>,
}

#[derive(Debug)]
pub enum AdaptationScope {
    Visual,
    Temporal,
    Interactive,
    Complexity,
    Personalization,
    Global,
}

#[derive(Debug)]
pub struct TriggerCondition {
    pub condition_type: String,
    pub threshold: f32,
    pub current_value: f32,
    pub condition_met: bool,
}

#[derive(Debug)]
pub struct ActiveAdaptation {
    pub adaptation_id: String,
    pub strategy_name: String,
    pub start_time: f64,
    pub target_parameters: HashMap<String, f32>,
    pub current_progress: f32,
    pub adaptation_rate: f32,
    pub expected_completion: f64,
}

#[derive(Debug)]
pub struct AdaptationEffectiveness {
    pub strategy_performance: HashMap<String, f32>,
    pub user_satisfaction_correlation: HashMap<String, f32>,
    pub adaptation_success_rate: f32,
    pub optimization_convergence: f32,
}

#[derive(Debug)]
pub struct AdaptationConstraints {
    pub rate_limits: HashMap<String, f32>,
    pub stability_requirements: HashMap<String, f32>,
    pub user_comfort_bounds: HashMap<String, (f32, f32)>,
    pub system_performance_limits: HashMap<String, f32>,
}

#[derive(Debug)]
pub struct PreferenceMemory {
    pub long_term_preferences: HashMap<String, LongTermPreference>,
    pub session_preferences: HashMap<String, SessionPreference>,
    pub contextual_preferences: HashMap<String, ContextualPreference>,
    pub preference_persistence: PreferencePersistence,
}

#[derive(Debug)]
pub struct LongTermPreference {
    pub preference_id: String,
    pub preference_value: f32,
    pub confidence_level: f32,
    pub establishment_time: f64,
    pub last_reinforcement: f64,
    pub stability_score: f32,
    pub influence_weight: f32,
}

#[derive(Debug)]
pub struct SessionPreference {
    pub preference_id: String,
    pub session_value: f32,
    pub session_confidence: f32,
    pub evolution_during_session: Vec<f32>,
    pub alignment_with_long_term: f32,
}

#[derive(Debug)]
pub struct ContextualPreference {
    pub context_id: String,
    pub context_specific_values: HashMap<String, f32>,
    pub context_activation_frequency: f32,
    pub context_influence_strength: f32,
}

#[derive(Debug)]
pub struct PreferencePersistence {
    pub persistence_strategy: PersistenceStrategy,
    pub decay_rates: HashMap<String, f32>,
    pub reinforcement_mechanisms: Vec<ReinforcementMechanism>,
    pub memory_consolidation: MemoryConsolidation,
}

#[derive(Debug)]
pub enum PersistenceStrategy {
    ExponentialDecay,
    LinearDecay,
    ThresholdBased,
    ReinforcementBased,
    Hybrid,
}

#[derive(Debug)]
pub struct ReinforcementMechanism {
    pub mechanism_type: String,
    pub reinforcement_strength: f32,
    pub activation_conditions: Vec<String>,
    pub effectiveness: f32,
}

#[derive(Debug)]
pub struct MemoryConsolidation {
    pub consolidation_intervals: Vec<f32>,
    pub consolidation_strength: f32,
    pub interference_resistance: f32,
    pub long_term_integration_rate: f32,
}

#[derive(Debug)]
pub struct EvolutionPathways {
    pub available_pathways: HashMap<String, EvolutionPathway>,
    pub current_pathway: Option<String>,
    pub pathway_transitions: HashMap<String, HashMap<String, f32>>,
    pub evolution_momentum: f32,
}

#[derive(Debug)]
pub struct EvolutionPathway {
    pub pathway_id: String,
    pub pathway_description: String,
    pub evolution_stages: Vec<EvolutionStage>,
    pub current_stage: usize,
    pub progression_criteria: Vec<ProgressionCriterion>,
    pub pathway_benefits: HashMap<String, f32>,
}

#[derive(Debug)]
pub struct EvolutionStage {
    pub stage_id: String,
    pub stage_description: String,
    pub required_adaptations: Vec<String>,
    pub stage_duration_estimate: f32,
    pub complexity_level: f32,
    pub user_readiness_requirements: Vec<String>,
}

#[derive(Debug)]
pub struct ProgressionCriterion {
    pub criterion_type: String,
    pub threshold: f32,
    pub current_progress: f32,
    pub weight: f32,
}

#[derive(Debug)]
pub struct PersonalizationMatrix {
    pub personalization_dimensions: HashMap<String, PersonalizationDimension>,
    pub cross_dimensional_interactions: HashMap<String, HashMap<String, f32>>,
    pub personalization_effectiveness: f32,
    pub adaptation_responsiveness: f32,
}

#[derive(Debug)]
pub struct PersonalizationDimension {
    pub dimension_name: String,
    pub current_value: f32,
    pub optimal_range: (f32, f32),
    pub adaptation_sensitivity: f32,
    pub user_control_level: f32,
    pub system_influence_level: f32,
}

impl EmergentCommunicationSystems {
    pub fn new() -> Self {
        Self {
            signal_vocabulary: SignalVocabulary::new(),
            grammar_evolution: GrammarEvolution::new(),
            semantic_networks: SemanticNetworks::new(),
            cultural_transmission: CulturalTransmission::new(),
            active_communications: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: f32, llamas: &[Llama], ecosystem: &DigitalEcosystem,
                  consciousness_level: f32, cosmic_time: f64) {
        // Update signal vocabulary evolution
        self.signal_vocabulary.update(dt, consciousness_level, cosmic_time);

        // Evolve grammar based on usage patterns
        self.grammar_evolution.update(dt, &self.active_communications, consciousness_level);

        // Update semantic associations
        self.semantic_networks.update(dt, &self.active_communications, &self.signal_vocabulary);

        // Handle cultural transmission between entities
        self.cultural_transmission.update(dt, llamas, &self.active_communications);

        // Update active communications
        self.update_active_communications(dt, llamas, ecosystem, cosmic_time);

        // Generate new communications based on entity interactions
        self.generate_new_communications(llamas, ecosystem, consciousness_level, cosmic_time);
    }

    fn update_active_communications(&mut self, dt: f32, llamas: &[Llama],
                                   ecosystem: &DigitalEcosystem, cosmic_time: f64) {
        for communication in &mut self.active_communications {
            communication.update(dt, llamas, ecosystem, cosmic_time);
        }

        // Remove completed communications
        self.active_communications.retain(|comm| !comm.is_completed());
    }

    fn generate_new_communications(&mut self, llamas: &[Llama], ecosystem: &DigitalEcosystem,
                                  consciousness_level: f32, cosmic_time: f64) {
        // Generate communications based on llama interactions and consciousness level
        for (i, llama) in llamas.iter().enumerate() {
            if llama.consciousness > 0.7 && cosmic_time as f32 % 2.0 < 0.1 {
                // High consciousness llamas attempt communication
                if let Some(communication) = self.create_communication(i, llamas, ecosystem, cosmic_time) {
                    self.active_communications.push(communication);
                }
            }
        }
    }

    fn create_communication(&self, sender_id: usize, llamas: &[Llama],
                           ecosystem: &DigitalEcosystem, cosmic_time: f64) -> Option<VisualMessage> {
        // Create a new visual message based on current context
        let sender = &llamas[sender_id];

        // Find nearby llamas as potential receivers
        let nearby_llamas: Vec<usize> = llamas.iter().enumerate()
            .filter(|(i, llama)| *i != sender_id &&
                    (sender.position - llama.position).length() < 200.0)
            .map(|(i, _)| i)
            .collect();

        if nearby_llamas.is_empty() {
            return None;
        }

        // Determine communication intent based on sender's state
        let intent = if sender.consciousness > 0.9 {
            CommunicationContext::Social
        } else if sender.prime_chaos_factor > 0.8 {
            CommunicationContext::Emotional
        } else {
            CommunicationContext::Informational
        };

        // Create signal based on available vocabulary
        let signal_sequence = vec![ActiveSignal {
            signal_id: "basic_greeting".to_string(),
            current_intensity: sender.consciousness,
            position: sender.position,
            visual_state: VisualState {
                current_color: Vec3::new(sender.color.x, sender.color.y, 0.6),
                current_size: 20.0 + sender.consciousness * 30.0,
                current_rotation: 0.0,
                movement_velocity: Vec2::ZERO,
                pulsing_phase: cosmic_time as f32,
            },
            duration_remaining: 2.0,
            interaction_radius: 50.0,
        }];

        Some(VisualMessage {
            sender_id,
            receiver_ids: nearby_llamas,
            signal_sequence,
            message_intent: intent,
            spatial_manifestation: SpatialManifestation {
                manifestation_type: ManifestationType::RadialExpansion,
                primary_position: sender.position,
                affected_area: 100.0,
                visual_trail: vec![sender.position],
                interference_patterns: Vec::new(),
            },
            temporal_properties: TemporalProperties {
                message_duration: 2.0,
                transmission_delay: 0.1,
                repetition_pattern: None,
                decay_rate: 0.5,
                resonance_buildup: 0.0,
            },
            transmission_effectiveness: 0.8,
            cultural_variant: "default".to_string(),
        })
    }
}

impl SignalVocabulary {
    fn new() -> Self {
        let mut basic_signals = HashMap::new();

        // Initialize basic signals
        basic_signals.insert("basic_greeting".to_string(), VisualSignal {
            signal_type: SignalType::Color(ColorSignal {
                hue_range: (120.0, 180.0), // Green-cyan range
                saturation_range: (0.7, 1.0),
                brightness_range: (0.6, 1.0),
                transition_speed: 2.0,
            }),
            color_pattern: Vec3::new(0.3, 0.8, 0.6),
            movement_pattern: MovementPattern {
                pattern_type: MovementPatternType::Circular,
                amplitude: 20.0,
                frequency: 1.0,
                phase_offset: 0.0,
            },
            intensity: 0.8,
            duration: 2.0,
            meaning_strength: 0.9,
            usage_frequency: 0.0,
        });

        Self {
            basic_signals,
            signal_complexity: 0.1,
            emergence_rate: 0.01,
            signal_evolution_time: 0.0,
        }
    }

    fn update(&mut self, dt: f32, consciousness_level: f32, cosmic_time: f64) {
        self.signal_evolution_time += dt as f64;

        // Evolve signal complexity based on consciousness
        self.signal_complexity += dt * consciousness_level * 0.01;
        self.signal_complexity = self.signal_complexity.min(1.0);

        // Increase emergence rate with higher consciousness
        self.emergence_rate = 0.01 + consciousness_level * 0.02;

        // Potentially evolve existing signals
        for signal in self.basic_signals.values_mut() {
            signal.evolve(dt, consciousness_level);
        }

        // Create new signals if complexity threshold is met
        if self.signal_complexity > 0.5 && self.basic_signals.len() < 10 {
            self.generate_new_signal(consciousness_level, cosmic_time);
        }
    }

    fn generate_new_signal(&mut self, consciousness_level: f32, cosmic_time: f64) {
        let signal_id = format!("evolved_signal_{}", self.basic_signals.len());

        let new_signal = VisualSignal {
            signal_type: SignalType::Hybrid(vec![
                SignalType::Color(ColorSignal {
                    hue_range: (cosmic_time as f32 % 360.0, (cosmic_time as f32 + 60.0) % 360.0),
                    saturation_range: (0.6, 1.0),
                    brightness_range: (0.5, 0.9),
                    transition_speed: 1.0 + consciousness_level,
                }),
                SignalType::Movement(MovementSignal {
                    pattern_type: MovementPatternType::Spiral,
                    amplitude: 15.0 + consciousness_level * 20.0,
                    frequency: 0.5 + consciousness_level * 1.5,
                    direction: Vec2::new(consciousness_level.cos(), consciousness_level.sin()),
                }),
            ]),
            color_pattern: Vec3::new(
                (cosmic_time as f32 * 0.1).sin() * 0.5 + 0.5,
                (cosmic_time as f32 * 0.15).cos() * 0.5 + 0.5,
                consciousness_level,
            ),
            movement_pattern: MovementPattern {
                pattern_type: MovementPatternType::Spiral,
                amplitude: 25.0,
                frequency: 1.2,
                phase_offset: cosmic_time as f32,
            },
            intensity: consciousness_level,
            duration: 3.0,
            meaning_strength: 0.5,
            usage_frequency: 0.0,
        };

        self.basic_signals.insert(signal_id, new_signal);
    }
}

impl VisualSignal {
    fn evolve(&mut self, dt: f32, consciousness_level: f32) {
        // Evolve the signal properties based on usage and consciousness
        self.meaning_strength += dt * consciousness_level * 0.01;
        self.meaning_strength = self.meaning_strength.min(1.0);

        // Adjust intensity based on consciousness
        self.intensity = (self.intensity + consciousness_level * dt * 0.1).min(1.0);

        // Evolve color pattern
        self.color_pattern = self.color_pattern.lerp(
            Vec3::new(consciousness_level, 0.8, 0.9),
            dt * 0.05
        );
    }
}

impl GrammarEvolution {
    fn new() -> Self {
        Self {
            grammar_rules: HashMap::new(),
            complexity_level: 0.1,
            rule_emergence_rate: 0.005,
            syntax_coherence: 0.8,
            message_complexity_capacity: 0.2,
        }
    }

    fn update(&mut self, dt: f32, active_communications: &[VisualMessage], consciousness_level: f32) {
        // Evolve grammar complexity
        self.complexity_level += dt * consciousness_level * 0.005;
        self.complexity_level = self.complexity_level.min(1.0);

        // Update rule emergence rate
        self.rule_emergence_rate = 0.005 + consciousness_level * 0.01;

        // Analyze active communications for pattern emergence
        self.analyze_communication_patterns(active_communications);

        // Generate new rules if complexity allows
        if self.complexity_level > 0.3 && self.grammar_rules.len() < 20 {
            self.generate_new_rule(consciousness_level);
        }
    }

    fn analyze_communication_patterns(&mut self, communications: &[VisualMessage]) {
        // Simple pattern analysis - look for repeated signal combinations
        for comm in communications {
            if comm.signal_sequence.len() > 1 {
                // This is a compound message - potential for grammar rule
                self.syntax_coherence += 0.001;
            }
        }
        self.syntax_coherence = self.syntax_coherence.min(1.0);
    }

    fn generate_new_rule(&mut self, consciousness_level: f32) {
        let rule_id = format!("rule_{}", self.grammar_rules.len());

        let new_rule = GrammarRule {
            rule_type: RuleType::Sequence,
            conditions: vec![MessageCondition {
                signal_presence: Some("basic_greeting".to_string()),
                intensity_threshold: Some(0.5),
                temporal_relationship: Some(TemporalRelation::Sequential(0.5)),
                spatial_relationship: None,
            }],
            transformations: vec![MessageTransformation {
                transform_type: TransformType::IntensityChange,
                parameters: vec![1.2], // Amplify by 20%
                strength: consciousness_level,
            }],
            reliability: 0.7,
            usage_context: CommunicationContext::Social,
        };

        self.grammar_rules.insert(rule_id, new_rule);
    }
}

impl SemanticNetworks {
    fn new() -> Self {
        Self {
            concept_associations: HashMap::new(),
            meaning_strength_matrix: Vec::new(),
            context_modifiers: HashMap::new(),
            emergence_pathways: Vec::new(),
        }
    }

    fn update(&mut self, dt: f32, communications: &[VisualMessage], vocabulary: &SignalVocabulary) {
        // Update concept associations based on communication patterns
        for comm in communications {
            self.strengthen_associations(&comm.message_intent, &comm.signal_sequence);
        }

        // Update emergence pathways
        self.update_emergence_pathways(dt);

        // Decay unused associations
        self.decay_unused_associations(dt);
    }

    fn strengthen_associations(&mut self, intent: &CommunicationContext, signals: &[ActiveSignal]) {
        for signal in signals {
            let concept_id = format!("{:?}_{}", intent, signal.signal_id);
            self.concept_associations.entry(concept_id.clone())
                .or_insert_with(|| ConceptNode {
                    concept_id: concept_id.clone(),
                    base_meaning: format!("{:?}", intent),
                    associated_signals: vec![signal.signal_id.clone()],
                    meaning_confidence: 0.5,
                    usage_frequency: 0.0,
                    context_variations: HashMap::new(),
                    emergence_timestamp: 0.0,
                })
                .usage_frequency += 0.1;
        }
    }

    fn update_emergence_pathways(&mut self, dt: f32) {
        for pathway in &mut self.emergence_pathways {
            pathway.pathway_strength += dt * 0.01;
            pathway.pathway_strength = pathway.pathway_strength.min(1.0);
        }
    }

    fn decay_unused_associations(&mut self, dt: f32) {
        for concept in self.concept_associations.values_mut() {
            concept.meaning_confidence -= dt * 0.001;
            concept.meaning_confidence = concept.meaning_confidence.max(0.0);
        }
    }
}

impl CulturalTransmission {
    fn new() -> Self {
        Self {
            transmission_networks: Vec::new(),
            language_variants: HashMap::new(),
            cultural_drift_rate: 0.01,
            innovation_rate: 0.005,
            adoption_threshold: 0.3,
        }
    }

    fn update(&mut self, dt: f32, llamas: &[Llama], communications: &[VisualMessage]) {
        // Update transmission networks based on llama interactions
        self.update_networks(llamas, communications);

        // Apply cultural drift
        self.apply_cultural_drift(dt);

        // Process innovation
        self.process_innovation(dt);
    }

    fn update_networks(&mut self, llamas: &[Llama], communications: &[VisualMessage]) {
        // Create or update transmission networks based on active communications
        for comm in communications {
            let network_id = format!("network_{}", comm.sender_id);
            if !self.transmission_networks.iter().any(|n| n.network_id == network_id) {
                self.transmission_networks.push(TransmissionNetwork {
                    network_id,
                    participating_entities: vec![comm.sender_id],
                    transmission_efficiency: 0.7,
                    network_stability: 0.8,
                    dominant_language_variant: "default".to_string(),
                    cultural_coherence: 0.6,
                });
            }
        }
    }

    fn apply_cultural_drift(&mut self, dt: f32) {
        for variant in self.language_variants.values_mut() {
            variant.innovation_rate += dt * self.cultural_drift_rate * 0.1;
        }
    }

    fn process_innovation(&mut self, dt: f32) {
        // Generate new language variants based on innovation rate
        if self.language_variants.len() < 5 && dt * self.innovation_rate > 0.01 {
            let variant_id = format!("variant_{}", self.language_variants.len());
            self.language_variants.insert(variant_id.clone(), LanguageVariant {
                variant_id,
                signal_modifications: HashMap::new(),
                grammar_differences: HashMap::new(),
                prevalence: 0.1,
                innovation_rate: self.innovation_rate,
                cultural_fitness: 0.5,
            });
        }
    }
}

impl VisualMessage {
    fn update(&mut self, dt: f32, llamas: &[Llama], ecosystem: &DigitalEcosystem, cosmic_time: f64) {
        // Update signal states
        for signal in &mut self.signal_sequence {
            signal.update(dt, cosmic_time);
        }

        // Update temporal properties
        self.temporal_properties.message_duration -= dt;
        self.temporal_properties.resonance_buildup += dt * 0.1;

        // Update spatial manifestation
        self.spatial_manifestation.update(dt, llamas, ecosystem);
    }

    fn is_completed(&self) -> bool {
        self.temporal_properties.message_duration <= 0.0
    }
}

impl ActiveSignal {
    fn update(&mut self, dt: f32, cosmic_time: f64) {
        self.duration_remaining -= dt;

        // Update visual state
        self.visual_state.pulsing_phase += dt * 2.0;
        self.visual_state.current_rotation += dt * 0.5;

        // Fade out as duration decreases
        if self.duration_remaining < 1.0 {
            self.current_intensity *= 0.95;
        }
    }
}

impl SpatialManifestation {
    fn update(&mut self, dt: f32, llamas: &[Llama], ecosystem: &DigitalEcosystem) {
        // Update manifestation based on type
        match self.manifestation_type {
            ManifestationType::RadialExpansion => {
                self.affected_area += dt * 50.0; // Expand outward
            }
            ManifestationType::LinearPath => {
                // Move along a path
                if let Some(last_pos) = self.visual_trail.last() {
                    let new_pos = *last_pos + Vec2::new(dt * 100.0, 0.0);
                    self.visual_trail.push(new_pos);
                }
            }
            _ => {}
        }

        // Update interference patterns
        for pattern in &mut self.interference_patterns {
            pattern.interference_strength *= 0.99; // Gradual decay
        }
    }
}

impl EventDrivenArchitecture {
    pub fn new() -> Self {
        Self {
            event_bus: EventBus::new(),
            cascade_engine: CascadeEngine::new(),
            synchronization_matrix: SynchronizationMatrix::new(),
            event_history: EventHistory::new(),
            system_resonance: SystemResonance::new(),
        }
    }

    pub fn update(&mut self, dt: f32, beat_intensity: f32, consciousness_level: f32,
                  cosmic_time: f64, user_interaction_intensity: f32) {
        // Process pending events
        self.event_bus.process_events(dt, &mut self.cascade_engine, &mut self.synchronization_matrix);

        // Update cascade engine
        self.cascade_engine.update(dt, &mut self.event_bus);

        // Update synchronization
        self.synchronization_matrix.update(dt, &self.event_bus.active_events);

        // Update system resonance
        self.system_resonance.update(dt, beat_intensity, consciousness_level, cosmic_time);

        // Record events in history
        self.event_history.update(dt, &self.event_bus.active_events);

        // Generate system events based on current state
        self.generate_system_events(beat_intensity, consciousness_level, user_interaction_intensity, cosmic_time);
    }

    fn generate_system_events(&mut self, beat_intensity: f32, consciousness_level: f32,
                             user_interaction_intensity: f32, cosmic_time: f64) {
        // Generate beat drop events
        if beat_intensity > 0.8 && cosmic_time as f32 % 1.0 < 0.1 {
            let beat_event = SystemEvent {
                event_id: format!("beat_drop_{}", cosmic_time as u64),
                event_type: EventType::BeatDrop,
                source_system: SystemComponent::BeatEngine,
                target_systems: vec![
                    SystemComponent::VisualizationEngine,
                    SystemComponent::RealityEngine,
                    SystemComponent::ConsciousnessFramework,
                ],
                event_data: EventData {
                    primary_value: beat_intensity,
                    secondary_values: HashMap::new(),
                    vector_data: HashMap::new(),
                    entity_references: Vec::new(),
                    spatial_information: None,
                    temporal_information: Some(TemporalEventInfo {
                        duration: 0.5,
                        frequency: Some(2.0),
                        phase_offset: 0.0,
                        temporal_decay: 0.8,
                    }),
                },
                timestamp: cosmic_time,
                priority: 0.9,
                cascade_potential: 0.8,
                synchronization_requirements: vec![
                    SynchronizationRequirement {
                        requirement_type: SyncType::Immediate,
                        target_system: SystemComponent::VisualizationEngine,
                        synchronization_tolerance: 0.05,
                        priority: 1.0,
                    }
                ],
            };
            self.event_bus.pending_events.push_back(beat_event);
        }

        // Generate consciousness spike events
        if consciousness_level > 0.9 {
            let consciousness_event = SystemEvent {
                event_id: format!("consciousness_spike_{}", cosmic_time as u64),
                event_type: EventType::ConsciousnessSpike,
                source_system: SystemComponent::ConsciousnessFramework,
                target_systems: vec![
                    SystemComponent::CommunicationSystems,
                    SystemComponent::RealityEngine,
                    SystemComponent::DigitalEcosystem,
                ],
                event_data: EventData {
                    primary_value: consciousness_level,
                    secondary_values: HashMap::new(),
                    vector_data: HashMap::new(),
                    entity_references: Vec::new(),
                    spatial_information: None,
                    temporal_information: Some(TemporalEventInfo {
                        duration: 2.0,
                        frequency: None,
                        phase_offset: 0.0,
                        temporal_decay: 0.3,
                    }),
                },
                timestamp: cosmic_time,
                priority: 0.8,
                cascade_potential: 0.9,
                synchronization_requirements: Vec::new(),
            };
            self.event_bus.pending_events.push_back(consciousness_event);
        }

        // Generate user interaction events
        if user_interaction_intensity > 0.5 {
            let interaction_event = SystemEvent {
                event_id: format!("user_interaction_{}", cosmic_time as u64),
                event_type: EventType::UserInteraction,
                source_system: SystemComponent::UserInterface,
                target_systems: vec![
                    SystemComponent::CoEvolutionEngine,
                    SystemComponent::ConsciousnessFramework,
                    SystemComponent::DigitalEcosystem,
                ],
                event_data: EventData {
                    primary_value: user_interaction_intensity,
                    secondary_values: HashMap::new(),
                    vector_data: HashMap::new(),
                    entity_references: Vec::new(),
                    spatial_information: None,
                    temporal_information: Some(TemporalEventInfo {
                        duration: 1.0,
                        frequency: None,
                        phase_offset: 0.0,
                        temporal_decay: 0.5,
                    }),
                },
                timestamp: cosmic_time,
                priority: 0.7,
                cascade_potential: 0.6,
                synchronization_requirements: Vec::new(),
            };
            self.event_bus.pending_events.push_back(interaction_event);
        }
    }

    pub fn trigger_beat_cascade(&mut self, beat_intensity: f32, cosmic_time: f64) {
        // Manually trigger a beat drop cascade for synchronized effects
        let cascade_event = SystemEvent {
            event_id: format!("manual_beat_cascade_{}", cosmic_time as u64),
            event_type: EventType::BeatDrop,
            source_system: SystemComponent::BeatEngine,
            target_systems: vec![
                SystemComponent::VisualizationEngine,
                SystemComponent::RealityEngine,
                SystemComponent::ConsciousnessFramework,
                SystemComponent::CommunicationSystems,
                SystemComponent::DigitalEcosystem,
            ],
            event_data: EventData {
                primary_value: beat_intensity,
                secondary_values: HashMap::new(),
                vector_data: HashMap::new(),
                entity_references: Vec::new(),
                spatial_information: None,
                temporal_information: Some(TemporalEventInfo {
                    duration: 1.0,
                    frequency: Some(1.0),
                    phase_offset: 0.0,
                    temporal_decay: 0.7,
                }),
            },
            timestamp: cosmic_time,
            priority: 1.0,
            cascade_potential: 1.0,
            synchronization_requirements: vec![
                SynchronizationRequirement {
                    requirement_type: SyncType::Immediate,
                    target_system: SystemComponent::VisualizationEngine,
                    synchronization_tolerance: 0.01,
                    priority: 1.0,
                }
            ],
        };

        self.event_bus.pending_events.push_front(cascade_event);
    }
}

impl EventBus {
    fn new() -> Self {
        let mut event_priorities = HashMap::new();
        event_priorities.insert(EventType::BeatDrop, 1.0);
        event_priorities.insert(EventType::ConsciousnessSpike, 0.9);
        event_priorities.insert(EventType::UserInteraction, 0.8);
        event_priorities.insert(EventType::RealityDistortion, 0.7);

        Self {
            pending_events: VecDeque::new(),
            active_events: HashMap::new(),
            event_priorities,
            propagation_rules: HashMap::new(),
            event_filtering: EventFiltering::new(),
        }
    }

    fn process_events(&mut self, dt: f32, cascade_engine: &mut CascadeEngine,
                     sync_matrix: &mut SynchronizationMatrix) {
        // Process pending events
        let mut events_to_activate = Vec::new();

        while let Some(event) = self.pending_events.pop_front() {
            if self.event_filtering.should_allow_event(&event) {
                events_to_activate.push(event);
            }

            // Limit processing per frame for performance
            if events_to_activate.len() >= 10 {
                break;
            }
        }

        // Activate approved events
        for event in events_to_activate {
            let active_event = ActiveEvent {
                base_event: event.clone(),
                current_phase: EventPhase::Initialization,
                affected_systems: HashMap::new(),
                cascade_events: Vec::new(),
                resonance_buildup: 0.0,
                synchronization_status: HashMap::new(),
            };

            self.active_events.insert(event.event_id.clone(), active_event);

            // Trigger cascades if applicable
            if event.cascade_potential > 0.5 {
                cascade_engine.trigger_cascade(&event);
            }
        }

        // Update active events
        let mut completed_events = Vec::new();
        for (event_id, active_event) in &mut self.active_events {
            active_event.update(dt);

            if active_event.is_completed() {
                completed_events.push(event_id.clone());
            }
        }

        // Remove completed events
        for event_id in completed_events {
            self.active_events.remove(&event_id);
        }
    }
}

impl EventFiltering {
    fn new() -> Self {
        Self {
            safety_filters: vec![
                SafetyFilter {
                    filter_type: SafetyFilterType::FlashRateLimit,
                    threshold: 3.0,
                    action: SafetyAction::Attenuate(0.5),
                },
                SafetyFilter {
                    filter_type: SafetyFilterType::IntensitySpike,
                    threshold: 0.9,
                    action: SafetyAction::Attenuate(0.8),
                },
            ],
            performance_filters: vec![
                PerformanceFilter {
                    max_simultaneous_events: 20,
                    priority_threshold: 0.3,
                    resource_limits: HashMap::new(),
                }
            ],
            coherence_filters: vec![
                CoherenceFilter {
                    coherence_threshold: 0.7,
                    interference_management: InterferenceManagement {
                        destructive_interference_threshold: 0.8,
                        constructive_amplification_limit: 2.0,
                        phase_alignment_tolerance: 0.1,
                    },
                    pattern_preservation: PatternPreservation {
                        maintain_rhythmic_integrity: true,
                        preserve_consciousness_patterns: true,
                        maintain_communication_coherence: true,
                    },
                }
            ],
        }
    }

    fn should_allow_event(&self, event: &SystemEvent) -> bool {
        // Apply safety filters
        for filter in &self.safety_filters {
            if !filter.allows_event(event) {
                return false;
            }
        }

        // Apply performance filters
        for filter in &self.performance_filters {
            if !filter.allows_event(event) {
                return false;
            }
        }

        // Apply coherence filters
        for filter in &self.coherence_filters {
            if !filter.allows_event(event) {
                return false;
            }
        }

        true
    }
}

impl SafetyFilter {
    fn allows_event(&self, event: &SystemEvent) -> bool {
        match self.filter_type {
            SafetyFilterType::FlashRateLimit => {
                // Simple flash rate check
                event.event_data.primary_value < self.threshold
            }
            SafetyFilterType::IntensitySpike => {
                event.event_data.primary_value < self.threshold
            }
            _ => true,
        }
    }
}

impl PerformanceFilter {
    fn allows_event(&self, _event: &SystemEvent) -> bool {
        // Simple performance check - always allow for now
        true
    }
}

impl CoherenceFilter {
    fn allows_event(&self, _event: &SystemEvent) -> bool {
        // Simple coherence check - always allow for now
        true
    }
}

impl ActiveEvent {
    fn update(&mut self, dt: f32) {
        // Update event phase
        match self.current_phase {
            EventPhase::Initialization => {
                self.current_phase = EventPhase::Propagation;
            }
            EventPhase::Propagation => {
                self.resonance_buildup += dt;
                if self.resonance_buildup > 0.5 {
                    self.current_phase = EventPhase::Peak;
                }
            }
            EventPhase::Peak => {
                if let Some(duration) = self.base_event.event_data.temporal_information.as_ref() {
                    if self.resonance_buildup > duration.duration * 0.8 {
                        self.current_phase = EventPhase::Decay;
                    }
                }
            }
            EventPhase::Decay => {
                self.resonance_buildup -= dt * 2.0; // Faster decay
                if self.resonance_buildup <= 0.0 {
                    self.current_phase = EventPhase::Completion;
                }
            }
            EventPhase::Completion => {}
        }
    }

    fn is_completed(&self) -> bool {
        matches!(self.current_phase, EventPhase::Completion)
    }
}

impl CascadeEngine {
    fn new() -> Self {
        let mut cascade_rules = HashMap::new();

        // Beat drop cascade rule
        cascade_rules.insert(EventType::BeatDrop, CascadeRule {
            trigger_event: EventType::BeatDrop,
            cascade_targets: vec![
                CascadeTarget {
                    target_system: SystemComponent::VisualizationEngine,
                    effect_type: CascadeEffectType::Amplification,
                    magnitude_multiplier: 1.5,
                    propagation_probability: 0.9,
                },
                CascadeTarget {
                    target_system: SystemComponent::RealityEngine,
                    effect_type: CascadeEffectType::Resonance,
                    magnitude_multiplier: 1.2,
                    propagation_probability: 0.8,
                },
            ],
            cascade_delay: 0.1,
            amplification_factor: 1.3,
            decay_rate: 0.5,
            maximum_iterations: 3,
        });

        Self {
            cascade_rules,
            active_cascades: Vec::new(),
            cascade_intensity_matrix: Vec::new(),
            feedback_loops: Vec::new(),
            resonance_amplifiers: Vec::new(),
        }
    }

    fn trigger_cascade(&mut self, event: &SystemEvent) {
        if let Some(rule) = self.cascade_rules.get(&event.event_type) {
            let cascade = ActiveCascade {
                cascade_id: format!("cascade_{}", event.event_id),
                source_event: event.event_id.clone(),
                current_iteration: 0,
                affected_systems: HashMap::new(),
                cascade_strength: event.cascade_potential,
                propagation_front: Vec::new(),
            };

            self.active_cascades.push(cascade);
        }
    }

    fn update(&mut self, dt: f32, event_bus: &mut EventBus) {
        for cascade in &mut self.active_cascades {
            cascade.update(dt, event_bus);
        }

        // Remove completed cascades
        self.active_cascades.retain(|cascade| !cascade.is_completed());
    }
}

impl ActiveCascade {
    fn update(&mut self, dt: f32, _event_bus: &mut EventBus) {
        self.cascade_strength *= 0.99; // Gradual decay
    }

    fn is_completed(&self) -> bool {
        self.cascade_strength < 0.1
    }
}

impl SynchronizationMatrix {
    fn new() -> Self {
        Self {
            system_synchronization: HashMap::new(),
            global_synchronization_state: 0.5,
            rhythm_coherence: 0.7,
            phase_alignments: HashMap::new(),
            synchronization_strength: 0.6,
        }
    }

    fn update(&mut self, dt: f32, active_events: &HashMap<String, ActiveEvent>) {
        // Update global synchronization based on active events
        let event_count = active_events.len() as f32;
        let sync_influence = (event_count * 0.1).min(1.0);

        self.global_synchronization_state = (self.global_synchronization_state + sync_influence * dt).min(1.0);

        // Update rhythm coherence
        self.rhythm_coherence += dt * 0.01;
        self.rhythm_coherence = self.rhythm_coherence.min(1.0);

        // Update synchronization strength
        self.synchronization_strength = self.global_synchronization_state * self.rhythm_coherence;
    }
}

impl EventHistory {
    fn new() -> Self {
        Self {
            recent_events: VecDeque::new(),
            pattern_recognition: PatternRecognition::new(),
            emergence_tracking: EmergenceTracking::new(),
            performance_metrics: PerformanceMetrics::new(),
        }
    }

    fn update(&mut self, dt: f32, active_events: &HashMap<String, ActiveEvent>) {
        // Update performance metrics
        self.performance_metrics.update(dt, active_events);

        // Update pattern recognition
        self.pattern_recognition.update(dt, active_events);

        // Update emergence tracking
        self.emergence_tracking.update(dt, active_events);
    }
}

impl PatternRecognition {
    fn new() -> Self {
        Self {
            recurring_patterns: HashMap::new(),
            pattern_prediction_accuracy: 0.5,
            learned_correlations: HashMap::new(),
        }
    }

    fn update(&mut self, dt: f32, _active_events: &HashMap<String, ActiveEvent>) {
        // Update pattern prediction accuracy
        self.pattern_prediction_accuracy += dt * 0.001;
        self.pattern_prediction_accuracy = self.pattern_prediction_accuracy.min(1.0);
    }
}

impl EmergenceTracking {
    fn new() -> Self {
        Self {
            emergence_indicators: HashMap::new(),
            threshold_monitoring: HashMap::new(),
            emergence_prediction: EmergencePrediction::new(),
        }
    }

    fn update(&mut self, dt: f32, _active_events: &HashMap<String, ActiveEvent>) {
        // Update emergence indicators
        for indicator in self.emergence_indicators.values_mut() {
            *indicator += dt * 0.01;
            *indicator = indicator.min(1.0);
        }
    }
}

impl EmergencePrediction {
    fn new() -> Self {
        Self {
            predicted_events: Vec::new(),
            confidence_levels: HashMap::new(),
            temporal_predictions: HashMap::new(),
        }
    }
}

impl PerformanceMetrics {
    fn new() -> Self {
        Self {
            event_processing_time: VecDeque::new(),
            cascade_efficiency: 0.8,
            synchronization_quality: 0.7,
            system_responsiveness: HashMap::new(),
        }
    }

    fn update(&mut self, dt: f32, _active_events: &HashMap<String, ActiveEvent>) {
        // Record processing time
        self.event_processing_time.push_back(dt as f64);

        // Keep only recent measurements
        while self.event_processing_time.len() > 100 {
            self.event_processing_time.pop_front();
        }

        // Update efficiency metrics
        self.cascade_efficiency += dt * 0.001;
        self.cascade_efficiency = self.cascade_efficiency.min(1.0);
    }
}

impl SystemResonance {
    fn new() -> Self {
        Self {
            global_resonance_state: 0.5,
            harmonic_frequencies: vec![1.0, 2.0, 4.0, 8.0], // Base harmonics
            resonance_nodes: Vec::new(),
            interference_patterns: Vec::new(),
            coherence_field: CoherenceField::new(),
        }
    }

    fn update(&mut self, dt: f32, beat_intensity: f32, consciousness_level: f32, cosmic_time: f64) {
        // Update global resonance
        self.global_resonance_state = (beat_intensity + consciousness_level) * 0.5;

        // Update harmonic frequencies based on beat
        for freq in &mut self.harmonic_frequencies {
            *freq += dt * beat_intensity * 0.1;
        }

        // Update coherence field
        self.coherence_field.update(dt, beat_intensity, consciousness_level);
    }
}

impl CoherenceField {
    fn new() -> Self {
        Self {
            field_strength: 0.5,
            coherence_gradient: HashMap::new(),
            stability_regions: Vec::new(),
            turbulence_zones: Vec::new(),
        }
    }

    fn update(&mut self, dt: f32, beat_intensity: f32, consciousness_level: f32) {
        // Update field strength based on system coherence
        self.field_strength = (beat_intensity * consciousness_level).sqrt();

        // Update stability regions
        for region in &mut self.stability_regions {
            region.stability_level += dt * 0.01;
            region.stability_level = region.stability_level.min(1.0);
        }
    }
}

impl UserCoEvolutionSystem {
    pub fn new() -> Self {
        Self {
            interaction_learning: InteractionLearning::new(),
            adaptation_engine: AdaptationEngine::new(),
            preference_memory: PreferenceMemory::new(),
            evolution_pathways: EvolutionPathways::new(),
            personalization_matrix: PersonalizationMatrix::new(),
        }
    }

    pub fn update(&mut self, dt: f32, user_interaction_intensity: f32, system_state: &HashMap<String, f32>,
                  cosmic_time: f64) {
        // Update interaction learning
        self.interaction_learning.update(dt, user_interaction_intensity, system_state, cosmic_time);

        // Update adaptation engine
        self.adaptation_engine.update(dt, &self.interaction_learning, &self.preference_memory);

        // Update preference memory
        self.preference_memory.update(dt, &self.interaction_learning);

        // Update evolution pathways
        self.evolution_pathways.update(dt, &self.interaction_learning, &self.adaptation_engine);

        // Update personalization matrix
        self.personalization_matrix.update(dt, &self.preference_memory, &self.evolution_pathways);
    }

    pub fn record_user_action(&mut self, action: UserAction) {
        self.interaction_learning.record_action(action);
    }

    pub fn get_adaptation_parameters(&self) -> HashMap<String, f32> {
        self.adaptation_engine.get_current_parameters()
    }

    pub fn get_personalization_factor(&self, dimension: &str) -> f32 {
        self.personalization_matrix.get_factor(dimension)
    }
}

impl InteractionLearning {
    fn new() -> Self {
        Self {
            interaction_patterns: HashMap::new(),
            behavioral_clustering: BehavioralClustering::new(),
            preference_inference: PreferenceInference::new(),
            engagement_tracking: EngagementTracking::new(),
            learning_algorithms: LearningAlgorithms::new(),
        }
    }

    fn update(&mut self, dt: f32, user_interaction_intensity: f32,
              system_state: &HashMap<String, f32>, cosmic_time: f64) {
        // Update engagement tracking
        self.engagement_tracking.update(dt, user_interaction_intensity, system_state);

        // Update behavioral clustering
        self.behavioral_clustering.update(dt, &self.interaction_patterns);

        // Update preference inference
        self.preference_inference.update(dt, &self.interaction_patterns, &self.engagement_tracking);

        // Update learning algorithms
        self.learning_algorithms.update(dt, user_interaction_intensity);
    }

    fn record_action(&mut self, action: UserAction) {
        // Create or update interaction pattern
        let pattern_id = format!("{:?}_{}", action.action_type, action.timestamp as u64 / 10);

        let pattern = self.interaction_patterns.entry(pattern_id.clone())
            .or_insert_with(|| InteractionPattern {
                pattern_id,
                interaction_sequence: Vec::new(),
                temporal_characteristics: TemporalCharacteristics::new(),
                preference_indicators: HashMap::new(),
                engagement_level: 0.5,
                pattern_frequency: 0.0,
                pattern_evolution: PatternEvolution::new(),
            });

        pattern.interaction_sequence.push(action);
        pattern.pattern_frequency += 1.0;

        // Limit sequence length for performance
        if pattern.interaction_sequence.len() > 50 {
            pattern.interaction_sequence.remove(0);
        }
    }
}

impl TemporalCharacteristics {
    fn new() -> Self {
        Self {
            action_intervals: Vec::new(),
            session_duration_preference: 5.0, // minutes
            interaction_density: 0.5,
            rhythm_alignment: 0.7,
            temporal_clustering: TemporalClustering::new(),
        }
    }
}

impl TemporalClustering {
    fn new() -> Self {
        Self {
            burst_patterns: Vec::new(),
            sustained_engagement_periods: Vec::new(),
            pause_patterns: Vec::new(),
        }
    }
}

impl PatternEvolution {
    fn new() -> Self {
        Self {
            evolution_rate: 0.1,
            adaptation_direction: vec![0.0, 0.0, 0.0],
            stability_factors: HashMap::new(),
            innovation_tendency: 0.3,
        }
    }
}

impl BehavioralClustering {
    fn new() -> Self {
        Self {
            clusters: HashMap::new(),
            cluster_transitions: HashMap::new(),
            current_cluster: None,
            cluster_stability: 0.7,
        }
    }

    fn update(&mut self, dt: f32, _patterns: &HashMap<String, InteractionPattern>) {
        // Update cluster stability
        self.cluster_stability += dt * 0.01;
        self.cluster_stability = self.cluster_stability.min(1.0);
    }
}

impl PreferenceInference {
    fn new() -> Self {
        Self {
            inferred_preferences: HashMap::new(),
            confidence_levels: HashMap::new(),
            preference_evolution: HashMap::new(),
            inference_accuracy: 0.6,
        }
    }

    fn update(&mut self, dt: f32, _patterns: &HashMap<String, InteractionPattern>,
              _engagement: &EngagementTracking) {
        // Update inference accuracy
        self.inference_accuracy += dt * 0.001;
        self.inference_accuracy = self.inference_accuracy.min(1.0);
    }
}

impl EngagementTracking {
    fn new() -> Self {
        Self {
            engagement_metrics: EngagementMetrics::new(),
            engagement_patterns: Vec::new(),
            flow_state_indicators: FlowStateIndicators::new(),
            attention_mapping: AttentionMapping::new(),
        }
    }

    fn update(&mut self, dt: f32, user_interaction_intensity: f32,
              _system_state: &HashMap<String, f32>) {
        // Update engagement metrics
        self.engagement_metrics.update(dt, user_interaction_intensity);

        // Update flow state indicators
        self.flow_state_indicators.update(dt, user_interaction_intensity);

        // Update attention mapping
        self.attention_mapping.update(dt);
    }
}

impl EngagementMetrics {
    fn new() -> Self {
        Self {
            overall_engagement: 0.5,
            visual_engagement: 0.5,
            interactive_engagement: 0.5,
            temporal_engagement: 0.5,
            cognitive_load: 0.3,
        }
    }

    fn update(&mut self, dt: f32, user_interaction_intensity: f32) {
        // Update engagement based on interaction intensity
        self.overall_engagement = (self.overall_engagement + user_interaction_intensity * dt * 0.1).min(1.0);
        self.interactive_engagement = user_interaction_intensity;

        // Adjust cognitive load based on engagement
        self.cognitive_load = (self.overall_engagement * 0.7).max(0.1);
    }
}

impl FlowStateIndicators {
    fn new() -> Self {
        Self {
            flow_probability: 0.3,
            challenge_balance: 0.5,
            skill_utilization: 0.6,
            attention_focus: 0.7,
            intrinsic_motivation: 0.8,
        }
    }

    fn update(&mut self, dt: f32, user_interaction_intensity: f32) {
        // Update flow probability based on sustained interaction
        if user_interaction_intensity > 0.7 {
            self.flow_probability += dt * 0.1;
        } else {
            self.flow_probability -= dt * 0.05;
        }
        self.flow_probability = self.flow_probability.clamp(0.0, 1.0);

        // Update attention focus
        self.attention_focus = (self.attention_focus + user_interaction_intensity * dt * 0.05).min(1.0);
    }
}

impl AttentionMapping {
    fn new() -> Self {
        Self {
            attention_zones: HashMap::new(),
            attention_transitions: Vec::new(),
            focus_sustainability: 0.6,
            distraction_resilience: 0.7,
        }
    }

    fn update(&mut self, dt: f32) {
        // Update focus sustainability
        self.focus_sustainability += dt * 0.01;
        self.focus_sustainability = self.focus_sustainability.min(1.0);
    }
}

impl LearningAlgorithms {
    fn new() -> Self {
        Self {
            pattern_recognition: PatternRecognitionAlgorithm::new(),
            preference_learning: PreferenceLearningAlgorithm::new(),
            adaptation_optimization: AdaptationOptimization::new(),
            feedback_integration: FeedbackIntegration::new(),
        }
    }

    fn update(&mut self, dt: f32, user_interaction_intensity: f32) {
        // Update learning rates based on interaction
        self.pattern_recognition.learning_rate = 0.1 + user_interaction_intensity * 0.05;
        self.preference_learning.learning_momentum = 0.9 + user_interaction_intensity * 0.05;
    }
}

impl PatternRecognitionAlgorithm {
    fn new() -> Self {
        Self {
            algorithm_type: "gradient_descent".to_string(),
            learning_rate: 0.1,
            pattern_sensitivity: 0.7,
            noise_tolerance: 0.3,
            adaptation_speed: 0.5,
        }
    }
}

impl PreferenceLearningAlgorithm {
    fn new() -> Self {
        Self {
            preference_weights: HashMap::new(),
            learning_momentum: 0.9,
            forgetting_factor: 0.95,
            confidence_threshold: 0.7,
        }
    }
}

impl AdaptationOptimization {
    fn new() -> Self {
        Self {
            optimization_target: OptimizationTarget::MaximizeEngagement,
            constraint_handling: ConstraintHandling::new(),
            exploration_exploitation_balance: 0.3,
            convergence_criteria: ConvergenceCriteria::new(),
        }
    }
}

impl ConstraintHandling {
    fn new() -> Self {
        Self {
            safety_constraints: vec![
                SafetyConstraint {
                    constraint_type: "flash_rate".to_string(),
                    threshold: 3.0,
                    enforcement_priority: 1.0,
                },
                SafetyConstraint {
                    constraint_type: "luminance_change".to_string(),
                    threshold: 0.1,
                    enforcement_priority: 0.9,
                },
            ],
            performance_constraints: vec![
                PerformanceConstraint {
                    resource_type: "cpu_usage".to_string(),
                    limit: 0.8,
                    degradation_strategy: "reduce_complexity".to_string(),
                }
            ],
            user_defined_constraints: Vec::new(),
        }
    }
}

impl ConvergenceCriteria {
    fn new() -> Self {
        Self {
            stability_threshold: 0.05,
            improvement_threshold: 0.01,
            maximum_iterations: 1000,
            time_limit: 300.0, // 5 minutes
        }
    }
}

impl FeedbackIntegration {
    fn new() -> Self {
        Self {
            explicit_feedback: ExplicitFeedback::new(),
            implicit_feedback: ImplicitFeedback::new(),
            feedback_weighting: FeedbackWeighting::new(),
            feedback_validation: FeedbackValidation::new(),
        }
    }
}

impl ExplicitFeedback {
    fn new() -> Self {
        Self {
            feedback_mechanisms: Vec::new(),
            feedback_history: VecDeque::new(),
            feedback_processing: FeedbackProcessing::new(),
        }
    }
}

impl FeedbackProcessing {
    fn new() -> Self {
        Self {
            processing_delay: 0.1,
            noise_filtering: 0.8,
            context_weighting: 0.7,
            temporal_weighting: 0.9,
        }
    }
}

impl ImplicitFeedback {
    fn new() -> Self {
        Self {
            behavioral_indicators: HashMap::new(),
            physiological_proxies: HashMap::new(),
            interaction_quality_metrics: HashMap::new(),
            inference_confidence: 0.6,
        }
    }
}

impl FeedbackWeighting {
    fn new() -> Self {
        Self {
            explicit_weight: 0.7,
            implicit_weight: 0.3,
            recency_weighting: 0.8,
            confidence_weighting: 0.9,
        }
    }
}

impl FeedbackValidation {
    fn new() -> Self {
        Self {
            validation_methods: Vec::new(),
            consistency_checking: ConsistencyChecking::new(),
            outlier_detection: OutlierDetection::new(),
        }
    }
}

impl ConsistencyChecking {
    fn new() -> Self {
        Self {
            temporal_consistency: 0.8,
            contextual_consistency: 0.7,
            behavioral_consistency: 0.9,
        }
    }
}

impl OutlierDetection {
    fn new() -> Self {
        Self {
            detection_threshold: 2.0, // Standard deviations
            outlier_handling_strategy: "quarantine".to_string(),
            false_positive_rate: 0.05,
        }
    }
}

impl AdaptationEngine {
    fn new() -> Self {
        let mut adaptation_strategies = HashMap::new();

        adaptation_strategies.insert("visual_complexity".to_string(), AdaptationStrategy {
            strategy_name: "visual_complexity".to_string(),
            adaptation_scope: AdaptationScope::Visual,
            adaptation_parameters: HashMap::new(),
            trigger_conditions: Vec::new(),
            effectiveness_history: VecDeque::new(),
        });

        Self {
            adaptation_strategies,
            active_adaptations: Vec::new(),
            adaptation_effectiveness: AdaptationEffectiveness::new(),
            adaptation_constraints: AdaptationConstraints::new(),
        }
    }

    fn update(&mut self, dt: f32, _interaction_learning: &InteractionLearning,
              _preference_memory: &PreferenceMemory) {
        // Update active adaptations
        for adaptation in &mut self.active_adaptations {
            adaptation.update(dt);
        }

        // Remove completed adaptations
        self.active_adaptations.retain(|adaptation| !adaptation.is_completed());

        // Update effectiveness tracking
        self.adaptation_effectiveness.update(dt);
    }

    fn get_current_parameters(&self) -> HashMap<String, f32> {
        let mut parameters = HashMap::new();

        for adaptation in &self.active_adaptations {
            for (key, value) in &adaptation.target_parameters {
                parameters.insert(key.clone(), *value);
            }
        }

        parameters
    }
}

impl ActiveAdaptation {
    fn update(&mut self, dt: f32) {
        self.current_progress += dt * self.adaptation_rate;
        self.current_progress = self.current_progress.min(1.0);
    }

    fn is_completed(&self) -> bool {
        self.current_progress >= 1.0
    }
}

impl AdaptationEffectiveness {
    fn new() -> Self {
        Self {
            strategy_performance: HashMap::new(),
            user_satisfaction_correlation: HashMap::new(),
            adaptation_success_rate: 0.7,
            optimization_convergence: 0.5,
        }
    }

    fn update(&mut self, dt: f32) {
        // Update success rate
        self.adaptation_success_rate += dt * 0.001;
        self.adaptation_success_rate = self.adaptation_success_rate.min(1.0);
    }
}

impl AdaptationConstraints {
    fn new() -> Self {
        let mut rate_limits = HashMap::new();
        rate_limits.insert("visual_change_rate".to_string(), 0.1);
        rate_limits.insert("complexity_change_rate".to_string(), 0.05);

        let mut stability_requirements = HashMap::new();
        stability_requirements.insert("minimum_coherence".to_string(), 0.3);

        let mut user_comfort_bounds = HashMap::new();
        user_comfort_bounds.insert("visual_intensity".to_string(), (0.0, 1.0));
        user_comfort_bounds.insert("flash_rate".to_string(), (0.0, 3.0));

        let mut system_performance_limits = HashMap::new();
        system_performance_limits.insert("cpu_usage".to_string(), 0.8);
        system_performance_limits.insert("memory_usage".to_string(), 0.9);

        Self {
            rate_limits,
            stability_requirements,
            user_comfort_bounds,
            system_performance_limits,
        }
    }
}

impl PreferenceMemory {
    fn new() -> Self {
        Self {
            long_term_preferences: HashMap::new(),
            session_preferences: HashMap::new(),
            contextual_preferences: HashMap::new(),
            preference_persistence: PreferencePersistence::new(),
        }
    }

    fn update(&mut self, dt: f32, _interaction_learning: &InteractionLearning) {
        // Update preference persistence
        self.preference_persistence.update(dt);

        // Apply decay to preferences
        for preference in self.long_term_preferences.values_mut() {
            preference.apply_decay(dt, &self.preference_persistence);
        }
    }
}

impl LongTermPreference {
    fn apply_decay(&mut self, dt: f32, persistence: &PreferencePersistence) {
        let decay_rate = persistence.decay_rates.get("default").unwrap_or(&0.001);
        self.confidence_level *= 1.0 - (decay_rate * dt);
        self.confidence_level = self.confidence_level.max(0.0);
    }
}

impl PreferencePersistence {
    fn new() -> Self {
        let mut decay_rates = HashMap::new();
        decay_rates.insert("default".to_string(), 0.001);
        decay_rates.insert("visual_preference".to_string(), 0.0005);
        decay_rates.insert("interaction_preference".to_string(), 0.002);

        Self {
            persistence_strategy: PersistenceStrategy::ExponentialDecay,
            decay_rates,
            reinforcement_mechanisms: Vec::new(),
            memory_consolidation: MemoryConsolidation::new(),
        }
    }

    fn update(&mut self, dt: f32) {
        // Update memory consolidation
        self.memory_consolidation.update(dt);
    }
}

impl MemoryConsolidation {
    fn new() -> Self {
        Self {
            consolidation_intervals: vec![1.0, 5.0, 30.0], // 1 min, 5 min, 30 min
            consolidation_strength: 0.8,
            interference_resistance: 0.7,
            long_term_integration_rate: 0.1,
        }
    }

    fn update(&mut self, dt: f32) {
        // Update consolidation strength
        self.consolidation_strength += dt * 0.001;
        self.consolidation_strength = self.consolidation_strength.min(1.0);
    }
}

impl EvolutionPathways {
    fn new() -> Self {
        let mut available_pathways = HashMap::new();

        available_pathways.insert("visual_mastery".to_string(), EvolutionPathway {
            pathway_id: "visual_mastery".to_string(),
            pathway_description: "Mastery of visual complexity and aesthetic preferences".to_string(),
            evolution_stages: vec![
                EvolutionStage {
                    stage_id: "basic_visual".to_string(),
                    stage_description: "Basic visual preference learning".to_string(),
                    required_adaptations: vec!["color_preference".to_string()],
                    stage_duration_estimate: 10.0,
                    complexity_level: 0.2,
                    user_readiness_requirements: vec!["engagement>0.5".to_string()],
                },
                EvolutionStage {
                    stage_id: "advanced_visual".to_string(),
                    stage_description: "Advanced visual complexity adaptation".to_string(),
                    required_adaptations: vec!["complexity_adaptation".to_string()],
                    stage_duration_estimate: 30.0,
                    complexity_level: 0.6,
                    user_readiness_requirements: vec!["engagement>0.7".to_string()],
                },
            ],
            current_stage: 0,
            progression_criteria: Vec::new(),
            pathway_benefits: HashMap::new(),
        });

        Self {
            available_pathways,
            current_pathway: Some("visual_mastery".to_string()),
            pathway_transitions: HashMap::new(),
            evolution_momentum: 0.1,
        }
    }

    fn update(&mut self, dt: f32, _interaction_learning: &InteractionLearning,
              _adaptation_engine: &AdaptationEngine) {
        // Update evolution momentum
        self.evolution_momentum += dt * 0.01;
        self.evolution_momentum = self.evolution_momentum.min(1.0);

        // Update current pathway
        if let Some(current_id) = &self.current_pathway {
            if let Some(pathway) = self.available_pathways.get_mut(current_id) {
                pathway.update_progression(dt);
            }
        }
    }
}

impl EvolutionPathway {
    fn update_progression(&mut self, dt: f32) {
        // Update progression criteria
        for criterion in &mut self.progression_criteria {
            criterion.current_progress += dt * 0.01;
            criterion.current_progress = criterion.current_progress.min(criterion.threshold);
        }
    }
}

impl PersonalizationMatrix {
    fn new() -> Self {
        let mut personalization_dimensions = HashMap::new();

        personalization_dimensions.insert("visual_complexity".to_string(), PersonalizationDimension {
            dimension_name: "visual_complexity".to_string(),
            current_value: 0.5,
            optimal_range: (0.3, 0.8),
            adaptation_sensitivity: 0.7,
            user_control_level: 0.3,
            system_influence_level: 0.7,
        });

        personalization_dimensions.insert("interaction_responsiveness".to_string(), PersonalizationDimension {
            dimension_name: "interaction_responsiveness".to_string(),
            current_value: 0.8,
            optimal_range: (0.5, 1.0),
            adaptation_sensitivity: 0.9,
            user_control_level: 0.5,
            system_influence_level: 0.5,
        });

        Self {
            personalization_dimensions,
            cross_dimensional_interactions: HashMap::new(),
            personalization_effectiveness: 0.6,
            adaptation_responsiveness: 0.7,
        }
    }

    fn update(&mut self, dt: f32, _preference_memory: &PreferenceMemory,
              _evolution_pathways: &EvolutionPathways) {
        // Update personalization effectiveness
        self.personalization_effectiveness += dt * 0.001;
        self.personalization_effectiveness = self.personalization_effectiveness.min(1.0);

        // Update adaptation responsiveness
        self.adaptation_responsiveness += dt * 0.001;
        self.adaptation_responsiveness = self.adaptation_responsiveness.min(1.0);
    }

    fn get_factor(&self, dimension: &str) -> f32 {
        self.personalization_dimensions.get(dimension)
            .map(|dim| dim.current_value)
            .unwrap_or(0.5)
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

    // CRITICAL SAFETY SYSTEMS - EPILEPSY PROTECTION
    safety_config: SafetyConfig,
    flash_tracker: FlashTracker,
    emergency_stop_requested: bool,
    previous_llama_colors: Vec<Vec3>, // Track previous colors for luminance limiting
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
            vertex_buffer,
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

            // CRITICAL SAFETY SYSTEMS - EPILEPSY PROTECTION
            safety_config: SafetyConfig::default(),
            flash_tracker: FlashTracker::new(),
            emergency_stop_requested: false,
            previous_llama_colors: Vec::new(),
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
                action_type: ActionType::Click,
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

        // Ensure we have color tracking for all llamas
        while self.previous_llama_colors.len() < self.llamas.len() {
            self.previous_llama_colors.push(Vec3::new(0.1, 0.1, 0.1)); // Safe default
        }

        // Generate vertices for all llamas with Phase 2 species-enhanced visuals AND SAFETY FILTERING
        let mut vertices = Vec::new();
        for (llama_id, llama) in self.llamas.iter().enumerate() {
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

            // Crystal rendered as a diamond shape
            vertices.extend([
                Vertex { position: [x, y - s, 0.0], color: crystal_color_array },
                Vertex { position: [x + s, y, 0.0], color: crystal_color_array },
                Vertex { position: [x, y + s, 0.0], color: crystal_color_array },
                Vertex { position: [x, y - s, 0.0], color: crystal_color_array },
                Vertex { position: [x - s, y, 0.0], color: crystal_color_array },
                Vertex { position: [x, y + s, 0.0], color: crystal_color_array },
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
                        Vertex { position: [x, y, 0.0], color: radius_color },
                        Vertex { position: [x + angle1.cos() * radius_size, y + angle1.sin() * radius_size, 0.0], color: radius_color },
                        Vertex { position: [x + angle2.cos() * radius_size, y + angle2.sin() * radius_size, 0.0], color: radius_color },
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

            vertices.extend([
                Vertex { position: [x - s, y - offset1, 0.0], color: tear_color_array },
                Vertex { position: [x + s, y + offset2, 0.0], color: tear_color_array },
                Vertex { position: [x - offset2, y + s, 0.0], color: tear_color_array },
                Vertex { position: [x + offset1, y - s, 0.0], color: tear_color_array },
                Vertex { position: [x + s, y - offset1, 0.0], color: tear_color_array },
                Vertex { position: [x - s, y + offset2, 0.0], color: tear_color_array },
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
                    Vertex { position: [x, y, 0.0], color: zone_color_array },
                    Vertex { position: [x + angle1.cos() * r, y + angle1.sin() * r, 0.0], color: zone_color_array },
                    Vertex { position: [x + angle2.cos() * r, y + angle2.sin() * r, 0.0], color: zone_color_array },
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
                                Vertex { position: [x, y, 0.0], color: comm_color_array },
                                Vertex { position: [x + angle1.cos() * s, y + angle1.sin() * s, 0.0], color: comm_color_array },
                                Vertex { position: [x + angle2.cos() * s, y + angle2.sin() * s, 0.0], color: comm_color_array },
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
                                Vertex { position: [x + angle1.cos() * s, y + angle1.sin() * s, 0.0], color: ring_color },
                                Vertex { position: [x + angle1.cos() * outer_s, y + angle1.sin() * outer_s, 0.0], color: ring_color },
                                Vertex { position: [x + angle2.cos() * outer_s, y + angle2.sin() * outer_s, 0.0], color: ring_color },
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
                                        Vertex { position: [x1 + perpx, y1 + perpy, 0.0], color: comm_color_array },
                                        Vertex { position: [x1 - perpx, y1 - perpy, 0.0], color: comm_color_array },
                                        Vertex { position: [x2 + perpx, y2 + perpy, 0.0], color: comm_color_array },
                                        Vertex { position: [x1 - perpx, y1 - perpy, 0.0], color: comm_color_array },
                                        Vertex { position: [x2 - perpx, y2 - perpy, 0.0], color: comm_color_array },
                                        Vertex { position: [x2 + perpx, y2 + perpy, 0.0], color: comm_color_array },
                                    ]);
                                }
                            }
                        }
                    }
                    _ => {
                        // Default: simple diamond shape for other types
                        vertices.extend([
                            Vertex { position: [x, y + s, 0.0], color: comm_color_array },     // Top
                            Vertex { position: [x - s, y, 0.0], color: comm_color_array },     // Left
                            Vertex { position: [x + s, y, 0.0], color: comm_color_array },     // Right
                            Vertex { position: [x, y + s, 0.0], color: comm_color_array },     // Top
                            Vertex { position: [x + s, y, 0.0], color: comm_color_array },     // Right
                            Vertex { position: [x, y - s, 0.0], color: comm_color_array },     // Bottom
                            Vertex { position: [x, y - s, 0.0], color: comm_color_array },     // Bottom
                            Vertex { position: [x - s, y, 0.0], color: comm_color_array },     // Left
                            Vertex { position: [x, y + s, 0.0], color: comm_color_array },     // Top
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
                            Vertex { position: [x1 - norm_x, y1 - norm_y, 0.0], color: final_connection_color },
                            Vertex { position: [x1 + norm_x, y1 + norm_y, 0.0], color: final_connection_color },
                            Vertex { position: [x2 - norm_x, y2 - norm_y, 0.0], color: final_connection_color },
                            Vertex { position: [x1 + norm_x, y1 + norm_y, 0.0], color: final_connection_color },
                            Vertex { position: [x2 + norm_x, y2 + norm_y, 0.0], color: final_connection_color },
                            Vertex { position: [x2 - norm_x, y2 - norm_y, 0.0], color: final_connection_color },
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
                Vertex { position: [center_x - center_size, center_y - center_size, 0.0], color: center_color },
                Vertex { position: [center_x + center_size, center_y - center_size, 0.0], color: center_color },
                Vertex { position: [center_x - center_size, center_y + center_size, 0.0], color: center_color },
                Vertex { position: [center_x + center_size, center_y - center_size, 0.0], color: center_color },
                Vertex { position: [center_x + center_size, center_y + center_size, 0.0], color: center_color },
                Vertex { position: [center_x - center_size, center_y + center_size, 0.0], color: center_color },
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
                        Vertex { position: [x1 - norm_x, y1 - norm_y, 0.0], color: beam_color },
                        Vertex { position: [x1 + norm_x, y1 + norm_y, 0.0], color: beam_color },
                        Vertex { position: [x2 - norm_x, y2 - norm_y, 0.0], color: beam_color },
                        Vertex { position: [x1 + norm_x, y1 + norm_y, 0.0], color: beam_color },
                        Vertex { position: [x2 + norm_x, y2 + norm_y, 0.0], color: beam_color },
                        Vertex { position: [x2 - norm_x, y2 - norm_y, 0.0], color: beam_color },
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
                    Vertex { position: [conflict_x, conflict_y, 0.0], color: war_color },
                    Vertex { position: [conflict_x + angle1.cos() * conflict_radius, conflict_y + angle1.sin() * conflict_radius, 0.0], color: war_color },
                    Vertex { position: [conflict_x + angle2.cos() * conflict_radius, conflict_y + angle2.sin() * conflict_radius, 0.0], color: war_color },
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
                Vertex { position: [obs_x, obs_y, 0.0], color: eye_color },
                Vertex { position: [obs_x + angle1.cos() * obs_size, obs_y + angle1.sin() * obs_size, 0.0], color: eye_color },
                Vertex { position: [obs_x + angle2.cos() * obs_size, obs_y + angle2.sin() * obs_size, 0.0], color: eye_color },
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
                    Vertex { position: [obs_x + angle.cos() * awareness_radius * 0.9, obs_y + angle.sin() * awareness_radius * 0.9, 0.0], color: awareness_color },
                    Vertex { position: [obs_x + angle.cos() * awareness_radius, obs_y + angle.sin() * awareness_radius, 0.0], color: awareness_color },
                    Vertex { position: [obs_x + next_angle.cos() * awareness_radius, obs_y + next_angle.sin() * awareness_radius, 0.0], color: awareness_color },
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
                            Vertex { position: [x + angle1.cos() * aura_size * 0.8, y + angle1.sin() * aura_size * 0.8, 0.0], color: final_aura_color },
                            Vertex { position: [x + angle1.cos() * aura_size, y + angle1.sin() * aura_size, 0.0], color: final_aura_color },
                            Vertex { position: [x + angle2.cos() * aura_size, y + angle2.sin() * aura_size, 0.0], color: final_aura_color },
                        ]);
                    }
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

    /// Handle keyboard input for emergency stop
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
                _ => {}
            }
        }
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

/// HSV to RGB conversion for Vec3 input
fn hsv_to_rgb_vec3(hsv: Vec3) -> Vec3 {
    hsv_to_rgb(hsv.x, hsv.y, hsv.z)
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