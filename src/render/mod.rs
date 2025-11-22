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
use crate::text::GlyphAtlas;
use std::sync::{Arc, Mutex};
use wgpu::util::DeviceExt;

/// Maximum number of objects that can be rendered in a single pass
const MAX_OBJECTS_PER_PASS: usize = 1024;

/// Alignment requirement for uniform buffers (must be 256 bytes on most GPUs)
const UNIFORM_ALIGNMENT: u64 = 256;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

// Text vertex with UV coordinates for texture sampling
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextVertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
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
    /// Current offset into transform buffer (in aligned units)
    current_transform_offset: std::cell::Cell<u32>,
    /// Size of each aligned transform slot
    aligned_transform_size: u64,
    // Text rendering components
    text_pipeline: Option<wgpu::RenderPipeline>,
    text_atlas: Option<Arc<Mutex<GlyphAtlas>>>,
    text_texture: Option<wgpu::Texture>,
    text_bind_group: Option<wgpu::BindGroup>,
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

        // Calculate aligned size for each transform (256-byte alignment required)
        let base_size = std::mem::size_of::<TransformUniform>() as u64;
        let aligned_transform_size =
            ((base_size + UNIFORM_ALIGNMENT - 1) / UNIFORM_ALIGNMENT) * UNIFORM_ALIGNMENT;

        // Create buffer large enough for MAX_OBJECTS_PER_PASS transforms
        let buffer_size = aligned_transform_size * MAX_OBJECTS_PER_PASS as u64;

        let transform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Transform Uniform Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Initialize first transform to identity
        let transform_uniform = TransformUniform::identity();
        queue.write_buffer(
            &transform_buffer,
            0,
            bytemuck::cast_slice(&[transform_uniform]),
        );

        // Create bind group layout for transform with dynamic offsets enabled
        let transform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Transform Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true, // Enable dynamic offsets
                        min_binding_size: std::num::NonZeroU64::new(std::mem::size_of::<
                            TransformUniform,
                        >()
                            as u64),
                    },
                    count: None,
                }],
            });

        // Create bind group (bind only one slot, dynamic offset will shift to others)
        let transform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Transform Bind Group"),
            layout: &transform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &transform_buffer,
                    offset: 0,
                    size: std::num::NonZeroU64::new(aligned_transform_size),
                }),
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
            current_transform_offset: std::cell::Cell::new(0),
            aligned_transform_size,
            text_pipeline: None,
            text_atlas: None,
            text_texture: None,
            text_bind_group: None,
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

    pub fn draw_circle(
        &self,
        circle: &Circle,
        color: Color,
        dynamic_offset: u32,
        render_pass: &mut wgpu::RenderPass,
    ) {
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

        // Set bind group with dynamic offset
        render_pass.set_bind_group(0, &self.transform_bind_group, &[dynamic_offset]);

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
        dynamic_offset: u32,
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

        // Set bind group with dynamic offset
        render_pass.set_bind_group(0, &self.transform_bind_group, &[dynamic_offset]);

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
        dynamic_offset: u32,
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

        // Set bind group with dynamic offset
        render_pass.set_bind_group(0, &self.transform_bind_group, &[dynamic_offset]);

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
        dynamic_offset: u32,
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
        self.draw_line(
            start,
            line_end,
            color,
            thickness,
            dynamic_offset,
            render_pass,
        );

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

        // Set bind group with dynamic offset
        render_pass.set_bind_group(0, &self.transform_bind_group, &[dynamic_offset]);

        // Set vertex and index buffers
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        // Draw the tip
        render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
    }

    /// Update transform for the next draw call
    /// Returns the offset to use with set_bind_group()
    pub fn update_transform(&self, transform: &TransformUniform) -> u32 {
        // Get current offset
        let offset_index = self.current_transform_offset.get();
        let byte_offset = offset_index as u64 * self.aligned_transform_size;

        // Write transform to the appropriate offset
        self.queue.write_buffer(
            &self.transform_buffer,
            byte_offset,
            bytemuck::cast_slice(&[*transform]),
        );

        // Increment offset for next object (with wraparound)
        let next_offset = (offset_index + 1) % MAX_OBJECTS_PER_PASS as u32;
        self.current_transform_offset.set(next_offset);

        // Return the dynamic offset for set_bind_group
        // NOTE: set_bind_group expects offset in BYTES, not indices
        offset_index * self.aligned_transform_size as u32
    }

    /// Reset transform offset counter (call at start of each frame)
    pub fn reset_transform_offset(&self) {
        self.current_transform_offset.set(0);
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

    /// Initialize text rendering system
    pub fn init_text_rendering(
        &mut self,
        font_size: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create glyph atlas
        let atlas = Arc::new(Mutex::new(GlyphAtlas::from_system_font(font_size)?));

        // Get atlas dimensions and data
        let (atlas_width, atlas_height) = {
            let atlas_guard = atlas.lock().unwrap();
            atlas_guard.atlas_dimensions()
        };

        // Create texture for glyph atlas
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Glyph Atlas Texture"),
            size: wgpu::Extent3d {
                width: atlas_width,
                height: atlas_height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // Create texture view and sampler
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Glyph Atlas Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Create bind group layout for text
        let text_bind_group_layout =
            self.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Text Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                });

        // Create bind group
        let text_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Text Bind Group"),
            layout: &text_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        // Load text shader
        let text_shader = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Text Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("text.wgsl").into()),
            });

        // Get transform bind group layout from existing pipeline
        let transform_bind_group_layout = self.pipeline.get_bind_group_layout(0);

        // Create text pipeline layout
        let text_pipeline_layout =
            self.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Text Pipeline Layout"),
                    bind_group_layouts: &[&transform_bind_group_layout, &text_bind_group_layout],
                    push_constant_ranges: &[],
                });

        // Create text rendering pipeline
        let text_pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Text Render Pipeline"),
                layout: Some(&text_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &text_shader,
                    entry_point: Some("vs_main"),
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<TextVertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            // position
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            // uv
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x2,
                            },
                            // color
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                                shader_location: 2,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                        ],
                    }],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &text_shader,
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
                    cull_mode: None,
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

        // Store everything
        self.text_pipeline = Some(text_pipeline);
        self.text_atlas = Some(atlas);
        self.text_texture = Some(texture);
        self.text_bind_group = Some(text_bind_group);

        Ok(())
    }

    pub fn get_instance(&self) -> &wgpu::Instance {
        &self.instance
    }

    pub fn draw_polygon(
        &self,
        vertices: &[Vector3],
        color: Color,
        dynamic_offset: u32,
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

        // Set bind group with dynamic offset
        render_pass.set_bind_group(0, &self.transform_bind_group, &[dynamic_offset]);

        // Set vertex and index buffers
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        // Draw
        render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
    }

    /// Draw text using glyph atlas
    pub fn draw_text(
        &mut self,
        content: &str,
        font_size: f32,
        color: Color,
        dynamic_offset: u32,
        render_pass: &mut wgpu::RenderPass,
    ) {
        // Check if text rendering is initialized
        let (text_pipeline, text_atlas, text_bind_group) =
            match (&self.text_pipeline, &self.text_atlas, &self.text_bind_group) {
                (Some(pipeline), Some(atlas), Some(bind_group)) => (pipeline, atlas, bind_group),
                _ => {
                    // Fallback to rectangle if not initialized
                    let char_width = 0.6 * font_size / 1000.0;
                    let width = char_width * content.len() as f32;
                    let height = font_size / 1000.0;
                    self.draw_rectangle(width, height, color, dynamic_offset, render_pass);
                    return;
                }
            };

        // Lock atlas and rasterize all glyphs
        let mut atlas_guard = text_atlas.lock().unwrap();
        if let Err(e) = atlas_guard.rasterize_string(content) {
            eprintln!("Failed to rasterize text: {}", e);
            return;
        }

        // Update texture with atlas data
        if let Some(texture) = &self.text_texture {
            let (atlas_width, atlas_height) = atlas_guard.atlas_dimensions();
            self.queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                atlas_guard.atlas_data(),
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(atlas_width * 4),
                    rows_per_image: Some(atlas_height),
                },
                wgpu::Extent3d {
                    width: atlas_width,
                    height: atlas_height,
                    depth_or_array_layers: 1,
                },
            );
        }

        // Build vertices for each glyph
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut cursor_x = 0.0f32;
        let color_array = color.to_f32_array();

        let scale = font_size / 1000.0; // Normalize to screen space

        for c in content.chars() {
            if let Some(glyph) = atlas_guard.get_glyph(c) {
                if glyph.width > 0 && glyph.height > 0 {
                    let glyph_width = glyph.width as f32 * scale;
                    let glyph_height = glyph.height as f32 * scale;
                    let bearing_x = glyph.bearing_x * scale;
                    let bearing_y = glyph.bearing_y * scale;

                    let x0 = cursor_x + bearing_x;
                    let y0 = -bearing_y;
                    let x1 = x0 + glyph_width;
                    let y1 = y0 + glyph_height;

                    let base_idx = vertices.len() as u16;

                    // Create quad for this glyph
                    vertices.push(TextVertex {
                        position: [x0, y0, 0.0],
                        uv: [glyph.uv.0, glyph.uv.1],
                        color: color_array,
                    });
                    vertices.push(TextVertex {
                        position: [x1, y0, 0.0],
                        uv: [glyph.uv.2, glyph.uv.1],
                        color: color_array,
                    });
                    vertices.push(TextVertex {
                        position: [x1, y1, 0.0],
                        uv: [glyph.uv.2, glyph.uv.3],
                        color: color_array,
                    });
                    vertices.push(TextVertex {
                        position: [x0, y1, 0.0],
                        uv: [glyph.uv.0, glyph.uv.3],
                        color: color_array,
                    });

                    // Two triangles for the quad
                    indices.extend_from_slice(&[
                        base_idx,
                        base_idx + 1,
                        base_idx + 2,
                        base_idx,
                        base_idx + 2,
                        base_idx + 3,
                    ]);
                }

                cursor_x += glyph.advance * scale;
            }
        }

        drop(atlas_guard);

        if vertices.is_empty() {
            return;
        }

        // Create GPU buffers
        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Text Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Text Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        // Render text
        render_pass.set_pipeline(text_pipeline);
        render_pass.set_bind_group(0, &self.transform_bind_group, &[dynamic_offset]);
        render_pass.set_bind_group(1, text_bind_group, &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
    }

    /// Draw a mathematical expression using LaTeX notation
    ///
    /// This method parses the LaTeX, lays out the components, and renders
    /// each text element using the existing text rendering system.
    pub fn draw_math(
        &mut self,
        latex: &str,
        base_font_size: f32,
        color: Color,
        dynamic_offset: u32,
        render_pass: &mut wgpu::RenderPass,
    ) {
        use crate::math::{expression::parse_latex, layout::MathLayout};

        // Parse the LaTeX into a tree
        let math_node = parse_latex(latex);

        // Layout the expression
        let layout = MathLayout::layout_node(&math_node, base_font_size);

        // Flatten into positioned text elements
        let elements = layout.flatten();

        // Render each text element
        // For now, we'll render all elements with the identity transform
        // TODO: In the future, we should properly position each element
        // based on the layout positions
        for (_position, text, font_size) in elements {
            // Draw the text at its relative position
            // The positioning is handled by the layout system
            self.draw_text(&text, font_size, color, dynamic_offset, render_pass);
        }
    }
}
