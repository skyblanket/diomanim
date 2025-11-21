//! Diomanim Animation Pipeline Demo
//!
//! This demonstrates the complete animation pipeline:
//! - GPU-accelerated rendering with WebGPU
//! - Hierarchical scene graph with transform inheritance
//! - Keyframe animation system with interpolation
//! - Real-time frame generation and video export

use diomanim::animation::property::{AnimationClip, AnimationInstance};
use diomanim::core::*;
use diomanim::render::ShapeRenderer;
use diomanim::scene::*;
use std::time::Instant;

/// Configuration for the demo animation
const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const DURATION: f32 = 3.0;
const FPS: f32 = 30.0;
const FRAME_TIME: f32 = 1.0 / FPS;

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Diomanim v0.1.0 - GPU Animation Pipeline Demo               ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        println!("Initializing GPU renderer...");
        let mut renderer = ShapeRenderer::new(WIDTH, HEIGHT)
            .await
            .expect("Failed to create renderer");
        println!("✓ WebGPU renderer ready ({}x{})", WIDTH, HEIGHT);

        println!("\nBuilding scene graph...");
        let mut scene = SceneGraph::new();

        // Create center node
        let center_id = scene.create_node("Center".to_string());
        scene
            .get_node_mut(center_id)
            .unwrap()
            .set_renderable(Renderable::Circle {
                radius: 0.15,
                color: Color::new(0.2, 0.2, 0.2),
            });
        create_scaling_animation(&mut scene, center_id);

        // Create orbiting children
        let colors = [
            Color::RED,
            Color::GREEN,
            Color::BLUE,
            Color::YELLOW,
            Color::CYAN,
        ];
        for (i, &color) in colors.iter().enumerate() {
            let angle = (i as f32 / colors.len() as f32) * std::f32::consts::TAU;
            let orbit_radius = 0.3;

            let child_id = scene.create_node_with_transform(
                format!("Orbiter_{}", i),
                Transform::from_translation(
                    angle.cos() * orbit_radius,
                    angle.sin() * orbit_radius,
                    0.0,
                ),
            );

            scene
                .get_node_mut(child_id)
                .unwrap()
                .set_renderable(Renderable::Circle { radius: 0.1, color });

            create_rotation_animation(&mut scene, child_id, angle, i);
            scene.parent(child_id, center_id).unwrap();
        }

        println!(
            "✓ Created {} nodes with {} animations",
            colors.len() + 1,
            colors.len() + 1
        );

        println!("\nRendering frames...");

        let frames_dir = "frames";
        std::fs::create_dir_all(frames_dir).expect("Failed to create frames directory");

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

        while start_time.elapsed().as_secs_f32() < DURATION {
            let delta_time = TimeValue::new(FRAME_TIME);

            scene.update_animations(delta_time);
            scene.update_transforms();

            // Create command encoder
            let mut encoder =
                renderer
                    .get_device()
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Frame Render Encoder"),
                    });

            // Begin render pass once per frame
            let mut render_pass = renderer.begin_render_pass(&mut encoder, &output_view, None);

            render_pass.set_pipeline(renderer.get_pipeline());
            render_pass.set_bind_group(0, renderer.get_transform_bind_group(), &[]);

            let renderables = scene.get_visible_renderables();
            for (transform_uniform, renderable, opacity) in renderables {
                renderer.update_transform(&transform_uniform);

                // Apply opacity to color
                let apply_opacity = |color: Color| -> Color {
                    Color::rgba(color.r, color.g, color.b, color.a * opacity)
                };

                if let Some((radius, color)) = renderable.as_circle() {
                    let circle = diomanim::mobjects::Circle {
                        radius: *radius,
                        color: apply_opacity(*color),
                        position: Vector3::zero(),
                    };
                    renderer.draw_circle(&circle, apply_opacity(*color), &mut render_pass);
                } else if let Some((width, height, color)) = renderable.as_rectangle() {
                    renderer.draw_rectangle(
                        *width,
                        *height,
                        apply_opacity(*color),
                        &mut render_pass,
                    );
                } else if let Some((start, end, color, thickness)) = renderable.as_line() {
                    renderer.draw_line(
                        *start,
                        *end,
                        apply_opacity(*color),
                        *thickness,
                        &mut render_pass,
                    );
                } else if let Some((start, end, color, thickness)) = renderable.as_arrow() {
                    renderer.draw_arrow(
                        *start,
                        *end,
                        apply_opacity(*color),
                        *thickness,
                        &mut render_pass,
                    );
                } else if let Some((vertices, color)) = renderable.as_polygon() {
                    renderer.draw_polygon(vertices, apply_opacity(*color), &mut render_pass);
                } else if let Some((content, font_size, color)) = renderable.as_text() {
                    renderer.draw_text(
                        content,
                        *font_size,
                        apply_opacity(*color),
                        &mut render_pass,
                    );
                }
            }

            // End render pass
            drop(render_pass);

            // Submit command (once per frame instead of per object)
            renderer
                .get_queue()
                .submit(std::iter::once(encoder.finish()));

            let frame_filename = format!("{}/frame_{:05}.png", frames_dir, frame_count);
            save_texture_to_png(&renderer, &output_texture, WIDTH, HEIGHT, &frame_filename);

            frame_count += 1;
            if frame_count % 10 == 0 {
                print!("\r  Rendering... {} frames", frame_count);
            }
        }
        println!();

        println!("\nConverting frames to video...");
        let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let output_video = format!("{}/Downloads/diomanim_demo.mp4", home_dir);

        let ffmpeg_result = std::process::Command::new("ffmpeg")
            .args(&[
                "-y",
                "-framerate",
                &format!("{}", FPS),
                "-i",
                "frames/frame_%05d.png",
                "-c:v",
                "libx264",
                "-pix_fmt",
                "yuv420p",
                "-crf",
                "18",
                &output_video,
            ])
            .output();

        match ffmpeg_result {
            Ok(output) if output.status.success() => {
                println!("✓ Video saved: {}", output_video);
            }
            Ok(output) => {
                println!(
                    "✗ FFmpeg error: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
            Err(_) => {
                println!("✗ FFmpeg not found. Install with: brew install ffmpeg");
            }
        }

        std::fs::remove_dir_all(frames_dir).ok();

        println!("\n╔═══════════════════════════════════════════════════════════════╗");
        println!("║  Demo Complete                                                ║");
        println!("╚═══════════════════════════════════════════════════════════════╝");
        println!("\nStatistics:");
        println!("  Frames rendered: {} @ {} FPS", frame_count, FPS);
        println!("  Duration: {:.1}s", DURATION);
        println!("  Objects rendered: {}", colors.len() + 1);
        println!("  Total draw calls: {}", frame_count * (colors.len() + 1));
        println!();
    });
}

/// Creates a rotation animation for an orbiting object
fn create_rotation_animation(
    scene: &mut SceneGraph,
    node_id: NodeId,
    start_angle: f32,
    index: usize,
) {
    use diomanim::animation::property::{AnimationTrack, Keyframe};

    let mut animation = AnimationClip::new(format!("rotate_{}", index));
    let mut track = AnimationTrack::new("rotation".to_string());

    track.add_keyframe(Keyframe::new(
        TimeValue::new(0.0),
        Vector3::new(0.0, 0.0, start_angle),
    ));
    track.add_keyframe(Keyframe::new(
        TimeValue::new(3.0),
        Vector3::new(0.0, 0.0, start_angle + std::f32::consts::TAU),
    ));

    animation.add_track(track);
    animation.loop_animation = true;

    scene
        .get_node_mut(node_id)
        .unwrap()
        .add_animation(AnimationInstance::new(animation, TimeValue::new(0.0)));
}

/// Creates a pulsing scale animation
fn create_scaling_animation(scene: &mut SceneGraph, node_id: NodeId) {
    use diomanim::animation::property::{AnimationTrack, Keyframe};

    let mut animation = AnimationClip::new("pulse".to_string());
    let mut track = AnimationTrack::new("scale".to_string());

    track.add_keyframe(Keyframe::new(TimeValue::new(0.0), Vector3::one()));
    track.add_keyframe(Keyframe::new(
        TimeValue::new(1.5),
        Vector3::new(1.5, 1.5, 1.5),
    ));
    track.add_keyframe(Keyframe::new(TimeValue::new(3.0), Vector3::one()));

    animation.add_track(track);
    animation.loop_animation = true;

    scene
        .get_node_mut(node_id)
        .unwrap()
        .add_animation(AnimationInstance::new(animation, TimeValue::new(0.0)));
}

/// Saves a WebGPU texture to a PNG file
fn save_texture_to_png(
    renderer: &ShapeRenderer,
    texture: &wgpu::Texture,
    width: u32,
    height: u32,
    filename: &str,
) {
    let buffer_size = (width * height * 4) as wgpu::BufferAddress;

    let staging_buffer = renderer
        .get_device()
        .create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

    // Copy texture to buffer
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
                bytes_per_row: Some(width * 4),
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

    // Map the buffer synchronously using pollster
    let buffer_slice = staging_buffer.slice(..);

    // Use a channel to wait for the async operation
    let (tx, rx) = std::sync::mpsc::channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
        tx.send(result).unwrap();
    });

    // Wait for the mapping to complete
    rx.recv().unwrap().unwrap();

    // Now we can read the data
    let data = buffer_slice.get_mapped_range();

    // Write to PNG
    use std::fs::File;
    use std::io::BufWriter;
    let file = File::create(filename).unwrap();
    let writer = BufWriter::new(file);

    let mut png_encoder = png::Encoder::new(writer, width, height);
    png_encoder.set_color(png::ColorType::Rgba);
    png_encoder.set_depth(png::BitDepth::Eight);
    let mut png_writer = png_encoder.write_header().unwrap();
    png_writer.write_image_data(&data).unwrap();

    drop(png_writer);
    drop(data);
    staging_buffer.unmap();
}
