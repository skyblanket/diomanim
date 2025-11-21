//! Performance benchmark - measures rendering speed

use diomanim::prelude::*;
use diomanim::scene::{SceneGraph, Renderable};
use diomanim::render::ShapeRenderer;
use diomanim::mobjects::Polygon;
use std::time::Instant;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const BENCHMARK_FRAMES: u32 = 300; // 5 seconds at 60 FPS

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Diomanim Performance Benchmark                               ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    println!("Configuration:");
    println!("  Resolution: {}x{}", WIDTH, HEIGHT);
    println!("  Frames: {}", BENCHMARK_FRAMES);
    println!("  Target: 60 FPS (16.67ms per frame)\n");

    // Initialize renderer
    println!("Initializing GPU renderer...");
    let start_init = Instant::now();
    let renderer = ShapeRenderer::new(WIDTH, HEIGHT).await?;
    let init_time = start_init.elapsed();
    println!("✓ Renderer initialized in {:.2}ms\n", init_time.as_secs_f32() * 1000.0);

    // Build test scene with multiple objects
    println!("Building test scene...");
    let mut scene = SceneGraph::new();

    // Add 10 shapes of different types
    for i in 0..10 {
        let x = -0.8 + (i as f32 * 0.18);
        let y = ((i as f32 * 0.5).sin() * 0.3);

        if i % 3 == 0 {
            // Circle
            let id = scene.create_node_with_transform(
                format!("circle_{}", i),
                Transform::from_translation(x, y, 0.0),
            );
            scene.get_node_mut(id).unwrap().set_renderable(Renderable::Circle {
                radius: 0.08,
                color: Color::from_hex("#FF6B6B"),
            });
        } else if i % 3 == 1 {
            // Pentagon
            let id = scene.create_node_with_transform(
                format!("pentagon_{}", i),
                Transform::from_translation(x, y, 0.0),
            );
            let pentagon = Polygon::regular(5, 0.08, Color::from_hex("#4ECDC4"));
            scene.get_node_mut(id).unwrap().set_renderable(Renderable::Polygon {
                vertices: pentagon.vertices,
                color: pentagon.color,
            });
        } else {
            // Star
            let id = scene.create_node_with_transform(
                format!("star_{}", i),
                Transform::from_translation(x, y, 0.0),
            );
            let star = Polygon::star(5, 0.08, 0.04, Color::from_hex("#FFC947"));
            scene.get_node_mut(id).unwrap().set_renderable(Renderable::Polygon {
                vertices: star.vertices,
                color: star.color,
            });
        }
    }

    scene.update_transforms();
    let object_count = scene.get_visible_renderables().len();
    println!("✓ Scene created with {} objects\n", object_count);

    // Create render target
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

    // Warmup
    println!("Warming up GPU...");
    for _ in 0..30 {
        render_frame(&renderer, &scene, &output_view);
    }
    println!("✓ Warmup complete\n");

    // Benchmark
    println!("Running benchmark...");
    let start_benchmark = Instant::now();

    for frame in 0..BENCHMARK_FRAMES {
        render_frame(&renderer, &scene, &output_view);

        if (frame + 1) % 60 == 0 || frame + 1 == BENCHMARK_FRAMES {
            let elapsed = start_benchmark.elapsed().as_secs_f32();
            let current_fps = (frame + 1) as f32 / elapsed;
            print!("\r  Frame {}/{} - {:.1} FPS", frame + 1, BENCHMARK_FRAMES, current_fps);
            std::io::Write::flush(&mut std::io::stdout()).ok();
        }
    }

    let total_time = start_benchmark.elapsed();
    println!("\n");

    // Results
    let total_secs = total_time.as_secs_f32();
    let avg_fps = BENCHMARK_FRAMES as f32 / total_secs;
    let avg_frame_time = (total_secs / BENCHMARK_FRAMES as f32) * 1000.0;
    let target_fps = 60.0;
    let speedup = avg_fps / target_fps;

    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  BENCHMARK RESULTS                                            ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
    println!("Performance:");
    println!("  Average FPS: {:.1}", avg_fps);
    println!("  Frame time: {:.2}ms (target: 16.67ms)", avg_frame_time);
    println!("  Total time: {:.2}s (for {} frames)", total_secs, BENCHMARK_FRAMES);
    println!("  Speedup: {:.1}x realtime rendering", speedup);
    println!("\nScene complexity:");
    println!("  Objects: {}", object_count);
    println!("  Resolution: {}x{} ({:.1}M pixels)", WIDTH, HEIGHT, (WIDTH * HEIGHT) as f32 / 1_000_000.0);
    println!("\n✨ Performance: {}",
        if avg_fps >= 60.0 { "EXCELLENT - Real-time capable!" }
        else if avg_fps >= 30.0 { "GOOD - Smooth rendering" }
        else { "Fair - Consider optimization" }
    );

    Ok(())
}

fn render_frame(renderer: &ShapeRenderer, scene: &SceneGraph, output_view: &wgpu::TextureView) {
    let mut encoder = renderer.get_device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Frame"),
    });

    let mut render_pass = renderer.begin_render_pass(&mut encoder, output_view, None);
    render_pass.set_pipeline(renderer.get_pipeline());
    render_pass.set_bind_group(0, renderer.get_transform_bind_group(), &[]);

    let renderables = scene.get_visible_renderables();
    for (transform_uniform, renderable, opacity) in renderables {
        renderer.update_transform(&transform_uniform);

        let apply_opacity = |color: Color| -> Color {
            Color::rgba(color.r, color.g, color.b, color.a * opacity)
        };

        match renderable {
            Renderable::Circle { radius, color } => {
                let circle = diomanim::mobjects::Circle {
                    radius,
                    color: apply_opacity(color),
                    position: Vector3::zero(),
                };
                renderer.draw_circle(&circle, apply_opacity(color), &mut render_pass);
            }
            Renderable::Polygon { vertices, color } => {
                renderer.draw_polygon(&vertices, apply_opacity(color), &mut render_pass);
            }
            _ => {}
        }
    }

    drop(render_pass);
    renderer.get_queue().submit(std::iter::once(encoder.finish()));
}
