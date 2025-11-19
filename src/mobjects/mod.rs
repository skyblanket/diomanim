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

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub position: Vector3,
}

impl Rectangle {
    pub fn new(width: f32, height: f32, color: Color) -> Self {
        Self {
            width,
            height,
            color,
            position: Vector3::zero(),
        }
    }
    
    pub fn from_square(side_length: f32, color: Color) -> Self {
        Self::new(side_length, side_length, color)
    }
    
    pub fn move_to(&mut self, position: Vector3) {
        self.position = position;
    }
}
#[derive(Debug, Clone)]
pub struct Line {
    pub start: Vector3,
    pub end: Vector3,
    pub color: Color,
    pub thickness: f32,
}

impl Line {
    pub fn new(start: Vector3, end: Vector3, color: Color, thickness: f32) -> Self {
        Self {
            start,
            end,
            color,
            thickness,
        }
    }
    
    pub fn from_points(start: Vector3, end: Vector3, color: Color) -> Self {
        Self::new(start, end, color, 2.0)
    }
    
    pub fn length(&self) -> f32 {
        ((self.end.x - self.start.x).powi(2) + 
         (self.end.y - self.start.y).powi(2) + 
         (self.end.z - self.start.z).powi(2)).sqrt()
    }
    
    pub fn direction(&self) -> Vector3 {
        let length = self.length();
        if length > 0.0 {
            Vector3::new(
                (self.end.x - self.start.x) / length,
                (self.end.y - self.start.y) / length,
                (self.end.z - self.start.z) / length,
            )
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        }
    }
    
    pub fn perpendicular(&self) -> Vector3 {
        let dir = self.direction();
        // In 2D, perpendicular is (y, -x)
        Vector3::new(-dir.y, dir.x, 0.0)
    }
}

#[derive(Debug, Clone)]
pub struct Arrow {
    pub start: Vector3,
    pub end: Vector3,
    pub color: Color,
    pub thickness: f32,
    pub tip_size: f32,
}

impl Arrow {
    pub fn new(start: Vector3, end: Vector3, color: Color, thickness: f32, tip_size: f32) -> Self {
        Self {
            start,
            end,
            color,
            thickness,
            tip_size,
        }
    }
    
    pub fn from_points(start: Vector3, end: Vector3, color: Color) -> Self {
        let thickness = 2.0;
        let tip_size = 8.0;
        Self::new(start, end, color, thickness, tip_size)
    }
    
    pub fn line(&self) -> Line {
        // Calculate line end (excluding tip)
        let dir = Vector3::new(
            self.end.x - self.start.x,
            self.end.y - self.start.y,
            self.end.z - self.start.z,
        );
        let length = (dir.x.powi(2) + dir.y.powi(2)).sqrt();
        
        if length > 0.0 && length > self.tip_size / 100.0 {
            let scale = (length - self.tip_size / 100.0) / length;
            let line_end = Vector3::new(
                self.start.x + dir.x * scale,
                self.start.y + dir.y * scale,
                self.start.z + dir.z * scale,
            );
            Line::new(self.start, line_end, self.color, self.thickness)
        } else {
            // If tip size is larger than line, return minimal line
            Line::new(self.start, self.start, self.color, self.thickness)
        }
    }
}
