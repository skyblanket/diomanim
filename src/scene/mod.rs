//! # Scene Graph System
//!
//! This module provides a hierarchical scene graph for organizing and managing
//! animated objects. It follows an ECS-inspired design pattern for efficient
//! transform propagation and animation updates.
//!
//! ## Core Concepts
//!
//! - **SceneGraph**: The root container managing all scene nodes
//! - **SceneNode**: A node in the hierarchy with transform, children, and renderable
//! - **NodeId**: Unique identifier for scene nodes
//! - **Renderable**: Attachable visual representation (Circle, Square, etc.)
//!
//! ## Hierarchy
//!
//! Nodes can be parented to create hierarchies where transforms are inherited:
//! - Parent position/rotation/scale affects all children
//! - Animations can be applied to any node
//! - Visibility can be toggled per-node
//!
//! ## Example
//!
//! ```rust
//! use diomanim::scene::*;
//! use diomanim::core::*;
//!
//! let mut scene = SceneGraph::new();
//!
//! // Fluent API
//! scene.add_circle("my_circle", 1.0, Color::RED)
//!     .at(2.0, 1.0, 0.0)
//!     .fade_in(0.0, 1.0);
//!
//! // Traditional API
//! let parent_id = scene.create_node("Parent".to_string());
//! let child_id = scene.create_node("Child".to_string());
//! scene.parent(child_id, parent_id).unwrap();
//!
//! // Update transforms
//! scene.update_transforms();
//! ```

pub mod builder;

use crate::animation::property::AnimationInstance;
use crate::core::{TimeValue, Transform, Vector3};
use crate::render::TransformUniform;
use std::collections::HashMap;

pub use builder::NodeBuilder;

/// Unique identifier for scene nodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u32);

impl NodeId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}

/// A scene node represents an object in the scene hierarchy
pub struct SceneNode {
    pub id: NodeId,
    pub name: String,
    /// Local transform relative to parent
    pub _local_transform: Transform,
    /// Cached world transform (updated during graph traversal)
    pub world_transform: Transform,
    /// Parent node reference
    pub parent: Option<NodeId>,
    /// Child node references
    pub children: Vec<NodeId>,
    /// Whether this node is visible
    pub visible: bool,
    /// Opacity (0.0 = fully transparent, 1.0 = fully opaque)
    pub opacity: f32,
    /// Attached renderable object
    pub renderable: Option<Renderable>,
    /// Active animations on this node
    pub animations: Vec<AnimationInstance>,
}

impl SceneNode {
    pub fn new(id: NodeId, name: String) -> Self {
        Self {
            id,
            name,
            _local_transform: Transform::new(),
            world_transform: Transform::new(),
            parent: None,
            children: Vec::new(),
            visible: true,
            opacity: 1.0,
            renderable: None,
            animations: Vec::new(),
        }
    }

    pub fn with_transform(id: NodeId, name: String, transform: Transform) -> Self {
        Self {
            id,
            name,
            _local_transform: transform,
            world_transform: Transform::new(),
            parent: None,
            children: Vec::new(),
            visible: true,
            opacity: 1.0,
            renderable: None,
            animations: Vec::new(),
        }
    }

    /// Set the renderable object for this node
    pub fn set_renderable(&mut self, renderable: Renderable) {
        self.renderable = Some(renderable);
    }

    /// Add an animation to this node
    pub fn add_animation(&mut self, animation: AnimationInstance) {
        self.animations.push(animation);
    }

    /// Update animations and return true if the transform was modified
    pub fn update_animations(&mut self, delta_time: TimeValue) -> bool {
        let mut transform_changed = false;

        for anim in &mut self.animations {
            if anim.is_playing {
                // Update animation time
                let local_time = anim.current_time;
                let duration = anim.clip.duration();
                let new_time = local_time + delta_time;

                if duration > TimeValue::new(0.0) {
                    if anim.clip.loop_animation {
                        let loop_time = (new_time % duration).seconds();
                        anim.current_time = TimeValue::new(loop_time);
                    } else if new_time >= duration {
                        anim.is_playing = false;
                        anim.current_time = duration;
                    } else {
                        anim.current_time = new_time;
                    }
                } else {
                    anim.current_time = new_time;
                }

                // Sample each track at current time
                for track_box in &anim.clip.tracks {
                    // Downcast to concrete AnimationTrack<Vector3>
                    if let Some(track) = track_box
                        .as_any()
                        .downcast_ref::<crate::animation::property::AnimationTrack<Vector3>>(
                    ) {
                        let sample = track.sample(anim.current_time);

                        match track.name.as_str() {
                            "position" => {
                                self._local_transform.position = sample;
                                transform_changed = true;
                            }
                            "rotation" => {
                                // For now, we only use Z rotation (2D)
                                self._local_transform.rotation.z = sample.z;
                                transform_changed = true;
                            }
                            "scale" => {
                                self._local_transform.scale = sample;
                                transform_changed = true;
                            }
                            "opacity" => {
                                self.opacity = sample.x.clamp(0.0, 1.0);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // Remove finished non-looping animations
        self.animations
            .retain(|anim| anim.is_playing || anim.clip.loop_animation);

        transform_changed
    }

    /// Convert world transform to GPU-compatible matrix
    pub fn compute_model_matrix(&self) -> TransformUniform {
        // Create a column-major 4x4 transformation matrix for WebGPU
        // WebGPU/WGSL uses column-major matrices by default

        let pos = self.world_transform.position;
        let scale = self.world_transform.scale;

        // Simple transform without rotation (2D)
        // Column-major matrix: each array is a column
        // | sx  0   0   tx |
        // | 0   sy  0   ty |
        // | 0   0   sz  tz |
        // | 0   0   0   1  |
        TransformUniform {
            model_view_proj: [
                [scale.x, 0.0, 0.0, 0.0],   // Column 0: X axis
                [0.0, scale.y, 0.0, 0.0],   // Column 1: Y axis
                [0.0, 0.0, scale.z, 0.0],   // Column 2: Z axis
                [pos.x, pos.y, pos.z, 1.0], // Column 3: Translation
            ],
        }
    }
}

/// Renderable objects that can be attached to scene nodes
#[derive(Debug, Clone)]
pub enum Renderable {
    Circle {
        radius: f32,
        color: crate::core::Color,
    },
    Rectangle {
        width: f32,
        height: f32,
        color: crate::core::Color,
    },
    Line {
        start: Vector3,
        end: Vector3,
        color: crate::core::Color,
        thickness: f32,
    },
    Arrow {
        start: Vector3,
        end: Vector3,
        color: crate::core::Color,
        thickness: f32,
    },
    Polygon {
        vertices: Vec<Vector3>,
        color: crate::core::Color,
    },
    // Future: Mesh, Sprite, etc.
}

impl Renderable {
    pub fn as_circle(&self) -> Option<(&f32, &crate::core::Color)> {
        match self {
            Renderable::Circle { radius, color } => Some((radius, color)),
            _ => None,
        }
    }

    pub fn as_rectangle(&self) -> Option<(&f32, &f32, &crate::core::Color)> {
        match self {
            Renderable::Rectangle {
                width,
                height,
                color,
            } => Some((width, height, color)),
            _ => None,
        }
    }

    pub fn as_line(&self) -> Option<(&Vector3, &Vector3, &crate::core::Color, &f32)> {
        match self {
            Renderable::Line {
                start,
                end,
                color,
                thickness,
            } => Some((start, end, color, thickness)),
            _ => None,
        }
    }

    pub fn as_arrow(&self) -> Option<(&Vector3, &Vector3, &crate::core::Color, &f32)> {
        match self {
            Renderable::Arrow {
                start,
                end,
                color,
                thickness,
            } => Some((start, end, color, thickness)),
            _ => None,
        }
    }

    pub fn as_polygon(&self) -> Option<(&Vec<Vector3>, &crate::core::Color)> {
        match self {
            Renderable::Polygon { vertices, color } => Some((vertices, color)),
            _ => None,
        }
    }
}

/// Scene graph manages the hierarchy of scene nodes
pub struct SceneGraph {
    nodes: HashMap<NodeId, SceneNode>,
    root_nodes: Vec<NodeId>,
    next_id: u32,
}

impl SceneGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            root_nodes: Vec::new(),
            next_id: 1, // Start from 1, 0 is reserved
        }
    }

    /// Create a new node and return its ID
    pub fn create_node(&mut self, name: String) -> NodeId {
        let id = NodeId::new(self.next_id);
        self.next_id += 1;

        let node = SceneNode::new(id, name);
        self.nodes.insert(id, node);
        self.root_nodes.push(id);

        id
    }

    /// Create a node with a specific transform
    pub fn create_node_with_transform(&mut self, name: String, transform: Transform) -> NodeId {
        let id = NodeId::new(self.next_id);
        self.next_id += 1;

        let node = SceneNode::with_transform(id, name, transform);
        self.nodes.insert(id, node);
        self.root_nodes.push(id);

        id
    }

    /// Parent one node under another
    pub fn parent(&mut self, child_id: NodeId, parent_id: NodeId) -> Result<(), String> {
        // Check if both nodes exist
        if !self.nodes.contains_key(&child_id) {
            return Err(format!("Child node {:?} does not exist", child_id));
        }
        if !self.nodes.contains_key(&parent_id) {
            return Err(format!("Parent node {:?} does not exist", parent_id));
        }

        // Prevent cycles
        if self.would_create_cycle(child_id, parent_id) {
            return Err("Parenting would create a cycle".to_string());
        }

        // Remove child from root nodes if it's there
        self.root_nodes.retain(|&id| id != child_id);

        // Clone data first to avoid multiple mutable borrows
        let old_parent_id = self.nodes.get(&child_id).and_then(|c| c.parent);

        // Remove from old parent's children
        if let Some(old_id) = old_parent_id {
            if old_id != parent_id {
                let mut old_children = self.nodes.get(&old_id).unwrap().children.clone();
                old_children.retain(|&id| id != child_id);
                self.nodes.get_mut(&old_id).unwrap().children = old_children;
            }
        }

        // Update child's parent
        self.nodes.get_mut(&child_id).unwrap().parent = Some(parent_id);

        // Add to new parent's children
        let mut parent_children = self.nodes.get(&parent_id).unwrap().children.clone();
        if !parent_children.contains(&child_id) {
            parent_children.push(child_id);
        }
        self.nodes.get_mut(&parent_id).unwrap().children = parent_children;

        Ok(())
    }

    /// Check if parenting would create a cycle
    fn would_create_cycle(&self, child_id: NodeId, parent_id: NodeId) -> bool {
        let mut current = parent_id;
        while let Some(node) = self.nodes.get(&current) {
            if current == child_id {
                return true;
            }
            if let Some(parent) = node.parent {
                current = parent;
            } else {
                break;
            }
        }
        false
    }

    /// Get a reference to a node
    pub fn get_node(&self, id: NodeId) -> Option<&SceneNode> {
        self.nodes.get(&id)
    }

    /// Get a mutable reference to a node
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut SceneNode> {
        self.nodes.get_mut(&id)
    }

    /// Update the world transforms for all nodes
    pub fn update_transforms(&mut self) {
        // Reset all world transforms
        for node in self.nodes.values_mut() {
            node.world_transform = node._local_transform.clone();
        }

        // Collect root IDs first to avoid borrow conflicts
        let root_ids = self.root_nodes.clone();

        // Update each root tree
        for root_id in root_ids {
            self.update_node_transform_recursive(root_id, Transform::new());
        }
    }

    /// Recursively update node transforms - uses internal helper to avoid borrow conflicts
    fn update_node_transform_recursive(&mut self, node_id: NodeId, parent_world: Transform) {
        // First, collect all the data we need without holding borrows
        let (children, _local_transform) = {
            if let Some(node) = self.nodes.get_mut(&node_id) {
                // Update the node's world transform
                node.world_transform.position =
                    parent_world.position + node._local_transform.position;
                node.world_transform.rotation = node._local_transform.rotation; // Simplified
                node.world_transform.scale = Vector3::new(
                    parent_world.scale.x * node._local_transform.scale.x,
                    parent_world.scale.y * node._local_transform.scale.y,
                    parent_world.scale.z * node._local_transform.scale.z,
                );

                (node.children.clone(), node._local_transform.clone())
            } else {
                (Vec::new(), Transform::new())
            }
        };

        // Get the world transform for children
        let world_transform = if let Some(node) = self.nodes.get(&node_id) {
            node.world_transform.clone()
        } else {
            parent_world
        };

        // Recursively update children
        for child_id in children {
            self.update_node_transform_recursive(child_id, world_transform.clone());
        }
    }

    /// Update animations for all nodes
    pub fn update_animations(&mut self, delta_time: TimeValue) {
        let mut update_transforms = false;

        for node in self.nodes.values_mut() {
            if node.update_animations(delta_time) {
                update_transforms = true;
            }
        }

        if update_transforms {
            self.update_transforms();
        }
    }

    /// Get all visible renderable objects with their transforms and opacity
    pub fn get_visible_renderables(&self) -> Vec<(TransformUniform, Renderable, f32)> {
        let mut renderables = Vec::new();

        for &root_id in &self.root_nodes {
            self.gather_renderables_recursive(root_id, &mut renderables);
        }

        renderables
    }

    /// Recursively gather renderables with opacity
    fn gather_renderables_recursive(
        &self,
        node_id: NodeId,
        renderables: &mut Vec<(TransformUniform, Renderable, f32)>,
    ) {
        if let Some(node) = self.nodes.get(&node_id) {
            if node.visible && node.opacity > 0.0 {
                if let Some(renderable) = &node.renderable {
                    renderables.push((
                        node.compute_model_matrix(),
                        renderable.clone(),
                        node.opacity,
                    ));
                }

                for &child_id in &node.children {
                    self.gather_renderables_recursive(child_id, renderables);
                }
            }
        }
    }

    /// Remove a node and its children from the scene
    pub fn remove_node(&mut self, node_id: NodeId) -> Option<SceneNode> {
        if let Some(node) = self.nodes.remove(&node_id) {
            // Remove from root nodes if present
            self.root_nodes.retain(|&id| id != node_id);

            // Remove from parent's children
            if let Some(parent_id) = node.parent {
                if let Some(parent) = self.nodes.get_mut(&parent_id) {
                    parent.children.retain(|&id| id != node_id);
                }
            }

            // Remove children recursively
            for child_id in &node.children {
                self.remove_node(*child_id);
            }

            Some(node)
        } else {
            None
        }
    }
}

impl Default for SceneGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Color;

    #[test]
    fn test_scene_node_creation() {
        let node = SceneNode::new(NodeId::new(1), "TestNode".to_string());
        assert_eq!(node.name, "TestNode");
        assert!(node.children.is_empty());
        assert!(node.renderable.is_none());
    }

    #[test]
    fn test_scene_graph_hierarchy() {
        let mut graph = SceneGraph::new();

        // Create nodes
        let root = graph.create_node("Root".to_string());
        let child1 = graph.create_node("Child1".to_string());
        let child2 = graph.create_node("Child2".to_string());

        // Parent nodes
        graph.parent(child1, root).unwrap();
        graph.parent(child2, root).unwrap();

        // Verify hierarchy
        let root_node = graph.get_node(root).unwrap();
        assert_eq!(root_node.children.len(), 2);

        let child1_node = graph.get_node(child1).unwrap();
        assert_eq!(child1_node.parent, Some(root));
    }

    #[test]
    fn test_transform_inheritance() {
        let mut graph = SceneGraph::new();

        // Create parent with transform
        let mut parent_transform = Transform::new();
        parent_transform.position = Vector3::new(10.0, 0.0, 0.0);
        let parent = graph.create_node_with_transform("Parent".to_string(), parent_transform);

        // Create child with local transform
        let mut child_transform = Transform::new();
        child_transform.position = Vector3::new(5.0, 0.0, 0.0);
        let child = graph.create_node_with_transform("Child".to_string(), child_transform);

        // Parent them
        graph.parent(child, parent).unwrap();

        // Update transforms
        graph.update_transforms();

        // Verify world transform
        let child_node = graph.get_node(child).unwrap();
        assert_eq!(child_node.world_transform.position.x, 15.0);
    }

    #[test]
    fn test_renderable_gathering() {
        let mut graph = SceneGraph::new();

        // Create node with renderable
        let node_id = graph.create_node("CircleNode".to_string());
        let node = graph.get_node_mut(node_id).unwrap();
        node.set_renderable(Renderable::Circle {
            radius: 1.0,
            color: Color::RED,
        });

        // Get renderables
        let renderables = graph.get_visible_renderables();
        assert_eq!(renderables.len(), 1);

        if let Some((_, Renderable::Circle { radius, color }, opacity)) = renderables.first() {
            assert_eq!(*radius, 1.0);
            assert_eq!(*color, Color::RED);
            assert_eq!(*opacity, 1.0); // Default opacity
        } else {
            panic!("Expected Circle renderable");
        }
    }
}
