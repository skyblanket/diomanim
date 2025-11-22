//! Gradient Descent Video Export
//! Renders the gradient descent optimization animation to MP4

use diomanim::animation::effects;
use diomanim::animation::property::AnimationInstance;
use diomanim::export::export_video;
use diomanim::prelude::*;
use diomanim::render::ShapeRenderer;
use diomanim::scene::{Renderable, SceneGraph};
use std::time::Instant;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const DURATION: f32 = 12.0;
const FPS: f32 = 60.0;
const FRAME_TIME: f32 = 1.0 / FPS;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Gradient Descent - Video Export                             ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    // Initialize renderer
    println!("Initializing GPU renderer...");
    let mut renderer = ShapeRenderer::new(WIDTH, HEIGHT).await?;
    renderer.init_text_rendering(48.0)?;
    println!("✓ WebGPU renderer ready ({}x{})", WIDTH, HEIGHT);

    println!("\nBuilding gradient descent scene...");
    let mut scene = build_gradient_descent_scene();

    println!("✓ Scene created");

    // Render frames
    let frames_dir = "output/gradient_frames";
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
            } else if let Some((content, font_size, color)) = renderable.as_text() {
                renderer.draw_text(content, *font_size, apply_opacity(*color), &mut render_pass);
            } else if let Some((latex, font_size, color)) = renderable.as_math() {
                renderer.draw_math(latex, *font_size, apply_opacity(*color), &mut render_pass);
            }
        }

        drop(render_pass);

        renderer
            .get_queue()
            .submit(std::iter::once(encoder.finish()));

        let frame_filename = format!("{}/frame_{:04}.png", frames_dir, frame_count);
        save_frame_to_png(&renderer, &output_texture, WIDTH, HEIGHT, &frame_filename);

        frame_count += 1;
        if frame_count % 60 == 0 || frame_count == total_frames {
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

    // Export to video
    println!("\n");
    match export_video(
        frames_dir,
        "output/gradient_descent.mp4",
        WIDTH,
        HEIGHT,
        FPS as u32,
    ) {
        Ok(()) => {
            println!("\n✨ Success! Video saved to output/gradient_descent.mp4");
            println!("📹 Resolution: {}x{} @ {} FPS", WIDTH, HEIGHT, FPS as u32);
            println!("⏱️  Duration: {:.1}s", DURATION);
        }
        Err(e) => {
            println!("\n⚠️  Video export failed: {}", e);
            println!("   Frames available in: {}", frames_dir);
        }
    }

    Ok(())
}

fn build_gradient_descent_scene() -> SceneGraph {
    let mut scene = SceneGraph::new();

    // Title
    let title_id = scene.create_node_with_transform(
        "Title".to_string(),
        Transform::from_translation(0.0, 0.85, 0.0),
    );
    scene
        .get_node_mut(title_id)
        .unwrap()
        .set_renderable(Renderable::Text {
            content: "Gradient Descent Optimization".to_string(),
            font_size: 56.0,
            color: Color::new(1.0, 1.0, 1.0),
        });
    scene
        .get_node_mut(title_id)
        .unwrap()
        .add_animation(AnimationInstance::new(
            effects::fade_in(1.0),
            TimeValue::new(0.0),
        ));

    // Equations
    let eq1_id = scene.create_node_with_transform(
        "Eq1".to_string(),
        Transform::from_translation(-0.5, 0.6, 0.0),
    );
    scene
        .get_node_mut(eq1_id)
        .unwrap()
        .set_renderable(Renderable::Math {
            latex: "\\nabla f = 2x".to_string(),
            font_size: 42.0,
            color: Color::new(0.3, 0.8, 1.0),
        });
    scene
        .get_node_mut(eq1_id)
        .unwrap()
        .add_animation(AnimationInstance::new(
            effects::fade_in(0.8),
            TimeValue::new(1.0),
        ));

    let eq2_id = scene.create_node_with_transform(
        "Eq2".to_string(),
        Transform::from_translation(-0.6, 0.4, 0.0),
    );
    scene
        .get_node_mut(eq2_id)
        .unwrap()
        .set_renderable(Renderable::Math {
            latex: "x_{t+1} = x_t - \\alpha \\nabla f".to_string(),
            font_size: 40.0,
            color: Color::new(0.3, 1.0, 0.6),
        });
    scene
        .get_node_mut(eq2_id)
        .unwrap()
        .add_animation(AnimationInstance::new(
            effects::fade_in(0.8),
            TimeValue::new(1.5),
        ));

    let eq3_id = scene.create_node_with_transform(
        "Eq3".to_string(),
        Transform::from_translation(-0.5, 0.2, 0.0),
    );
    scene
        .get_node_mut(eq3_id)
        .unwrap()
        .set_renderable(Renderable::Math {
            latex: "\\alpha = 0.1".to_string(),
            font_size: 38.0,
            color: Color::new(1.0, 0.8, 0.3),
        });
    scene
        .get_node_mut(eq3_id)
        .unwrap()
        .add_animation(AnimationInstance::new(
            effects::fade_in(0.8),
            TimeValue::new(2.0),
        ));

    // Gradient descent steps
    let steps = vec![0.8, 0.64, 0.512, 0.41, 0.328, 0.26];
    let y_offset = -0.2;
    let x_scale = 0.8;
    let y_scale = 0.4;

    for (i, &x) in steps.iter().enumerate() {
        let y = x * x;
        let pos_x = (x - 0.5) * x_scale;
        let pos_y = y_offset - y * y_scale;

        let point_id = scene.create_node_with_transform(
            format!("Point{}", i),
            Transform::from_translation(pos_x, pos_y, 0.0),
        );
        let t = i as f32 / (steps.len() - 1) as f32;
        let color = Color::new(0.3 + t * 0.6, 0.5 + t * 0.5, 1.0 - t * 0.4);

        scene
            .get_node_mut(point_id)
            .unwrap()
            .set_renderable(Renderable::Circle {
                radius: 0.04,
                color,
            });
        scene
            .get_node_mut(point_id)
            .unwrap()
            .add_animation(AnimationInstance::new(
                effects::grow_from_center(0.5),
                TimeValue::new(2.5 + i as f32 * 0.8),
            ));

        let label_id = scene.create_node_with_transform(
            format!("Label{}", i),
            Transform::from_translation(pos_x, pos_y - 0.12, 0.0),
        );
        scene
            .get_node_mut(label_id)
            .unwrap()
            .set_renderable(Renderable::Text {
                content: format!("{}", i),
                font_size: 24.0,
                color: Color::new(0.7, 0.7, 0.7),
            });
        scene
            .get_node_mut(label_id)
            .unwrap()
            .add_animation(AnimationInstance::new(
                effects::fade_in(0.3),
                TimeValue::new(2.7 + i as f32 * 0.8),
            ));
    }

    // Arrows
    for i in 0..(steps.len() - 1) {
        let x1 = (steps[i] - 0.5) * x_scale;
        let y1 = y_offset - steps[i] * steps[i] * y_scale;
        let x2 = (steps[i + 1] - 0.5) * x_scale;
        let y2 = y_offset - steps[i + 1] * steps[i + 1] * y_scale;

        let arrow_id = scene.create_node("Arrow".to_string());
        scene
            .get_node_mut(arrow_id)
            .unwrap()
            .set_renderable(Renderable::Arrow {
                start: Vector3::new(x1, y1, 0.0),
                end: Vector3::new(x2, y2, 0.0),
                color: Color::new(0.8, 0.8, 0.2),
                thickness: 0.015,
            });
        scene
            .get_node_mut(arrow_id)
            .unwrap()
            .add_animation(AnimationInstance::new(
                effects::create(0.4),
                TimeValue::new(2.8 + i as f32 * 0.8),
            ));
    }

    // Loss curve
    for i in 0..20 {
        let t1 = i as f32 / 20.0;
        let t2 = (i + 1) as f32 / 20.0;
        let x1 = t1;
        let x2 = t2;
        let y1 = x1 * x1;
        let y2 = x2 * x2;
        let pos_x1 = (x1 - 0.5) * x_scale;
        let pos_y1 = y_offset - y1 * y_scale;
        let pos_x2 = (x2 - 0.5) * x_scale;
        let pos_y2 = y_offset - y2 * y_scale;

        let line_id = scene.create_node("Curve".to_string());
        scene
            .get_node_mut(line_id)
            .unwrap()
            .set_renderable(Renderable::Line {
                start: Vector3::new(pos_x1, pos_y1, 0.0),
                end: Vector3::new(pos_x2, pos_y2, 0.0),
                color: Color::new(0.5, 0.5, 0.5),
                thickness: 0.01,
            });
        scene
            .get_node_mut(line_id)
            .unwrap()
            .add_animation(AnimationInstance::new(
                effects::create(0.2),
                TimeValue::new(1.5 + i as f32 * 0.03),
            ));
    }

    // Labels
    let xlabel_id = scene.create_node_with_transform(
        "XLabel".to_string(),
        Transform::from_translation(0.5, -0.65, 0.0),
    );
    scene
        .get_node_mut(xlabel_id)
        .unwrap()
        .set_renderable(Renderable::Text {
            content: "x".to_string(),
            font_size: 32.0,
            color: Color::new(0.8, 0.8, 0.8),
        });

    let ylabel_id = scene.create_node_with_transform(
        "YLabel".to_string(),
        Transform::from_translation(-0.6, 0.0, 0.0),
    );
    scene
        .get_node_mut(ylabel_id)
        .unwrap()
        .set_renderable(Renderable::Math {
            latex: "f(x) = x^2".to_string(),
            font_size: 32.0,
            color: Color::new(0.8, 0.8, 0.8),
        });

    scene
}

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
            Err(std::sync::mpsc::TryRecvError::Disconnected) => panic!("Channel disconnected"),
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
