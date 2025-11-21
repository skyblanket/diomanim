//! # Core Mathematical Foundations
//!
//! This module provides the fundamental mathematical types and operations for the animation engine.
//! It includes vector math, color handling, transformations, timing, and camera systems.
//!
//! ## Key Components
//!
//! - **Vectors**: 2D and 3D vector operations with SIMD optimization
//! - **Colors**: RGBA color representation with conversion utilities
//! - **Transforms**: Position, rotation, and scale transformations
//! - **Time**: High-precision timing with nanosecond accuracy
//! - **Camera**: View and projection matrix calculations
//!
//! ## Example
//!
//! ```rust
//! use diomanim::core::*;
//!
//! // Create a vector
//! let v = Vector3::new(1.0, 2.0, 3.0);
//!
//! // Create a color
//! let color = Color::RED;
//!
//! // Create a transform
//! let mut transform = Transform::new();
//! transform.position = Vector3::new(10.0, 5.0, 0.0);
//! transform.scale = Vector3::new(2.0, 2.0, 1.0);
//! ```

pub mod camera;
pub mod color;
pub mod time;
pub mod transform;
pub mod vector;

pub use camera::*;
pub use color::*;
pub use time::*;
pub use transform::*;
pub use vector::*;
