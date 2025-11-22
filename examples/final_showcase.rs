//! Final Showcase - High quality demonstration

use diomanim::prelude::*;
use diomanim::render::ShapeRenderer;
use diomanim::scene::{Renderable, SceneGraph};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer = ShapeRenderer::new(WIDTH, HEIGHT).await?;

    let mut scene = build();
    scene.update_transforms();

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
        }
    }

    drop(pass);
    renderer.get_queue().submit(std::iter::once(enc.finish()));

    std::fs::create_dir_all("docs/images")?;
    save(&renderer, &tex, WIDTH, HEIGHT, "docs/images/diomanim_showcase.png");
    println!("✅ Saved showcase image ({}x{})", WIDTH, HEIGHT);

    Ok(())
}

fn build() -> SceneGraph {
    let mut s = SceneGraph::new();

    // Top row - 6 large colorful circles
    let top_colors = [
        (1.0, 0.2, 0.3),   // Red
        (1.0, 0.6, 0.1),   // Orange
        (0.9, 0.9, 0.2),   // Yellow
        (0.2, 0.9, 0.3),   // Green
        (0.2, 0.6, 1.0),   // Blue
        (0.7, 0.2, 0.9),   // Purple
    ];

    for (i, (r, g, b)) in top_colors.iter().enumerate() {
        let x = -0.75 + i as f32 * 0.3;
        let c = s.create_node_with_transform(
            format!("Top{}", i),
            Transform::from_translation(x, 0.5, 0.0),
        );
        s.get_node_mut(c).unwrap().set_renderable(Renderable::Circle {
            radius: 0.18,
            color: Color::new(*r, *g, *b),
        });
    }

    // Middle row - gradient of circles
    for i in 0..9 {
        let x = -0.8 + i as f32 * 0.2;
        let t = i as f32 / 8.0;
        let c = s.create_node_with_transform(
            format!("Mid{}", i),
            Transform::from_translation(x, 0.0, 0.0),
        );
        s.get_node_mut(c).unwrap().set_renderable(Renderable::Circle {
            radius: 0.14,
            color: Color::new(0.3 + t * 0.6, 0.4 + t * 0.4, 1.0 - t * 0.5),
        });
    }

    // Bottom row - smaller circles
    for i in 0..11 {
        let x = -0.8 + i as f32 * 0.16;
        let t = i as f32 / 10.0;
        let c = s.create_node_with_transform(
            format!("Bot{}", i),
            Transform::from_translation(x, -0.35, 0.0),
        );
        s.get_node_mut(c).unwrap().set_renderable(Renderable::Circle {
            radius: 0.1,
            color: Color::new(1.0 - t * 0.5, 0.5 + t * 0.3, 0.5 + t * 0.5),
        });
    }

    // Lines showing thickness variations
    for i in 0..4 {
        let y = -0.6 - i as f32 * 0.08;
        let thickness = 0.015 + i as f32 * 0.01;
        let t = i as f32 / 3.0;

        let line = s.create_node(format!("Line{}", i));
        s.get_node_mut(line).unwrap().set_renderable(Renderable::Line {
            start: Vector3::new(-0.75, y, 0.0),
            end: Vector3::new(-0.15, y, 0.0),
            color: Color::new(0.4 + t * 0.4, 0.5 + t * 0.3, 1.0 - t * 0.3),
            thickness,
        });
    }

    // Arrows showing direction
    for i in 0..4 {
        let y = -0.6 - i as f32 * 0.08;
        let thickness = 0.015 + i as f32 * 0.01;
        let t = i as f32 / 3.0;

        let arrow = s.create_node(format!("Arrow{}", i));
        s.get_node_mut(arrow).unwrap().set_renderable(Renderable::Arrow {
            start: Vector3::new(0.15, y, 0.0),
            end: Vector3::new(0.75, y, 0.0),
            color: Color::new(1.0 - t * 0.3, 0.5 + t * 0.4, 0.4 + t * 0.5),
            thickness,
        });
    }

    s
}

fn save(renderer: &ShapeRenderer, texture: &wgpu::Texture, w: u32, h: u32, filename: &str) {
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
            Ok(Err(e)) => panic!("Failed: {:?}", e),
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                if timeout.elapsed().as_secs() > 10 {
                    panic!("Timeout");
                }
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
