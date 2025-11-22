//! Gradient Descent Animation Demo
//!
//! Visualizes gradient descent optimization algorithm with:
//! - Mathematical notation (∇f, learning rate formula)
//! - Animated optimization path
//! - Loss function visualization

use diomanim::animation::effects;
use diomanim::animation::property::AnimationInstance;
use diomanim::core::*;
use diomanim::preview::run_preview;
use diomanim::scene::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Gradient Descent Optimization Animation                     ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
    println!("📉 Visualizing gradient descent on f(x) = x²\n");

    let mut scene = SceneGraph::new();
    let duration = 12.0;

    // Title
    create_title(&mut scene);

    // Mathematical notation
    create_equations(&mut scene);

    // Gradient descent visualization
    create_gradient_descent_viz(&mut scene);

    // Loss curve
    create_loss_curve(&mut scene);

    println!("  ✓ Mathematical equations");
    println!("  ✓ Gradient descent path (5 steps)");
    println!("  ✓ Loss function curve");
    println!("  ✓ Animated convergence\n");

    println!("✓ Duration: {:.1} seconds", duration);
    println!("✓ Starting live preview...\n");

    run_preview(scene, duration, 1920, 1080)?;

    Ok(())
}

fn create_title(scene: &mut SceneGraph) {
    let title_id = scene.create_node_with_transform(
        "Title".to_string(),
        Transform::from_translation(0.0, 0.85, 0.0),
    );
    scene
        .get_node_mut(title_id)
        .unwrap()
        .set_renderable(Renderable::Text {
            content: "Gradient Descent Optimization".to_string(),
            font_size: 56.0,
            color: Color::new(1.0, 1.0, 1.0),
        });

    // Fade in title
    let fade_in = effects::fade_in(1.0);
    scene
        .get_node_mut(title_id)
        .unwrap()
        .add_animation(AnimationInstance::new(fade_in, TimeValue::new(0.0)));
}

fn create_equations(scene: &mut SceneGraph) {
    // Gradient formula
    let eq1_id = scene.create_node_with_transform(
        "GradientFormula".to_string(),
        Transform::from_translation(-0.5, 0.6, 0.0),
    );
    scene
        .get_node_mut(eq1_id)
        .unwrap()
        .set_renderable(Renderable::Math {
            latex: "\\nabla f = 2x".to_string(),
            font_size: 42.0,
            color: Color::new(0.3, 0.8, 1.0),
        });

    let fade_in1 = effects::fade_in(0.8);
    scene
        .get_node_mut(eq1_id)
        .unwrap()
        .add_animation(AnimationInstance::new(fade_in1, TimeValue::new(1.0)));

    // Update rule
    let eq2_id = scene.create_node_with_transform(
        "UpdateRule".to_string(),
        Transform::from_translation(-0.6, 0.4, 0.0),
    );
    scene
        .get_node_mut(eq2_id)
        .unwrap()
        .set_renderable(Renderable::Math {
            latex: "x_{t+1} = x_t - \\alpha \\nabla f".to_string(),
            font_size: 40.0,
            color: Color::new(0.3, 1.0, 0.6),
        });

    let fade_in2 = effects::fade_in(0.8);
    scene
        .get_node_mut(eq2_id)
        .unwrap()
        .add_animation(AnimationInstance::new(fade_in2, TimeValue::new(1.5)));

    // Learning rate
    let eq3_id = scene.create_node_with_transform(
        "LearningRate".to_string(),
        Transform::from_translation(-0.5, 0.2, 0.0),
    );
    scene
        .get_node_mut(eq3_id)
        .unwrap()
        .set_renderable(Renderable::Math {
            latex: "\\alpha = 0.1".to_string(),
            font_size: 38.0,
            color: Color::new(1.0, 0.8, 0.3),
        });

    let fade_in3 = effects::fade_in(0.8);
    scene
        .get_node_mut(eq3_id)
        .unwrap()
        .add_animation(AnimationInstance::new(fade_in3, TimeValue::new(2.0)));
}

fn create_gradient_descent_viz(scene: &mut SceneGraph) {
    // Simulate gradient descent on f(x) = x^2
    // Starting from x = 0.8, learning_rate = 0.1
    let learning_rate = 0.1;
    let steps = vec![
        0.8,   // Step 0: initial
        0.64,  // Step 1: 0.8 - 0.1 * (2 * 0.8) = 0.8 - 0.16 = 0.64
        0.512, // Step 2: 0.64 - 0.1 * (2 * 0.64) = 0.64 - 0.128 = 0.512
        0.41,  // Step 3: 0.512 - 0.1 * (2 * 0.512) = 0.512 - 0.1024 ≈ 0.41
        0.328, // Step 4: 0.41 - 0.1 * (2 * 0.41) = 0.41 - 0.082 = 0.328
        0.26,  // Step 5: 0.328 - 0.1 * (2 * 0.328) = 0.328 - 0.0656 ≈ 0.26
    ];

    let y_offset = -0.2;
    let x_scale = 0.8;
    let y_scale = 0.4;

    // Create points for each step
    for (i, &x) in steps.iter().enumerate() {
        let y = x * x; // f(x) = x^2
        let pos_x = (x - 0.5) * x_scale;
        let pos_y = y_offset - y * y_scale;

        // Create point
        let point_id = scene.create_node_with_transform(
            format!("Point{}", i),
            Transform::from_translation(pos_x, pos_y, 0.0),
        );

        // Color gradient: blue -> green as we converge
        let t = i as f32 / (steps.len() - 1) as f32;
        let color = Color::new(0.3 + t * 0.6, 0.5 + t * 0.5, 1.0 - t * 0.4);

        scene
            .get_node_mut(point_id)
            .unwrap()
            .set_renderable(Renderable::Circle {
                radius: 0.04,
                color,
            });

        // Animate point appearance
        let start_time = 2.5 + i as f32 * 0.8;
        let grow = effects::grow_from_center(0.5);
        scene
            .get_node_mut(point_id)
            .unwrap()
            .add_animation(AnimationInstance::new(grow, TimeValue::new(start_time)));

        // Add step label
        let label_id = scene.create_node_with_transform(
            format!("Label{}", i),
            Transform::from_translation(pos_x, pos_y - 0.12, 0.0),
        );
        scene
            .get_node_mut(label_id)
            .unwrap()
            .set_renderable(Renderable::Text {
                content: format!("{}", i),
                font_size: 24.0,
                color: Color::new(0.7, 0.7, 0.7),
            });

        let fade_in_label = effects::fade_in(0.3);
        scene
            .get_node_mut(label_id)
            .unwrap()
            .add_animation(AnimationInstance::new(
                fade_in_label,
                TimeValue::new(start_time + 0.2),
            ));
    }

    // Add connecting arrows between points
    for i in 0..(steps.len() - 1) {
        let x1 = (steps[i] - 0.5) * x_scale;
        let y1 = y_offset - steps[i] * steps[i] * y_scale;
        let x2 = (steps[i + 1] - 0.5) * x_scale;
        let y2 = y_offset - steps[i + 1] * steps[i + 1] * y_scale;

        let arrow_id = scene.create_node_with_transform(
            format!("Arrow{}", i),
            Transform::from_translation(0.0, 0.0, 0.0),
        );

        scene
            .get_node_mut(arrow_id)
            .unwrap()
            .set_renderable(Renderable::Arrow {
                start: Vector3::new(x1, y1, 0.0),
                end: Vector3::new(x2, y2, 0.0),
                color: Color::new(0.8, 0.8, 0.2),
                thickness: 0.015,
            });

        let start_time = 2.8 + i as f32 * 0.8;
        let create_anim = effects::create(0.4);
        scene
            .get_node_mut(arrow_id)
            .unwrap()
            .add_animation(AnimationInstance::new(
                create_anim,
                TimeValue::new(start_time),
            ));
    }
}

fn create_loss_curve(scene: &mut SceneGraph) {
    // Draw parabola representing f(x) = x^2
    let y_offset = -0.2;
    let x_scale = 0.8;
    let y_scale = 0.4;
    let num_segments = 20;

    for i in 0..num_segments {
        let t1 = i as f32 / num_segments as f32;
        let t2 = (i + 1) as f32 / num_segments as f32;

        let x1 = t1;
        let x2 = t2;
        let y1 = x1 * x1;
        let y2 = x2 * x2;

        let pos_x1 = (x1 - 0.5) * x_scale;
        let pos_y1 = y_offset - y1 * y_scale;
        let pos_x2 = (x2 - 0.5) * x_scale;
        let pos_y2 = y_offset - y2 * y_scale;

        let line_id = scene.create_node_with_transform(
            format!("Curve{}", i),
            Transform::from_translation(0.0, 0.0, 0.0),
        );

        scene
            .get_node_mut(line_id)
            .unwrap()
            .set_renderable(Renderable::Line {
                start: Vector3::new(pos_x1, pos_y1, 0.0),
                end: Vector3::new(pos_x2, pos_y2, 0.0),
                color: Color::new(0.5, 0.5, 0.5),
                thickness: 0.01,
            });

        // Animate curve drawing
        let start_time = 1.5 + i as f32 * 0.03;
        let create_anim = effects::create(0.2);
        scene
            .get_node_mut(line_id)
            .unwrap()
            .add_animation(AnimationInstance::new(
                create_anim,
                TimeValue::new(start_time),
            ));
    }

    // Add axis labels
    let xlabel_id = scene.create_node_with_transform(
        "XLabel".to_string(),
        Transform::from_translation(0.5, -0.65, 0.0),
    );
    scene
        .get_node_mut(xlabel_id)
        .unwrap()
        .set_renderable(Renderable::Text {
            content: "x".to_string(),
            font_size: 32.0,
            color: Color::new(0.8, 0.8, 0.8),
        });

    let ylabel_id = scene.create_node_with_transform(
        "YLabel".to_string(),
        Transform::from_translation(-0.6, 0.0, 0.0),
    );
    scene
        .get_node_mut(ylabel_id)
        .unwrap()
        .set_renderable(Renderable::Math {
            latex: "f(x) = x^2".to_string(),
            font_size: 32.0,
            color: Color::new(0.8, 0.8, 0.8),
        });
}
