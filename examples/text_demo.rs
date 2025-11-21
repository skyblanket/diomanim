//! Text Rendering Demo
//!
//! Demonstrates Phase 3 text rendering with animations.
//! Note: This is MVP with placeholder rendering - proper glyph rasterization coming soon!

use diomanim::core::*;
use diomanim::preview::run_preview;
use diomanim::scene::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Diomanim - Text Rendering Demo (Phase 3 MVP)                ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
    println!("Note: Text currently renders as colored rectangles (placeholder)");
    println!("Full glyph rasterization coming in next iteration!\n");

    let mut scene = SceneGraph::new();

    // Title text
    scene
        .add_text("title", "Diomanim", 64.0, Color::new(0.2, 0.6, 1.0))
        .at(0.0, 0.5, 0.0)
        .fade_in(0.0, 1.0);
    println!("  ✓ Title text");

    // Subtitle
    scene
        .add_text(
            "subtitle",
            "Animation Engine",
            32.0,
            Color::new(0.6, 0.6, 0.6),
        )
        .at(0.0, 0.3, 0.0)
        .fade_in(0.5, 1.0);
    println!("  ✓ Subtitle text");

    // Feature list (staggered fade-in)
    let features = vec![
        ("GPU Accelerated", Color::new(1.0, 0.4, 0.4)),
        ("Real-time Preview", Color::new(0.4, 1.0, 0.4)),
        ("Phase 3: Text!", Color::new(0.4, 0.4, 1.0)),
    ];

    for (i, (text, color)) in features.iter().enumerate() {
        let y = -0.1 - (i as f32 * 0.15);
        scene
            .add_text(format!("feature_{}", i), *text, 24.0, *color)
            .at(0.0, y, 0.0)
            .fade_in(1.0 + i as f32 * 0.3, 0.8);
    }
    println!("  ✓ Feature list (3 items)");

    // Animated text (moving + fading)
    scene
        .add_text("moving", "< Animated >", 28.0, Color::new(1.0, 0.8, 0.2))
        .at(-0.6, -0.5, 0.0)
        .fade_in(2.0, 0.5)
        .move_to(2.5, Vector3::new(0.6, -0.5, 0.0), 2.0);
    println!("  ✓ Animated moving text");

    // Add some shapes for context
    scene
        .add_circle("circle1", 0.05, Color::new(1.0, 0.3, 0.3))
        .at(-0.7, 0.5, 0.0)
        .create(0.0, 0.5)
        .spin(0.5, 2.0, 5.0);

    scene
        .add_circle("circle2", 0.05, Color::new(0.3, 1.0, 0.3))
        .at(0.7, 0.5, 0.0)
        .create(0.2, 0.5)
        .spin(0.7, 2.0, 5.0);

    scene
        .add_star("star", 5, 0.06, 0.03, Color::new(1.0, 0.9, 0.3))
        .at(0.0, -0.7, 0.0)
        .grow(3.0, 1.0)
        .spin(4.0, 1.0, 2.0);

    println!("  ✓ Decorative shapes\n");

    println!("✓ Total elements: 8 text + 3 shapes");
    println!("✓ Duration: 6.0 seconds");
    println!("✓ Starting live preview...\n");

    // Run live preview
    run_preview(scene, 6.0, 1280, 720)?;

    Ok(())
}
