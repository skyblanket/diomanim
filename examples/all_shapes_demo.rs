//! All Shapes Demo
//!
//! Comprehensive demonstration of all available shapes:
//! - Circle
//! - Rectangle
//! - Line
//! - Arrow
//! - Polygon (triangle, pentagon, hexagon, star)

use diomanim::animation::{effects, property::AnimationInstance};
use diomanim::core::*;
use diomanim::preview::run_preview;
use diomanim::scene::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Diomanim - All Shapes Showcase                              ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    let mut scene = SceneGraph::new();

    // Row 1: Circles and rectangles
    println!("Creating shapes:");

    // Circle (red)
    let circle_id = scene.create_node_with_transform(
        "Circle".to_string(),
        Transform::from_translation(-0.6, 0.4, 0.0),
    );
    scene.get_node_mut(circle_id).unwrap().set_renderable(Renderable::Circle {
        radius: 0.12,
        color: Color::new(0.9, 0.2, 0.2),
    });
    let fade_in = effects::fade_in(1.0);
    scene.get_node_mut(circle_id).unwrap()
        .add_animation(AnimationInstance::new(fade_in, TimeValue::new(0.0)));
    println!("  ✓ Circle (red)");

    // Rectangle (blue)
    let rect_id = scene.create_node_with_transform(
        "Rectangle".to_string(),
        Transform::from_translation(-0.3, 0.4, 0.0),
    );
    scene.get_node_mut(rect_id).unwrap().set_renderable(Renderable::Rectangle {
        width: 0.2,
        height: 0.15,
        color: Color::new(0.2, 0.4, 0.9),
    });
    let fade_in = effects::fade_in(1.0);
    scene.get_node_mut(rect_id).unwrap()
        .add_animation(AnimationInstance::new(fade_in, TimeValue::new(0.2)));
    println!("  ✓ Rectangle (blue)");

    // Square (via Rectangle)
    let square_id = scene.create_node_with_transform(
        "Square".to_string(),
        Transform::from_translation(0.0, 0.4, 0.0),
    );
    scene.get_node_mut(square_id).unwrap().set_renderable(Renderable::Rectangle {
        width: 0.16,
        height: 0.16,
        color: Color::new(0.2, 0.8, 0.4),
    });
    let fade_in = effects::fade_in(1.0);
    scene.get_node_mut(square_id).unwrap()
        .add_animation(AnimationInstance::new(fade_in, TimeValue::new(0.4)));
    println!("  ✓ Square (green)");

    // Triangle (polygon)
    let triangle_id = scene.create_node_with_transform(
        "Triangle".to_string(),
        Transform::from_translation(0.3, 0.4, 0.0),
    );
    let triangle_vertices = vec![
        Vector3::new(0.0, 0.1, 0.0),    // Top
        Vector3::new(-0.1, -0.1, 0.0),  // Bottom left
        Vector3::new(0.1, -0.1, 0.0),   // Bottom right
    ];
    scene.get_node_mut(triangle_id).unwrap().set_renderable(Renderable::Polygon {
        vertices: triangle_vertices,
        color: Color::new(0.9, 0.7, 0.2),
    });
    let fade_in = effects::fade_in(1.0);
    scene.get_node_mut(triangle_id).unwrap()
        .add_animation(AnimationInstance::new(fade_in, TimeValue::new(0.6)));
    println!("  ✓ Triangle (yellow)");

    // Pentagon (polygon)
    let pentagon_id = scene.create_node_with_transform(
        "Pentagon".to_string(),
        Transform::from_translation(0.6, 0.4, 0.0),
    );
    let angle_step = 2.0 * std::f32::consts::PI / 5.0;
    let pentagon_vertices: Vec<Vector3> = (0..5)
        .map(|i| {
            let angle = i as f32 * angle_step - std::f32::consts::PI / 2.0;
            Vector3::new(0.1 * angle.cos(), 0.1 * angle.sin(), 0.0)
        })
        .collect();
    scene.get_node_mut(pentagon_id).unwrap().set_renderable(Renderable::Polygon {
        vertices: pentagon_vertices,
        color: Color::new(0.8, 0.3, 0.8),
    });
    let fade_in = effects::fade_in(1.0);
    scene.get_node_mut(pentagon_id).unwrap()
        .add_animation(AnimationInstance::new(fade_in, TimeValue::new(0.8)));
    println!("  ✓ Pentagon (purple)");

    // Row 2: Lines and Arrows

    // Horizontal line
    let line1_id = scene.create_node("Line1".to_string());
    scene.get_node_mut(line1_id).unwrap().set_renderable(Renderable::Line {
        start: Vector3::new(-0.6, -0.1, 0.0),
        end: Vector3::new(-0.3, -0.1, 0.0),
        color: Color::new(0.3, 0.3, 0.3),
        thickness: 3.0,
    });
    let create = effects::create(1.0);
    scene.get_node_mut(line1_id).unwrap()
        .add_animation(AnimationInstance::new(create, TimeValue::new(1.0)));
    println!("  ✓ Horizontal Line");

    // Vertical line
    let line2_id = scene.create_node("Line2".to_string());
    scene.get_node_mut(line2_id).unwrap().set_renderable(Renderable::Line {
        start: Vector3::new(-0.15, 0.05, 0.0),
        end: Vector3::new(-0.15, -0.25, 0.0),
        color: Color::new(0.3, 0.3, 0.3),
        thickness: 3.0,
    });
    let create = effects::create(1.0);
    scene.get_node_mut(line2_id).unwrap()
        .add_animation(AnimationInstance::new(create, TimeValue::new(1.2)));
    println!("  ✓ Vertical Line");

    // Diagonal line
    let line3_id = scene.create_node("Line3".to_string());
    scene.get_node_mut(line3_id).unwrap().set_renderable(Renderable::Line {
        start: Vector3::new(0.0, 0.05, 0.0),
        end: Vector3::new(0.2, -0.25, 0.0),
        color: Color::new(0.3, 0.3, 0.3),
        thickness: 3.0,
    });
    let create = effects::create(1.0);
    scene.get_node_mut(line3_id).unwrap()
        .add_animation(AnimationInstance::new(create, TimeValue::new(1.4)));
    println!("  ✓ Diagonal Line");

    // Right arrow
    let arrow1_id = scene.create_node("Arrow1".to_string());
    scene.get_node_mut(arrow1_id).unwrap().set_renderable(Renderable::Arrow {
        start: Vector3::new(0.3, -0.1, 0.0),
        end: Vector3::new(0.55, -0.1, 0.0),
        color: Color::new(0.9, 0.3, 0.3),
        thickness: 3.0,
    });
    let create = effects::create(1.0);
    scene.get_node_mut(arrow1_id).unwrap()
        .add_animation(AnimationInstance::new(create, TimeValue::new(1.6)));
    println!("  ✓ Right Arrow");

    // Down arrow
    let arrow2_id = scene.create_node("Arrow2".to_string());
    scene.get_node_mut(arrow2_id).unwrap().set_renderable(Renderable::Arrow {
        start: Vector3::new(0.7, 0.05, 0.0),
        end: Vector3::new(0.7, -0.25, 0.0),
        color: Color::new(0.3, 0.7, 0.9),
        thickness: 3.0,
    });
    let create = effects::create(1.0);
    scene.get_node_mut(arrow2_id).unwrap()
        .add_animation(AnimationInstance::new(create, TimeValue::new(1.8)));
    println!("  ✓ Down Arrow");

    // Row 3: Complex polygons

    // Hexagon
    let hexagon_id = scene.create_node_with_transform(
        "Hexagon".to_string(),
        Transform::from_translation(-0.45, -0.45, 0.0),
    );
    let angle_step = 2.0 * std::f32::consts::PI / 6.0;
    let hexagon_vertices: Vec<Vector3> = (0..6)
        .map(|i| {
            let angle = i as f32 * angle_step;
            Vector3::new(0.1 * angle.cos(), 0.1 * angle.sin(), 0.0)
        })
        .collect();
    scene.get_node_mut(hexagon_id).unwrap().set_renderable(Renderable::Polygon {
        vertices: hexagon_vertices,
        color: Color::new(0.2, 0.9, 0.9),
    });
    let grow = effects::grow_from_center(1.5);
    scene.get_node_mut(hexagon_id).unwrap()
        .add_animation(AnimationInstance::new(grow, TimeValue::new(2.0)));
    println!("  ✓ Hexagon (cyan)");

    // 5-pointed star
    let star_id = scene.create_node_with_transform(
        "Star".to_string(),
        Transform::from_translation(0.0, -0.45, 0.0),
    );
    let points = 5;
    let outer_radius = 0.12;
    let inner_radius = 0.05;
    let angle_step = std::f32::consts::PI / points as f32;
    let star_vertices: Vec<Vector3> = (0..(points * 2))
        .map(|i| {
            let angle = i as f32 * angle_step - std::f32::consts::PI / 2.0;
            let radius = if i % 2 == 0 {
                outer_radius
            } else {
                inner_radius
            };
            Vector3::new(radius * angle.cos(), radius * angle.sin(), 0.0)
        })
        .collect();
    scene.get_node_mut(star_id).unwrap().set_renderable(Renderable::Polygon {
        vertices: star_vertices,
        color: Color::new(1.0, 0.8, 0.2),
    });
    let grow = effects::grow_from_center(1.5);
    scene.get_node_mut(star_id).unwrap()
        .add_animation(AnimationInstance::new(grow, TimeValue::new(2.2)));
    println!("  ✓ Star (gold)");

    // Complex polygon (arbitrary shape)
    let complex_id = scene.create_node_with_transform(
        "Complex".to_string(),
        Transform::from_translation(0.45, -0.45, 0.0),
    );
    let complex_vertices = vec![
        Vector3::new(0.0, 0.1, 0.0),
        Vector3::new(0.08, 0.05, 0.0),
        Vector3::new(0.1, -0.05, 0.0),
        Vector3::new(0.0, -0.1, 0.0),
        Vector3::new(-0.1, -0.05, 0.0),
        Vector3::new(-0.08, 0.05, 0.0),
    ];
    scene.get_node_mut(complex_id).unwrap().set_renderable(Renderable::Polygon {
        vertices: complex_vertices,
        color: Color::new(0.9, 0.4, 0.6),
    });
    let grow = effects::grow_from_center(1.5);
    scene.get_node_mut(complex_id).unwrap()
        .add_animation(AnimationInstance::new(grow, TimeValue::new(2.4)));
    println!("  ✓ Complex Polygon (pink)");

    println!("\n✓ Total shapes: 16");
    println!("✓ Animation duration: 6.0 seconds");
    println!("✓ Starting live preview...\n");

    // Run live preview
    run_preview(scene, 6.0, 1280, 720)?;

    Ok(())
}
