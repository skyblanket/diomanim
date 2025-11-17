//! Simple Circle Example
//!
//! This example demonstrates the basic usage of Diomanim:
//! - Creating a renderer
//! - Creating a circle shape
//! - Applying a simple animation
//! - Rendering to an image file

use diomanim::prelude::*;
use diomanim::animation::property::{AnimationClip, AnimationTrack, Keyframe};
use diomanim::scene::{SceneGraph, Renderable};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Diomanim - Simple Circle Example");
    println!("=================================\n");

    // Create the GPU renderer
    println!("Initializing renderer...");
    let renderer = ShapeRenderer::new(800, 600).await?;
    println!("✓ Renderer created (800x600)");

    // Create a scene graph
    let mut scene = SceneGraph::new();

    // Create a circle node
    let circle_id = scene.create_node("MyCircle".to_string());
    let circle_node = scene.get_node_mut(circle_id).unwrap();

    // Attach a circle renderable
    circle_node.set_renderable(Renderable::Circle {
        radius: 0.2,
        color: Color::BLUE,
    });

    println!("✓ Circle created with radius 0.2");

    // Create a simple scale animation
    let mut animation = AnimationClip::new("pulse".to_string());
    let mut scale_track = AnimationTrack::new("scale".to_string());

    // Keyframes: scale from 1.0 to 1.5 and back
    scale_track.add_keyframe(Keyframe::new(
        TimeValue::new(0.0),
        Vector3::new(1.0, 1.0, 1.0),
    ));
    scale_track.add_keyframe(Keyframe::new(
        TimeValue::new(1.0),
        Vector3::new(1.5, 1.5, 1.5),
    ));
    scale_track.add_keyframe(Keyframe::new(
        TimeValue::new(2.0),
        Vector3::new(1.0, 1.0, 1.0),
    ));

    animation.add_track(scale_track);
    animation.loop_animation = true;

    println!("✓ Animation created (2 second pulse loop)");

    // Attach animation to the circle
    let circle_node = scene.get_node_mut(circle_id).unwrap();
    circle_node.add_animation(diomanim::animation::property::AnimationInstance::new(
        animation,
        TimeValue::new(0.0),
    ));

    // Create output texture
    let output_texture = renderer.get_device().create_texture(&wgpu::TextureDescriptor {
        label: Some("Output Texture"),
        size: wgpu::Extent3d {
            width: 800,
            height: 600,
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

    println!("\nRendering frame at t=1.0s (maximum scale)...");

    // Update animation to 1.0 seconds (peak of animation)
    scene.update_animations(TimeValue::new(1.0));
    scene.update_transforms();

    // Render the scene
    let renderables = scene.get_visible_renderables();
    for (transform_uniform, renderable) in renderables {
        renderer.update_transform(&transform_uniform);

        if let Some((radius, color)) = renderable.as_circle() {
            let circle = diomanim::mobjects::Circle {
                radius: *radius,
                color: *color,
                position: Vector3::zero(),
            };
            renderer.render_circle(&circle, *color, &output_view);
        }
    }

    println!("✓ Frame rendered successfully");
    println!("\n✅ Example complete!");
    println!("\nThis example demonstrated:");
    println!("  • Creating a WebGPU renderer");
    println!("  • Building a scene graph");
    println!("  • Adding animated objects");
    println!("  • Rendering a frame");

    Ok(())
}
