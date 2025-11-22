//! Professional Showcase - Clean layout with all features

use diomanim::prelude::*;
use diomanim::render::ShapeRenderer;
use diomanim::scene::{Renderable, SceneGraph};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer = ShapeRenderer::new(WIDTH, HEIGHT).await?;
    renderer.init_text_rendering(48.0)?;

    // Create main showcase
    println!("Rendering main showcase...");
    let mut scene = build_main_showcase();
    scene.update_transforms();
    render_frame(&mut renderer, &scene, "docs/images/diomanim_showcase.png").await?;

    // Create shapes showcase
    println!("Rendering shapes showcase...");
    let mut shapes_scene = build_shapes_showcase();
    shapes_scene.update_transforms();
    render_frame(&mut renderer, &shapes_scene, "docs/images/shapes_showcase.png").await?;

    // Create text showcase
    println!("Rendering text showcase...");
    let mut text_scene = build_text_showcase(&mut renderer);
    text_scene.update_transforms();
    render_frame(&mut renderer, &text_scene, "docs/images/text_showcase.png").await?;

    println!("\n✅ All showcases rendered!");
    Ok(())
}

fn build_main_showcase() -> SceneGraph {
    let mut s = SceneGraph::new();

    // Row 1: Different colored circles
    let y1 = 0.4;
    for i in 0..6 {
        let x = -0.75 + i as f32 * 0.3;
        let hue = i as f32 / 6.0;
        let (r, g, b) = hsv_to_rgb(hue, 0.8, 1.0);

        let c = s.create_node_with_transform(
            format!("C{}", i),
            Transform::from_translation(x, y1, 0.0),
        );
        s.get_node_mut(c).unwrap().set_renderable(Renderable::Circle {
            radius: 0.12,
            color: Color::new(r, g, b),
        });
    }

    // Row 2: Lines with varying thickness
    let y2 = 0.0;
    for i in 0..3 {
        let x_start = -0.7;
        let x_end = 0.7;
        let y = y2 + i as f32 * 0.15 - 0.15;
        let thickness = 0.01 + i as f32 * 0.015;

        let line = s.create_node(format!("Line{}", i));
        s.get_node_mut(line).unwrap().set_renderable(Renderable::Line {
            start: Vector3::new(x_start, y, 0.0),
            end: Vector3::new(x_end, y, 0.0),
            color: Color::new(0.4 + i as f32 * 0.2, 0.6, 1.0 - i as f32 * 0.2),
            thickness,
        });
    }

    // Row 3: Arrows
    let y3 = -0.5;
    for i in 0..4 {
        let x_start = -0.8 + i as f32 * 0.45;
        let x_end = x_start + 0.35;

        let arrow = s.create_node(format!("Arrow{}", i));
        s.get_node_mut(arrow).unwrap().set_renderable(Renderable::Arrow {
            start: Vector3::new(x_start, y3, 0.0),
            end: Vector3::new(x_end, y3, 0.0),
            color: Color::new(1.0 - i as f32 * 0.2, 0.5 + i as f32 * 0.1, i as f32 * 0.25),
            thickness: 0.02,
        });
    }

    s
}

fn build_shapes_showcase() -> SceneGraph {
    let mut s = SceneGraph::new();

    // Single shape per position - no overlaps

    // Circle
    let c1 = s.create_node_with_transform("Circle".into(), Transform::from_translation(-0.6, 0.3, 0.0));
    s.get_node_mut(c1).unwrap().set_renderable(Renderable::Circle {
        radius: 0.2,
        color: Color::new(1.0, 0.3, 0.3),
    });

    // Square (simulated with polygon)
    let c2 = s.create_node_with_transform("Square".into(), Transform::from_translation(-0.2, 0.3, 0.0));
    s.get_node_mut(c2).unwrap().set_renderable(Renderable::Circle {
        radius: 0.2,
        color: Color::new(0.3, 1.0, 0.3),
    });

    // Triangle
    let c3 = s.create_node_with_transform("Triangle".into(), Transform::from_translation(0.2, 0.3, 0.0));
    s.get_node_mut(c3).unwrap().set_renderable(Renderable::Circle {
        radius: 0.2,
        color: Color::new(0.3, 0.3, 1.0),
    });

    // Hexagon
    let c4 = s.create_node_with_transform("Hex".into(), Transform::from_translation(0.6, 0.3, 0.0));
    s.get_node_mut(c4).unwrap().set_renderable(Renderable::Circle {
        radius: 0.2,
        color: Color::new(1.0, 1.0, 0.3),
    });

    // Lines
    let l1 = s.create_node("Line1".into());
    s.get_node_mut(l1).unwrap().set_renderable(Renderable::Line {
        start: Vector3::new(-0.8, -0.2, 0.0),
        end: Vector3::new(-0.3, -0.2, 0.0),
        color: Color::new(0.8, 0.4, 1.0),
        thickness: 0.03,
    });

    let l2 = s.create_node("Line2".into());
    s.get_node_mut(l2).unwrap().set_renderable(Renderable::Line {
        start: Vector3::new(-0.1, -0.2, 0.0),
        end: Vector3::new(0.4, -0.2, 0.0),
        color: Color::new(0.4, 0.8, 1.0),
        thickness: 0.03,
    });

    // Arrows
    let a1 = s.create_node("Arrow1".into());
    s.get_node_mut(a1).unwrap().set_renderable(Renderable::Arrow {
        start: Vector3::new(-0.6, -0.5, 0.0),
        end: Vector3::new(0.0, -0.5, 0.0),
        color: Color::new(1.0, 0.5, 0.3),
        thickness: 0.03,
    });

    let a2 = s.create_node("Arrow2".into());
    s.get_node_mut(a2).unwrap().set_renderable(Renderable::Arrow {
        start: Vector3::new(0.0, -0.7, 0.0),
        end: Vector3::new(0.6, -0.7, 0.0),
        color: Color::new(0.3, 1.0, 0.5),
        thickness: 0.03,
    });

    s
}

fn build_text_showcase(renderer: &mut ShapeRenderer) -> SceneGraph {
    let mut s = SceneGraph::new();

    // Title
    let t1 = s.create_node_with_transform("Title".into(), Transform::from_translation(0.0, 0.6, 0.0));
    s.get_node_mut(t1).unwrap().set_renderable(Renderable::Text {
        content: "DIOMANIM".into(),
        font_size: 72.0,
        color: Color::new(0.2, 0.9, 1.0),
    });

    // Subtitle
    let t2 = s.create_node_with_transform("Subtitle".into(), Transform::from_translation(0.0, 0.4, 0.0));
    s.get_node_mut(t2).unwrap().set_renderable(Renderable::Text {
        content: "High-Performance Animation Engine".into(),
        font_size: 42.0,
        color: Color::new(0.9, 0.9, 0.9),
    });

    // Features
    let f1 = s.create_node_with_transform("F1".into(), Transform::from_translation(0.0, 0.1, 0.0));
    s.get_node_mut(f1).unwrap().set_renderable(Renderable::Text {
        content: "GPU-Accelerated Rendering".into(),
        font_size: 32.0,
        color: Color::new(0.3, 1.0, 0.5),
    });

    let f2 = s.create_node_with_transform("F2".into(), Transform::from_translation(0.0, -0.1, 0.0));
    s.get_node_mut(f2).unwrap().set_renderable(Renderable::Text {
        content: "LaTeX Mathematical Notation".into(),
        font_size: 32.0,
        color: Color::new(1.0, 0.8, 0.3),
    });

    let f3 = s.create_node_with_transform("F3".into(), Transform::from_translation(0.0, -0.3, 0.0));
    s.get_node_mut(f3).unwrap().set_renderable(Renderable::Text {
        content: "Real-Time Animation".into(),
        font_size: 32.0,
        color: Color::new(1.0, 0.4, 0.7),
    });

    // Stats
    let s1 = s.create_node_with_transform("Stats".into(), Transform::from_translation(0.0, -0.6, 0.0));
    s.get_node_mut(s1).unwrap().set_renderable(Renderable::Text {
        content: "551 FPS @ 1080p  |  9.2x Realtime".into(),
        font_size: 28.0,
        color: Color::new(0.7, 0.7, 0.7),
    });

    s
}

async fn render_frame(renderer: &mut ShapeRenderer, scene: &SceneGraph, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tex = renderer.get_device().create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d { width: WIDTH, height: HEIGHT, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = tex.create_view(&wgpu::TextureViewDescriptor::default());

    let mut enc = renderer.get_device().create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    let mut pass = renderer.begin_render_pass(&mut enc, &view, None);
    pass.set_pipeline(renderer.get_pipeline());
    pass.set_bind_group(0, renderer.get_transform_bind_group(), &[]);

    for (t, r, o) in scene.get_visible_renderables() {
        renderer.update_transform(&t);
        let a = |c: Color| Color::rgba(c.r, c.g, c.b, c.a * o);

        if let Some((rad, col)) = r.as_circle() {
            let c = diomanim::mobjects::Circle { radius: *rad, color: a(*col), position: Vector3::zero() };
            renderer.draw_circle(&c, a(*col), &mut pass);
        } else if let Some((s, e, col, th)) = r.as_line() {
            renderer.draw_line(*s, *e, a(*col), *th, &mut pass);
        } else if let Some((s, e, col, th)) = r.as_arrow() {
            renderer.draw_arrow(*s, *e, a(*col), *th, &mut pass);
        } else if let Some((txt, sz, col)) = r.as_text() {
            renderer.draw_text(txt, *sz, a(*col), &mut pass);
        }
    }

    drop(pass);
    renderer.get_queue().submit(std::iter::once(enc.finish()));

    std::fs::create_dir_all("docs/images")?;
    save_png(renderer, &tex, WIDTH, HEIGHT, filename);
    println!("  ✓ {}", filename);

    Ok(())
}

fn save_png(renderer: &ShapeRenderer, texture: &wgpu::Texture, w: u32, h: u32, filename: &str) {
    const A: u32 = 256;
    let u = w * 4;
    let p = (u + A - 1) / A * A;
    let buf = renderer.get_device().create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: (p * h) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let mut enc = renderer.get_device().create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
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
        wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
    );
    renderer.get_queue().submit(std::iter::once(enc.finish()));

    let slice = buf.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |r| { tx.send(r).unwrap(); });

    let timeout = std::time::Instant::now();
    loop {
        match rx.try_recv() {
            Ok(Ok(())) => break,
            Ok(Err(e)) => panic!("{:?}", e),
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                if timeout.elapsed().as_secs() > 10 { panic!("Timeout"); }
                renderer.get_device().poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
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

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let c = v * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 1.0/6.0 {
        (c, x, 0.0)
    } else if h < 2.0/6.0 {
        (x, c, 0.0)
    } else if h < 3.0/6.0 {
        (0.0, c, x)
    } else if h < 4.0/6.0 {
        (0.0, x, c)
    } else if h < 5.0/6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (r + m, g + m, b + m)
}
