//! Bubble Sort Algorithm Visualization
//!
//! Demonstrates:
//! - Algorithm visualization (sorting)
//! - Dynamic bar chart animation
//! - Color-coded comparisons
//! - Step-by-step execution
//! - Educational CS content

use diomanim::prelude::*;
use diomanim::render::ShapeRenderer;
use diomanim::scene::{Renderable, SceneGraph};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let renderer = ShapeRenderer::new(WIDTH, HEIGHT).await?;

    println!("Creating bubble sort visualization...");

    // Initial array to sort
    let mut arr = vec![8, 3, 7, 1, 5, 9, 2, 6, 4];
    let n = arr.len();

    // Create output directory
    std::fs::create_dir_all("frames/bubble_sort")?;

    let mut frame_count = 0;

    // Add some initial frames showing the unsorted array
    for _ in 0..30 {
        let scene = build_bars_scene(&arr, None, None, "Unsorted Array");
        render_frame(&renderer, &scene, frame_count);
        frame_count += 1;
    }

    // Bubble sort algorithm with visualization
    for i in 0..n {
        for j in 0..(n - i - 1) {
            // Show comparison
            let compare_scene = build_bars_scene(&arr, Some(j), Some(j + 1), "Comparing...");
            for _ in 0..10 {
                // Show comparison for 10 frames
                render_frame(&renderer, &compare_scene, frame_count);
                frame_count += 1;
            }

            if arr[j] > arr[j + 1] {
                // Show swap
                let swap_scene = build_bars_scene(&arr, Some(j), Some(j + 1), "Swapping!");
                for _ in 0..10 {
                    render_frame(&renderer, &swap_scene, frame_count);
                    frame_count += 1;
                }

                // Perform swap
                arr.swap(j, j + 1);

                // Show result of swap
                let swapped_scene = build_bars_scene(&arr, Some(j), Some(j + 1), "Swapped");
                for _ in 0..10 {
                    render_frame(&renderer, &swapped_scene, frame_count);
                    frame_count += 1;
                }
            }
        }

        // Show completed pass
        let pass_scene = build_bars_scene(&arr, None, None, &format!("Pass {} Complete", i + 1));
        for _ in 0..20 {
            render_frame(&renderer, &pass_scene, frame_count);
            frame_count += 1;
        }
    }

    // Final sorted array
    let final_scene = build_bars_scene(&arr, None, None, "Sorted!");
    for _ in 0..60 {
        // Hold final frame longer
        render_frame(&renderer, &final_scene, frame_count);
        frame_count += 1;
    }

    println!("✓ Bubble sort animation saved to frames/bubble_sort/");
    println!("  Total frames: {}", frame_count);
    println!("  Convert to video with: ffmpeg -framerate 30 -i frames/bubble_sort/frame_%04d.png -c:v libx264 -pix_fmt yuv420p bubble_sort.mp4");

    // Save a static preview frame
    std::fs::create_dir_all("docs/images")?;
    let preview_arr = vec![8, 3, 7, 1, 5, 9, 2, 6, 4];
    let preview_scene = build_bars_scene(&preview_arr, Some(2), Some(3), "Bubble Sort");

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

    for (t, r, o) in preview_scene.get_visible_renderables().iter() {
        let offset = renderer.update_transform(&t);
        let a = |c: Color| Color::rgba(c.r, c.g, c.b, c.a * o);

        if let Some((w, h, col)) = r.as_rectangle() {
            renderer.draw_rectangle(*w, *h, a(*col), offset, &mut pass);
        } else if let Some((rad, col)) = r.as_circle() {
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
    save_png(&renderer, &tex, WIDTH, HEIGHT, "docs/images/bubble_sort.png");
    println!("✓ Preview saved to docs/images/bubble_sort.png");

    Ok(())
}

fn build_bars_scene(
    arr: &[i32],
    compare_idx1: Option<usize>,
    compare_idx2: Option<usize>,
    _title: &str,
) -> SceneGraph {
    let mut scene = SceneGraph::new();

    let bar_width = 0.08;
    let max_height = 0.35;
    let spacing = 0.095;
    let total_width = arr.len() as f32 * spacing;
    let start_x = -total_width / 2.0;

    for (i, &value) in arr.iter().enumerate() {
        let height = (value as f32 / 9.0) * max_height; // Normalize to max value
        let x = start_x + i as f32 * spacing;
        let y = -0.2; // Base position

        // Color based on state
        let color = if Some(i) == compare_idx1 || Some(i) == compare_idx2 {
            // Highlighted for comparison/swap
            Color::new(1.0, 0.3, 0.3) // Red
        } else {
            // Normal color based on value
            let hue = value as f32 / 9.0; // 0.0 to 1.0
            let (r, g, b) = hsv_to_rgb(0.6 - hue * 0.5, 0.7, 0.9); // Blue to cyan gradient
            Color::new(r, g, b)
        };

        // Create rectangle for bar
        let node = scene.create_node_with_transform(
            format!("bar_{}", i),
            Transform::from_translation(x, y + height / 2.0, 0.0),
        );
        scene.get_node_mut(node).unwrap().set_renderable(
            Renderable::Rectangle {
                width: bar_width,
                height,
                color,
            },
        );

        // Add value label circle above each bar
        let label_y = y + height + 0.05;
        let label_node = scene.create_node_with_transform(
            format!("label_{}", i),
            Transform::from_translation(x, label_y, 0.0),
        );

        // Use circle size to represent the value
        let circle_radius = 0.015 + (value as f32 / 9.0) * 0.01;
        scene.get_node_mut(label_node).unwrap().set_renderable(
            Renderable::Circle {
                radius: circle_radius,
                color: Color::new(0.9, 0.9, 0.9),
            },
        );
    }

    scene.update_transforms();
    scene
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

        if let Some((w, h, col)) = r.as_rectangle() {
            renderer.draw_rectangle(*w, *h, a(*col), offset, &mut pass);
        } else if let Some((rad, col)) = r.as_circle() {
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

    let filename = format!("frames/bubble_sort/frame_{:04}.png", frame_num);
    save_png(&renderer, &tex, WIDTH, HEIGHT, &filename);

    if (frame_num + 1) % 100 == 0 {
        println!("  Rendered {} frames...", frame_num + 1);
    }
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
