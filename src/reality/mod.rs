pub mod renderer;
pub mod fractals;
pub mod colors;
pub mod chaos;
pub mod buffer_manager;

use wgpu::*;
use crate::consciousness::LlamaRenderData;

pub use renderer::PsychedelicRenderer;
pub use colors::ColorConsciousness;
pub use fractals::FractalGenerator;
pub use chaos::ChaosEffects;
pub use buffer_manager::{DynamicVertexBuffer, VertexBudgetManager, BufferConfig};

#[derive(Debug, Clone)]
pub struct RenderData {
    pub llamas: Vec<LlamaRenderData>,
    pub reality_distortion: f32,
    pub consciousness_level: f32,
    pub cosmic_time: f64,
    pub beat_intensity: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub uv: [f32; 2],
    pub species_id: f32,     // 0=Disco, 1=Quantum, 2=Hypno, 3=Fractal, 4=BassDrop
    pub consciousness: f32,   // Individual consciousness level
    pub trip_intensity: f32,  // Psychedelic trip intensity
}

impl Vertex {
    const ATTRIBS: [VertexAttribute; 6] = vertex_attr_array![
        0 => Float32x3,  // position
        1 => Float32x3,  // color
        2 => Float32x2,  // uv
        3 => Float32,    // species_id
        4 => Float32,    // consciousness
        5 => Float32     // trip_intensity
    ];

    pub fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UniformData {
    pub time: f32,
    pub reality_distortion: f32,
    pub consciousness_level: f32,
    pub beat_intensity: f32,
    pub screen_resolution: [f32; 2],
    pub beat_frequency: f32,
    pub cosmic_phase: f32,
}

pub fn create_llama_geometry(
    position: glam::Vec2,
    size: f32,
    color: glam::Vec3,
    species_id: f32,
    consciousness: f32,
    trip_intensity: f32
) -> Vec<Vertex> {
    // Create a simple quad for each llama with full psychedelic data
    let half_size = size * 0.5;

    vec![
        // Triangle 1
        Vertex {
            position: [position.x - half_size, position.y - half_size, 0.0],
            color: [color.x, color.y, color.z],
            uv: [0.0, 0.0],
            species_id,
            consciousness,
            trip_intensity,
        },
        Vertex {
            position: [position.x + half_size, position.y - half_size, 0.0],
            color: [color.x, color.y, color.z],
            uv: [1.0, 0.0],
            species_id,
            consciousness,
            trip_intensity,
        },
        Vertex {
            position: [position.x - half_size, position.y + half_size, 0.0],
            color: [color.x, color.y, color.z],
            uv: [0.0, 1.0],
            species_id,
            consciousness,
            trip_intensity,
        },
        // Triangle 2
        Vertex {
            position: [position.x + half_size, position.y - half_size, 0.0],
            color: [color.x, color.y, color.z],
            uv: [1.0, 0.0],
            species_id,
            consciousness,
            trip_intensity,
        },
        Vertex {
            position: [position.x + half_size, position.y + half_size, 0.0],
            color: [color.x, color.y, color.z],
            uv: [1.0, 1.0],
            species_id,
            consciousness,
            trip_intensity,
        },
        Vertex {
            position: [position.x - half_size, position.y + half_size, 0.0],
            color: [color.x, color.y, color.z],
            uv: [0.0, 1.0],
            species_id,
            consciousness,
            trip_intensity,
        },
    ]
}

pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> glam::Vec3 {
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