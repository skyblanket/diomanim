//! Video Export Demo - Generate animation frames and export to MP4
//!
//! This example demonstrates:
//! - Rendering an animated scene frame-by-frame
//! - Saving frames as PNG files
//! - Exporting frames to MP4 video using ffmpeg
//!
//! Requirements: ffmpeg must be installed on the system

use diomanim::animation::property::{AnimationClip, AnimationInstance, AnimationTrack, Keyframe};
use diomanim::export::export_video;
use diomanim::mobjects::Polygon;
use diomanim::prelude::*;
use diomanim::render::ShapeRenderer;
use diomanim::scene::{Renderable, SceneGraph};
use std::time::Instant;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const DURATION: f32 = 3.0;
const FPS: f32 = 30.0;
const FRAME_TIME: f32 = 1.0 / FPS;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Diomanim Video Export Demo                                  ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    // Initialize renderer
    println!("Initializing GPU renderer...");
    let renderer = ShapeRenderer::new(WIDTH, HEIGHT).await?;
    println!("✓ WebGPU renderer ready ({}x{})", WIDTH, HEIGHT);

    println!("\nBuilding animated scene...");
    let mut scene = SceneGraph::new();

    // ========================================================================
    // Create animated shapes
    // ========================================================================

    // Circle that moves across the screen
    let circle_id = scene.create_node_with_transform(
        "Moving_Circle".to_string(),
        Transform::from_translation(-0.6, 0.0, 0.0),
    );
    scene
        .get_node_mut(circle_id)
        .unwrap()
        .set_renderable(Renderable::Circle {
            radius: 0.15,
            color: Color::RED,
        });

    // Animate circle movement
    let mut move_anim = AnimationClip::new("move".to_string());
    let mut move_track = AnimationTrack::new("position".to_string());
    move_track.add_keyframe(Keyframe::new(
        TimeValue::new(0.0),
        Vector3::new(-0.6, 0.0, 0.0),
    ));
    move_track.add_keyframe(Keyframe::new(
        TimeValue::new(DURATION),
        Vector3::new(0.6, 0.0, 0.0),
    ));
    move_anim.add_track(move_track);
    scene
        .get_node_mut(circle_id)
        .unwrap()
        .add_animation(AnimationInstance::new(move_anim, TimeValue::new(0.0)));

    // Star that pulses
    let star_id = scene.create_node_with_transform(
        "Pulsing_Star".to_string(),
        Transform::from_translation(0.0, 0.0, 0.0),
    );
    let star = Polygon::star(5, 0.2, 0.1, Color::YELLOW);
    scene
        .get_node_mut(star_id)
        .unwrap()
        .set_renderable(Renderable::Polygon {
            vertices: star.vertices,
            color: star.color,
        });

    // Animate star pulsing
    let mut pulse_anim = AnimationClip::new("pulse".to_string());
    let mut pulse_track = AnimationTrack::new("scale".to_string());
    pulse_track.add_keyframe(Keyframe::new(TimeValue::new(0.0), Vector3::one()));
    pulse_track.add_keyframe(Keyframe::new(
        TimeValue::new(DURATION / 2.0),
        Vector3::new(1.5, 1.5, 1.5),
    ));
    pulse_track.add_keyframe(Keyframe::new(TimeValue::new(DURATION), Vector3::one()));
    pulse_anim.add_track(pulse_track);
    scene
        .get_node_mut(star_id)
        .unwrap()
        .add_animation(AnimationInstance::new(pulse_anim, TimeValue::new(0.0)));

    println!(
        "✓ Created scene with {} objects",
        scene.get_visible_renderables().len()
    );

    // ========================================================================
    // Render frames
    // ========================================================================
    let frames_dir = "output/video_frames";
    std::fs::create_dir_all(frames_dir)?;

    println!(
        "\nRendering {} frames at {} FPS...",
        (DURATION * FPS) as u32,
        FPS
    );

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

        // Update animations and transforms
        scene.update_animations(delta_time);
        scene.update_transforms();

        // Create command encoder
        let mut encoder =
            renderer
                .get_device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
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
                Renderable::Polygon { vertices, color } => {
                    renderer.draw_polygon(&vertices, apply_opacity(color), &mut render_pass);
                }
                _ => {}
            }
        }

        // End render pass
        drop(render_pass);

        // Submit command
        renderer
            .get_queue()
            .submit(std::iter::once(encoder.finish()));

        // Save frame as PNG
        let frame_filename = format!("{}/frame_{:04}.png", frames_dir, frame_count);
        save_frame_to_png(&renderer, &output_texture, WIDTH, HEIGHT, &frame_filename);

        frame_count += 1;
        if frame_count % 10 == 0 || frame_count == total_frames {
            let progress = (frame_count as f32 / total_frames as f32) * 100.0;
            print!(
                "\r  Progress: {}/{} frames ({:.1}%)",
                frame_count, total_frames, progress
            );
            std::io::Write::flush(&mut std::io::stdout()).ok();
        }
    }
    println!();

    let elapsed = start_time.elapsed().as_secs_f32();
    let avg_fps = frame_count as f32 / elapsed;

    println!("\n✅ Frame rendering complete!");
    println!("   Frames: {}", frame_count);
    println!("   Time: {:.2}s ({:.1} FPS)", elapsed, avg_fps);
    println!("   Directory: {}", frames_dir);

    // ========================================================================
    // Export to video
    // ========================================================================
    println!("\n");

    match export_video(
        frames_dir,
        "output/animation.mp4",
        WIDTH,
        HEIGHT,
        FPS as u32,
    ) {
        Ok(()) => {
            println!("\n✨ Success! Open output/animation.mp4 to view the animation.");
        }
        Err(e) => {
            println!("\n⚠️  Video export failed: {}", e);
            println!("   Frames are still available in: {}", frames_dir);
            println!("   You can manually convert them to video with ffmpeg:");
            println!("   ffmpeg -framerate {} -i {}/frame_%04d.png -c:v libx264 -pix_fmt yuv420p output/animation.mp4", FPS, frames_dir);
        }
    }

    Ok(())
}

/// Save a single frame to PNG
fn save_frame_to_png(
    renderer: &ShapeRenderer,
    texture: &wgpu::Texture,
    width: u32,
    height: u32,
    filename: &str,
) {
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
