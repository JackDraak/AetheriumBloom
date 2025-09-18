use anyhow::Result;
use wgpu::*;
use crate::reality::{RenderData, Vertex, UniformData, create_llama_geometry, hsv_to_rgb, DynamicVertexBuffer, VertexBudgetManager, BufferConfig};

pub struct PsychedelicRenderer {
    render_pipeline: RenderPipeline,
    dynamic_vertex_buffer: DynamicVertexBuffer,
    uniform_buffer: Buffer,
    uniform_bind_group: BindGroup,
    budget_manager: VertexBudgetManager,
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

        // Initialize dynamic buffer management system
        let buffer_config = BufferConfig {
            initial_capacity: 100_000,
            max_capacity: 1_000_000, // Increased max capacity with safety limits
            growth_factor: 1.5,
            usage_history_frames: 60,
            resize_threshold: 0.8,
        };

        let dynamic_vertex_buffer = DynamicVertexBuffer::new(
            buffer_config,
            std::mem::size_of::<Vertex>() as u64
        );

        // Initialize vertex budget manager with category budgets
        let mut budget_manager = VertexBudgetManager::new(500_000); // Total frame budget
        budget_manager.set_category_budget("llamas", 400_000);
        budget_manager.set_category_budget("effects", 100_000);

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
            dynamic_vertex_buffer,
            uniform_buffer,
            uniform_bind_group,
            budget_manager,
        })
    }

    pub fn render(&mut self, device: &Device, queue: &Queue, view: &TextureView, render_data: RenderData) -> Result<(), SurfaceError> {
        // Start new frame for budget tracking
        self.budget_manager.start_frame();

        // Generate vertices for all llamas with budget management
        let mut vertices = Vec::new();

        // Estimate total vertices needed for prediction
        let estimated_vertices_per_llama = 6; // Each llama is a quad (2 triangles)
        let estimated_total = render_data.llamas.len() * estimated_vertices_per_llama;

        // Check budget allocation for llamas
        let allocated_vertices = self.budget_manager.check_allocation("llamas", estimated_total);
        let max_llamas = allocated_vertices / estimated_vertices_per_llama;

        println!("Rendering {} llamas (limited to {} by budget), estimated {} vertices",
                   render_data.llamas.len(), max_llamas, estimated_total);

        for (i, llama) in render_data.llamas.iter().enumerate() {
            if i >= max_llamas {
                println!("Llama rendering limited by vertex budget at {}/{}", i, render_data.llamas.len());
                break;
            }

            let color = hsv_to_rgb(
                llama.color_wavelength.x,
                llama.color_wavelength.y,
                0.7 + llama.trip_intensity * 0.3,
            );

            let size = 10.0 + llama.trip_intensity * 5.0 + llama.reality_distortion * 10.0;
            let llama_vertices = create_llama_geometry(llama.position, size, color);
            vertices.extend(llama_vertices);
        }

        // Ensure buffer capacity and validate vertex count
        if let Err(e) = self.dynamic_vertex_buffer.ensure_capacity(device, vertices.len()) {
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

        // Update vertex buffer
        if !vertices.is_empty() {
            if let Some(buffer) = self.dynamic_vertex_buffer.get_buffer() {
                queue.write_buffer(buffer, 0, bytemuck::cast_slice(&vertices));
            } else {
                eprintln!("No vertex buffer available for rendering");
                return Err(SurfaceError::Lost);
            }
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

            if !vertices.is_empty() {
                if let Some(buffer) = self.dynamic_vertex_buffer.get_buffer() {
                    render_pass.set_vertex_buffer(0, buffer.slice(..));
                    render_pass.draw(0..vertices.len() as u32, 0..1);
                } else {
                    println!("No vertex buffer available for render pass");
                }
            }
        }

        queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }

    pub fn resize(&mut self, _device: &Device, _config: &SurfaceConfiguration) {
        // TODO: Update screen resolution in uniforms
    }

    /// Get current buffer statistics for monitoring and debugging
    pub fn get_buffer_stats(&self) -> (usize, bool, f32) {
        (
            self.dynamic_vertex_buffer.get_capacity(),
            self.dynamic_vertex_buffer.is_circuit_breaker_active(),
            self.budget_manager.get_average_usage(),
        )
    }

    /// Get detailed usage statistics
    pub fn get_usage_stats(&self) -> String {
        let stats = self.dynamic_vertex_buffer.get_usage_stats();
        format!(
            "Buffer: {}k capacity, Peak: {}k, Avg: {:.1}k, Predicted: {}k, Budget Avg: {:.1}k",
            self.dynamic_vertex_buffer.get_capacity() / 1000,
            stats.get_peak_usage() / 1000,
            stats.get_average_usage() / 1000.0,
            stats.get_predicted_usage() / 1000,
            self.budget_manager.get_average_usage() / 1000.0
        )
    }
}