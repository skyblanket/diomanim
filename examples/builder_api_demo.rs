//! Builder API Demo
//!
//! Demonstrates the fluent builder API for creating scenes with minimal boilerplate.
//! Compare this to the traditional verbose API.

use diomanim::core::*;
use diomanim::preview::run_preview;
use diomanim::scene::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Diomanim - Fluent Builder API Demo                          ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    let mut scene = SceneGraph::new();

    println!("Creating animated scene with fluent API:\n");

    // Bouncing ball effect with circles
    scene
        .add_circle("ball1", 0.08, Color::new(1.0, 0.3, 0.3))
        .at(-0.6, 0.4, 0.0)
        .create(0.0, 0.5)
        .move_to(0.7, Vector3::new(-0.6, -0.4, 0.0), 1.0)
        .move_to(1.7, Vector3::new(-0.6, 0.4, 0.0), 1.0);
    println!("  ✓ Red bouncing ball");

    scene
        .add_circle("ball2", 0.08, Color::new(0.3, 0.8, 1.0))
        .at(-0.2, 0.4, 0.0)
        .create(0.2, 0.5)
        .move_to(0.9, Vector3::new(-0.2, -0.4, 0.0), 1.0)
        .move_to(1.9, Vector3::new(-0.2, 0.4, 0.0), 1.0);
    println!("  ✓ Blue bouncing ball");

    scene
        .add_circle("ball3", 0.08, Color::new(0.3, 1.0, 0.3))
        .at(0.2, 0.4, 0.0)
        .create(0.4, 0.5)
        .move_to(1.1, Vector3::new(0.2, -0.4, 0.0), 1.0)
        .move_to(2.1, Vector3::new(0.2, 0.4, 0.0), 1.0);
    println!("  ✓ Green bouncing ball");

    scene
        .add_circle("ball4", 0.08, Color::new(1.0, 1.0, 0.3))
        .at(0.6, 0.4, 0.0)
        .create(0.6, 0.5)
        .move_to(1.3, Vector3::new(0.6, -0.4, 0.0), 1.0)
        .move_to(2.3, Vector3::new(0.6, 0.4, 0.0), 1.0);
    println!("  ✓ Yellow bouncing ball\n");

    // Spinning geometric shapes
    scene
        .add_square("square", 0.15, Color::new(0.9, 0.3, 0.5))
        .at(-0.5, 0.0, 0.0)
        .grow(0.0, 1.0)
        .spin(1.0, 2.0, 3.0);
    println!("  ✓ Spinning square");

    scene
        .add_triangle("triangle", 0.12, Color::new(0.5, 0.3, 0.9))
        .at(0.0, 0.0, 0.0)
        .grow(0.3, 1.0)
        .spin(1.3, 2.0, 3.0);
    println!("  ✓ Spinning triangle");

    scene
        .add_pentagon("pentagon", 0.12, Color::new(0.3, 0.9, 0.6))
        .at(0.5, 0.0, 0.0)
        .grow(0.6, 1.0)
        .spin(1.6, 2.0, 3.0);
    println!("  ✓ Spinning pentagon\n");

    // Animated arrows showing directions
    scene
        .add_arrow(
            "arrow_right",
            Vector3::new(-0.5, -0.5, 0.0),
            Vector3::new(-0.2, -0.5, 0.0),
            Color::new(0.8, 0.3, 0.3),
            3.0,
        )
        .create(0.0, 1.0);

    scene
        .add_arrow(
            "arrow_down",
            Vector3::new(0.0, -0.3, 0.0),
            Vector3::new(0.0, -0.6, 0.0),
            Color::new(0.3, 0.8, 0.3),
            3.0,
        )
        .create(0.3, 1.0);

    scene
        .add_arrow(
            "arrow_up",
            Vector3::new(0.5, -0.6, 0.0),
            Vector3::new(0.5, -0.3, 0.0),
            Color::new(0.3, 0.3, 0.8),
            3.0,
        )
        .create(0.6, 1.0);
    println!("  ✓ Directional arrows\n");

    // Starfield effect
    scene
        .add_star("star1", 5, 0.08, 0.03, Color::new(1.0, 0.9, 0.3))
        .at(-0.7, 0.7, 0.0)
        .scale(0.0)
        .grow(2.0, 0.5)
        .spin(2.5, 1.0, 2.0);

    scene
        .add_star("star2", 5, 0.06, 0.025, Color::new(1.0, 0.9, 0.3))
        .at(0.0, 0.7, 0.0)
        .scale(0.0)
        .grow(2.2, 0.5)
        .spin(2.7, 1.0, 2.0);

    scene
        .add_star("star3", 5, 0.07, 0.028, Color::new(1.0, 0.9, 0.3))
        .at(0.7, 0.7, 0.0)
        .scale(0.0)
        .grow(2.4, 0.5)
        .spin(2.9, 1.0, 2.0);
    println!("  ✓ Twinkling stars\n");

    println!("✓ Total objects: 14");
    println!("✓ Duration: 5.0 seconds");
    println!("✓ All created with fluent builder API!");
    println!("✓ Starting live preview...\n");

    // Run live preview
    run_preview(scene, 5.0, 1280, 720)?;

    Ok(())
}
