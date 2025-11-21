//! Glyph Rasterization and Texture Atlas
//!
//! Handles converting TrueType glyphs to GPU textures for rendering.

use ab_glyph::{Font as AbFont, FontRef, PxScale, ScaleFont};
use std::collections::HashMap;

/// A rasterized glyph with texture coordinates
#[derive(Debug, Clone)]
pub struct RasterizedGlyph {
    /// Width of the glyph in pixels
    pub width: u32,
    /// Height of the glyph in pixels
    pub height: u32,
    /// Horizontal bearing (offset from origin)
    pub bearing_x: f32,
    /// Vertical bearing (offset from baseline)
    pub bearing_y: f32,
    /// Horizontal advance (how much to move cursor)
    pub advance: f32,
    /// UV coordinates in texture atlas (left, top, right, bottom)
    pub uv: (f32, f32, f32, f32),
    /// Bitmap data (grayscale alpha)
    pub bitmap: Vec<u8>,
}

/// Texture atlas for caching rasterized glyphs
pub struct GlyphAtlas {
    /// Font data
    font_data: Vec<u8>,
    /// Parsed font
    font: FontRef<'static>,
    /// Font size
    font_size: f32,
    /// Cache of rasterized glyphs (char -> glyph)
    glyphs: HashMap<char, RasterizedGlyph>,
    /// Atlas texture width
    atlas_width: u32,
    /// Atlas texture height
    atlas_height: u32,
    /// Current x position in atlas
    current_x: u32,
    /// Current y position in atlas
    current_y: u32,
    /// Current row height
    row_height: u32,
    /// Atlas texture data (RGBA8)
    atlas_data: Vec<u8>,
}

impl GlyphAtlas {
    /// Create a new glyph atlas
    pub fn new(font_data: Vec<u8>, font_size: f32) -> Result<Self, Box<dyn std::error::Error>> {
        // Parse font
        let font = unsafe {
            let data_ptr = font_data.as_ptr();
            let data_slice = std::slice::from_raw_parts(data_ptr, font_data.len());
            FontRef::try_from_slice(data_slice)?
        };

        // Create atlas (1024x1024 should be plenty for most use cases)
        let atlas_width = 1024;
        let atlas_height = 1024;
        let atlas_data = vec![0u8; (atlas_width * atlas_height * 4) as usize]; // RGBA8

        Ok(Self {
            font_data,
            font,
            font_size,
            glyphs: HashMap::new(),
            atlas_width,
            atlas_height,
            current_x: 0,
            current_y: 0,
            row_height: 0,
            atlas_data,
        })
    }

    /// Load from system font
    pub fn from_system_font(font_size: f32) -> Result<Self, Box<dyn std::error::Error>> {
        let font_path = crate::text::font::SystemFonts::sans_serif();
        let font_data = std::fs::read(font_path)?;
        Self::new(font_data, font_size)
    }

    /// Rasterize a character and add to atlas
    pub fn rasterize_char(
        &mut self,
        c: char,
    ) -> Result<&RasterizedGlyph, Box<dyn std::error::Error>> {
        // Check if already cached
        if self.glyphs.contains_key(&c) {
            return Ok(&self.glyphs[&c]);
        }

        // Get glyph
        let glyph_id = self.font.glyph_id(c);
        let scaled_font = self.font.as_scaled(PxScale::from(self.font_size));
        let glyph = glyph_id
            .with_scale_and_position(PxScale::from(self.font_size), ab_glyph::point(0.0, 0.0));

        // Get glyph metrics
        let h_metrics = scaled_font.h_advance(glyph_id);

        // Try to outline and rasterize
        if let Some(outlined) = scaled_font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            let width = bounds.width().ceil() as u32;
            let height = bounds.height().ceil() as u32;

            // Check if we need a new row
            if self.current_x + width > self.atlas_width {
                self.current_x = 0;
                self.current_y += self.row_height;
                self.row_height = 0;
            }

            // Check if atlas is full
            if self.current_y + height > self.atlas_height {
                return Err("Glyph atlas is full".into());
            }

            // Rasterize glyph
            let mut bitmap = vec![0u8; (width * height) as usize];
            outlined.draw(|x, y, v| {
                let idx = (y * width + x) as usize;
                if idx < bitmap.len() {
                    bitmap[idx] = (v * 255.0) as u8;
                }
            });

            // Copy to atlas
            for y in 0..height {
                for x in 0..width {
                    let atlas_x = self.current_x + x;
                    let atlas_y = self.current_y + y;
                    let atlas_idx = ((atlas_y * self.atlas_width + atlas_x) * 4) as usize;
                    let bitmap_idx = (y * width + x) as usize;

                    if atlas_idx + 3 < self.atlas_data.len() && bitmap_idx < bitmap.len() {
                        let alpha = bitmap[bitmap_idx];
                        // White color with alpha from glyph
                        self.atlas_data[atlas_idx] = 255; // R
                        self.atlas_data[atlas_idx + 1] = 255; // G
                        self.atlas_data[atlas_idx + 2] = 255; // B
                        self.atlas_data[atlas_idx + 3] = alpha; // A
                    }
                }
            }

            // Calculate UV coordinates
            let uv = (
                self.current_x as f32 / self.atlas_width as f32,
                self.current_y as f32 / self.atlas_height as f32,
                (self.current_x + width) as f32 / self.atlas_width as f32,
                (self.current_y + height) as f32 / self.atlas_height as f32,
            );

            // Create rasterized glyph
            let rasterized = RasterizedGlyph {
                width,
                height,
                bearing_x: bounds.min.x,
                bearing_y: -bounds.min.y,
                advance: h_metrics,
                uv,
                bitmap,
            };

            // Update atlas position
            self.current_x += width;
            self.row_height = self.row_height.max(height);

            // Cache and return
            self.glyphs.insert(c, rasterized);
            Ok(&self.glyphs[&c])
        } else {
            // Glyph has no outline (e.g., space), create empty glyph
            let rasterized = RasterizedGlyph {
                width: 0,
                height: 0,
                bearing_x: 0.0,
                bearing_y: 0.0,
                advance: h_metrics,
                uv: (0.0, 0.0, 0.0, 0.0),
                bitmap: Vec::new(),
            };

            self.glyphs.insert(c, rasterized);
            Ok(&self.glyphs[&c])
        }
    }

    /// Rasterize all characters in a string
    pub fn rasterize_string(&mut self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        for c in text.chars() {
            self.rasterize_char(c)?;
        }
        Ok(())
    }

    /// Get the atlas texture data
    pub fn atlas_data(&self) -> &[u8] {
        &self.atlas_data
    }

    /// Get atlas dimensions
    pub fn atlas_dimensions(&self) -> (u32, u32) {
        (self.atlas_width, self.atlas_height)
    }

    /// Get a cached glyph
    pub fn get_glyph(&self, c: char) -> Option<&RasterizedGlyph> {
        self.glyphs.get(&c)
    }

    /// Measure the width of a string
    pub fn measure_text(&mut self, text: &str) -> Result<f32, Box<dyn std::error::Error>> {
        let mut width = 0.0;
        for c in text.chars() {
            let glyph = self.rasterize_char(c)?;
            width += glyph.advance;
        }
        Ok(width)
    }
}
