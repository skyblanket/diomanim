//! Ultra simple showcase - just 3 circles

use diomanim::prelude::*;
use diomanim::render::ShapeRenderer;
use diomanim::scene::{Renderable, SceneGraph};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer = ShapeRenderer::new(WIDTH, HEIGHT).await?;
    let mut scene = SceneGraph::new();

    // Just 3 big circles - guaranteed to be separate
    let c1 =
        scene.create_node_with_transform("C1".into(), Transform::from_translation(-0.5, 0.0, 0.0));
    scene
        .get_node_mut(c1)
        .unwrap()
        .set_renderable(Renderable::Circle {
            radius: 0.3,
            color: Color::new(1.0, 0.3, 0.3),
        });

    let c2 =
        scene.create_node_with_transform("C2".into(), Transform::from_translation(0.0, 0.0, 0.0));
    scene
        .get_node_mut(c2)
        .unwrap()
        .set_renderable(Renderable::Circle {
            radius: 0.3,
            color: Color::new(0.3, 1.0, 0.3),
        });

    let c3 =
        scene.create_node_with_transform("C3".into(), Transform::from_translation(0.5, 0.0, 0.0));
    scene
        .get_node_mut(c3)
        .unwrap()
        .set_renderable(Renderable::Circle {
            radius: 0.3,
            color: Color::new(0.3, 0.3, 1.0),
        });

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
        if let Some((rad, col)) = r.as_circle() {
            let c = diomanim::mobjects::Circle {
                radius: *rad,
                color: Color::rgba(col.r, col.g, col.b, col.a * o),
                position: Vector3::zero(),
            };
            renderer.draw_circle(&c, Color::rgba(col.r, col.g, col.b, col.a * o), &mut pass);
        }
    }

    drop(pass);
    renderer.get_queue().submit(std::iter::once(enc.finish()));

    std::fs::create_dir_all("docs/images")?;

    // Save
    const A: u32 = 256;
    let u = WIDTH * 4;
    let p = (u + A - 1) / A * A;
    let buf = renderer
        .get_device()
        .create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (p * HEIGHT) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

    let mut enc = renderer
        .get_device()
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    enc.copy_texture_to_buffer(
        tex.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &buf,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(p),
                rows_per_image: Some(HEIGHT),
            },
        },
        wgpu::Extent3d {
            width: WIDTH,
            height: HEIGHT,
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
    let file = std::fs::File::create("docs/images/diomanim_showcase.png")?;
    let writer = std::io::BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, WIDTH, HEIGHT);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut png_writer = encoder.write_header()?;

    if p != u {
        let mut unpadded = Vec::with_capacity((u * HEIGHT) as usize);
        for row in 0..HEIGHT {
            let start = (row * p) as usize;
            let end = start + u as usize;
            unpadded.extend_from_slice(&data[start..end]);
        }
        png_writer.write_image_data(&unpadded)?;
    } else {
        png_writer.write_image_data(&data)?;
    }

    drop(data);
    buf.unmap();

    println!("✅ Created showcase: 3 circles");
    Ok(())
}
