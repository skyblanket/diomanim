//! # GPU Rendering Pipeline
//!
//! This module provides WebGPU-based rendering for high-performance graphics.
//! It supports Vulkan, Metal, and DirectX 12 backends through the WGPU abstraction layer.
//!
//! ## Key Components
//!
//! - **ShapeRenderer**: Main rendering engine for geometric shapes
//! - **Vertex**: GPU-compatible vertex data structure
//! - **TransformUniform**: Transform matrix uniform buffer for GPU shaders
//!
//! ## Architecture
//!
//! The rendering pipeline follows this flow:
//! 1. CPU: Create geometry and transform data
//! 2. Upload to GPU buffers
//! 3. WGSL shader transforms vertices
//! 4. Rasterization and fragment shading
//! 5. Output to texture/screen
//!
//! ## Example
//!
//! ```rust,no_run
//! use diomanim::render::*;
//! use diomanim::mobjects::*;
//! use diomanim::core::*;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create renderer
//! let renderer = ShapeRenderer::new(1920, 1080).await?;
//!
//! // Create a circle
//! let circle = Circle::new(2.0, Color::BLUE);
//!
//! // Render (requires output texture view)
//! // renderer.render_circle(&circle, Color::BLUE, &output_view);
//! # Ok(())
//! # }
//! ```

use crate::core::{Color, Vector3};
use crate::mobjects::Circle;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

// Uniform buffer for transform matrices
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformUniform {
    pub model_view_proj: [[f32; 4]; 4],
}

impl TransformUniform {
    pub fn identity() -> Self {
        Self {
            model_view_proj: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

pub struct ShapeRenderer {
    #[allow(dead_code)]
    width: u32,
    #[allow(dead_code)]
    height: u32,
    instance: wgpu::Instance,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    transform_bind_group: wgpu::BindGroup,
    transform_buffer: wgpu::Buffer,
}

impl ShapeRenderer {
    pub async fn new(width: u32, height: u32) -> Result<Self, Box<dyn std::error::Error>> {
        // Create instance and adapter
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await?;

        // Create device and queue
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
            })
            .await?;

        // Create transform uniform buffer
        let transform_uniform = TransformUniform::identity();
        let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Transform Uniform Buffer"),
            contents: bytemuck::cast_slice(&[transform_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create bind group layout for transform
        let transform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Transform Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        // Create bind group
        let transform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Transform Bind Group"),
            layout: &transform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: transform_buffer.as_entire_binding(),
            }],
        });

        // Create shader module
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shape Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shapes.wgsl").into()),
        });

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Shape Pipeline Layout"),
            bind_group_layouts: &[&transform_bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create render pipeline
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Shape Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x3,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                    ],
                }],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, // Disable back-face culling for debugging
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        Ok(Self {
            width,
            height,
            instance,
            device,
            queue,
            pipeline,
            transform_bind_group,
            transform_buffer,
        })
    }

    pub fn begin_render_pass<'a>(
        &self,
        encoder: &'a mut wgpu::CommandEncoder,
        output_view: &'a wgpu::TextureView,
        clear_color: Option<wgpu::Color>,
    ) -> wgpu::RenderPass<'a> {
        let clear_color = clear_color.unwrap_or(wgpu::Color {
            r: 0.95,
            g: 0.95,
            b: 0.95,
            a: 1.0,
        });

        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Shape Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: output_view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear_color),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        })
    }

    pub fn render_circle(&self, circle: &Circle, color: Color, output_view: &wgpu::TextureView) {
        // Create vertices for a circle
        let mut vertices = Vec::new();
        let segments = 32;
        let center = circle.position;
        let radius = circle.radius;

        let color_array = color.to_f32_array();

        // Create center vertex
        vertices.push(Vertex {
            position: [center.x, center.y, 0.0],
            color: color_array,
        });

        // Create circle vertices
        for i in 0..=segments {
            let angle = 2.0 * std::f32::consts::PI * (i as f32) / (segments as f32);
            let x = center.x + radius * angle.cos();
            let y = center.y + radius * angle.sin();

            vertices.push(Vertex {
                position: [x, y, 0.0],
                color: color_array,
            });
        }

        // Create index buffer
        let mut indices = Vec::new();
        for i in 1..=segments {
            indices.push(0);
            indices.push(i);
            indices.push(i + 1);
        }

        // Create GPU buffers
        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        // Create command encoder
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Begin render pass
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Shape Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: output_view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.95,
                        g: 0.95,
                        b: 0.95,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        // Set pipeline and bind groups
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.transform_bind_group, &[]);

        // Set vertex and index buffers
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        // Draw
        render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);

        // Drop render_pass to release borrow
        drop(render_pass);

        // Submit command
        self.queue.submit(std::iter::once(encoder.finish()));
    }

    pub fn draw_circle(&self, circle: &Circle, color: Color, render_pass: &mut wgpu::RenderPass) {
        // Create vertices for a circle centered at origin
        // Position is handled by transform uniform
        let mut vertices = Vec::new();
        let segments = 32;
        let radius = circle.radius;

        let color_array = color.to_f32_array();

        // Create center vertex at origin
        vertices.push(Vertex {
            position: [0.0, 0.0, 0.0],
            color: color_array,
        });

        // Create circle vertices around origin
        for i in 0..=segments {
            let angle = 2.0 * std::f32::consts::PI * (i as f32) / (segments as f32);
            let x = radius * angle.cos();
            let y = radius * angle.sin();

            vertices.push(Vertex {
                position: [x, y, 0.0],
                color: color_array,
            });
        }

        // Create index buffer
        let mut indices: Vec<u16> = Vec::new();
        for i in 1..=segments {
            indices.push(0u16);
            indices.push(i as u16);
            indices.push((i + 1) as u16);
        }

        // Create GPU buffers
        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        // Set vertex and index buffers
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        // Draw
        render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
    }

    pub fn draw_rectangle(
        &self,
        width: f32,
        height: f32,
        color: Color,
        render_pass: &mut wgpu::RenderPass,
    ) {
        let center = [0.0, 0.0, 0.0]; // Position handled by transform uniform
        let half_width = width / 2.0;
        let half_height = height / 2.0;

        let color_array = color.to_f32_array();

        // Create vertices for a rectangle (two triangles)
        let vertices = vec![
            Vertex {
                position: [center[0] - half_width, center[1] - half_height, center[2]],
                color: color_array,
            },
            Vertex {
                position: [center[0] + half_width, center[1] - half_height, center[2]],
                color: color_array,
            },
            Vertex {
                position: [center[0] + half_width, center[1] + half_height, center[2]],
                color: color_array,
            },
            Vertex {
                position: [center[0] - half_width, center[1] + half_height, center[2]],
                color: color_array,
            },
        ];

        // Create index buffer for two triangles (CCW winding)
        let indices: Vec<u16> = vec![0, 1, 2, 0, 2, 3];

        // Create GPU buffers
        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Rectangle Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Rectangle Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        // Set vertex and index buffers
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        // Draw
        render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
    }

    pub fn draw_line(
        &self,
        start: Vector3,
        end: Vector3,
        color: Color,
        thickness: f32,
        render_pass: &mut wgpu::RenderPass,
    ) {
        let dir = Vector3::new(end.x - start.x, end.y - start.y, 0.0);
        let length = (dir.x * dir.x + dir.y * dir.y).sqrt();

        if length < 0.001 {
            return; // Skip degenerate lines
        }

        // Normalize direction
        let dir_norm = Vector3::new(dir.x / length, dir.y / length, 0.0);
        let perp = Vector3::new(-dir_norm.y, dir_norm.x, 0.0);

        let half_thickness = thickness / 200.0; // Scale down for reasonable thickness
        let half_length = length / 2.0;

        // Center point of the line
        let center_x = (start.x + end.x) / 2.0;
        let center_y = (start.y + end.y) / 2.0;

        let color_array = color.to_f32_array();

        // Create vertices for a thick line (rectangle)
        let vertices = vec![
            Vertex {
                position: [
                    center_x - dir_norm.x * half_length - perp.x * half_thickness,
                    center_y - dir_norm.y * half_length - perp.y * half_thickness,
                    0.0,
                ],
                color: color_array,
            },
            Vertex {
                position: [
                    center_x + dir_norm.x * half_length - perp.x * half_thickness,
                    center_y + dir_norm.y * half_length - perp.y * half_thickness,
                    0.0,
                ],
                color: color_array,
            },
            Vertex {
                position: [
                    center_x + dir_norm.x * half_length + perp.x * half_thickness,
                    center_y + dir_norm.y * half_length + perp.y * half_thickness,
                    0.0,
                ],
                color: color_array,
            },
            Vertex {
                position: [
                    center_x - dir_norm.x * half_length + perp.x * half_thickness,
                    center_y - dir_norm.y * half_length + perp.y * half_thickness,
                    0.0,
                ],
                color: color_array,
            },
        ];

        // Create index buffer for two triangles
        let indices = vec![0, 1, 2, 0, 2, 3];

        // Create GPU buffers
        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Line Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Line Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        // Set vertex and index buffers
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        // Draw
        render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
    }

    pub fn draw_arrow(
        &self,
        start: Vector3,
        end: Vector3,
        color: Color,
        thickness: f32,
        render_pass: &mut wgpu::RenderPass,
    ) {
        // First draw the line (shaft)
        let dir = Vector3::new(end.x - start.x, end.y - start.y, 0.0);
        let length = (dir.x * dir.x + dir.y * dir.y).sqrt();

        if length < 0.001 {
            return; // Skip degenerate arrows
        }

        let tip_size = 0.05; // 5% of scene size for tip

        // Calculate line end (where tip starts)
        let line_end = if length > tip_size {
            Vector3::new(
                start.x + dir.x * (1.0 - tip_size / length),
                start.y + dir.y * (1.0 - tip_size / length),
                start.z,
            )
        } else {
            start // Very short arrow, minimal shaft
        };

        // Draw the shaft
        self.draw_line(start, line_end, color, thickness, render_pass);

        // Draw the triangular tip
        let dir_norm = Vector3::new(dir.x / length, dir.y / length, 0.0);
        let perp = Vector3::new(-dir_norm.y, dir_norm.x, 0.0);
        let tip_half_width = tip_size * 0.5;

        let color_array = color.to_f32_array();

        // Tip vertices - triangle pointing to end point
        let vertices = vec![
            Vertex {
                position: [end.x, end.y, end.z],
                color: color_array,
            },
            Vertex {
                position: [
                    line_end.x + perp.x * tip_half_width,
                    line_end.y + perp.y * tip_half_width,
                    end.z,
                ],
                color: color_array,
            },
            Vertex {
                position: [
                    line_end.x - perp.x * tip_half_width,
                    line_end.y - perp.y * tip_half_width,
                    end.z,
                ],
                color: color_array,
            },
        ];

        // Triangle indices
        let indices = vec![0, 1, 2];

        // Create GPU buffers
        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Arrow Tip Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Arrow Tip Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        // Set vertex and index buffers
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        // Draw the tip
        render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
    }

    pub fn update_transform(&self, transform: &TransformUniform) {
        self.queue.write_buffer(
            &self.transform_buffer,
            0,
            bytemuck::cast_slice(&[*transform]),
        );
    }

    pub fn get_device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn get_queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn get_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }

    pub fn get_transform_bind_group(&self) -> &wgpu::BindGroup {
        &self.transform_bind_group
    }

    pub fn get_instance(&self) -> &wgpu::Instance {
        &self.instance
    }

    pub fn draw_polygon(
        &self,
        vertices: &[Vector3],
        color: Color,
        render_pass: &mut wgpu::RenderPass,
    ) {
        if vertices.len() < 3 {
            return; // Need at least 3 vertices for a triangle
        }

        let color_array = color.to_f32_array();

        // Create vertex buffer from polygon vertices
        let gpu_vertices: Vec<Vertex> = vertices
            .iter()
            .map(|v| Vertex {
                position: [v.x, v.y, v.z],
                color: color_array,
            })
            .collect();

        // Simple fan triangulation from first vertex
        let mut indices = Vec::new();
        for i in 1..(vertices.len() - 1) {
            indices.push(0u16);
            indices.push(i as u16);
            indices.push((i + 1) as u16);
        }

        // Create GPU buffers
        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Polygon Vertex Buffer"),
                contents: bytemuck::cast_slice(&gpu_vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Polygon Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        // Set vertex and index buffers
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        // Draw
        render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
    }

    /// Draw text (MVP: renders as colored rectangle placeholder)
    /// TODO: Implement proper glyph rasterization
    pub fn draw_text(
        &self,
        content: &str,
        font_size: f32,
        color: Color,
        render_pass: &mut wgpu::RenderPass,
    ) {
        // For MVP, render as a colored rectangle placeholder
        // Width approximation: 0.6 * font_size per character
        let char_width = 0.6 * font_size / 1000.0; // Normalize to screen space
        let width = char_width * content.len() as f32;
        let height = font_size / 1000.0; // Normalize to screen space

        // Render as rectangle
        self.draw_rectangle(width, height, color, render_pass);
    }
}
