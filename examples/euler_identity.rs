//! Euler's Identity Animation with LaTeX
//!
//! The most beautiful equation in mathematics: e^(iπ) + 1 = 0
//! Demonstrates LaTeX mathematical notation with GPU rendering

use diomanim::prelude::*;
use diomanim::render::ShapeRenderer;
use diomanim::scene::{Renderable, SceneGraph};

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;
const NUM_FRAMES: u32 = 240; // 8 seconds at 30 FPS

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer = ShapeRenderer::new(WIDTH, HEIGHT).await?;
    renderer.init_text_rendering(48.0)?;

    println!("Creating Euler's Identity animation...");
    println!("e^(iπ) + 1 = 0 - The most beautiful equation");

    std::fs::create_dir_all("frames/euler")?;

    let output_texture = renderer
        .get_device()
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("Output"),
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

    for frame in 0..NUM_FRAMES {
        let t = frame as f32 / NUM_FRAMES as f32;
        let scene = build_scene(t);

        let mut encoder = renderer
            .get_device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let mut render_pass = renderer.begin_render_pass(&mut encoder, &output_view, None);
        render_pass.set_pipeline(renderer.get_pipeline());
        render_pass.set_bind_group(0, renderer.get_transform_bind_group(), &[]);

        for (transform_uniform, renderable, opacity) in scene.get_visible_renderables() {
            let offset = renderer.update_transform(&transform_uniform);
            let apply = |c: Color| Color::rgba(c.r, c.g, c.b, c.a * opacity);

            if let Some((r, c)) = renderable.as_circle() {
                renderer.draw_circle(
                    &diomanim::mobjects::Circle {
                        radius: *r,
                        color: apply(*c),
                        position: Vector3::zero(),
                    },
                    apply(*c),
                    offset,
                    &mut render_pass,
                );
            } else if let Some((txt, fs, c)) = renderable.as_text() {
                renderer.draw_text(txt, *fs, apply(*c), offset, &mut render_pass);
            } else if let Some((ltx, fs, c)) = renderable.as_math() {
                renderer.draw_math(ltx, *fs, apply(*c), offset, &mut render_pass);
            }
        }

        drop(render_pass);
        renderer
            .get_queue()
            .submit(std::iter::once(encoder.finish()));

        save_frame(
            &renderer,
            &output_texture,
            WIDTH,
            HEIGHT,
            &format!("frames/euler/frame_{:04}.png", frame),
        );

        if (frame + 1) % 30 == 0 {
            println!("  Rendered {}/{} frames...", frame + 1, NUM_FRAMES);
        }
    }

    println!("✓ Euler's identity animation saved to frames/euler/");
    println!("\n📹 Creating GIF...");

    let gif_result = std::process::Command::new("ffmpeg")
        .args(&[
            "-y",
            "-framerate",
            "30",
            "-i",
            "frames/euler/frame_%04d.png",
            "-vf",
            "fps=30,scale=1200:800:flags=lanczos,split[s0][s1];[s0]palettegen=max_colors=256[p];[s1][p]paletteuse=dither=bayer:bayer_scale=5",
            "-loop",
            "0",
            "docs/images/euler_identity.gif",
        ])
        .output();

    match gif_result {
        Ok(output) if output.status.success() => {
            println!("✓ GIF created: docs/images/euler_identity.gif");
            if let Ok(metadata) = std::fs::metadata("docs/images/euler_identity.gif") {
                println!("  Size: {} KB", metadata.len() / 1024);
            }
        }
        Ok(output) => {
            println!("⚠ FFmpeg error");
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }
        Err(e) => println!("⚠ Could not run ffmpeg: {}", e),
    }

    Ok(())
}

fn build_scene(t: f32) -> SceneGraph {
    let mut s = SceneGraph::new();

    // Title with fade-in
    if t >= 0.0 {
        let alpha = (t / 0.2).min(1.0);
        let title = s.create_node_with_transform(
            "title".to_string(),
            Transform::from_translation(0.0, 0.8, 0.0),
        );
        s.get_node_mut(title).unwrap().opacity = alpha;
        s.get_node_mut(title).unwrap().set_renderable(
            Renderable::Text {
                content: "Euler's Identity".to_string(),
                font_size: 64.0,
                color: Color::new(0.9, 0.9, 0.9),
            },
        );
    }

    // Phase 1: Show unit circle (0.2-0.5)
    if t >= 0.2 {
        let alpha = ((t - 0.2) / 0.3).min(1.0);
        let circle = s.create_node_with_transform(
            "circle".to_string(),
            Transform::from_translation(0.0, 0.2, 0.0),
        );
        s.get_node_mut(circle).unwrap().opacity = alpha;
        s.get_node_mut(circle).unwrap().set_renderable(
            Renderable::Circle {
                radius: 0.25,
                color: Color::rgba(0.4, 0.6, 1.0, 0.3),
            },
        );

        // Center point
        let center = s.create_node_with_transform(
            "center".to_string(),
            Transform::from_translation(0.0, 0.2, 0.0),
        );
        s.get_node_mut(center).unwrap().opacity = alpha;
        s.get_node_mut(center).unwrap().set_renderable(
            Renderable::Circle {
                radius: 0.015,
                color: Color::new(0.8, 0.8, 0.8),
            },
        );
    }

    // Phase 2: Point at e^(iπ) = -1 (0.4-0.7)
    if t >= 0.4 {
        let alpha = ((t - 0.4) / 0.3).min(1.0);
        let point = s.create_node_with_transform(
            "point".to_string(),
            Transform::from_translation(-0.25, 0.2, 0.0),
        );
        s.get_node_mut(point).unwrap().opacity = alpha;
        s.get_node_mut(point).unwrap().set_renderable(
            Renderable::Circle {
                radius: 0.025,
                color: Color::new(1.0, 0.3, 0.3),
            },
        );
    }

    // Phase 3: Show e^(iπ) (0.5-0.75)
    if t >= 0.5 {
        let alpha = ((t - 0.5) / 0.25).min(1.0);
        let eq1 = s.create_node_with_transform(
            "eq1".to_string(),
            Transform::from_translation(-0.15, -0.35, 0.0),
        );
        s.get_node_mut(eq1).unwrap().opacity = alpha;
        s.get_node_mut(eq1).unwrap().set_renderable(
            Renderable::Math {
                latex: "e^{i\\pi}".to_string(),
                font_size: 72.0,
                color: Color::new(0.3, 0.8, 1.0),
            },
        );
    }

    // Phase 4: Show = -1 (0.7-0.9)
    if t >= 0.7 {
        let alpha = ((t - 0.7) / 0.2).min(1.0);
        let eq2 = s.create_node_with_transform(
            "eq2".to_string(),
            Transform::from_translation(0.2, -0.35, 0.0),
        );
        s.get_node_mut(eq2).unwrap().opacity = alpha;
        s.get_node_mut(eq2).unwrap().set_renderable(
            Renderable::Math {
                latex: "= -1".to_string(),
                font_size: 72.0,
                color: Color::new(0.3, 0.8, 1.0),
            },
        );
    }

    // Phase 5: Show + 1 = 0 (0.85-1.0)
    if t >= 0.85 {
        let alpha = ((t - 0.85) / 0.15).min(1.0);

        // +1
        let plus_one = s.create_node_with_transform(
            "plus_one".to_string(),
            Transform::from_translation(-0.1, -0.6, 0.0),
        );
        s.get_node_mut(plus_one).unwrap().opacity = alpha;
        s.get_node_mut(plus_one).unwrap().set_renderable(
            Renderable::Math {
                latex: "+ 1".to_string(),
                font_size: 64.0,
                color: Color::new(0.3, 1.0, 0.6),
            },
        );

        // = 0
        let equals_zero = s.create_node_with_transform(
            "equals_zero".to_string(),
            Transform::from_translation(0.2, -0.6, 0.0),
        );
        s.get_node_mut(equals_zero).unwrap().opacity = alpha;
        s.get_node_mut(equals_zero).unwrap().set_renderable(
            Renderable::Math {
                latex: "= 0".to_string(),
                font_size: 64.0,
                color: Color::new(1.0, 0.3, 0.6),
            },
        );
    }

    s.update_transforms();
    s
}

fn save_frame(renderer: &ShapeRenderer, texture: &wgpu::Texture, w: u32, h: u32, filename: &str) {
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
