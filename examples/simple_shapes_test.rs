//! Simple Shapes Test - Test each shape type individually

use diomanim::prelude::*;
use diomanim::render::ShapeRenderer;
use diomanim::scene::{Renderable, SceneGraph};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Simple Shapes Test");
    println!("==================\n");

    let renderer = ShapeRenderer::new(WIDTH, HEIGHT).await?;
    println!("✓ Renderer ready\n");

    // Test 1: Single large red circle at center
    println!("Test 1: Large red circle at center (0, 0)");
    test_shape(&renderer, "test1_circle_center.png", |scene| {
        let circle_id = scene.create_node_with_transform(
            "BigCircle".to_string(),
            Transform::from_translation(0.0, 0.0, 0.0),
        );
        scene
            .get_node_mut(circle_id)
            .unwrap()
            .set_renderable(Renderable::Circle {
                radius: 0.3,
                color: Color::RED,
            });
    })
    .await?;

    // Test 2: Circle at top
    println!("Test 2: Blue circle at top (0, 0.5)");
    test_shape(&renderer, "test2_circle_top.png", |scene| {
        let circle_id = scene.create_node_with_transform(
            "TopCircle".to_string(),
            Transform::from_translation(0.0, 0.5, 0.0),
        );
        scene
            .get_node_mut(circle_id)
            .unwrap()
            .set_renderable(Renderable::Circle {
                radius: 0.2,
                color: Color::BLUE,
            });
    })
    .await?;

    // Test 3: Rectangle at center
    println!("Test 3: Green rectangle at center");
    test_shape(&renderer, "test3_rectangle.png", |scene| {
        let rect_id = scene.create_node_with_transform(
            "Rect".to_string(),
            Transform::from_translation(0.0, 0.0, 0.0),
        );
        scene
            .get_node_mut(rect_id)
            .unwrap()
            .set_renderable(Renderable::Rectangle {
                width: 0.4,
                height: 0.3,
                color: Color::GREEN,
            });
    })
    .await?;

    // Test 4: Polygon at center
    println!("Test 4: Yellow star at center");
    test_shape(&renderer, "test4_star.png", |scene| {
        let star_id = scene.create_node_with_transform(
            "Star".to_string(),
            Transform::from_translation(0.0, 0.0, 0.0),
        );
        let star = diomanim::mobjects::Polygon::star(5, 0.3, 0.15, Color::YELLOW);
        scene
            .get_node_mut(star_id)
            .unwrap()
            .set_renderable(Renderable::Polygon {
                vertices: star.vertices,
                color: star.color,
            });
    })
    .await?;

    println!("\n✅ All tests complete!");
    println!("Check output/*.png files");

    Ok(())
}

async fn test_shape<F>(
    renderer: &ShapeRenderer,
    filename: &str,
    setup_fn: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnOnce(&mut SceneGraph),
{
    let mut scene = SceneGraph::new();
    setup_fn(&mut scene);
    scene.update_transforms();

    // Create output texture
    let output_texture = renderer
        .get_device()
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("Output Texture"),
            size: wgpu::Extent3d {
                width: WIDTH,
                height: HEIGHT,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
    let output_view = output_texture.create_view(&wgpu::TextureViewDescriptor::default());

    // Render
    let mut encoder =
        renderer
            .get_device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

    let mut render_pass = renderer.begin_render_pass(&mut encoder, &output_view, None);
    render_pass.set_pipeline(renderer.get_pipeline());
    render_pass.set_bind_group(0, renderer.get_transform_bind_group(), &[]);

    let renderables = scene.get_visible_renderables();
    for (transform_uniform, renderable, opacity) in renderables {
        renderer.update_transform(&transform_uniform);

        // Apply opacity to color
        let apply_opacity = |color: diomanim::core::Color| -> diomanim::core::Color {
            diomanim::core::Color::rgba(color.r, color.g, color.b, color.a * opacity)
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
            Renderable::Rectangle {
                width,
                height,
                color,
            } => {
                renderer.draw_rectangle(width, height, apply_opacity(color), &mut render_pass);
            }
            Renderable::Polygon { vertices, color } => {
                renderer.draw_polygon(&vertices, apply_opacity(color), &mut render_pass);
            }
            _ => {}
        }
    }

    drop(render_pass);
    renderer
        .get_queue()
        .submit(std::iter::once(encoder.finish()));

    // Save to PNG
    save_texture_to_png(
        renderer,
        &output_texture,
        WIDTH,
        HEIGHT,
        &format!("output/{}", filename),
    );
    println!("  ✓ Saved to output/{}", filename);

    Ok(())
}

fn save_texture_to_png(
    renderer: &ShapeRenderer,
    texture: &wgpu::Texture,
    width: u32,
    height: u32,
    filename: &str,
) {
    // Calculate aligned bytes per row (must be multiple of 256)
    const COPY_BYTES_PER_ROW_ALIGNMENT: u32 = 256;
    let unpadded_bytes_per_row = width * 4;
    let padded_bytes_per_row = (unpadded_bytes_per_row + COPY_BYTES_PER_ROW_ALIGNMENT - 1)
        / COPY_BYTES_PER_ROW_ALIGNMENT
        * COPY_BYTES_PER_ROW_ALIGNMENT;

    let buffer_size = (padded_bytes_per_row * height) as wgpu::BufferAddress;

    let staging_buffer = renderer
        .get_device()
        .create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

    let mut encoder =
        renderer
            .get_device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
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
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );

    renderer
        .get_queue()
        .submit(std::iter::once(encoder.finish()));

    let buffer_slice = staging_buffer.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
        tx.send(result).unwrap();
    });

    let timeout = std::time::Instant::now();
    loop {
        match rx.try_recv() {
            Ok(Ok(())) => break,
            Ok(Err(e)) => panic!("Buffer mapping failed: {:?}", e),
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                if timeout.elapsed().as_secs() > 10 {
                    panic!("Buffer mapping timed out");
                }
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

    let data = buffer_slice.get_mapped_range();

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
