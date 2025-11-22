//! Simple Feature Showcase - Single Frame
//! Based on working gradient_descent_quick.rs

use diomanim::prelude::*;
use diomanim::render::ShapeRenderer;
use diomanim::scene::{Renderable, SceneGraph};

const WIDTH: u32 = 1600;
const HEIGHT: u32 = 900;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Diomanim Simple Showcase                                    ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    let mut renderer = ShapeRenderer::new(WIDTH, HEIGHT).await?;
    renderer.init_text_rendering(48.0)?;
    println!("✓ Renderer ready");

    let mut scene = build_scene();
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

    println!("✓ Rendering...");
    let mut enc = renderer
        .get_device()
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    let mut pass = renderer.begin_render_pass(&mut enc, &view, None);
    pass.set_pipeline(renderer.get_pipeline());
    pass.set_bind_group(0, renderer.get_transform_bind_group(), &[]);

    for (t, r, o) in scene.get_visible_renderables() {
        renderer.update_transform(&t);
        let a = |c: Color| Color::rgba(c.r, c.g, c.b, c.a * o);

        if let Some((rad, col)) = r.as_circle() {
            let c = diomanim::mobjects::Circle {
                radius: *rad,
                color: a(*col),
                position: Vector3::zero(),
            };
            renderer.draw_circle(&c, a(*col), &mut pass);
        } else if let Some((s, e, col, th)) = r.as_line() {
            renderer.draw_line(*s, *e, a(*col), *th, &mut pass);
        } else if let Some((s, e, col, th)) = r.as_arrow() {
            renderer.draw_arrow(*s, *e, a(*col), *th, &mut pass);
        } else if let Some((txt, sz, col)) = r.as_text() {
            renderer.draw_text(txt, *sz, a(*col), &mut pass);
        } else if let Some((ltx, sz, col)) = r.as_math() {
            renderer.draw_math(ltx, *sz, a(*col), &mut pass);
        }
    }

    drop(pass);
    renderer.get_queue().submit(std::iter::once(enc.finish()));

    std::fs::create_dir_all("docs/images")?;
    save_frame(
        &renderer,
        &tex,
        WIDTH,
        HEIGHT,
        "docs/images/diomanim_showcase.png",
    );
    println!("✅ Saved to docs/images/diomanim_showcase.png");

    Ok(())
}

fn build_scene() -> SceneGraph {
    let mut s = SceneGraph::new();

    // Title - big and centered
    let t = s.create_node_with_transform("T".into(), Transform::from_translation(0.0, 0.6, 0.0));
    s.get_node_mut(t).unwrap().set_renderable(Renderable::Text {
        content: "DIOMANIM".into(),
        font_size: 90.0,
        color: Color::new(0.2, 0.9, 1.0),
    });

    // Subtitle - centered below title
    let st = s.create_node_with_transform("ST".into(), Transform::from_translation(0.0, 0.35, 0.0));
    s.get_node_mut(st)
        .unwrap()
        .set_renderable(Renderable::Text {
            content: "High-Performance Animation Engine".into(),
            font_size: 44.0,
            color: Color::new(0.9, 0.9, 0.9),
        });

    // Big row of colorful circles showing GPU rendering
    let colors = vec![
        (1.0, 0.2, 0.3), // Red
        (1.0, 0.7, 0.2), // Orange
        (0.3, 1.0, 0.4), // Green
        (0.3, 0.6, 1.0), // Blue
        (0.8, 0.3, 1.0), // Purple
    ];

    for (i, (r, g, b)) in colors.iter().enumerate() {
        let x = -0.7 + i as f32 * 0.35;
        let c = s.create_node_with_transform(
            format!("C{}", i),
            Transform::from_translation(x, -0.05, 0.0),
        );
        s.get_node_mut(c)
            .unwrap()
            .set_renderable(Renderable::Circle {
                radius: 0.15,
                color: Color::new(*r, *g, *b),
            });
    }

    // Lines showing different primitives
    let l1 = s.create_node("L1".into());
    s.get_node_mut(l1)
        .unwrap()
        .set_renderable(Renderable::Line {
            start: Vector3::new(-0.8, -0.5, 0.0),
            end: Vector3::new(-0.3, -0.5, 0.0),
            color: Color::new(0.6, 0.7, 1.0),
            thickness: 0.03,
        });

    // Arrow in the middle
    let a1 = s.create_node("A1".into());
    s.get_node_mut(a1)
        .unwrap()
        .set_renderable(Renderable::Arrow {
            start: Vector3::new(-0.25, -0.5, 0.0),
            end: Vector3::new(0.25, -0.5, 0.0),
            color: Color::new(1.0, 0.4, 0.7),
            thickness: 0.03,
        });

    // Another line
    let l2 = s.create_node("L2".into());
    s.get_node_mut(l2)
        .unwrap()
        .set_renderable(Renderable::Line {
            start: Vector3::new(0.3, -0.5, 0.0),
            end: Vector3::new(0.8, -0.5, 0.0),
            color: Color::new(0.4, 1.0, 0.7),
            thickness: 0.03,
        });

    // Performance stats - centered at bottom
    let perf =
        s.create_node_with_transform("Perf".into(), Transform::from_translation(0.0, -0.8, 0.0));
    s.get_node_mut(perf)
        .unwrap()
        .set_renderable(Renderable::Text {
            content: "551 FPS @ 1080p  |  9.2x Realtime  |  Rust + WebGPU".into(),
            font_size: 36.0,
            color: Color::new(0.75, 0.75, 0.75),
        });

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
            Ok(Err(e)) => panic!("Buffer mapping failed: {:?}", e),
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                if timeout.elapsed().as_secs() > 10 {
                    panic!("Buffer mapping timed out");
                }
                renderer.get_device().poll(wgpu::PollType::Wait {
                    submission_index: None,
                    timeout: None,
                });
                std::thread::yield_now();
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => panic!("Channel disconnected"),
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
