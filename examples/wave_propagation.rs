//! Wave Propagation Showcase
//!
//! Demonstrates:
//! - Physics simulation (wave equation)
//! - Dynamic animation system
//! - Smooth wave motion
//! - Time-based updates
//! - Beautiful gradient effects

use diomanim::prelude::*;
use diomanim::render::ShapeRenderer;
use diomanim::scene::{Renderable, SceneGraph};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const NUM_FRAMES: u32 = 180; // 3 seconds at 60 FPS

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let renderer = ShapeRenderer::new(WIDTH, HEIGHT).await?;

    println!("Creating wave propagation animation...");

    // Create output directory
    std::fs::create_dir_all("frames/wave")?;

    // Render animation frames
    for frame in 0..NUM_FRAMES {
        let time = frame as f32 / 60.0; // Convert to seconds
        let mut scene = build_wave_scene(time);
        scene.update_transforms();

        // Create texture for this frame
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
            }
        }

        drop(pass);
        renderer.get_queue().submit(std::iter::once(enc.finish()));

        // Save frame
        let filename = format!("frames/wave/frame_{:04}.png", frame);
        save_png(&renderer, &tex, WIDTH, HEIGHT, &filename);

        if (frame + 1) % 30 == 0 {
            println!("  Rendered {}/{} frames...", frame + 1, NUM_FRAMES);
        }
    }

    println!("✓ Wave animation saved to frames/wave/");
    println!("  Convert to video with: ffmpeg -framerate 60 -i frames/wave/frame_%04d.png -c:v libx264 -pix_fmt yuv420p wave.mp4");

    // Also save a single representative frame
    std::fs::create_dir_all("docs/images")?;
    let time = 1.5; // Mid-animation
    let mut scene = build_wave_scene(time);
    scene.update_transforms();

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
        }
    }

    drop(pass);
    renderer.get_queue().submit(std::iter::once(enc.finish()));
    save_png(&renderer, &tex, WIDTH, HEIGHT, "docs/images/wave.png");
    println!("✓ Static frame saved to docs/images/wave.png");

    Ok(())
}

fn build_wave_scene(time: f32) -> SceneGraph {
    let mut s = SceneGraph::new();

    // Wave parameters
    let num_particles = 80; // Particles along the wave
    let wavelength = 1.2; // How wide each wave is
    let amplitude = 0.15; // How tall the wave is
    let wave_speed = 2.0; // How fast it moves
    let particle_size = 0.018;
    let spacing = 0.025; // Horizontal spacing between particles

    // Create multiple waves with phase offsets for nice visual effect
    let num_waves = 3;

    for wave_idx in 0..num_waves {
        let phase_offset = wave_idx as f32 * std::f32::consts::PI * 0.66;
        let y_offset = (wave_idx as f32 - 1.0) * 0.3; // Stack waves vertically

        for i in 0..num_particles {
            let x = (i as f32 - num_particles as f32 / 2.0) * spacing;

            // Wave equation: y = A * sin(k*x - ω*t + φ)
            // where k = 2π/λ (wave number), ω = 2π*f (angular frequency)
            let k = 2.0 * std::f32::consts::PI / wavelength;
            let omega = wave_speed * k;
            let y = y_offset + amplitude * (k * x - omega * time + phase_offset).sin();

            // Color based on wave position (phase)
            let phase = (k * x - omega * time + phase_offset) % (2.0 * std::f32::consts::PI);
            let hue = (phase / (2.0 * std::f32::consts::PI) + wave_idx as f32 * 0.33) % 1.0;
            let (r, g, b) = hsv_to_rgb(hue, 0.8, 0.95);

            // Size varies slightly with phase for extra visual interest
            let size_factor = 1.0 + 0.3 * (phase / (2.0 * std::f32::consts::PI));
            let circle_size = particle_size * size_factor;

            let node = s.create_node_with_transform(
                format!("wave{}_{}", wave_idx, i),
                Transform::from_translation(x, y, 0.0),
            );
            s.get_node_mut(node)
                .unwrap()
                .set_renderable(Renderable::Circle {
                    radius: circle_size,
                    color: Color::new(r, g, b),
                });
        }
    }

    s
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
                renderer.get_device().poll(wgpu::PollType::Wait {
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
