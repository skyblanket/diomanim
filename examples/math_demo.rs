//! Mathematical Notation Demo
//!
//! Demonstrates Phase 4 mathematical notation rendering with LaTeX support.

use diomanim::core::*;
use diomanim::preview::run_preview;
use diomanim::scene::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Diomanim - Mathematical Notation Demo (Phase 4)             ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
    println!("✨ LaTeX-style mathematical notation rendering!\n");

    let mut scene = SceneGraph::new();
    let duration = 8.0;

    // Title
    let title_id = scene.create_node_with_transform(
        "Title".to_string(),
        Transform::from_translation(0.0, 0.7, 0.0),
    );
    scene
        .get_node_mut(title_id)
        .unwrap()
        .set_renderable(Renderable::Text {
            content: "Mathematical Notation".to_string(),
            font_size: 72.0,
            color: Color::new(1.0, 1.0, 1.0),
        });

    // Simple expression: x^2 + y^2 = r^2
    let eq1_id = scene.create_node_with_transform(
        "Pythagorean".to_string(),
        Transform::from_translation(-0.5, 0.4, 0.0),
    );
    scene
        .get_node_mut(eq1_id)
        .unwrap()
        .set_renderable(Renderable::Math {
            latex: "x^2 + y^2 = r^2".to_string(),
            font_size: 48.0,
            color: Color::new(0.3, 0.8, 1.0),
        });

    // Fraction: a/b
    let eq2_id = scene.create_node_with_transform(
        "Fraction".to_string(),
        Transform::from_translation(-0.5, 0.1, 0.0),
    );
    scene
        .get_node_mut(eq2_id)
        .unwrap()
        .set_renderable(Renderable::Math {
            latex: "\\frac{a}{b} + \\frac{c}{d}".to_string(),
            font_size: 48.0,
            color: Color::new(0.3, 1.0, 0.6),
        });

    // Greek letters
    let eq3_id = scene.create_node_with_transform(
        "Greek".to_string(),
        Transform::from_translation(-0.5, -0.2, 0.0),
    );
    scene
        .get_node_mut(eq3_id)
        .unwrap()
        .set_renderable(Renderable::Math {
            latex: "\\alpha + \\beta = \\gamma".to_string(),
            font_size: 48.0,
            color: Color::new(1.0, 0.3, 0.6),
        });

    // Square root
    let eq4_id = scene.create_node_with_transform(
        "SquareRoot".to_string(),
        Transform::from_translation(-0.5, -0.5, 0.0),
    );
    scene
        .get_node_mut(eq4_id)
        .unwrap()
        .set_renderable(Renderable::Math {
            latex: "\\sqrt{x + y}".to_string(),
            font_size: 48.0,
            color: Color::new(1.0, 0.8, 0.2),
        });

    // Complex expression with multiple features
    let eq5_id = scene.create_node_with_transform(
        "Complex".to_string(),
        Transform::from_translation(-0.6, -0.8, 0.0),
    );
    scene
        .get_node_mut(eq5_id)
        .unwrap()
        .set_renderable(Renderable::Math {
            latex: "E = mc^2".to_string(),
            font_size: 56.0,
            color: Color::new(1.0, 1.0, 0.3),
        });

    // Add decorative shapes
    create_decorative_shapes(&mut scene);

    println!("  ✓ Pythagorean theorem");
    println!("  ✓ Fractions");
    println!("  ✓ Greek letters");
    println!("  ✓ Square roots");
    println!("  ✓ Famous equations\n");

    println!("✓ Total elements: 5 equations + 3 shapes");
    println!("✓ Duration: {:.1} seconds", duration);
    println!("✓ Starting live preview...\n");

    // Run live preview
    run_preview(scene, duration, 1920, 1080)?;

    Ok(())
}

fn create_decorative_shapes(scene: &mut SceneGraph) {
    // Left accent circle
    let circle1_id = scene.create_node_with_transform(
        "Circle1".to_string(),
        Transform::from_translation(-0.9, 0.0, 0.0),
    );
    scene
        .get_node_mut(circle1_id)
        .unwrap()
        .set_renderable(Renderable::Circle {
            radius: 0.05,
            color: Color::new(0.3, 0.8, 1.0),
        });

    // Right accent circle
    let circle2_id = scene.create_node_with_transform(
        "Circle2".to_string(),
        Transform::from_translation(0.9, 0.0, 0.0),
    );
    scene
        .get_node_mut(circle2_id)
        .unwrap()
        .set_renderable(Renderable::Circle {
            radius: 0.05,
            color: Color::new(1.0, 0.3, 0.6),
        });

    // Bottom accent line
    let line_id = scene.create_node_with_transform(
        "Line".to_string(),
        Transform::from_translation(0.0, -0.95, 0.0),
    );
    scene
        .get_node_mut(line_id)
        .unwrap()
        .set_renderable(Renderable::Line {
            start: Vector3::new(-0.8, 0.0, 0.0),
            end: Vector3::new(0.8, 0.0, 0.0),
            color: Color::new(0.5, 0.5, 0.5),
            thickness: 0.01,
        });
}
