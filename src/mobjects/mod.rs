//! # Mathematical Objects (Mobjects)
//!
//! This module provides renderable geometric shapes and mathematical objects,
//! inspired by Manim's mobject system. Each mobject represents a visual element
//! that can be animated and rendered.
//!
//! ## Available Shapes
//!
//! - **Circle**: A circular shape with configurable radius and color
//! - **Square**: A square shape with configurable side length and color
//!
//! ## Example
//!
//! ```rust
//! use diomanim::mobjects::*;
//! use diomanim::core::*;
//!
//! // Create a circle
//! let mut circle = Circle::new(2.0, Color::RED);
//! circle.move_to(Vector3::new(5.0, 5.0, 0.0));
//!
//! // Create a square
//! let mut square = Square::new(3.0, Color::BLUE);
//! square.move_to(Vector3::new(-5.0, 0.0, 0.0));
//! ```

use crate::core::{Vector3, Color};

#[derive(Debug, Clone)]
pub struct Circle {
    pub radius: f32,
    pub color: Color,
    pub position: Vector3,
}

impl Circle {
    pub fn new(radius: f32, color: Color) -> Self {
        Self {
            radius,
            color,
            position: Vector3::zero(),
        }
    }
    
    pub fn move_to(&mut self, position: Vector3) {
        self.position = position;
    }
}

#[derive(Debug, Clone)]
pub struct Square {
    pub side_length: f32,
    pub color: Color,
    pub position: Vector3,
}

impl Square {
    pub fn new(side_length: f32, color: Color) -> Self {
        Self {
            side_length,
            color,
            position: Vector3::zero(),
        }
    }
    
    pub fn move_to(&mut self, position: Vector3) {
        self.position = position;
    }
}