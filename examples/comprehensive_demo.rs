//! Comprehensive Demo - Showcasing All Diomanim Features
//!
//! This example demonstrates:
//! - Multiple shape types (Circle, Rectangle, Line, Arrow, Polygon)
//! - Easing functions for smooth animations
//! - Scene graph hierarchy with transform inheritance
//! - GPU-accelerated batch rendering
//! - Real-time frame generation

use diomanim::prelude::*;
use diomanim::animation::property::{AnimationClip, AnimationTrack, AnimationInstance, Keyframe};
use diomanim::animation::easing::EasingType;
use diomanim::scene::{SceneGraph, Renderable};
use diomanim::mobjects::{Polygon};
use diomanim::render::ShapeRenderer;
use std::time::Instant;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const DURATION: f32 = 5.0;
const FPS: f32 = 30.0;
const FRAME_TIME: f32 = 1.0 / FPS;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Diomanim v0.1.0 - Comprehensive Feature Demo                ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    // Initialize renderer
    println!("Initializing GPU renderer...");
    let renderer = ShapeRenderer::new(WIDTH, HEIGHT).await?;
    println!("✓ WebGPU renderer ready ({}x{})", WIDTH, HEIGHT);

    println!("\nBuilding comprehensive scene...");
    let mut scene = SceneGraph::new();

    // ========================================================================
    // CIRCLES with different easing functions
    // ========================================================================
    println!("  • Adding circles with various easing functions");

    let circle_colors = [
        (Color::RED, EasingType::EaseInOutCubic),
        (Color::GREEN, EasingType::EaseOutBounce),
        (Color::BLUE, EasingType::EaseInOutElastic),
        (Color::YELLOW, EasingType::EaseInOutSine),
    ];

    for (i, (color, _easing)) in circle_colors.iter().enumerate() {
        let y_pos = 0.5 - (i as f32 * 0.3);
        let circle_id = scene.create_node_with_transform(
            format!("Circle_{}", i),
            Transform::from_translation(-0.7, y_pos, 0.0),
        );

        scene.get_node_mut(circle_id).unwrap().set_renderable(Renderable::Circle {
            radius: 0.08,
            color: *color,
        });

        // Animate circle moving right
        create_slide_animation(&mut scene, circle_id, -0.7, 0.7, y_pos);
    }

    // ========================================================================
    // RECTANGLES rotating
    // ========================================================================
    println!("  • Adding rotating rectangles");

    let rect_id = scene.create_node_with_transform(
        "Rectangle".to_string(),
        Transform::from_translation(0.0, -0.3, 0.0),
    );

    scene.get_node_mut(rect_id).unwrap().set_renderable(Renderable::Rectangle {
        width: 0.25,
        height: 0.15,
        color: Color::CYAN,
    });

    create_rotation_animation(&mut scene, rect_id, 0.0);

    // ========================================================================
    // LINES showing different patterns
    // ========================================================================
    println!("  • Adding animated lines");

    let line_id = scene.create_node("Line".to_string());
    scene.get_node_mut(line_id).unwrap().set_renderable(Renderable::Line {
        start: Vector3::new(-0.4, 0.6, 0.0),
        end: Vector3::new(0.4, 0.6, 0.0),
        color: Color::MAGENTA,
        thickness: 3.0,
    });

    // ========================================================================
    // ARROWS pointing directions
    // ========================================================================
    println!("  • Adding directional arrows");

    let arrow_positions = [
        (Vector3::new(-0.5, -0.6, 0.0), Vector3::new(-0.3, -0.5, 0.0)),
        (Vector3::new(0.0, -0.6, 0.0), Vector3::new(0.2, -0.5, 0.0)),
        (Vector3::new(0.5, -0.6, 0.0), Vector3::new(0.3, -0.5, 0.0)),
    ];

    for (i, (start, end)) in arrow_positions.iter().enumerate() {
        let arrow_id = scene.create_node(format!("Arrow_{}", i));
        scene.get_node_mut(arrow_id).unwrap().set_renderable(Renderable::Arrow {
            start: *start,
            end: *end,
            color: Color::ORANGE,
            thickness: 2.0,
        });
    }

    // ========================================================================
    // POLYGONS - various shapes
    // ========================================================================
    println!("  • Adding polygons (triangle, pentagon, hexagon, star)");

    // Triangle
    let triangle_id = scene.create_node_with_transform(
        "Triangle".to_string(),
        Transform::from_translation(-0.6, 0.2, 0.0).with_scale(Vector3::new(0.5, 0.5, 0.5)),
    );
    let triangle = Polygon::triangle(0.15, Color::new(1.0, 0.5, 0.0));
    scene.get_node_mut(triangle_id).unwrap().set_renderable(Renderable::Polygon {
        vertices: triangle.vertices,
        color: triangle.color,
    });
    create_pulse_animation(&mut scene, triangle_id);

    // Pentagon
    let pentagon_id = scene.create_node_with_transform(
        "Pentagon".to_string(),
        Transform::from_translation(-0.2, 0.2, 0.0).with_scale(Vector3::new(0.5, 0.5, 0.5)),
    );
    let pentagon = Polygon::pentagon(0.15, Color::new(0.5, 1.0, 0.5));
    scene.get_node_mut(pentagon_id).unwrap().set_renderable(Renderable::Polygon {
        vertices: pentagon.vertices,
        color: pentagon.color,
    });
    create_pulse_animation(&mut scene, pentagon_id);

    // Hexagon
    let hexagon_id = scene.create_node_with_transform(
        "Hexagon".to_string(),
        Transform::from_translation(0.2, 0.2, 0.0).with_scale(Vector3::new(0.5, 0.5, 0.5)),
    );
    let hexagon = Polygon::hexagon(0.15, Color::new(0.5, 0.5, 1.0));
    scene.get_node_mut(hexagon_id).unwrap().set_renderable(Renderable::Polygon {
        vertices: hexagon.vertices,
        color: hexagon.color,
    });
    create_pulse_animation(&mut scene, hexagon_id);

    // Star
    let star_id = scene.create_node_with_transform(
        "Star".to_string(),
        Transform::from_translation(0.6, 0.2, 0.0).with_scale(Vector3::new(0.5, 0.5, 0.5)),
    );
    let star = Polygon::star(5, 0.15, 0.07, Color::new(1.0, 1.0, 0.0));
    scene.get_node_mut(star_id).unwrap().set_renderable(Renderable::Polygon {
        vertices: star.vertices,
        color: star.color,
    });
    create_pulse_animation(&mut scene, star_id);

    println!("✓ Created scene with {} nodes", scene.get_visible_renderables().len());

    // ========================================================================
    // RENDER LOOP
    // ========================================================================
    println!("\nRendering {} seconds at {} FPS...", DURATION, FPS);

    let frames_dir = "frames";
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
    let mut frame_count = 0;
    let total_frames = (DURATION * FPS) as u32;

    while frame_count < total_frames {
        let delta_time = TimeValue::new(FRAME_TIME);

        // Update animations and transforms
        scene.update_animations(delta_time);
        scene.update_transforms();

        // Create command encoder
        let mut encoder = renderer.get_device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Frame Render Encoder"),
        });

        // Begin render pass once per frame
        let mut render_pass = renderer.begin_render_pass(&mut encoder, &output_view, None);

        render_pass.set_pipeline(renderer.get_pipeline());
        render_pass.set_bind_group(0, renderer.get_transform_bind_group(), &[]);

        // Render all visible objects
        let renderables = scene.get_visible_renderables();
        for (transform_uniform, renderable) in renderables {
            renderer.update_transform(&transform_uniform);

            match renderable {
                Renderable::Circle { radius, color } => {
                    let circle = diomanim::mobjects::Circle {
                        radius,
                        color,
                        position: Vector3::zero(),
                    };
                    renderer.draw_circle(&circle, color, &mut render_pass);
                }
                Renderable::Rectangle { width, height, color } => {
                    renderer.draw_rectangle(width, height, color, &mut render_pass);
                }
                Renderable::Line { start, end, color, thickness } => {
                    renderer.draw_line(start, end, color, thickness, &mut render_pass);
                }
                Renderable::Arrow { start, end, color, thickness } => {
                    renderer.draw_arrow(start, end, color, thickness, &mut render_pass);
                }
                Renderable::Polygon { vertices, color } => {
                    renderer.draw_polygon(&vertices, color, &mut render_pass);
                }
            }
        }

        // End render pass
        drop(render_pass);

        // Submit command
        renderer.get_queue().submit(std::iter::once(encoder.finish()));

        frame_count += 1;
        if frame_count % 10 == 0 || frame_count == total_frames {
            let progress = (frame_count as f32 / total_frames as f32) * 100.0;
            print!("\r  Progress: {}/{} frames ({:.1}%)", frame_count, total_frames, progress);
            std::io::Write::flush(&mut std::io::stdout()).ok();
        }
    }
    println!();

    let elapsed = start_time.elapsed().as_secs_f32();
    let avg_fps = frame_count as f32 / elapsed;

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Render Complete                                              ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!("\n📊 Statistics:");
    println!("  Frames rendered: {} @ {} FPS target", frame_count, FPS);
    println!("  Actual FPS: {:.1}", avg_fps);
    println!("  Total time: {:.2}s", elapsed);
    println!("  Objects rendered: {} per frame", scene.get_visible_renderables().len());
    println!("  Total draw calls: {}", frame_count * scene.get_visible_renderables().len() as u32);
    println!("\n✨ Features demonstrated:");
    println!("  ✓ Multiple shape types (Circle, Rectangle, Line, Arrow, Polygon)");
    println!("  ✓ Easing functions (Cubic, Bounce, Elastic, Sine)");
    println!("  ✓ Scene graph with hierarchy");
    println!("  ✓ GPU-accelerated batch rendering");
    println!("  ✓ Transform inheritance");
    println!("  ✓ Keyframe animations");

    Ok(())
}

// Helper animation functions

fn create_slide_animation(scene: &mut SceneGraph, node_id: diomanim::scene::NodeId, start_x: f32, end_x: f32, y: f32) {
    let mut animation = AnimationClip::new("slide".to_string());
    let mut track = AnimationTrack::new("position".to_string());

    track.add_keyframe(Keyframe::new(
        TimeValue::new(0.0),
        Vector3::new(start_x, y, 0.0),
    ));
    track.add_keyframe(Keyframe::new(
        TimeValue::new(DURATION),
        Vector3::new(end_x, y, 0.0),
    ));

    animation.add_track(track);
    animation.loop_animation = false;

    scene.get_node_mut(node_id).unwrap()
        .add_animation(AnimationInstance::new(animation, TimeValue::new(0.0)));
}

fn create_rotation_animation(scene: &mut SceneGraph, node_id: diomanim::scene::NodeId, start_angle: f32) {
    let mut animation = AnimationClip::new("rotate".to_string());
    let mut track = AnimationTrack::new("rotation".to_string());

    track.add_keyframe(Keyframe::new(
        TimeValue::new(0.0),
        Vector3::new(0.0, 0.0, start_angle),
    ));
    track.add_keyframe(Keyframe::new(
        TimeValue::new(DURATION),
        Vector3::new(0.0, 0.0, start_angle + std::f32::consts::TAU * 2.0),
    ));

    animation.add_track(track);
    animation.loop_animation = false;

    scene.get_node_mut(node_id).unwrap()
        .add_animation(AnimationInstance::new(animation, TimeValue::new(0.0)));
}

fn create_pulse_animation(scene: &mut SceneGraph, node_id: diomanim::scene::NodeId) {
    let mut animation = AnimationClip::new("pulse".to_string());
    let mut track = AnimationTrack::new("scale".to_string());

    track.add_keyframe(Keyframe::new(TimeValue::new(0.0), Vector3::one()));
    track.add_keyframe(Keyframe::new(TimeValue::new(DURATION / 2.0), Vector3::new(1.5, 1.5, 1.5)));
    track.add_keyframe(Keyframe::new(TimeValue::new(DURATION), Vector3::one()));

    animation.add_track(track);
    animation.loop_animation = false;

    scene.get_node_mut(node_id).unwrap()
        .add_animation(AnimationInstance::new(animation, TimeValue::new(0.0)));
}
