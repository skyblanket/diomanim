//! Rose Curve Mathematical Animation
//!
//! Demonstrates:
//! - Parametric equations: r = cos(k·θ)
//! - Beautiful mathematical curve tracing
//! - Smooth gradient animation
//! - Perfect for GIF export
//!
//! Rose curve equation:
//! x(θ) = r·cos(θ) = cos(k·θ)·cos(θ)
//! y(θ) = r·sin(θ) = cos(k·θ)·sin(θ)

use diomanim::prelude::*;
use diomanim::render::ShapeRenderer;
use diomanim::scene::{Renderable, SceneGraph};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const NUM_FRAMES: u32 = 180; // 3 seconds at 60 FPS (will convert to 30 FPS GIF)

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let renderer = ShapeRenderer::new(WIDTH, HEIGHT).await?;

    println!("Creating rose curve animation...");
    println!("Equation: r = cos(5θ), creating a 5-petal rose");

    std::fs::create_dir_all("frames/rose")?;

    let k = 5.0; // Number of petals
    let num_points = 1000; // Points along the curve
    let scale = 0.35; // Size of the rose

    for frame in 0..NUM_FRAMES {
        let progress = frame as f32 / NUM_FRAMES as f32;
        let reveal_progress = progress; // How much of the curve to show

        let mut scene = SceneGraph::new();

        // Calculate all points on the rose curve
        let points_to_draw = (num_points as f32 * reveal_progress) as usize;

        for i in 0..points_to_draw {
            let t = i as f32 / num_points as f32;
            let theta = t * 2.0 * std::f32::consts::PI * k; // Complete k rotations

            // Rose curve: r = cos(k·θ)
            let r = (k * theta).cos() * scale;
            let x = r * theta.cos();
            let y = r * theta.sin();

            // Color gradient along the curve
            let hue = t * 0.8; // Rainbow gradient
            let (r_col, g_col, b_col) = hsv_to_rgb(hue, 0.9, 0.95);

            // Circle size varies slightly for visual interest
            let size_factor = 1.0 + 0.3 * (t * std::f32::consts::PI * 4.0).sin();
            let radius = 0.008 * size_factor;

            let node = scene.create_node_with_transform(
                format!("point_{}", i),
                Transform::from_translation(x, y, 0.0),
            );
            scene.get_node_mut(node).unwrap().set_renderable(
                Renderable::Circle {
                    radius,
                    color: Color::new(r_col, g_col, b_col),
                },
            );
        }

        // Add a glowing circle at the current position (the "pen")
        if points_to_draw > 0 && points_to_draw < num_points {
            let t = points_to_draw as f32 / num_points as f32;
            let theta = t * 2.0 * std::f32::consts::PI * k;
            let r = (k * theta).cos() * scale;
            let x = r * theta.cos();
            let y = r * theta.sin();

            // Bright white "pen" circle
            let pen_node = scene.create_node_with_transform(
                "pen".to_string(),
                Transform::from_translation(x, y, 0.0),
            );
            scene.get_node_mut(pen_node).unwrap().set_renderable(
                Renderable::Circle {
                    radius: 0.02,
                    color: Color::new(1.0, 1.0, 1.0),
                },
            );

            // Add a construction line from origin to current point
            let line_node = scene.create_node_with_transform(
                "construction_line".to_string(),
                Transform::new(),
            );
            scene.get_node_mut(line_node).unwrap().set_renderable(
                Renderable::Line {
                    start: Vector3::zero(),
                    end: Vector3::new(x, y, 0.0),
                    thickness: 0.004,
                    color: Color::rgba(1.0, 1.0, 1.0, 0.3),
                },
            );
        }

        // Center circle (origin)
        let origin_node = scene.create_node_with_transform(
            "origin".to_string(),
            Transform::new(),
        );
        scene.get_node_mut(origin_node).unwrap().set_renderable(
            Renderable::Circle {
                radius: 0.015,
                color: Color::new(0.8, 0.8, 0.8),
            },
        );

        scene.update_transforms();
        render_frame(&renderer, &scene, frame);

        if (frame + 1) % 30 == 0 {
            println!("  Rendered {}/{} frames...", frame + 1, NUM_FRAMES);
        }
    }

    println!("✓ Rose curve animation saved to frames/rose/");
    println!("\n📹 Creating GIF...");

    // Convert to GIF using ffmpeg
    let gif_output = std::process::Command::new("ffmpeg")
        .args(&[
            "-y",
            "-framerate", "30",
            "-i", "frames/rose/frame_%04d.png",
            "-vf", "fps=30,scale=800:800:flags=lanczos,split[s0][s1];[s0]palettegen=max_colors=256[p];[s1][p]paletteuse=dither=bayer:bayer_scale=5",
            "-loop", "0",
            "docs/images/rose_curve.gif"
        ])
        .output();

    match gif_output {
        Ok(output) => {
            if output.status.success() {
                println!("✓ GIF created: docs/images/rose_curve.gif");

                // Check file size
                if let Ok(metadata) = std::fs::metadata("docs/images/rose_curve.gif") {
                    let size_kb = metadata.len() / 1024;
                    println!("  Size: {} KB", size_kb);
                }
            } else {
                println!("⚠ FFmpeg error: {}", String::from_utf8_lossy(&output.stderr));
                println!("  You can manually convert with:");
                println!("  ffmpeg -framerate 30 -i frames/rose/frame_%04d.png -vf \"fps=30,scale=800:800:flags=lanczos\" -loop 0 docs/images/rose_curve.gif");
            }
        }
        Err(e) => {
            println!("⚠ Could not run ffmpeg: {}", e);
            println!("  Install ffmpeg to create GIF automatically");
            println!("  Or manually convert frames in frames/rose/ directory");
        }
    }

    // Also save a single preview frame
    let preview_frame = NUM_FRAMES / 2;
    std::process::Command::new("cp")
        .args(&[
            &format!("frames/rose/frame_{:04}.png", preview_frame),
            "docs/images/rose_curve_preview.png"
        ])
        .output()
        .ok();

    println!("✓ Preview saved to docs/images/rose_curve_preview.png");

    Ok(())
}

fn render_frame(renderer: &ShapeRenderer, scene: &SceneGraph, frame_num: u32) {
    let tex = renderer
        .get_device()
        .create_texture(&wgpu::TextureDescriptor {
            label: None,
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
    let view = tex.create_view(&wgpu::TextureViewDescriptor::default());

    renderer.reset_transform_offset();

    let mut enc = renderer
        .get_device()
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    let mut pass = renderer.begin_render_pass(&mut enc, &view, None);
    pass.set_pipeline(renderer.get_pipeline());

    for (t, r, o) in scene.get_visible_renderables().iter() {
        let offset = renderer.update_transform(&t);
        let a = |c: Color| Color::rgba(c.r, c.g, c.b, c.a * o);

        if let Some((rad, col)) = r.as_circle() {
            let c = diomanim::mobjects::Circle {
                radius: *rad,
                color: a(*col),
                position: Vector3::zero(),
            };
            renderer.draw_circle(&c, a(*col), offset, &mut pass);
        } else if let Renderable::Line { start, end, color, thickness } = r {
            renderer.draw_line(*start, *end, a(*color), *thickness, offset, &mut pass);
        }
    }

    drop(pass);
    renderer.get_queue().submit(std::iter::once(enc.finish()));

    let filename = format!("frames/rose/frame_{:04}.png", frame_num);
    save_png(&renderer, &tex, WIDTH, HEIGHT, &filename);
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let h = h * 6.0;
    let i = h.floor() as i32;
    let f = h - i as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));

    match i % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    }
}

fn save_png(renderer: &ShapeRenderer, texture: &wgpu::Texture, w: u32, h: u32, filename: &str) {
    const A: u32 = 256;
    let u = w * 4;
    let p = (u + A - 1) / A * A;
    let buf = renderer
        .get_device()
        .create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (p * h) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

    let mut enc = renderer
        .get_device()
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    enc.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &buf,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(p),
                rows_per_image: Some(h),
            },
        },
        wgpu::Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
    );
    renderer.get_queue().submit(std::iter::once(enc.finish()));

    let slice = buf.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |r| {
        tx.send(r).unwrap();
    });

    let timeout = std::time::Instant::now();
    loop {
        match rx.try_recv() {
            Ok(Ok(())) => break,
            Ok(Err(e)) => panic!("{:?}", e),
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                if timeout.elapsed().as_secs() > 10 {
                    panic!("Timeout");
                }
                let _ = renderer.get_device().poll(wgpu::PollType::Wait {
                    submission_index: None,
                    timeout: None,
                });
                std::thread::yield_now();
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => panic!("Disconnected"),
        }
    }

    let data = slice.get_mapped_range();
    let file = std::fs::File::create(filename).unwrap();
    let writer = std::io::BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, w, h);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut png_writer = encoder.write_header().unwrap();

    if p != u {
        let mut unpadded = Vec::with_capacity((u * h) as usize);
        for row in 0..h {
            let start = (row * p) as usize;
            let end = start + u as usize;
            unpadded.extend_from_slice(&data[start..end]);
        }
        png_writer.write_image_data(&unpadded).unwrap();
    } else {
        png_writer.write_image_data(&data).unwrap();
    }

    drop(data);
    buf.unmap();
}
