use anyhow::Result;
use wgpu::*;
use crate::reality::{RenderData, Vertex, UniformData, create_llama_geometry, hsv_to_rgb};

pub struct PsychedelicRenderer {
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    uniform_buffer: Buffer,
    uniform_bind_group: BindGroup,
    max_vertices: usize,
}

impl PsychedelicRenderer {
    pub fn new(device: &Device, config: &SurfaceConfiguration) -> Result<Self> {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Psychedelic Shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/psychedelic.wgsl").into()),
        });

        let uniform_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
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
            label: Some("uniform_bind_group_layout"),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Psychedelic Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let max_vertices = 100000; // Support many llamas
        let vertex_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: (max_vertices * std::mem::size_of::<Vertex>()) as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: std::mem::size_of::<UniformData>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

        Ok(Self {
            render_pipeline,
            vertex_buffer,
            uniform_buffer,
            uniform_bind_group,
            max_vertices,
        })
    }

    pub fn render(&mut self, device: &Device, queue: &Queue, view: &TextureView, render_data: RenderData) -> Result<(), SurfaceError> {
        // Generate vertices for all llamas
        let mut vertices = Vec::new();

        for llama in &render_data.llamas {
            let color = hsv_to_rgb(
                llama.color_wavelength.x,
                llama.color_wavelength.y,
                0.7 + llama.trip_intensity * 0.3,
            );

            let size = 10.0 + llama.trip_intensity * 5.0 + llama.reality_distortion * 10.0;
            let llama_vertices = create_llama_geometry(llama.position, size, color);
            vertices.extend(llama_vertices);
        }

        // Clamp vertices to buffer size
        if vertices.len() > self.max_vertices {
            vertices.truncate(self.max_vertices);
        }

        // Update vertex buffer
        if !vertices.is_empty() {
            queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
        }

        // Update uniforms
        let uniform_data = UniformData {
            time: render_data.cosmic_time as f32,
            reality_distortion: render_data.reality_distortion,
            consciousness_level: render_data.consciousness_level,
            beat_intensity: render_data.beat_intensity,
            screen_resolution: [1200.0, 800.0], // TODO: Get from config
            _padding: [0.0, 0.0],
        };

        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniform_data]));

        // Render
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Psychedelic Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
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
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

            if !vertices.is_empty() {
                render_pass.draw(0..vertices.len() as u32, 0..1);
            }
        }

        queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }

    pub fn resize(&mut self, _device: &Device, _config: &SurfaceConfiguration) {
        // TODO: Update screen resolution in uniforms
    }
}