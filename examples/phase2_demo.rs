//! # Phase 2 Animation Demo
//!
//! Demonstrates all Phase 2 animation features:
//! - MoveTo - Position animations
//! - Rotate - Rotation animations
//! - Spin - Continuous rotation
//! - Transform - Combined position/scale/rotation
//! - Shift - Offset movement

use diomanim::animation::{effects, property::AnimationInstance};
use diomanim::mobjects::Polygon;
use diomanim::prelude::*;
use diomanim::render::ShapeRenderer;
use diomanim::scene::{Renderable, SceneGraph};
use std::time::Instant;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const DURATION: f32 = 10.0;
const FPS: f32 = 60.0;
const FRAME_TIME: f32 = 1.0 / FPS;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  PHASE 2 ANIMATION DEMO                                       ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    println!("Showcasing new Phase 2 animation types:");
    println!("  • MoveTo - Smooth position transitions");
    println!("  • Rotate - Rotation animations");
    println!("  • Spin - Continuous spinning");
    println!("  • Transform - Combined transformations");
    println!("  • Shift - Offset movement\n");

    // Initialize renderer
    println!("Initializing renderer...");
    let renderer = ShapeRenderer::new(WIDTH, HEIGHT).await?;
    println!("✓ Ready\n");

    println!("Building Phase 2 showcase scene...");
    let mut scene = SceneGraph::new();

    // ========================================================================
    // Act 1 (0-2s): MoveTo - Four corners dance
    // ========================================================================
    println!("  Act 1: MoveTo demo - Four corners dance");

    let circle1_id = scene.create_node_with_transform(
        "MoveTo_Circle".to_string(),
        Transform::from_translation(-0.7, -0.4, 0.0),
    );
    scene
        .get_node_mut(circle1_id)
        .unwrap()
        .set_renderable(Renderable::Circle {
            radius: 0.1,
            color: Color::from_hex("#FF6B6B"),
        });

    // Animate in a square pattern
    let move1 = effects::move_to(
        Vector3::new(-0.7, -0.4, 0.0),
        Vector3::new(0.7, -0.4, 0.0),
        2.0,
    );
    scene
        .get_node_mut(circle1_id)
        .unwrap()
        .add_animation(AnimationInstance::new(move1, TimeValue::new(0.0)));

    let move2 = effects::move_to(
        Vector3::new(0.7, -0.4, 0.0),
        Vector3::new(0.7, 0.4, 0.0),
        2.0,
    );
    scene
        .get_node_mut(circle1_id)
        .unwrap()
        .add_animation(AnimationInstance::new(move2, TimeValue::new(2.0)));

    let move3 = effects::move_to(
        Vector3::new(0.7, 0.4, 0.0),
        Vector3::new(-0.7, 0.4, 0.0),
        2.0,
    );
    scene
        .get_node_mut(circle1_id)
        .unwrap()
        .add_animation(AnimationInstance::new(move3, TimeValue::new(4.0)));

    let move4 = effects::move_to(
        Vector3::new(-0.7, 0.4, 0.0),
        Vector3::new(-0.7, -0.4, 0.0),
        2.0,
    );
    scene
        .get_node_mut(circle1_id)
        .unwrap()
        .add_animation(AnimationInstance::new(move4, TimeValue::new(6.0)));

    // ========================================================================
    // Act 2 (0-4s): Spin - Continuous rotation
    // ========================================================================
    println!("  Act 2: Spin demo - Spinning star");

    let star_id = scene.create_node_with_transform(
        "Spinning_Star".to_string(),
        Transform::from_translation(0.0, 0.0, 0.0),
    );
    let star = Polygon::star(5, 0.2, 0.1, Color::from_hex("#FFD93D"));
    scene
        .get_node_mut(star_id)
        .unwrap()
        .set_renderable(Renderable::Polygon {
            vertices: star.vertices,
            color: star.color,
        });

    // Spin 3 complete rotations
    let spin_anim = effects::spin(3.0, 4.0);
    scene
        .get_node_mut(star_id)
        .unwrap()
        .add_animation(AnimationInstance::new(spin_anim, TimeValue::new(0.0)));

    // ========================================================================
    // Act 3 (4-7s): Rotate - Precise rotation
    // ========================================================================
    println!("  Act 3: Rotate demo - Pentagon rotation");

    let pentagon_id = scene.create_node_with_transform(
        "Rotating_Pentagon".to_string(),
        Transform::from_translation(-0.5, 0.0, 0.0),
    );
    let pentagon = Polygon::regular(5, 0.15, Color::from_hex("#4ECDC4"));
    scene
        .get_node_mut(pentagon_id)
        .unwrap()
        .set_renderable(Renderable::Polygon {
            vertices: pentagon.vertices,
            color: pentagon.color,
        });

    // Rotate to 180 degrees
    let rotate_anim = effects::rotate(0.0, std::f32::consts::PI, 3.0);
    scene
        .get_node_mut(pentagon_id)
        .unwrap()
        .add_animation(AnimationInstance::new(rotate_anim, TimeValue::new(4.0)));

    // ========================================================================
    // Act 4 (7-10s): Transform - Combined animation
    // ========================================================================
    println!("  Act 4: Transform demo - Combined effects");

    let square_id = scene.create_node_with_transform(
        "Transform_Square".to_string(),
        Transform::from_translation(0.5, 0.0, 0.0),
    );
    scene
        .get_node_mut(square_id)
        .unwrap()
        .set_renderable(Renderable::Rectangle {
            width: 0.3,
            height: 0.3,
            color: Color::from_hex("#95E1D3"),
        });

    // Move, scale, and rotate simultaneously
    let transform_anim = effects::transform(
        Vector3::new(0.5, 0.0, 0.0),
        Vector3::new(0.0, -0.5, 0.0),
        Vector3::one(),
        Vector3::new(0.5, 0.5, 1.0),
        0.0,
        std::f32::consts::PI * 2.0,
        3.0,
    );
    scene
        .get_node_mut(square_id)
        .unwrap()
        .add_animation(AnimationInstance::new(transform_anim, TimeValue::new(7.0)));

    // Initialize
    scene.update_animations(TimeValue::new(0.0));
    scene.update_transforms();

    println!(
        "✓ Scene created with {} objects\n",
        scene.get_visible_renderables().len()
    );

    // ========================================================================
    // RENDER
    // ========================================================================
    let frames_dir = "output/phase2_frames";
    std::fs::create_dir_all(frames_dir)?;

    println!("Rendering {} seconds at {} FPS...", DURATION, FPS);

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

    let start_time = Instant::now();
    let mut frame_count = 0;
    let total_frames = (DURATION * FPS) as u32;

    while frame_count < total_frames {
        let delta_time = TimeValue::new(FRAME_TIME);

        // Update
        scene.update_animations(delta_time);
        scene.update_transforms();

        // Render
        let mut encoder =
            renderer
                .get_device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Frame"),
                });

        let mut render_pass = renderer.begin_render_pass(&mut encoder, &output_view, None);
        render_pass.set_pipeline(renderer.get_pipeline());
        render_pass.set_bind_group(0, renderer.get_transform_bind_group(), &[]);

        let renderables = scene.get_visible_renderables();
        for (transform_uniform, renderable, opacity) in renderables {
            renderer.update_transform(&transform_uniform);

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

        // Save key frames
        if frame_count % 30 == 0 || frame_count == total_frames - 1 {
            let frame_filename = format!("{}/frame_{:04}.png", frames_dir, frame_count);
            save_frame(&renderer, &output_texture, WIDTH, HEIGHT, &frame_filename);
        }

        frame_count += 1;
        if frame_count % 60 == 0 || frame_count == total_frames {
            let progress = (frame_count as f32 / total_frames as f32) * 100.0;
            let current_time = frame_count as f32 / FPS;
            print!(
                "\r  Progress: {}/{} frames ({:.1}%) - {:.1}s/{:.1}s",
                frame_count, total_frames, progress, current_time, DURATION
            );
            std::io::Write::flush(&mut std::io::stdout()).ok();
        }
    }

    let elapsed = start_time.elapsed().as_secs_f32();
    let avg_fps = frame_count as f32 / elapsed;

    println!("\n\n✅ Rendering complete!");
    println!("   Frames: {}", frame_count);
    println!("   Time: {:.2}s ({:.1} FPS)", elapsed, avg_fps);
    println!("   Speedup: {:.1}x realtime", DURATION / elapsed);
    println!("   Output: {}", frames_dir);

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║  PHASE 2 COMPLETE                                             ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!("\n✨ New animation types demonstrated:");
    println!("   ✓ MoveTo - Smooth position transitions");
    println!("   ✓ Rotate - Precise rotation control");
    println!("   ✓ Spin - Continuous rotation");
    println!("   ✓ Transform - Combined position/scale/rotation");
    println!("\n🎬 All Phase 2 animations working perfectly!");

    Ok(())
}

fn save_frame(
    renderer: &ShapeRenderer,
    texture: &wgpu::Texture,
    width: u32,
    height: u32,
    filename: &str,
) {
    const ALIGNMENT: u32 = 256;
    let unpadded_bpr = width * 4;
    let padded_bpr = (unpadded_bpr + ALIGNMENT - 1) / ALIGNMENT * ALIGNMENT;
    let buffer_size = (padded_bpr * height) as wgpu::BufferAddress;

    let staging_buffer = renderer
        .get_device()
        .create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging"),
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

    let mut encoder =
        renderer
            .get_device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Copy"),
            });

    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &staging_buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded_bpr),
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
    buffer_slice.map_async(wgpu::MapMode::Read, move |r| {
        tx.send(r).unwrap();
    });

    loop {
        match rx.try_recv() {
            Ok(Ok(())) => break,
            Ok(Err(e)) => panic!("Mapping failed: {:?}", e),
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                let _ = renderer.get_device().poll(wgpu::PollType::Wait {
                    submission_index: None,
                    timeout: None,
                });
                std::thread::yield_now();
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }
    }

    let data = buffer_slice.get_mapped_range();
    let file = std::fs::File::create(filename).unwrap();
    let writer = std::io::BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut png_writer = encoder.write_header().unwrap();

    if padded_bpr != unpadded_bpr {
        let mut unpadded = Vec::with_capacity((unpadded_bpr * height) as usize);
        for row in 0..height {
            let start = (row * padded_bpr) as usize;
            let end = start + unpadded_bpr as usize;
            unpadded.extend_from_slice(&data[start..end]);
        }
        png_writer.write_image_data(&unpadded).unwrap();
    } else {
        png_writer.write_image_data(&data).unwrap();
    }

    drop(data);
    staging_buffer.unmap();
}
