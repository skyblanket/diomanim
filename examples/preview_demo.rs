//! Live Preview Demo
//!
//! Demonstrates the live preview window with real-time playback controls.

use diomanim::animation::{effects, property::AnimationInstance};
use diomanim::core::*;
use diomanim::preview::run_preview;
use diomanim::scene::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Diomanim Live Preview Demo                                  ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    // Create scene
    let mut scene = SceneGraph::new();

    // Create center circle with pulsing animation
    let center_id = scene.create_node("Center".to_string());
    scene
        .get_node_mut(center_id)
        .unwrap()
        .set_renderable(Renderable::Circle {
            radius: 0.2,
            color: Color::new(0.9, 0.2, 0.2),
        });

    // Add grow/shrink animation
    let pulse = effects::fade_in_out(0.5, 2.0, 0.5);
    scene
        .get_node_mut(center_id)
        .unwrap()
        .add_animation(AnimationInstance::new(pulse, TimeValue::new(0.0)));

    // Create orbiting circles
    let colors = [
        Color::CYAN,
        Color::YELLOW,
        Color::GREEN,
        Color::MAGENTA,
        Color::new(1.0, 0.5, 0.0), // Orange
    ];

    for (i, &color) in colors.iter().enumerate() {
        let angle = (i as f32 / colors.len() as f32) * std::f32::consts::TAU;
        let orbit_radius = 0.4;

        let child_id = scene.create_node_with_transform(
            format!("Orbiter_{}", i),
            Transform::from_translation(
                angle.cos() * orbit_radius,
                angle.sin() * orbit_radius,
                0.0,
            ),
        );

        scene
            .get_node_mut(child_id)
            .unwrap()
            .set_renderable(Renderable::Circle {
                radius: 0.08,
                color,
            });

        // Create animation (Phase 2 effects!)
        let spin_duration = 5.0;
        let spin_anim = effects::spin(2.0, spin_duration); // 2 rotations in 5 seconds
        scene
            .get_node_mut(child_id)
            .unwrap()
            .add_animation(AnimationInstance::new(spin_anim, TimeValue::new(0.0)));

        // Parent to center for orbit effect
        scene.parent(child_id, center_id).unwrap();

        // Add fade in animation at staggered times
        let fade_start = i as f32 * 0.2;
        let fade_in = effects::fade_in(1.0);
        scene
            .get_node_mut(child_id)
            .unwrap()
            .add_animation(AnimationInstance::new(fade_in, TimeValue::new(fade_start)));
    }

    // Create a moving square
    let square_id = scene.create_node_with_transform(
        "Square".to_string(),
        Transform::from_translation(-0.6, 0.5, 0.0),
    );
    scene
        .get_node_mut(square_id)
        .unwrap()
        .set_renderable(Renderable::Circle {
            radius: 0.1,
            color: Color::new(0.5, 0.5, 1.0),
        });

    // Move it across the screen
    let move_anim = effects::move_to(
        Vector3::new(-0.6, 0.5, 0.0),
        Vector3::new(0.6, 0.5, 0.0),
        4.0,
    );
    scene
        .get_node_mut(square_id)
        .unwrap()
        .add_animation(AnimationInstance::new(move_anim, TimeValue::new(0.0)));

    println!("✓ Created {} animated objects", colors.len() + 2);
    println!("✓ Scene duration: 5.0 seconds");
    println!("✓ Starting live preview...\n");

    // Run live preview
    run_preview(scene, 5.0, 1280, 720)?;

    Ok(())
}
