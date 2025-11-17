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

use crate::mobjects::Circle;
use crate::core::Color;
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
                cull_mode: Some(wgpu::Face::Back),
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
            device,
            queue,
            pipeline,
            transform_bind_group,
            transform_buffer,
        })
    }

    pub fn render_circle(&self, circle: &Circle, color: Color, output_view: &wgpu::TextureView) {
        // Create vertices for a circle
        let mut vertices = Vec::new();
        let segments = 32;
        let center = circle.position;
        let radius = circle.radius;

        // Convert color from 0-255 to 0.0-1.0
        let r = (color.r as f32) / 255.0;
        let g = (color.g as f32) / 255.0;
        let b = (color.b as f32) / 255.0;
        let a = (color.a as f32) / 255.0;

        let color_array = [r, g, b, a];

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
        let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Create command encoder
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
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
}
