//! Scene Builder API
//!
//! Provides a fluent, chainable API for building scenes more ergonomically.
//!
//! ## Example
//!
//! ```rust,no_run
//! use diomanim::scene::*;
//! use diomanim::core::*;
//!
//! let mut scene = SceneGraph::new();
//!
//! // Fluent API
//! scene.add_circle("my_circle", 1.0, Color::RED)
//!     .at(0.0, 0.0, 0.0)
//!     .scale(2.0)
//!     .fade_in(0.0, 1.0);
//!
//! scene.add_rectangle("my_rect", 2.0, 1.0, Color::BLUE)
//!     .at(3.0, 0.0, 0.0)
//!     .rotate_z(45.0);
//! ```

use super::{NodeId, Renderable, SceneGraph};
use crate::animation::{effects, property::AnimationInstance};
use crate::core::{transform::Quaternion, Color, TimeValue, Vector3};

/// Builder for constructing and configuring scene nodes
pub struct NodeBuilder<'a> {
    scene: &'a mut SceneGraph,
    node_id: NodeId,
}

impl<'a> NodeBuilder<'a> {
    pub(crate) fn new(scene: &'a mut SceneGraph, node_id: NodeId) -> Self {
        Self { scene, node_id }
    }

    /// Get the node ID
    pub fn id(&self) -> NodeId {
        self.node_id
    }

    /// Set position (x, y, z)
    pub fn at(self, x: f32, y: f32, z: f32) -> Self {
        if let Some(node) = self.scene.get_node_mut(self.node_id) {
            node._local_transform.position = Vector3::new(x, y, z);
        }
        self
    }

    /// Set position from Vector3
    pub fn at_vec(self, position: Vector3) -> Self {
        if let Some(node) = self.scene.get_node_mut(self.node_id) {
            node._local_transform.position = position;
        }
        self
    }

    /// Set uniform scale
    pub fn scale(self, scale: f32) -> Self {
        if let Some(node) = self.scene.get_node_mut(self.node_id) {
            node._local_transform.scale = Vector3::new(scale, scale, scale);
        }
        self
    }

    /// Set scale (x, y, z)
    pub fn scale_xyz(self, x: f32, y: f32, z: f32) -> Self {
        if let Some(node) = self.scene.get_node_mut(self.node_id) {
            node._local_transform.scale = Vector3::new(x, y, z);
        }
        self
    }

    /// Set Z-axis rotation (in radians)
    pub fn rotate_z(self, angle: f32) -> Self {
        if let Some(node) = self.scene.get_node_mut(self.node_id) {
            let z_axis = Vector3::new(0.0, 0.0, 1.0);
            node._local_transform.rotation = Quaternion::from_axis_angle(z_axis, angle);
        }
        self
    }

    /// Set Z-axis rotation (in degrees)
    pub fn rotate_z_degrees(self, degrees: f32) -> Self {
        self.rotate_z(degrees.to_radians())
    }

    /// Set rotation from Euler angles (roll, pitch, yaw) in radians
    pub fn rotate(self, roll: f32, pitch: f32, yaw: f32) -> Self {
        if let Some(node) = self.scene.get_node_mut(self.node_id) {
            node._local_transform.rotation = Quaternion::from_euler_angles(roll, pitch, yaw);
        }
        self
    }

    /// Set opacity (0.0 = transparent, 1.0 = opaque)
    pub fn opacity(self, opacity: f32) -> Self {
        if let Some(node) = self.scene.get_node_mut(self.node_id) {
            node.opacity = opacity.clamp(0.0, 1.0);
        }
        self
    }

    /// Set visibility
    pub fn visible(self, visible: bool) -> Self {
        if let Some(node) = self.scene.get_node_mut(self.node_id) {
            node.visible = visible;
        }
        self
    }

    /// Parent this node to another
    pub fn parent_to(self, parent_id: NodeId) -> Self {
        self.scene.parent(self.node_id, parent_id).ok();
        self
    }

    // ========== Animation Methods ==========

    /// Add fade in animation
    pub fn fade_in(self, start_time: f32, duration: f32) -> Self {
        let anim = effects::fade_in(duration);
        if let Some(node) = self.scene.get_node_mut(self.node_id) {
            node.add_animation(AnimationInstance::new(anim, TimeValue::new(start_time)));
        }
        self
    }

    /// Add fade out animation
    pub fn fade_out(self, start_time: f32, duration: f32) -> Self {
        let anim = effects::fade_out(duration);
        if let Some(node) = self.scene.get_node_mut(self.node_id) {
            node.add_animation(AnimationInstance::new(anim, TimeValue::new(start_time)));
        }
        self
    }

    /// Add create animation (fade in + scale from 0)
    pub fn create(self, start_time: f32, duration: f32) -> Self {
        let anim = effects::create(duration);
        if let Some(node) = self.scene.get_node_mut(self.node_id) {
            node.add_animation(AnimationInstance::new(anim, TimeValue::new(start_time)));
        }
        self
    }

    /// Add uncreate animation (reverse of create)
    pub fn uncreate(self, start_time: f32, duration: f32) -> Self {
        let anim = effects::uncreate(duration);
        if let Some(node) = self.scene.get_node_mut(self.node_id) {
            node.add_animation(AnimationInstance::new(anim, TimeValue::new(start_time)));
        }
        self
    }

    /// Add grow from center animation
    pub fn grow(self, start_time: f32, duration: f32) -> Self {
        let anim = effects::grow_from_center(duration);
        if let Some(node) = self.scene.get_node_mut(self.node_id) {
            node.add_animation(AnimationInstance::new(anim, TimeValue::new(start_time)));
        }
        self
    }

    /// Add shrink to center animation
    pub fn shrink(self, start_time: f32, duration: f32) -> Self {
        let anim = effects::shrink_to_center(duration);
        if let Some(node) = self.scene.get_node_mut(self.node_id) {
            node.add_animation(AnimationInstance::new(anim, TimeValue::new(start_time)));
        }
        self
    }

    /// Add move to position animation
    pub fn move_to(self, start_time: f32, target: Vector3, duration: f32) -> Self {
        if let Some(node) = self.scene.get_node_mut(self.node_id) {
            let from = node._local_transform.position;
            let anim = effects::move_to(from, target, duration);
            node.add_animation(AnimationInstance::new(anim, TimeValue::new(start_time)));
        }
        self
    }

    /// Add shift by offset animation
    pub fn shift(self, start_time: f32, offset: Vector3, duration: f32) -> Self {
        if let Some(node) = self.scene.get_node_mut(self.node_id) {
            let current = node._local_transform.position;
            let anim = effects::shift(current, offset, duration);
            node.add_animation(AnimationInstance::new(anim, TimeValue::new(start_time)));
        }
        self
    }

    /// Add rotation animation (Z-axis, in radians)
    pub fn rotate_anim(
        self,
        start_time: f32,
        from_angle: f32,
        to_angle: f32,
        duration: f32,
    ) -> Self {
        let anim = effects::rotate(from_angle, to_angle, duration);
        if let Some(node) = self.scene.get_node_mut(self.node_id) {
            node.add_animation(AnimationInstance::new(anim, TimeValue::new(start_time)));
        }
        self
    }

    /// Add spin animation (continuous rotation)
    pub fn spin(self, start_time: f32, rotations: f32, duration: f32) -> Self {
        let anim = effects::spin(rotations, duration);
        if let Some(node) = self.scene.get_node_mut(self.node_id) {
            node.add_animation(AnimationInstance::new(anim, TimeValue::new(start_time)));
        }
        self
    }

    /// Finish building and return the node ID
    pub fn build(self) -> NodeId {
        self.node_id
    }
}

/// Extension trait for SceneGraph to add fluent builder methods
impl SceneGraph {
    /// Create a circle with fluent API
    pub fn add_circle(
        &mut self,
        name: impl Into<String>,
        radius: f32,
        color: Color,
    ) -> NodeBuilder {
        let node_id = self.create_node(name.into());
        self.get_node_mut(node_id)
            .unwrap()
            .set_renderable(Renderable::Circle { radius, color });
        NodeBuilder::new(self, node_id)
    }

    /// Create a rectangle with fluent API
    pub fn add_rectangle(
        &mut self,
        name: impl Into<String>,
        width: f32,
        height: f32,
        color: Color,
    ) -> NodeBuilder {
        let node_id = self.create_node(name.into());
        self.get_node_mut(node_id)
            .unwrap()
            .set_renderable(Renderable::Rectangle {
                width,
                height,
                color,
            });
        NodeBuilder::new(self, node_id)
    }

    /// Create a square with fluent API
    pub fn add_square(&mut self, name: impl Into<String>, side: f32, color: Color) -> NodeBuilder {
        self.add_rectangle(name, side, side, color)
    }

    /// Create a line with fluent API
    pub fn add_line(
        &mut self,
        name: impl Into<String>,
        start: Vector3,
        end: Vector3,
        color: Color,
        thickness: f32,
    ) -> NodeBuilder {
        let node_id = self.create_node(name.into());
        self.get_node_mut(node_id)
            .unwrap()
            .set_renderable(Renderable::Line {
                start,
                end,
                color,
                thickness,
            });
        NodeBuilder::new(self, node_id)
    }

    /// Create an arrow with fluent API
    pub fn add_arrow(
        &mut self,
        name: impl Into<String>,
        start: Vector3,
        end: Vector3,
        color: Color,
        thickness: f32,
    ) -> NodeBuilder {
        let node_id = self.create_node(name.into());
        self.get_node_mut(node_id)
            .unwrap()
            .set_renderable(Renderable::Arrow {
                start,
                end,
                color,
                thickness,
            });
        NodeBuilder::new(self, node_id)
    }

    /// Create a polygon with fluent API
    pub fn add_polygon(
        &mut self,
        name: impl Into<String>,
        vertices: Vec<Vector3>,
        color: Color,
    ) -> NodeBuilder {
        let node_id = self.create_node(name.into());
        self.get_node_mut(node_id)
            .unwrap()
            .set_renderable(Renderable::Polygon { vertices, color });
        NodeBuilder::new(self, node_id)
    }

    /// Create a regular polygon (n-sided)
    pub fn add_regular_polygon(
        &mut self,
        name: impl Into<String>,
        sides: usize,
        radius: f32,
        color: Color,
    ) -> NodeBuilder {
        let angle_step = 2.0 * std::f32::consts::PI / sides as f32;
        let vertices: Vec<Vector3> = (0..sides)
            .map(|i| {
                let angle = i as f32 * angle_step - std::f32::consts::PI / 2.0;
                Vector3::new(radius * angle.cos(), radius * angle.sin(), 0.0)
            })
            .collect();
        self.add_polygon(name, vertices, color)
    }

    /// Create a triangle
    pub fn add_triangle(
        &mut self,
        name: impl Into<String>,
        size: f32,
        color: Color,
    ) -> NodeBuilder {
        self.add_regular_polygon(name, 3, size, color)
    }

    /// Create a pentagon
    pub fn add_pentagon(
        &mut self,
        name: impl Into<String>,
        size: f32,
        color: Color,
    ) -> NodeBuilder {
        self.add_regular_polygon(name, 5, size, color)
    }

    /// Create a hexagon
    pub fn add_hexagon(&mut self, name: impl Into<String>, size: f32, color: Color) -> NodeBuilder {
        self.add_regular_polygon(name, 6, size, color)
    }

    /// Create a star with specified points
    pub fn add_star(
        &mut self,
        name: impl Into<String>,
        points: usize,
        outer_radius: f32,
        inner_radius: f32,
        color: Color,
    ) -> NodeBuilder {
        let angle_step = std::f32::consts::PI / points as f32;
        let vertices: Vec<Vector3> = (0..(points * 2))
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
        self.add_polygon(name, vertices, color)
    }
}
