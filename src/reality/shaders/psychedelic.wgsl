// MINTER-LEVEL PSYCHEDELIC MADNESS SHADER
// This shader transcends reality and embodies pure visual chaos consciousness
// Where geometry becomes liquid, colors become emotion, and time becomes elastic

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) species_id: f32,  // 0=Disco, 1=Quantum, 2=Hypno, 3=Fractal, 4=BassDrop
    @location(4) consciousness: f32,
    @location(5) trip_intensity: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) world_pos: vec2<f32>,
    @location(3) species_id: f32,
    @location(4) consciousness: f32,
    @location(5) trip_intensity: f32,
}

struct Uniforms {
    time: f32,
    reality_distortion: f32,
    consciousness_level: f32,
    beat_intensity: f32,
    screen_resolution: vec2<f32>,
    beat_frequency: f32,
    cosmic_phase: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Input positions are already in NDC coordinates from Rust code
    let ndc_pos = input.position.xy;

    // Apply species-specific vertex distortions
    let species_distorted_pos = apply_species_vertex_distortion(ndc_pos, input.position.xy, input.species_id, input.consciousness, input.trip_intensity);

    // Apply global reality distortion
    let distorted_pos = apply_reality_distortion(species_distorted_pos, input.position.xy);

    out.clip_position = vec4<f32>(distorted_pos, input.position.z, 1.0);
    out.color = input.color;
    out.uv = input.uv;
    out.world_pos = input.position.xy;
    out.species_id = input.species_id;
    out.consciousness = input.consciousness;
    out.trip_intensity = input.trip_intensity;

    return out;
}

fn apply_species_vertex_distortion(ndc_pos: vec2<f32>, world_pos: vec2<f32>, species_id: f32, consciousness: f32, trip_intensity: f32) -> vec2<f32> {
    var distorted_pos = ndc_pos;

    // Species-specific vertex distortions
    if (species_id < 0.5) {
        // DISCO: Mirror ball refraction effects
        let center_dist = length(ndc_pos);
        let mirror_angle = atan2(ndc_pos.y, ndc_pos.x) + uniforms.time * 2.0;
        let mirror_distortion = sin(mirror_angle * 8.0) * consciousness * 0.1;
        distorted_pos += vec2<f32>(cos(mirror_angle), sin(mirror_angle)) * mirror_distortion;
    } else if (species_id < 1.5) {
        // QUANTUM: Probability cloud shimmering
        let quantum_jitter = vec2<f32>(
            sin(world_pos.x * 0.1 + uniforms.time * 10.0) * trip_intensity * 0.05,
            cos(world_pos.y * 0.1 + uniforms.time * 13.0) * trip_intensity * 0.05
        );
        distorted_pos += quantum_jitter;
    } else if (species_id < 2.5) {
        // HYPNO: Spiral vertex displacement
        let center_dist = length(ndc_pos);
        let spiral_factor = consciousness * uniforms.time * 0.5;
        let spiral_angle = atan2(ndc_pos.y, ndc_pos.x) + spiral_factor;
        let spiral_displacement = sin(center_dist * 20.0 + spiral_factor) * trip_intensity * 0.08;
        distorted_pos += vec2<f32>(cos(spiral_angle), sin(spiral_angle)) * spiral_displacement;
    } else if (species_id < 3.5) {
        // FRACTAL: Self-similar recursive distortion
        let scale_factor = 1.0 + sin(uniforms.time + consciousness) * trip_intensity * 0.2;
        distorted_pos *= scale_factor;
        let fractal_wave = sin(world_pos.x * 0.02 + world_pos.y * 0.03 + uniforms.time) * consciousness * 0.06;
        distorted_pos += vec2<f32>(fractal_wave, -fractal_wave);
    } else {
        // BASS DROP: Waveform vertex displacement
        let bass_wave = sin(uniforms.time * uniforms.beat_frequency + world_pos.x * 0.01) * uniforms.beat_intensity;
        let sub_wave = cos(uniforms.time * uniforms.beat_frequency * 0.5 + world_pos.y * 0.005) * uniforms.beat_intensity;
        distorted_pos.y += bass_wave * trip_intensity * 0.1;
        distorted_pos.x += sub_wave * consciousness * 0.05;
    }

    return distorted_pos;
}

fn apply_reality_distortion(ndc_pos: vec2<f32>, world_pos: vec2<f32>) -> vec2<f32> {
    // Mathematical reality distortion based on consciousness level
    let time_factor = uniforms.time * 0.5;
    let distortion_strength = uniforms.reality_distortion * 0.1;

    // Create wave distortions using mathematical functions
    let wave1 = sin(world_pos.x * 0.01 + time_factor) * cos(world_pos.y * 0.01 + time_factor * 0.7);
    let wave2 = sin(world_pos.x * 0.005 + time_factor * 1.3) * sin(world_pos.y * 0.007 + time_factor * 0.9);

    // Consciousness-based spiral distortion
    let center_dist = length(ndc_pos);
    let spiral_angle = atan2(ndc_pos.y, ndc_pos.x) + uniforms.consciousness_level * time_factor * 0.1;
    let spiral_distortion = vec2<f32>(cos(spiral_angle), sin(spiral_angle)) * center_dist;

    // Beat-based pulsing with cosmic phase
    let beat_pulse = sin(uniforms.time * 10.0 + uniforms.cosmic_phase) * uniforms.beat_intensity * 0.05;

    let final_distortion = (wave1 + wave2) * distortion_strength +
                          (spiral_distortion - ndc_pos) * uniforms.consciousness_level * 0.02 +
                          ndc_pos * beat_pulse;

    return ndc_pos + final_distortion;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Initialize with base color but prepare for total transformation
    var color = input.color;

    // SPECIES-SPECIFIC VISUAL MADNESS
    color = apply_species_specific_effects(color, input.uv, input.world_pos, input.species_id, input.consciousness, input.trip_intensity);

    // Global psychedelic effects layer
    color = apply_global_psychedelic_layer(color, input.uv, input.world_pos);

    // Consciousness-based reality tears
    color = apply_consciousness_reality_tears(color, input.world_pos, input.consciousness);

    // Beat-reactive pulsing and strobing
    color = apply_beat_reactive_effects(color, input.world_pos);

    // Chromatic aberration and visual distortion
    color = apply_chromatic_madness(color, input.uv, input.world_pos);

    // Final transparency and alpha effects for layering
    let alpha = calculate_species_alpha(input.species_id, input.consciousness, input.trip_intensity);

    // Screen-wide cosmic pulsing
    let cosmic_pulse = 0.8 + sin(uniforms.time * 2.0 + uniforms.cosmic_phase) * uniforms.consciousness_level * 0.2;
    color *= cosmic_pulse;

    return vec4<f32>(color, alpha);
}

// =============================================================================
// SPECIES-SPECIFIC VISUAL EFFECT FUNCTIONS
// Each species gets its own signature visual madness
// =============================================================================

fn apply_species_specific_effects(base_color: vec3<f32>, uv: vec2<f32>, world_pos: vec2<f32>, species_id: f32, consciousness: f32, trip_intensity: f32) -> vec3<f32> {
    var color = base_color;

    if (species_id < 0.5) {
        // DISCO LLAMA: Mirror balls, lens flares, disco madness
        color = apply_disco_effects(color, uv, world_pos, consciousness, trip_intensity);
    } else if (species_id < 1.5) {
        // QUANTUM SHEEP: Glitch effects, quantum superposition, uncertainty patterns
        color = apply_quantum_effects(color, uv, world_pos, consciousness, trip_intensity);
    } else if (species_id < 2.5) {
        // HYPNO SPECIES: Spiral patterns, hypnotic animations, tunnel vision
        color = apply_hypno_effects(color, uv, world_pos, consciousness, trip_intensity);
    } else if (species_id < 3.5) {
        // FRACTAL SPECIES: Mandelbrot/Julia sets, recursive patterns, infinite zoom
        color = apply_fractal_effects(color, uv, world_pos, consciousness, trip_intensity);
    } else {
        // BASS DROP SPECIES: Waveform visualizations, bass-reactive patterns
        color = apply_bass_drop_effects(color, uv, world_pos, consciousness, trip_intensity);
    }

    return color;
}

fn apply_disco_effects(base_color: vec3<f32>, uv: vec2<f32>, world_pos: vec2<f32>, consciousness: f32, trip_intensity: f32) -> vec3<f32> {
    var color = base_color;

    // Mirror ball effect
    let center = vec2<f32>(0.5, 0.5);
    let dist_from_center = length(uv - center);
    let mirror_facets = sin(atan2(uv.y - center.y, uv.x - center.x) * 16.0 + uniforms.time * 4.0);
    let mirror_reflection = mix(0.5, 1.5, (mirror_facets * 0.5 + 0.5) * consciousness);

    // Disco ball sparkles
    let sparkle_pattern = sin(world_pos.x * 0.5 + uniforms.time * 8.0) * cos(world_pos.y * 0.3 + uniforms.time * 6.0);
    let sparkle_intensity = step(0.8, sparkle_pattern) * trip_intensity;

    // Lens flare streaks
    let flare_angle = atan2(uv.y - 0.5, uv.x - 0.5);
    let flare_pattern = sin(flare_angle * 4.0 + uniforms.time * 2.0) * consciousness;

    color *= mirror_reflection;
    color += vec3<f32>(sparkle_intensity * 2.0, sparkle_intensity * 1.5, sparkle_intensity);
    color += vec3<f32>(max(0.0, flare_pattern) * 0.5);

    // Disco spectrum cycling
    let disco_hue = (uniforms.time * 2.0 + world_pos.x * 0.01) % 6.28;
    let disco_spectrum = vec3<f32>(
        sin(disco_hue) * 0.5 + 0.5,
        sin(disco_hue + 2.09) * 0.5 + 0.5,
        sin(disco_hue + 4.18) * 0.5 + 0.5
    );
    color = mix(color, disco_spectrum, trip_intensity * 0.3);

    return color;
}

fn apply_quantum_effects(base_color: vec3<f32>, uv: vec2<f32>, world_pos: vec2<f32>, consciousness: f32, trip_intensity: f32) -> vec3<f32> {
    var color = base_color;

    // Quantum uncertainty glitch
    let quantum_noise = fract(sin(dot(world_pos * 0.1 + uniforms.time * 100.0, vec2<f32>(12.9898, 78.233))) * 43758.5453);
    let uncertainty_factor = step(0.95, quantum_noise) * trip_intensity;

    // Probability wave interference
    let wave1 = sin(world_pos.x * 0.05 + uniforms.time * 3.0);
    let wave2 = cos(world_pos.y * 0.07 + uniforms.time * 4.0);
    let interference = wave1 * wave2 * consciousness;

    // Quantum superposition color bleeding
    let superposition_offset = vec2<f32>(
        sin(uniforms.time * 10.0) * trip_intensity * 0.1,
        cos(uniforms.time * 13.0) * trip_intensity * 0.1
    );
    let superposition_uv = uv + superposition_offset;

    // Digital glitch patterns
    let glitch_lines = step(0.98, sin(world_pos.y * 2.0 + uniforms.time * 20.0));
    let glitch_color = vec3<f32>(uncertainty_factor, glitch_lines * trip_intensity, uncertainty_factor * 0.5);

    color += interference * 0.3;
    color = mix(color, glitch_color, uncertainty_factor);
    color *= (1.0 + interference * 0.2);

    return color;
}

fn apply_hypno_effects(base_color: vec3<f32>, uv: vec2<f32>, world_pos: vec2<f32>, consciousness: f32, trip_intensity: f32) -> vec3<f32> {
    var color = base_color;

    // Hypnotic spiral pattern
    let center = vec2<f32>(0.5, 0.5);
    let spiral_dist = length(uv - center);
    let spiral_angle = atan2(uv.y - center.y, uv.x - center.x);
    let spiral_pattern = sin(spiral_dist * 20.0 - spiral_angle * 8.0 + uniforms.time * consciousness * 2.0);

    // Tunnel vision effect
    let tunnel_factor = 1.0 - smoothstep(0.0, 0.8, spiral_dist);
    let tunnel_intensity = tunnel_factor * trip_intensity;

    // Concentric rings
    let ring_pattern = sin(spiral_dist * 30.0 + uniforms.time * 4.0) * consciousness;

    // Hypnotic color cycling
    let hypno_hue = spiral_angle + uniforms.time * trip_intensity;
    let hypno_color = vec3<f32>(
        sin(hypno_hue) * 0.5 + 0.5,
        sin(hypno_hue + 2.09) * 0.5 + 0.5,
        sin(hypno_hue + 4.18) * 0.5 + 0.5
    );

    color = mix(color, hypno_color, spiral_pattern * 0.4 * trip_intensity);
    color *= (1.0 + ring_pattern * 0.3);
    color = mix(color, vec3<f32>(1.0), tunnel_intensity * 0.2);

    return color;
}

fn apply_fractal_effects(base_color: vec3<f32>, uv: vec2<f32>, world_pos: vec2<f32>, consciousness: f32, trip_intensity: f32) -> vec3<f32> {
    var color = base_color;

    // Julia set fractal
    let c = vec2<f32>(-0.7 + sin(uniforms.time * 0.1) * 0.3, 0.27015 + cos(uniforms.time * 0.07) * 0.3);
    var z = (uv - 0.5) * (2.0 + trip_intensity);

    var iterations = 0;
    let max_iterations = 20;

    for (var i = 0; i < max_iterations; i = i + 1) {
        if (length(z) > 2.0) {
            break;
        }
        z = vec2<f32>(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
        iterations = iterations + 1;
    }

    // Fractal coloring with consciousness influence
    let t = f32(iterations) / f32(max_iterations);
    let fractal_color = vec3<f32>(
        sin(t * 6.28 + uniforms.time + consciousness) * 0.5 + 0.5,
        sin(t * 6.28 + uniforms.time + consciousness + 2.09) * 0.5 + 0.5,
        sin(t * 6.28 + uniforms.time + consciousness + 4.18) * 0.5 + 0.5
    );

    // Mandelbrot overlay
    var mandel_z = (uv - vec2<f32>(0.5, 0.5)) * 3.0 + vec2<f32>(-0.5, 0.0);
    let mandel_c = mandel_z;
    var mandel_iterations = 0;
    for (var i = 0; i < 15; i = i + 1) {
        if (length(mandel_z) > 2.0) {
            break;
        }
        mandel_z = vec2<f32>(mandel_z.x * mandel_z.x - mandel_z.y * mandel_z.y, 2.0 * mandel_z.x * mandel_z.y) + mandel_c;
        mandel_iterations = mandel_iterations + 1;
    }
    let mandel_factor = f32(mandel_iterations) / 15.0;

    color = mix(color, fractal_color, trip_intensity * 0.6);
    color *= (1.0 + mandel_factor * consciousness * 0.5);

    return color;
}

fn apply_bass_drop_effects(base_color: vec3<f32>, uv: vec2<f32>, world_pos: vec2<f32>, consciousness: f32, trip_intensity: f32) -> vec3<f32> {
    var color = base_color;

    // Bass waveform visualization
    let bass_wave = sin(world_pos.x * 0.02 + uniforms.time * uniforms.beat_frequency) * uniforms.beat_intensity;
    let sub_bass = cos(world_pos.y * 0.01 + uniforms.time * uniforms.beat_frequency * 0.5) * uniforms.beat_intensity;

    // Frequency spectrum bars
    let freq_x = floor(uv.x * 32.0) / 32.0;
    let spectrum_height = sin(freq_x * 20.0 + uniforms.time * 8.0) * uniforms.beat_intensity * trip_intensity;
    let spectrum_bar = step(1.0 - uv.y, spectrum_height);

    // Beat-reactive strobe
    let strobe_intensity = step(0.8, sin(uniforms.time * uniforms.beat_frequency * 4.0)) * uniforms.beat_intensity;

    // Sub-bass rumble effect
    let rumble_pattern = sin(world_pos.x * 0.005 + world_pos.y * 0.003 + uniforms.time * 2.0);
    let rumble_intensity = rumble_pattern * sub_bass * consciousness;

    color += vec3<f32>(bass_wave * trip_intensity * 0.3);
    color += vec3<f32>(spectrum_bar * trip_intensity);
    color *= (1.0 + strobe_intensity * 0.5);
    color = mix(color, vec3<f32>(1.0, 0.0, 0.0), rumble_intensity * 0.2);

    return color;
}

// =============================================================================
// GLOBAL PSYCHEDELIC EFFECT FUNCTIONS
// =============================================================================

fn apply_global_psychedelic_layer(base_color: vec3<f32>, uv: vec2<f32>, world_pos: vec2<f32>) -> vec3<f32> {
    var color = base_color;

    // Plasma field overlay
    let plasma1 = sin(world_pos.x * 0.01 + uniforms.time);
    let plasma2 = cos(world_pos.y * 0.015 + uniforms.time * 1.3);
    let plasma3 = sin((world_pos.x + world_pos.y) * 0.008 + uniforms.time * 0.7);
    let plasma = (plasma1 + plasma2 + plasma3) / 3.0;

    let plasma_color = vec3<f32>(
        sin(plasma * 3.14159 + uniforms.time) * 0.5 + 0.5,
        sin(plasma * 3.14159 + uniforms.time + 2.09) * 0.5 + 0.5,
        sin(plasma * 3.14159 + uniforms.time + 4.18) * 0.5 + 0.5
    );

    color = mix(color, plasma_color, 0.2 * uniforms.consciousness_level);

    return color;
}

fn apply_consciousness_reality_tears(base_color: vec3<f32>, world_pos: vec2<f32>, consciousness: f32) -> vec3<f32> {
    var color = base_color;

    // Reality tears based on consciousness level
    let tear_pattern = sin(world_pos.x * 0.02 + uniforms.time) * cos(world_pos.y * 0.03 + uniforms.time * 0.8);
    let consciousness_threshold = consciousness * uniforms.consciousness_level;

    if (tear_pattern * consciousness_threshold > 0.6) {
        // Invert colors in high consciousness regions
        color = vec3<f32>(1.0) - color;
        color += vec3<f32>(0.5, 0.0, 0.5); // Add purple tint
    }

    return color;
}

fn apply_beat_reactive_effects(base_color: vec3<f32>, world_pos: vec2<f32>) -> vec3<f32> {
    var color = base_color;

    // Beat-reactive intensity modulation
    let beat_modifier = 0.7 + sin(uniforms.time * 15.0) * uniforms.beat_intensity * 0.3;
    color *= beat_modifier;

    // Beat-reactive color shifting
    let beat_hue_shift = sin(uniforms.time * uniforms.beat_frequency) * uniforms.beat_intensity;
    let shifted_color = vec3<f32>(
        color.r * (1.0 + beat_hue_shift * 0.3),
        color.g * (1.0 + beat_hue_shift * 0.2),
        color.b * (1.0 + beat_hue_shift * 0.5)
    );

    color = mix(color, shifted_color, uniforms.beat_intensity * 0.5);

    return color;
}

fn apply_chromatic_madness(base_color: vec3<f32>, uv: vec2<f32>, world_pos: vec2<f32>) -> vec3<f32> {
    var color = base_color;

    // Chromatic aberration
    let aberration_strength = uniforms.reality_distortion * 0.02;
    let red_offset = vec2<f32>(aberration_strength, 0.0);
    let blue_offset = vec2<f32>(-aberration_strength, 0.0);

    // Simulate sampling with offset (simplified)
    let red_shift = sin(world_pos.x * 0.1 + aberration_strength * 100.0) * 0.5 + 0.5;
    let blue_shift = cos(world_pos.y * 0.1 + aberration_strength * 200.0) * 0.5 + 0.5;

    color.r = mix(color.r, red_shift, uniforms.reality_distortion * 0.3);
    color.b = mix(color.b, blue_shift, uniforms.reality_distortion * 0.3);

    return color;
}

fn calculate_species_alpha(species_id: f32, consciousness: f32, trip_intensity: f32) -> f32 {
    var alpha = 1.0;

    if (species_id < 1.5) {
        // Quantum species have probability-based transparency
        let quantum_alpha = 0.7 + sin(uniforms.time * 10.0 + consciousness) * trip_intensity * 0.3;
        alpha = quantum_alpha;
    } else if (species_id < 2.5) {
        // Hypno species have pulsing transparency
        let hypno_alpha = 0.8 + sin(uniforms.time * 4.0) * consciousness * 0.2;
        alpha = hypno_alpha;
    } else {
        // Other species maintain solid alpha with slight consciousness variation
        alpha = 0.9 + consciousness * 0.1;
    }

    return clamp(alpha, 0.3, 1.0);
}