//! Text Rendering Module
//!
//! Provides text rendering capabilities for mathematical annotations,
//! labels, and educational content.
//!
//! ## Features
//! - TrueType font loading
//! - Glyph rasterization with texture atlas
//! - Basic text rendering with color and size
//! - Text positioning and alignment
//! - Future: LaTeX support
//!
//! ## Example
//! ```rust,no_run
//! use diomanim::text::*;
//! use diomanim::core::*;
//!
//! let text = Text::new("Hello, Diomanim!", 48.0);
//! ```

pub mod font;
pub mod rasterizer;

use crate::core::{Color, Vector3};
pub use font::{Font, SystemFonts};
pub use rasterizer::{GlyphAtlas, RasterizedGlyph};

/// Text mobject for rendering text in animations
#[derive(Clone)]
pub struct Text {
    /// The text content
    pub content: String,
    /// Font size in pixels
    pub font_size: f32,
    /// Text color
    pub color: Color,
    /// Position in world space
    pub position: Vector3,
    /// Font (for now, we'll use a default font)
    /// In the future, this can reference a loaded font
    pub font_path: Option<String>,
}

impl Text {
    /// Create new text with default settings
    pub fn new(content: impl Into<String>, font_size: f32) -> Self {
        Self {
            content: content.into(),
            font_size,
            color: Color::WHITE,
            position: Vector3::zero(),
            font_path: None,
        }
    }

    /// Create text with specified color
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set text position
    pub fn at(mut self, position: Vector3) -> Self {
        self.position = position;
        self
    }

    /// Set font path
    pub fn with_font(mut self, font_path: String) -> Self {
        self.font_path = Some(font_path);
        self
    }

    /// Get the font to use (default or custom)
    pub fn get_font_path(&self) -> &str {
        self.font_path
            .as_deref()
            .unwrap_or(SystemFonts::sans_serif())
    }
}

impl std::fmt::Debug for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Text")
            .field("content", &self.content)
            .field("font_size", &self.font_size)
            .field("color", &self.color)
            .field("position", &self.position)
            .finish()
    }
}
