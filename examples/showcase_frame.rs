//! Single Frame Showcase
//! Renders one high-quality frame showing all Diomanim features

use diomanim::prelude::*;
use diomanim::render::ShapeRenderer;
use diomanim::scene::{Renderable, SceneGraph};
use std::f32::consts::PI;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Diomanim Feature Showcase - Single Frame                    ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    // Initialize renderer
    println!("Initializing GPU renderer...");
    let mut renderer = ShapeRenderer::new(WIDTH, HEIGHT).await?;
    renderer.init_text_rendering(48.0)?;
    println!("✓ WebGPU renderer ready ({}x{})", WIDTH, HEIGHT);

    println!("\nBuilding showcase scene...");
    let mut scene = build_showcase_scene();
    scene.update_transforms();

    println!("✓ Scene created");

    // Create output texture
    let output_texture = renderer.get_device().create_texture(&wgpu::TextureDescriptor {
        label: Some("Output Texture"),
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

    println!("\nRendering showcase frame...");

    // Render frame
    let mut encoder = renderer.get_device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Frame Render Encoder"),
    });

    let mut render_pass = renderer.begin_render_pass(&mut encoder, &output_view, None);
    render_pass.set_pipeline(renderer.get_pipeline());
    render_pass.set_bind_group(0, renderer.get_transform_bind_group(), &[]);

    // Render all objects
    let renderables = scene.get_visible_renderables();
    for (transform_uniform, renderable, opacity) in renderables {
        renderer.update_transform(&transform_uniform);

        let apply_opacity = |color: Color| -> Color {
            Color::rgba(color.r, color.g, color.b, color.a * opacity)
        };

        if let Some((radius, color)) = renderable.as_circle() {
            let circle = diomanim::mobjects::Circle {
                radius: *radius,
                color: apply_opacity(*color),
                position: Vector3::zero(),
            };
            renderer.draw_circle(&circle, apply_opacity(*color), &mut render_pass);
        } else if let Some((start, end, color, thickness)) = renderable.as_line() {
            renderer.draw_line(*start, *end, apply_opacity(*color), *thickness, &mut render_pass);
        } else if let Some((start, end, color, thickness)) = renderable.as_arrow() {
            renderer.draw_arrow(*start, *end, apply_opacity(*color), *thickness, &mut render_pass);
        } else if let Some((content, font_size, color)) = renderable.as_text() {
            renderer.draw_text(content, *font_size, apply_opacity(*color), &mut render_pass);
        } else if let Some((latex, font_size, color)) = renderable.as_math() {
            renderer.draw_math(latex, *font_size, apply_opacity(*color), &mut render_pass);
        }
    }

    drop(render_pass);
    renderer.get_queue().submit(std::iter::once(encoder.finish()));

    // Save frame
    std::fs::create_dir_all("docs/images")?;
    let filename = "docs/images/diomanim_showcase.png";
    save_frame_to_png(&renderer, &output_texture, WIDTH, HEIGHT, filename);

    println!("✅ Showcase frame saved to: {}", filename);
    println!("   Resolution: {}x{}", WIDTH, HEIGHT);

    Ok(())
}

fn build_showcase_scene() -> SceneGraph {
    let mut scene = SceneGraph::new();

    // Title - centered at top
    let title = scene.create_node_with_transform(
        "Title".into(),
        Transform::from_translation(0.0, 0.75, 0.0),
    );
    scene.get_node_mut(title).unwrap().set_renderable(Renderable::Text {
        content: "DIOMANIM".into(),
        font_size: 72.0,
        color: Color::new(0.2, 0.8, 1.0),
    });

    let subtitle = scene.create_node_with_transform(
        "Subtitle".into(),
        Transform::from_translation(0.0, 0.55, 0.0),
    );
    scene.get_node_mut(subtitle).unwrap().set_renderable(Renderable::Text {
        content: "High-Performance Animation Engine".into(),
        font_size: 38.0,
        color: Color::new(0.9, 0.9, 0.9),
    });

    // Row 1: Colorful circles showing GPU shapes
    let circle1 = scene.create_node_with_transform(
        "Circle1".into(),
        Transform::from_translation(-0.6, 0.2, 0.0),
    );
    scene.get_node_mut(circle1).unwrap().set_renderable(Renderable::Circle {
        radius: 0.12,
        color: Color::new(1.0, 0.2, 0.3),
    });

    let circle2 = scene.create_node_with_transform(
        "Circle2".into(),
        Transform::from_translation(-0.2, 0.2, 0.0),
    );
    scene.get_node_mut(circle2).unwrap().set_renderable(Renderable::Circle {
        radius: 0.12,
        color: Color::new(1.0, 0.7, 0.2),
    });

    let circle3 = scene.create_node_with_transform(
        "Circle3".into(),
        Transform::from_translation(0.2, 0.2, 0.0),
    );
    scene.get_node_mut(circle3).unwrap().set_renderable(Renderable::Circle {
        radius: 0.12,
        color: Color::new(0.3, 1.0, 0.4),
    });

    let circle4 = scene.create_node_with_transform(
        "Circle4".into(),
        Transform::from_translation(0.6, 0.2, 0.0),
    );
    scene.get_node_mut(circle4).unwrap().set_renderable(Renderable::Circle {
        radius: 0.12,
        color: Color::new(0.3, 0.5, 1.0),
    });

    // Math equations - larger and more visible
    let eq1 = scene.create_node_with_transform(
        "Eq1".into(),
        Transform::from_translation(-0.5, -0.15, 0.0),
    );
    scene.get_node_mut(eq1).unwrap().set_renderable(Renderable::Math {
        latex: "E = mc^2".into(),
        font_size: 52.0,
        color: Color::new(1.0, 1.0, 1.0),
    });

    let eq2 = scene.create_node_with_transform(
        "Eq2".into(),
        Transform::from_translation(0.2, -0.15, 0.0),
    );
    scene.get_node_mut(eq2).unwrap().set_renderable(Renderable::Math {
        latex: "\\nabla \\cdot E = \\frac{\\rho}{\\epsilon_0}".into(),
        font_size: 48.0,
        color: Color::new(0.3, 0.9, 1.0),
    });

    let eq3 = scene.create_node_with_transform(
        "Eq3".into(),
        Transform::from_translation(-0.5, -0.45, 0.0),
    );
    scene.get_node_mut(eq3).unwrap().set_renderable(Renderable::Math {
        latex: "\\int_0^\\pi \\sin(x) dx = 2".into(),
        font_size: 48.0,
        color: Color::new(1.0, 0.8, 0.3),
    });

    let eq4 = scene.create_node_with_transform(
        "Eq4".into(),
        Transform::from_translation(0.25, -0.45, 0.0),
    );
    scene.get_node_mut(eq4).unwrap().set_renderable(Renderable::Math {
        latex: "x = \\frac{-b \\pm \\sqrt{b^2-4ac}}{2a}".into(),
        font_size: 44.0,
        color: Color::new(0.4, 1.0, 0.5),
    });

    // Lines and arrows
    let line1 = scene.create_node("Line1".into());
    scene.get_node_mut(line1).unwrap().set_renderable(Renderable::Line {
        start: Vector3::new(-0.8, -0.7, 0.0),
        end: Vector3::new(-0.3, -0.7, 0.0),
        color: Color::new(0.6, 0.6, 1.0),
        thickness: 0.02,
    });

    let arrow1 = scene.create_node("Arrow1".into());
    scene.get_node_mut(arrow1).unwrap().set_renderable(Renderable::Arrow {
        start: Vector3::new(-0.15, -0.7, 0.0),
        end: Vector3::new(0.35, -0.7, 0.0),
        color: Color::new(1.0, 0.4, 0.8),
        thickness: 0.02,
    });

    let line2 = scene.create_node("Line2".into());
    scene.get_node_mut(line2).unwrap().set_renderable(Renderable::Line {
        start: Vector3::new(0.5, -0.7, 0.0),
        end: Vector3::new(0.8, -0.7, 0.0),
        color: Color::new(0.4, 1.0, 0.8),
        thickness: 0.02,
    });

    // Bottom text
    let tech = scene.create_node_with_transform(
        "Tech".into(),
        Transform::from_translation(0.0, -0.85, 0.0),
    );
    scene.get_node_mut(tech).unwrap().set_renderable(Renderable::Text {
        content: "Rust + WebGPU + LaTeX    |    551 FPS @ 1080p    |    9.2x Realtime".into(),
        font_size: 28.0,
        color: Color::new(0.7, 0.7, 0.7),
    });

    scene
}

fn save_frame_to_png(
    renderer: &ShapeRenderer,
    texture: &wgpu::Texture,
    width: u32,
    height: u32,
    filename: &str,
) {
    const COPY_BYTES_PER_ROW_ALIGNMENT: u32 = 256;
    let unpadded_bytes_per_row = width * 4;
    let padded_bytes_per_row = (unpadded_bytes_per_row + COPY_BYTES_PER_ROW_ALIGNMENT - 1)
        / COPY_BYTES_PER_ROW_ALIGNMENT
        * COPY_BYTES_PER_ROW_ALIGNMENT;
    let buffer_size = (padded_bytes_per_row * height) as wgpu::BufferAddress;

    let staging_buffer = renderer.get_device().create_buffer(&wgpu::BufferDescriptor {
        label: Some("Staging Buffer"),
        size: buffer_size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let mut encoder = renderer
        .get_device()
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Copy Encoder"),
        });

    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &staging_buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded_bytes_per_row),
                rows_per_image: Some(height),
            },
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );

    renderer
        .get_queue()
        .submit(std::iter::once(encoder.finish()));

    let buffer_slice = staging_buffer.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
        tx.send(result).unwrap();
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
                let _ = renderer.get_device().poll(wgpu::PollType::Wait {
                    submission_index: None,
                    timeout: None,
                });
                std::thread::yield_now();
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                panic!("Channel disconnected")
            }
        }
    }

    let data = buffer_slice.get_mapped_range();
    use std::fs::File;
    use std::io::BufWriter;
    let file = File::create(filename).unwrap();
    let writer = BufWriter::new(file);

    let mut encoder = png::Encoder::new(writer, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut png_writer = encoder.write_header().unwrap();

    if padded_bytes_per_row != unpadded_bytes_per_row {
        let mut unpadded_data =
            Vec::with_capacity((unpadded_bytes_per_row * height) as usize);
        for row in 0..height {
            let row_start = (row * padded_bytes_per_row) as usize;
            let row_end = row_start + unpadded_bytes_per_row as usize;
            unpadded_data.extend_from_slice(&data[row_start..row_end]);
        }
        png_writer.write_image_data(&unpadded_data).unwrap();
    } else {
        png_writer.write_image_data(&data).unwrap();
    }

    drop(data);
    staging_buffer.unmap();
}
