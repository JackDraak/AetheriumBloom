// Psychedelic shader for AetheriumBloom
// This shader embodies pure visual chaos and mathematical consciousness

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) world_pos: vec2<f32>,
}

struct Uniforms {
    time: f32,
    reality_distortion: f32,
    consciousness_level: f32,
    beat_intensity: f32,
    screen_resolution: vec2<f32>,
    _padding: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Convert screen coordinates to NDC
    let ndc_pos = vec2<f32>(
        (input.position.x / uniforms.screen_resolution.x) * 2.0 - 1.0,
        1.0 - (input.position.y / uniforms.screen_resolution.y) * 2.0
    );

    // Apply reality distortion
    let distorted_pos = apply_reality_distortion(ndc_pos, input.position.xy);

    out.clip_position = vec4<f32>(distorted_pos, input.position.z, 1.0);
    out.color = input.color;
    out.uv = input.uv;
    out.world_pos = input.position.xy;

    return out;
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

    // Beat-based pulsing
    let beat_pulse = sin(uniforms.time * 10.0) * uniforms.beat_intensity * 0.05;

    let final_distortion = (wave1 + wave2) * distortion_strength +
                          (spiral_distortion - ndc_pos) * uniforms.consciousness_level * 0.02 +
                          ndc_pos * beat_pulse;

    return ndc_pos + final_distortion;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Base llama color with psychedelic modifications
    var color = input.color;

    // Add fractal patterns to llamas
    let fractal_color = calculate_fractal_pattern(input.uv, input.world_pos);
    color = mix(color, fractal_color, 0.3);

    // Consciousness-based color shifting
    color = apply_consciousness_shift(color);

    // Beat-based intensity modulation
    let beat_modifier = 0.8 + sin(uniforms.time * 15.0) * uniforms.beat_intensity * 0.2;
    color *= beat_modifier;

    // Reality distortion color effects
    color = apply_reality_color_effects(color, input.world_pos);

    // Add screen-wide pulsing based on consciousness level
    let screen_pulse = 0.9 + sin(uniforms.time * 2.0) * uniforms.consciousness_level * 0.1;
    color *= screen_pulse;

    return vec4<f32>(color, 1.0);
}

fn calculate_fractal_pattern(uv: vec2<f32>, world_pos: vec2<f32>) -> vec3<f32> {
    // Generate Julia set fractal for llama textures
    let c = vec2<f32>(-0.7, 0.27015); // Julia set constant
    var z = (uv - 0.5) * 2.0; // Map UV to [-1, 1]

    // Add time-based animation
    z += vec2<f32>(sin(uniforms.time * 0.1), cos(uniforms.time * 0.07)) * 0.1;

    var iterations = 0;
    let max_iterations = 20;

    for (var i = 0; i < max_iterations; i = i + 1) {
        if (length(z) > 2.0) {
            break;
        }
        z = vec2<f32>(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
        iterations = iterations + 1;
    }

    // Convert iterations to color
    let t = f32(iterations) / f32(max_iterations);
    return vec3<f32>(
        sin(t * 6.28 + uniforms.time) * 0.5 + 0.5,
        sin(t * 6.28 + uniforms.time + 2.09) * 0.5 + 0.5,
        sin(t * 6.28 + uniforms.time + 4.18) * 0.5 + 0.5
    );
}

fn apply_consciousness_shift(color: vec3<f32>) -> vec3<f32> {
    // Shift colors through impossible spectrums based on consciousness level
    let consciousness_hue = uniforms.consciousness_level * 6.28;

    let shift_matrix = mat3x3<f32>(
        cos(consciousness_hue), -sin(consciousness_hue), 0.0,
        sin(consciousness_hue), cos(consciousness_hue), 0.0,
        0.0, 0.0, 1.0
    );

    return shift_matrix * color;
}

fn apply_reality_color_effects(color: vec3<f32>, world_pos: vec2<f32>) -> vec3<f32> {
    // Reality distortion affects color in mathematical ways
    let distortion_factor = uniforms.reality_distortion;

    // Create color tears in spacetime
    let tear_pattern = sin(world_pos.x * 0.02 + uniforms.time) *
                      cos(world_pos.y * 0.02 + uniforms.time * 0.8);

    // Mathematical color transformations
    var modified_color = color;

    // Invert colors in distorted regions
    if (tear_pattern * distortion_factor > 0.5) {
        modified_color = vec3<f32>(1.0) - color;
    }

    // Add chromatic aberration
    let aberration_offset = distortion_factor * 0.01;
    modified_color.r = color.r; // Red channel stays
    modified_color.g = mix(color.g, sample_green_shift(world_pos, aberration_offset), distortion_factor);
    modified_color.b = mix(color.b, sample_blue_shift(world_pos, aberration_offset * 2.0), distortion_factor);

    return modified_color;
}

fn sample_green_shift(world_pos: vec2<f32>, offset: f32) -> f32 {
    // Simulate sampling green channel with offset (simplified for this shader)
    return sin(world_pos.x * 0.1 + offset * 100.0) * 0.5 + 0.5;
}

fn sample_blue_shift(world_pos: vec2<f32>, offset: f32) -> f32 {
    // Simulate sampling blue channel with offset (simplified for this shader)
    return cos(world_pos.y * 0.1 + offset * 100.0) * 0.5 + 0.5;
}