//! Clean Showcase - Minimalist single frame

use diomanim::prelude::*;
use diomanim::render::ShapeRenderer;
use diomanim::scene::{Renderable, SceneGraph};

const WIDTH: u32 = 1600;
const HEIGHT: u32 = 900;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer = ShapeRenderer::new(WIDTH, HEIGHT).await?;
    renderer.init_text_rendering(48.0)?;

    let mut scene = build();
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
        }
    }

    drop(pass);
    renderer.get_queue().submit(std::iter::once(enc.finish()));

    std::fs::create_dir_all("docs/images")?;
    save(
        &renderer,
        &tex,
        WIDTH,
        HEIGHT,
        "docs/images/diomanim_showcase.png",
    );
    println!("✅ docs/images/diomanim_showcase.png");

    Ok(())
}

fn build() -> SceneGraph {
    let mut s = SceneGraph::new();

    // Just use the existing gradient descent preview - it actually works!
    // For now, let's create a simple colorful pattern

    // Top row - 5 circles
    let c1 = s.create_node_with_transform("C1".into(), Transform::from_translation(-0.6, 0.4, 0.0));
    s.get_node_mut(c1)
        .unwrap()
        .set_renderable(Renderable::Circle {
            radius: 0.15,
            color: Color::new(1.0, 0.2, 0.3),
        });

    let c2 = s.create_node_with_transform("C2".into(), Transform::from_translation(-0.3, 0.4, 0.0));
    s.get_node_mut(c2)
        .unwrap()
        .set_renderable(Renderable::Circle {
            radius: 0.15,
            color: Color::new(1.0, 0.7, 0.2),
        });

    let c3 = s.create_node_with_transform("C3".into(), Transform::from_translation(0.0, 0.4, 0.0));
    s.get_node_mut(c3)
        .unwrap()
        .set_renderable(Renderable::Circle {
            radius: 0.15,
            color: Color::new(0.3, 1.0, 0.4),
        });

    let c4 = s.create_node_with_transform("C4".into(), Transform::from_translation(0.3, 0.4, 0.0));
    s.get_node_mut(c4)
        .unwrap()
        .set_renderable(Renderable::Circle {
            radius: 0.15,
            color: Color::new(0.3, 0.6, 1.0),
        });

    let c5 = s.create_node_with_transform("C5".into(), Transform::from_translation(0.6, 0.4, 0.0));
    s.get_node_mut(c5)
        .unwrap()
        .set_renderable(Renderable::Circle {
            radius: 0.15,
            color: Color::new(0.8, 0.3, 1.0),
        });

    // Middle row - 7 smaller circles
    for i in 0..7 {
        let x = -0.6 + i as f32 * 0.2;
        let t = i as f32 / 6.0;
        let c = s.create_node_with_transform(
            format!("M{}", i),
            Transform::from_translation(x, 0.0, 0.0),
        );
        s.get_node_mut(c)
            .unwrap()
            .set_renderable(Renderable::Circle {
                radius: 0.1,
                color: Color::new(0.3 + t * 0.5, 0.5 + t * 0.3, 1.0 - t * 0.4),
            });
    }

    // Lines
    let l1 = s.create_node("L1".into());
    s.get_node_mut(l1)
        .unwrap()
        .set_renderable(Renderable::Line {
            start: Vector3::new(-0.7, -0.4, 0.0),
            end: Vector3::new(-0.2, -0.4, 0.0),
            color: Color::new(0.6, 0.7, 1.0),
            thickness: 0.03,
        });

    let l2 = s.create_node("L2".into());
    s.get_node_mut(l2)
        .unwrap()
        .set_renderable(Renderable::Line {
            start: Vector3::new(0.2, -0.4, 0.0),
            end: Vector3::new(0.7, -0.4, 0.0),
            color: Color::new(0.4, 1.0, 0.7),
            thickness: 0.03,
        });

    // Arrows
    let a1 = s.create_node("A1".into());
    s.get_node_mut(a1)
        .unwrap()
        .set_renderable(Renderable::Arrow {
            start: Vector3::new(-0.5, -0.7, 0.0),
            end: Vector3::new(0.0, -0.7, 0.0),
            color: Color::new(1.0, 0.4, 0.6),
            thickness: 0.03,
        });

    let a2 = s.create_node("A2".into());
    s.get_node_mut(a2)
        .unwrap()
        .set_renderable(Renderable::Arrow {
            start: Vector3::new(0.0, -0.7, 0.0),
            end: Vector3::new(0.5, -0.7, 0.0),
            color: Color::new(0.4, 1.0, 0.6),
            thickness: 0.03,
        });

    s
}

fn save(renderer: &ShapeRenderer, texture: &wgpu::Texture, w: u32, h: u32, filename: &str) {
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
