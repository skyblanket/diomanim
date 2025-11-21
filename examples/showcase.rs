//! # Diomanim Showcase - Complete Feature Demo
//!
//! This example demonstrates all Phase 1 features of diomanim:
//! - All shape types (Circle, Rectangle, Polygon, Line, Arrow)
//! - Animation effects (FadeIn, FadeOut, Create, GrowFromCenter)
//! - Transform animations (position, rotation, scale)
//! - Opacity blending
//! - Video export to MP4
//!
//! This is the definitive example showing diomanim's capabilities.

use diomanim::animation::effects;
use diomanim::animation::property::{
    AnimationClip, AnimationInstance, AnimationTrack, InterpolationType, Keyframe,
};
use diomanim::export::export_video;
use diomanim::mobjects::Polygon;
use diomanim::prelude::*;
use diomanim::render::ShapeRenderer;
use diomanim::scene::{Renderable, SceneGraph};
use std::time::Instant;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const DURATION: f32 = 5.0;
const FPS: f32 = 60.0;
const FRAME_TIME: f32 = 1.0 / FPS;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  DIOMANIM SHOWCASE - Complete Feature Demonstration          ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    println!("This demo showcases:");
    println!("  ✓ All shape types");
    println!("  ✓ Animation effects");
    println!("  ✓ Transform animations");
    println!("  ✓ Opacity blending");
    println!("  ✓ Video export\n");

    // Initialize renderer
    println!("Initializing GPU renderer ({}x{})...", WIDTH, HEIGHT);
    let renderer = ShapeRenderer::new(WIDTH, HEIGHT).await?;
    println!("✓ WebGPU renderer ready\n");

    println!("Building showcase scene...");
    let mut scene = SceneGraph::new();

    // ========================================================================
    // Act 1: TITLE - FadeIn effect (0-1s)
    // ========================================================================
    println!("  Act 1: Title with FadeIn effect");

    // Title circle
    let title_circle_id = scene.create_node_with_transform(
        "Title_Circle".to_string(),
        Transform::from_translation(0.0, 0.3, 0.0),
    );
    scene
        .get_node_mut(title_circle_id)
        .unwrap()
        .set_renderable(Renderable::Circle {
            radius: 0.25,
            color: Color::from_hex("#FF6B6B"),
        });
    let fade_in = effects::fade_in(1.0);
    scene
        .get_node_mut(title_circle_id)
        .unwrap()
        .add_animation(AnimationInstance::new(fade_in, TimeValue::new(0.0)));

    // ========================================================================
    // Act 2: SHAPES - Create effects (1-2s)
    // ========================================================================
    println!("  Act 2: Shape showcase with Create effects");

    // Circle (left)
    let shape1_id = scene.create_node_with_transform(
        "Circle_Shape".to_string(),
        Transform::from_translation(-0.5, -0.2, 0.0),
    );
    scene
        .get_node_mut(shape1_id)
        .unwrap()
        .set_renderable(Renderable::Circle {
            radius: 0.15,
            color: Color::from_hex("#4ECDC4"),
        });
    let create1 = effects::create(0.8);
    scene
        .get_node_mut(shape1_id)
        .unwrap()
        .add_animation(AnimationInstance::new(create1, TimeValue::new(1.0)));

    // Rectangle (center-left)
    let shape2_id = scene.create_node_with_transform(
        "Rectangle_Shape".to_string(),
        Transform::from_translation(-0.15, -0.2, 0.0),
    );
    scene
        .get_node_mut(shape2_id)
        .unwrap()
        .set_renderable(Renderable::Rectangle {
            width: 0.3,
            height: 0.3,
            color: Color::from_hex("#95E1D3"),
        });
    let create2 = effects::create(0.8);
    scene
        .get_node_mut(shape2_id)
        .unwrap()
        .add_animation(AnimationInstance::new(create2, TimeValue::new(1.2)));

    // Pentagon (center-right)
    let shape3_id = scene.create_node_with_transform(
        "Pentagon_Shape".to_string(),
        Transform::from_translation(0.15, -0.2, 0.0),
    );
    let pentagon = Polygon::regular(5, 0.15, Color::from_hex("#F38181"));
    scene
        .get_node_mut(shape3_id)
        .unwrap()
        .set_renderable(Renderable::Polygon {
            vertices: pentagon.vertices,
            color: pentagon.color,
        });
    let create3 = effects::create(0.8);
    scene
        .get_node_mut(shape3_id)
        .unwrap()
        .add_animation(AnimationInstance::new(create3, TimeValue::new(1.4)));

    // Star (right)
    let shape4_id = scene.create_node_with_transform(
        "Star_Shape".to_string(),
        Transform::from_translation(0.5, -0.2, 0.0),
    );
    let star = Polygon::star(5, 0.15, 0.07, Color::from_hex("#FFC947"));
    scene
        .get_node_mut(shape4_id)
        .unwrap()
        .set_renderable(Renderable::Polygon {
            vertices: star.vertices,
            color: star.color,
        });
    let create4 = effects::create(0.8);
    scene
        .get_node_mut(shape4_id)
        .unwrap()
        .add_animation(AnimationInstance::new(create4, TimeValue::new(1.6)));

    // ========================================================================
    // Act 3: MOVEMENT - Position animations (2-4s)
    // ========================================================================
    println!("  Act 3: Transform animations with easing");

    // Moving circle with ease-in-out
    let mover1_id = scene.create_node_with_transform(
        "Mover1".to_string(),
        Transform::from_translation(-0.7, 0.5, 0.0),
    );
    scene
        .get_node_mut(mover1_id)
        .unwrap()
        .set_renderable(Renderable::Circle {
            radius: 0.1,
            color: Color::from_hex("#A8E6CF"),
        });

    let mut move_anim1 = AnimationClip::new("move1".to_string());
    let mut move_track1 = AnimationTrack::new("position".to_string());
    let kf1 = Keyframe::new(TimeValue::new(2.0), Vector3::new(-0.7, 0.5, 0.0))
        .with_interpolation(InterpolationType::EaseInOut);
    let kf2 = Keyframe::new(TimeValue::new(4.0), Vector3::new(0.7, 0.5, 0.0))
        .with_interpolation(InterpolationType::EaseInOut);
    move_track1.add_keyframe(kf1);
    move_track1.add_keyframe(kf2);
    move_anim1.add_track(move_track1);
    scene
        .get_node_mut(mover1_id)
        .unwrap()
        .add_animation(AnimationInstance::new(move_anim1, TimeValue::new(0.0)));

    // Pulsing star
    let pulser_id = scene.create_node_with_transform(
        "Pulser".to_string(),
        Transform::from_translation(0.0, 0.5, 0.0),
    );
    let pulser_star = Polygon::star(6, 0.12, 0.06, Color::from_hex("#FFD93D"));
    scene
        .get_node_mut(pulser_id)
        .unwrap()
        .set_renderable(Renderable::Polygon {
            vertices: pulser_star.vertices,
            color: pulser_star.color,
        });

    let mut pulse_anim = AnimationClip::new("pulse".to_string());
    let mut pulse_track = AnimationTrack::new("scale".to_string());
    pulse_track.add_keyframe(Keyframe::new(TimeValue::new(2.0), Vector3::one()));
    pulse_track.add_keyframe(Keyframe::new(
        TimeValue::new(3.0),
        Vector3::new(1.5, 1.5, 1.5),
    ));
    pulse_track.add_keyframe(Keyframe::new(TimeValue::new(4.0), Vector3::one()));
    pulse_anim.add_track(pulse_track);
    scene
        .get_node_mut(pulser_id)
        .unwrap()
        .add_animation(AnimationInstance::new(pulse_anim, TimeValue::new(0.0)));

    // ========================================================================
    // Act 4: FINALE - FadeOut (4-5s)
    // ========================================================================
    println!("  Act 4: Finale with FadeOut");

    // Add fade-out to all objects
    for node_id in [
        title_circle_id,
        shape1_id,
        shape2_id,
        shape3_id,
        shape4_id,
        mover1_id,
        pulser_id,
    ] {
        let fade_out = effects::fade_out(1.0);
        scene
            .get_node_mut(node_id)
            .unwrap()
            .add_animation(AnimationInstance::new(fade_out, TimeValue::new(4.0)));
    }

    // Initialize all animations
    scene.update_animations(TimeValue::new(0.0));
    scene.update_transforms();

    let object_count = scene.get_visible_renderables().len();
    println!("✓ Scene created with {} objects\n", object_count);

    // ========================================================================
    // RENDER FRAMES
    // ========================================================================
    let frames_dir = "output/showcase_frames";
    std::fs::create_dir_all(frames_dir)?;

    println!(
        "Rendering animation ({} seconds at {} FPS)...",
        DURATION, FPS
    );
    let total_frames = (DURATION * FPS) as u32;

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

    while frame_count < total_frames {
        let delta_time = TimeValue::new(FRAME_TIME);

        // Update animations and transforms
        scene.update_animations(delta_time);
        scene.update_transforms();

        // Create command encoder
        let mut encoder =
            renderer
                .get_device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Frame Encoder"),
                });

        // Begin render pass
        let mut render_pass = renderer.begin_render_pass(&mut encoder, &output_view, None);
        render_pass.set_pipeline(renderer.get_pipeline());
        render_pass.set_bind_group(0, renderer.get_transform_bind_group(), &[]);

        // Render all objects
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

        // Save frame
        let frame_filename = format!("{}/frame_{:04}.png", frames_dir, frame_count);
        save_frame(&renderer, &output_texture, WIDTH, HEIGHT, &frame_filename);

        frame_count += 1;
        if frame_count % 30 == 0 || frame_count == total_frames {
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
    println!("   Time: {:.2}s (avg {:.1} FPS)", elapsed, avg_fps);
    println!("   Speedup: {:.1}x realtime", DURATION / elapsed);

    // ========================================================================
    // EXPORT VIDEO
    // ========================================================================
    println!("\n");
    match export_video(frames_dir, "output/showcase.mp4", WIDTH, HEIGHT, FPS as u32) {
        Ok(()) => {
            println!("\n╔═══════════════════════════════════════════════════════════════╗");
            println!("║  SHOWCASE COMPLETE                                            ║");
            println!("╚═══════════════════════════════════════════════════════════════╝");
            println!("\n✨ Video ready: output/showcase.mp4");
            println!("   Resolution: {}x{}", WIDTH, HEIGHT);
            println!("   Duration: {}s", DURATION);
            println!("   Frame rate: {} FPS", FPS);
            println!("\n🎬 Features demonstrated:");
            println!("   ✓ Circle, Rectangle, Pentagon, Star shapes");
            println!("   ✓ FadeIn, FadeOut, Create animations");
            println!("   ✓ Position and scale transforms");
            println!("   ✓ Ease-in-out interpolation");
            println!("   ✓ Opacity blending");
            println!("   ✓ MP4 video export");
        }
        Err(e) => {
            println!("\n⚠️  Video export failed: {}", e);
            println!("   Frames available in: {}", frames_dir);
        }
    }

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
