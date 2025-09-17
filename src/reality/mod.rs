pub mod renderer;
pub mod fractals;
pub mod colors;
pub mod chaos;

use anyhow::Result;
use wgpu::*;
use crate::consciousness::LlamaRenderData;
use crate::core::events::LlamaSpecies;

pub use renderer::PsychedelicRenderer;
pub use colors::ColorConsciousness;
pub use fractals::FractalGenerator;
pub use chaos::ChaosEffects;

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
}

impl Vertex {
    const ATTRIBS: [VertexAttribute; 3] = vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3,
        2 => Float32x2
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
    pub _padding: [f32; 2],
}

pub fn create_llama_geometry(position: glam::Vec2, size: f32, color: glam::Vec3) -> Vec<Vertex> {
    // Create a simple quad for each llama
    let half_size = size * 0.5;

    vec![
        // Triangle 1
        Vertex {
            position: [position.x - half_size, position.y - half_size, 0.0],
            color: [color.x, color.y, color.z],
            uv: [0.0, 0.0],
        },
        Vertex {
            position: [position.x + half_size, position.y - half_size, 0.0],
            color: [color.x, color.y, color.z],
            uv: [1.0, 0.0],
        },
        Vertex {
            position: [position.x - half_size, position.y + half_size, 0.0],
            color: [color.x, color.y, color.z],
            uv: [0.0, 1.0],
        },
        // Triangle 2
        Vertex {
            position: [position.x + half_size, position.y - half_size, 0.0],
            color: [color.x, color.y, color.z],
            uv: [1.0, 0.0],
        },
        Vertex {
            position: [position.x + half_size, position.y + half_size, 0.0],
            color: [color.x, color.y, color.z],
            uv: [1.0, 1.0],
        },
        Vertex {
            position: [position.x - half_size, position.y + half_size, 0.0],
            color: [color.x, color.y, color.z],
            uv: [0.0, 1.0],
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