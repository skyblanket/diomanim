//! Animation Test - Test FadeIn, FadeOut, and Create effects
//!
//! This example tests the animation effects system by creating three circles
//! that demonstrate different animation effects.

use diomanim::prelude::*;
use diomanim::scene::{SceneGraph, Renderable};
use diomanim::animation::effects;
use diomanim::animation::property::AnimationInstance;
use diomanim::render::ShapeRenderer;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const DURATION: f32 = 3.0;
const FPS: f32 = 30.0;
const FRAME_TIME: f32 = 1.0 / FPS;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Animation Effects Test                                       ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    // Initialize renderer
    println!("Initializing GPU renderer...");
    let renderer = ShapeRenderer::new(WIDTH, HEIGHT).await?;
    println!("✓ WebGPU renderer ready ({}x{})", WIDTH, HEIGHT);

    println!("\nBuilding animation test scene...");
    let mut scene = SceneGraph::new();

    // ========================================================================
    // Circle 1: FadeIn animation (left)
    // ========================================================================
    println!("  • Adding FadeIn circle (left)");
    let circle1_id = scene.create_node_with_transform(
        "FadeIn_Circle".to_string(),
        Transform::from_translation(-0.5, 0.0, 0.0),
    );
    scene.get_node_mut(circle1_id).unwrap().set_renderable(Renderable::Circle {
        radius: 0.2,
        color: Color::RED,
    });

    // Start with opacity 0, fade in over DURATION seconds
    scene.get_node_mut(circle1_id).unwrap().opacity = 0.0;
    let fade_in_anim = effects::fade_in(DURATION);
    scene.get_node_mut(circle1_id).unwrap()
        .add_animation(AnimationInstance::new(fade_in_anim, TimeValue::new(0.0)));

    // ========================================================================
    // Circle 2: FadeOut animation (center)
    // ========================================================================
    println!("  • Adding FadeOut circle (center)");
    let circle2_id = scene.create_node_with_transform(
        "FadeOut_Circle".to_string(),
        Transform::from_translation(0.0, 0.0, 0.0),
    );
    scene.get_node_mut(circle2_id).unwrap().set_renderable(Renderable::Circle {
        radius: 0.2,
        color: Color::GREEN,
    });

    // Start with opacity 1, fade out over DURATION seconds
    let fade_out_anim = effects::fade_out(DURATION);
    scene.get_node_mut(circle2_id).unwrap()
        .add_animation(AnimationInstance::new(fade_out_anim, TimeValue::new(0.0)));

    // ========================================================================
    // Circle 3: Create animation (right)
    // ========================================================================
    println!("  • Adding Create circle (right)");
    let circle3_id = scene.create_node_with_transform(
        "Create_Circle".to_string(),
        Transform::from_translation(0.5, 0.0, 0.0),
    );
    scene.get_node_mut(circle3_id).unwrap().set_renderable(Renderable::Circle {
        radius: 0.2,
        color: Color::BLUE,
    });

    // Start with opacity 0 and scale 0, grow and fade in together
    scene.get_node_mut(circle3_id).unwrap().opacity = 0.0;
    scene.get_node_mut(circle3_id).unwrap().world_transform.scale = Vector3::zero();
    let create_anim = effects::create(DURATION);
    scene.get_node_mut(circle3_id).unwrap()
        .add_animation(AnimationInstance::new(create_anim, TimeValue::new(0.0)));

    println!("✓ Created scene with {} animated objects", scene.get_visible_renderables().len());

    // ========================================================================
    // RENDER LOOP
    // ========================================================================
    println!("\nRendering {} seconds at {} FPS...", DURATION, FPS);

    let frames_dir = "output/animation_frames";
    std::fs::create_dir_all(frames_dir)?;

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

    let total_frames = (DURATION * FPS) as u32;

    // Render key frames: 0%, 25%, 50%, 75%, 100%
    let key_frame_percentages = vec![0, 25, 50, 75, 100];

    // Initialize animation time tracking
    let mut last_time = TimeValue::new(0.0);

    for percentage in &key_frame_percentages {
        let frame_idx = (total_frames as f32 * (*percentage) as f32 / 100.0) as u32;
        let current_time = TimeValue::new(frame_idx as f32 * FRAME_TIME);
        let delta_time = current_time - last_time;

        println!("\n  Rendering frame {} ({}% - t={:.2}s, delta={:.3}s)",
            frame_idx, percentage, current_time.seconds(), delta_time.seconds());

        // Update animations by the actual time delta
        scene.update_animations(delta_time);
        scene.update_transforms();

        last_time = current_time;

        // Debug: Print opacity values
        if let Some(node1) = scene.get_node(circle1_id) {
            println!("    FadeIn circle opacity: {:.3}", node1.opacity);
        }
        if let Some(node2) = scene.get_node(circle2_id) {
            println!("    FadeOut circle opacity: {:.3}", node2.opacity);
        }
        if let Some(node3) = scene.get_node(circle3_id) {
            println!("    Create circle opacity: {:.3}, scale: {:.3}",
                node3.opacity, node3.world_transform.scale.x);
        }

        // Create command encoder
        let mut encoder = renderer.get_device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Frame Render Encoder"),
        });

        // Begin render pass
        let mut render_pass = renderer.begin_render_pass(&mut encoder, &output_view, None);
        render_pass.set_pipeline(renderer.get_pipeline());
        render_pass.set_bind_group(0, renderer.get_transform_bind_group(), &[]);

        // Render all visible objects
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
                _ => {}
            }
        }

        // End render pass
        drop(render_pass);

        // Submit command
        renderer.get_queue().submit(std::iter::once(encoder.finish()));

        // Save frame to PNG
        let frame_filename = format!("{}/frame_{:03}_{}pct.png", frames_dir, frame_idx, percentage);
        save_texture_to_png(&renderer, &output_texture, WIDTH, HEIGHT, &frame_filename);
        println!("    ✓ Saved {}", frame_filename);
    }

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Test Complete                                                ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!("\n📊 Results:");
    println!("  Frames rendered: {}", key_frame_percentages.len());
    println!("  Output directory: {}", frames_dir);
    println!("\n✨ Animations tested:");
    println!("  ✓ FadeIn (left circle - red)");
    println!("  ✓ FadeOut (center circle - green)");
    println!("  ✓ Create (right circle - blue)");
    println!("\n🔍 Visual verification:");
    println!("  • FadeIn: Should go from invisible (0%) to fully visible (100%)");
    println!("  • FadeOut: Should go from fully visible (0%) to invisible (100%)");
    println!("  • Create: Should grow from point while fading in");

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

    // Poll the device to process GPU commands
    let timeout = std::time::Instant::now();
    loop {
        match rx.try_recv() {
            Ok(Ok(())) => break,
            Ok(Err(e)) => panic!("Buffer mapping failed: {:?}", e),
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                if timeout.elapsed().as_secs() > 10 {
                    panic!("Buffer mapping timed out after 10 seconds");
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
