//! Quick Gradient Descent Video - Low res for demo
//! 480x270 @ 30 FPS, 6 seconds = 180 frames

use diomanim::animation::effects;
use diomanim::animation::property::AnimationInstance;
use diomanim::export::export_video;
use diomanim::prelude::*;
use diomanim::render::ShapeRenderer;
use diomanim::scene::{Renderable, SceneGraph};
use std::time::Instant;

const WIDTH: u32 = 480;
const HEIGHT: u32 = 270;
const DURATION: f32 = 6.0;
const FPS: f32 = 30.0;
const FRAME_TIME: f32 = 1.0 / FPS;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Gradient Descent - Quick Demo Video                         ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
    println!("📹 Rendering {}x{} @ {} FPS for {}s", WIDTH, HEIGHT, FPS as u32, DURATION);
    println!("🎬 Total frames: {}\n", (DURATION * FPS) as u32);

    let mut renderer = ShapeRenderer::new(WIDTH, HEIGHT).await?;
    renderer.init_text_rendering(24.0)?;

    let mut scene = build_scene();
    let frames_dir = "output/demo_frames";
    std::fs::create_dir_all(frames_dir)?;

    let output_texture = renderer.get_device().create_texture(&wgpu::TextureDescriptor {
        label: Some("Output Texture"),
        size: wgpu::Extent3d { width: WIDTH, height: HEIGHT, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let output_view = output_texture.create_view(&wgpu::TextureViewDescriptor::default());

    let start_time = Instant::now();
    let total_frames = (DURATION * FPS) as u32;

    for frame_count in 0..total_frames {
        scene.update_animations(TimeValue::new(FRAME_TIME));
        scene.update_transforms();

        let mut encoder = renderer.get_device().create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let mut render_pass = renderer.begin_render_pass(&mut encoder, &output_view, None);
        render_pass.set_pipeline(renderer.get_pipeline());
        render_pass.set_bind_group(0, renderer.get_transform_bind_group(), &[]);

        for (transform_uniform, renderable, opacity) in scene.get_visible_renderables() {
            renderer.update_transform(&transform_uniform);
            let apply = |c: Color| Color::rgba(c.r, c.g, c.b, c.a * opacity);

            if let Some((r, c)) = renderable.as_circle() {
                renderer.draw_circle(&diomanim::mobjects::Circle { radius: *r, color: apply(*c), position: Vector3::zero() }, apply(*c), &mut render_pass);
            } else if let Some((s, e, c, t)) = renderable.as_line() {
                renderer.draw_line(*s, *e, apply(*c), *t, &mut render_pass);
            } else if let Some((s, e, c, t)) = renderable.as_arrow() {
                renderer.draw_arrow(*s, *e, apply(*c), *t, &mut render_pass);
            } else if let Some((txt, fs, c)) = renderable.as_text() {
                renderer.draw_text(txt, *fs, apply(*c), &mut render_pass);
            } else if let Some((ltx, fs, c)) = renderable.as_math() {
                renderer.draw_math(ltx, *fs, apply(*c), &mut render_pass);
            }
        }

        drop(render_pass);
        renderer.get_queue().submit(std::iter::once(encoder.finish()));

        save_frame(&renderer, &output_texture, WIDTH, HEIGHT, &format!("{}/frame_{:04}.png", frames_dir, frame_count));

        if (frame_count + 1) % 30 == 0 || frame_count + 1 == total_frames {
            println!("  Frame {}/{} ({:.0}%)", frame_count + 1, total_frames, (frame_count + 1) as f32 / total_frames as f32 * 100.0);
        }
    }

    let elapsed = start_time.elapsed().as_secs_f32();
    println!("\n✅ Rendered in {:.1}s ({:.1} FPS)", elapsed, total_frames as f32 / elapsed);

    match export_video(frames_dir, "output/gradient_descent_demo.mp4", WIDTH, HEIGHT, FPS as u32) {
        Ok(()) => println!("\n🎉 Video: output/gradient_descent_demo.mp4"),
        Err(e) => println!("\n⚠️  Export failed: {}", e),
    }

    Ok(())
}

fn build_scene() -> SceneGraph {
    let mut s = SceneGraph::new();

    // Title (smaller font for low res)
    let t = s.create_node_with_transform("T".into(), Transform::from_translation(0.0, 0.85, 0.0));
    s.get_node_mut(t).unwrap().set_renderable(Renderable::Text { content: "Gradient Descent".into(), font_size: 28.0, color: Color::WHITE });
    s.get_node_mut(t).unwrap().add_animation(AnimationInstance::new(effects::fade_in(0.5), TimeValue::new(0.0)));

    // Equation (smaller)
    let e1 = s.create_node_with_transform("E1".into(), Transform::from_translation(-0.5, 0.6, 0.0));
    s.get_node_mut(e1).unwrap().set_renderable(Renderable::Math { latex: "\\nabla f = 2x".into(), font_size: 20.0, color: Color::new(0.3, 0.8, 1.0) });
    s.get_node_mut(e1).unwrap().add_animation(AnimationInstance::new(effects::fade_in(0.5), TimeValue::new(0.5)));

    let e2 = s.create_node_with_transform("E2".into(), Transform::from_translation(-0.6, 0.4, 0.0));
    s.get_node_mut(e2).unwrap().set_renderable(Renderable::Math { latex: "x_{t+1} = x_t - \\alpha \\nabla f".into(), font_size: 18.0, color: Color::new(0.3, 1.0, 0.6) });
    s.get_node_mut(e2).unwrap().add_animation(AnimationInstance::new(effects::fade_in(0.5), TimeValue::new(0.8)));

    // Steps
    let steps = vec![0.8, 0.64, 0.512, 0.41];
    for (i, &x) in steps.iter().enumerate() {
        let y = x * x;
        let px = (x - 0.5) * 0.8;
        let py = -0.2 - y * 0.4;

        let p = s.create_node_with_transform(format!("P{}", i), Transform::from_translation(px, py, 0.0));
        let t = i as f32 / 3.0;
        s.get_node_mut(p).unwrap().set_renderable(Renderable::Circle { radius: 0.04, color: Color::new(0.3 + t * 0.6, 0.5 + t * 0.5, 1.0 - t * 0.4) });
        s.get_node_mut(p).unwrap().add_animation(AnimationInstance::new(effects::grow_from_center(0.3), TimeValue::new(1.5 + i as f32 * 0.6)));

        if i < 3 {
            let x2 = steps[i + 1];
            let y2 = x2 * x2;
            let a = s.create_node("A".into());
            s.get_node_mut(a).unwrap().set_renderable(Renderable::Arrow {
                start: Vector3::new(px, py, 0.0),
                end: Vector3::new((x2 - 0.5) * 0.8, -0.2 - y2 * 0.4, 0.0),
                color: Color::new(0.8, 0.8, 0.2),
                thickness: 0.015,
            });
            s.get_node_mut(a).unwrap().add_animation(AnimationInstance::new(effects::create(0.3), TimeValue::new(1.7 + i as f32 * 0.6)));
        }
    }

    // Curve
    for i in 0..15 {
        let t1 = i as f32 / 15.0;
        let t2 = (i + 1) as f32 / 15.0;
        let l = s.create_node("L".into());
        s.get_node_mut(l).unwrap().set_renderable(Renderable::Line {
            start: Vector3::new((t1 - 0.5) * 0.8, -0.2 - t1 * t1 * 0.4, 0.0),
            end: Vector3::new((t2 - 0.5) * 0.8, -0.2 - t2 * t2 * 0.4, 0.0),
            color: Color::new(0.5, 0.5, 0.5),
            thickness: 0.01,
        });
        s.get_node_mut(l).unwrap().add_animation(AnimationInstance::new(effects::create(0.15), TimeValue::new(0.8 + i as f32 * 0.03)));
    }

    s
}

fn save_frame(renderer: &ShapeRenderer, texture: &wgpu::Texture, w: u32, h: u32, filename: &str) {
    const A: u32 = 256;
    let u = w * 4;
    let p = (u + A - 1) / A * A;
    let buf = renderer.get_device().create_buffer(&wgpu::BufferDescriptor {
        label: None, size: (p * h) as u64, usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false,
    });
    let mut enc = renderer.get_device().create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    enc.copy_texture_to_buffer(texture.as_image_copy(), wgpu::TexelCopyBufferInfo {
        buffer: &buf, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(p), rows_per_image: Some(h) }
    }, wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 });
    renderer.get_queue().submit(std::iter::once(enc.finish()));

    let slice = buf.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |r| { tx.send(r).unwrap(); });
    loop {
        match rx.try_recv() {
            Ok(Ok(())) => break,
            Ok(Err(e)) => panic!("{:?}", e),
            Err(std::sync::mpsc::TryRecvError::Empty) => { let _ = renderer.get_device().poll(wgpu::PollType::Wait { submission_index: None, timeout: None }); }
            Err(_) => panic!("Disconnected"),
        }
    }

    let data = slice.get_mapped_range();
    let mut enc = png::Encoder::new(std::io::BufWriter::new(std::fs::File::create(filename).unwrap()), w, h);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    let mut wr = enc.write_header().unwrap();

    if p != u {
        let mut ud = Vec::with_capacity((u * h) as usize);
        for row in 0..h { ud.extend_from_slice(&data[(row * p) as usize..(row * p + u) as usize]); }
        wr.write_image_data(&ud).unwrap();
    } else {
        wr.write_image_data(&data).unwrap();
    }
    drop(data);
    buf.unmap();
}
