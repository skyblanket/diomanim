//! Visual Test - Simple test to verify shapes render correctly
//!
//! Creates a single frame with all shapes to verify rendering works

use diomanim::prelude::*;
use diomanim::scene::{SceneGraph, Renderable};
use diomanim::mobjects::Polygon;
use diomanim::render::ShapeRenderer;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Visual Test - All Shapes                                     ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    // Initialize renderer
    println!("Initializing GPU renderer...");
    let renderer = ShapeRenderer::new(WIDTH, HEIGHT).await?;
    println!("✓ WebGPU renderer ready ({}x{})", WIDTH, HEIGHT);

    println!("\nBuilding test scene...");
    let mut scene = SceneGraph::new();

    // Row 1: Circles
    println!("  • Adding circles...");
    for i in 0..4 {
        let x = -0.6 + (i as f32 * 0.4);
        let circle_id = scene.create_node_with_transform(
            format!("Circle_{}", i),
            Transform::from_translation(x, 0.5, 0.0),
        );
        let colors = [Color::RED, Color::GREEN, Color::BLUE, Color::YELLOW];
        scene.get_node_mut(circle_id).unwrap().set_renderable(Renderable::Circle {
            radius: 0.1,
            color: colors[i],
        });
    }

    // Row 2: Rectangles
    println!("  • Adding rectangles...");
    for i in 0..3 {
        let x = -0.5 + (i as f32 * 0.5);
        let rect_id = scene.create_node_with_transform(
            format!("Rectangle_{}", i),
            Transform::from_translation(x, 0.0, 0.0),
        );
        scene.get_node_mut(rect_id).unwrap().set_renderable(Renderable::Rectangle {
            width: 0.2,
            height: 0.15,
            color: Color::CYAN,
        });
    }

    // Row 3: Polygons
    println!("  • Adding polygons...");
    let poly_data = [
        ("Triangle", Polygon::triangle(0.1, Color::new(1.0, 0.5, 0.0)), -0.6),
        ("Pentagon", Polygon::pentagon(0.1, Color::new(0.5, 1.0, 0.5)), -0.2),
        ("Hexagon", Polygon::hexagon(0.1, Color::new(0.5, 0.5, 1.0)), 0.2),
        ("Star", Polygon::star(5, 0.1, 0.05, Color::new(1.0, 1.0, 0.0)), 0.6),
    ];

    for (name, poly, x) in poly_data {
        let poly_id = scene.create_node_with_transform(
            name.to_string(),
            Transform::from_translation(x, -0.5, 0.0),
        );
        scene.get_node_mut(poly_id).unwrap().set_renderable(Renderable::Polygon {
            vertices: poly.vertices,
            color: poly.color,
        });
    }

    // Row 4: Lines and Arrows
    println!("  • Adding lines and arrows...");
    let line_id = scene.create_node("Line".to_string());
    scene.get_node_mut(line_id).unwrap().set_renderable(Renderable::Line {
        start: Vector3::new(-0.6, -0.8, 0.0),
        end: Vector3::new(-0.2, -0.7, 0.0),
        color: Color::MAGENTA,
        thickness: 3.0,
    });

    let arrow_id = scene.create_node("Arrow".to_string());
    scene.get_node_mut(arrow_id).unwrap().set_renderable(Renderable::Arrow {
        start: Vector3::new(0.0, -0.8, 0.0),
        end: Vector3::new(0.4, -0.7, 0.0),
        color: Color::ORANGE,
        thickness: 3.0,
    });

    println!("✓ Created {} objects", scene.get_visible_renderables().len());

    // Update transforms
    scene.update_transforms();

    // Create output texture
    println!("\nRendering single test frame...");
    let output_texture = renderer.get_device().create_texture(&wgpu::TextureDescriptor {
        label: Some("Output Texture"),
        size: wgpu::Extent3d { width: WIDTH, height: HEIGHT, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let output_view = output_texture.create_view(&wgpu::TextureViewDescriptor::default());

    // Create command encoder
    let mut encoder = renderer.get_device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    // Begin render pass
    let mut render_pass = renderer.begin_render_pass(&mut encoder, &output_view, None);
    render_pass.set_pipeline(renderer.get_pipeline());
    render_pass.set_bind_group(0, renderer.get_transform_bind_group(), &[]);

    // Render all objects
    let renderables = scene.get_visible_renderables();
    for (transform_uniform, renderable, opacity) in renderables {
        renderer.update_transform(&transform_uniform);

        // Apply opacity to color
        let apply_opacity = |color: Color| -> Color {
            Color::rgba(color.r, color.g, color.b, color.a * opacity)
        };

        match renderable {
            Renderable::Circle { radius, color } => {
                let circle = diomanim::mobjects::Circle {
                    radius,
                    color: apply_opacity(color),
                    position: Vector3::zero(),
                };
                renderer.draw_circle(&circle, apply_opacity(color), &mut render_pass);
            }
            Renderable::Rectangle { width, height, color } => {
                renderer.draw_rectangle(width, height, apply_opacity(color), &mut render_pass);
            }
            Renderable::Line { start, end, color, thickness } => {
                renderer.draw_line(start, end, apply_opacity(color), thickness, &mut render_pass);
            }
            Renderable::Arrow { start, end, color, thickness } => {
                renderer.draw_arrow(start, end, apply_opacity(color), thickness, &mut render_pass);
            }
            Renderable::Polygon { vertices, color } => {
                renderer.draw_polygon(&vertices, apply_opacity(color), &mut render_pass);
            }
        }
    }

    // End render pass
    drop(render_pass);

    // Submit command
    renderer.get_queue().submit(std::iter::once(encoder.finish()));

    // Save frame to PNG
    println!("Saving frame to PNG...");
    std::fs::create_dir_all("output")?;
    save_texture_to_png(&renderer, &output_texture, WIDTH, HEIGHT, "output/visual_test.png");

    println!("\n✅ Test complete!");
    println!("📸 Output saved to: output/visual_test.png");
    println!("   Open this file to verify all shapes rendered correctly");

    Ok(())
}

/// Saves a WebGPU texture to a PNG file
fn save_texture_to_png(renderer: &ShapeRenderer, texture: &wgpu::Texture, width: u32, height: u32, filename: &str) {
    // Calculate aligned bytes per row (must be multiple of 256)
    const COPY_BYTES_PER_ROW_ALIGNMENT: u32 = 256;
    let unpadded_bytes_per_row = width * 4;
    let padded_bytes_per_row = (unpadded_bytes_per_row + COPY_BYTES_PER_ROW_ALIGNMENT - 1)
        / COPY_BYTES_PER_ROW_ALIGNMENT
        * COPY_BYTES_PER_ROW_ALIGNMENT;

    let buffer_size = (padded_bytes_per_row * height) as wgpu::BufferAddress;

    let staging_buffer = renderer.get_device().create_buffer(&wgpu::BufferDescriptor {
        label: Some("Staging Buffer"),
        size: buffer_size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    // Copy texture to buffer
    let mut encoder = renderer.get_device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Copy Encoder"),
    });

    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &staging_buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded_bytes_per_row),
                rows_per_image: Some(height),
            },
        },
        wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
    );

    renderer.get_queue().submit(std::iter::once(encoder.finish()));

    // Map the buffer synchronously
    let buffer_slice = staging_buffer.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
        tx.send(result).unwrap();
    });

    // CRITICAL: Poll the device to process GPU commands
    // Keep polling until the buffer mapping completes
    let timeout = std::time::Instant::now();
    loop {
        match rx.try_recv() {
            Ok(Ok(())) => break,
            Ok(Err(e)) => panic!("Buffer mapping failed: {:?}", e),
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                // Check timeout (10 seconds max)
                if timeout.elapsed().as_secs() > 10 {
                    panic!("Buffer mapping timed out after 10 seconds");
                }
                // Poll device to process GPU commands (wgpu 27 API)
                let _ = renderer.get_device().poll(wgpu::PollType::Wait {
                    submission_index: None,
                    timeout: None,
                });
                std::thread::yield_now();
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                panic!("Channel disconnected");
            }
        }
    }

    // Read the data
    let data = buffer_slice.get_mapped_range();

    // Write to PNG
    use std::fs::File;
    use std::io::BufWriter;
    let file = File::create(filename).unwrap();
    let writer = BufWriter::new(file);

    let mut encoder = png::Encoder::new(writer, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut png_writer = encoder.write_header().unwrap();

    // If padded, we need to copy row by row, skipping padding
    if padded_bytes_per_row != unpadded_bytes_per_row {
        let mut unpadded_data = Vec::with_capacity((unpadded_bytes_per_row * height) as usize);
        for row in 0..height {
            let row_start = (row * padded_bytes_per_row) as usize;
            let row_end = row_start + unpadded_bytes_per_row as usize;
            unpadded_data.extend_from_slice(&data[row_start..row_end]);
        }
        png_writer.write_image_data(&unpadded_data).unwrap();
    } else {
        png_writer.write_image_data(&data).unwrap();
    }

    drop(data);
    staging_buffer.unmap();
}
