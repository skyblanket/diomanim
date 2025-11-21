//! Font Loading and Management
//!
//! Handles TrueType font loading and glyph metrics using ttf-parser.

use std::sync::Arc;

/// A loaded TrueType font
pub struct Font {
    /// Font data (owned for lifetime management)
    data: Arc<Vec<u8>>,
    /// Parsed font face
    face: ttf_parser::Face<'static>,
}

impl Font {
    /// Load a font from a TTF file
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let data = std::fs::read(path)?;
        let data = Arc::new(data);

        // Safety: We're using Arc to keep the data alive as long as the Font exists
        // The 'static lifetime is justified because the Arc keeps the data valid
        let face = unsafe {
            let data_ptr = Arc::as_ptr(&data) as *const u8;
            let data_slice = std::slice::from_raw_parts(data_ptr, data.len());
            ttf_parser::Face::parse(data_slice, 0)?
        };

        Ok(Self { data, face })
    }

    /// Load a font from bytes (for embedded fonts)
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        let data = Arc::new(bytes);

        let face = unsafe {
            let data_ptr = Arc::as_ptr(&data) as *const u8;
            let data_slice = std::slice::from_raw_parts(data_ptr, data.len());
            ttf_parser::Face::parse(data_slice, 0)?
        };

        Ok(Self { data, face })
    }

    /// Get the font's ascender (height above baseline)
    pub fn ascender(&self) -> i16 {
        self.face.ascender()
    }

    /// Get the font's descender (depth below baseline)
    pub fn descender(&self) -> i16 {
        self.face.descender()
    }

    /// Get the font's units per em
    pub fn units_per_em(&self) -> u16 {
        self.face.units_per_em()
    }

    /// Get the advance width for a character
    pub fn advance_width(&self, c: char) -> Option<u16> {
        let glyph_id = self.face.glyph_index(c)?;
        self.face.glyph_hor_advance(glyph_id)
    }

    /// Measure the width of a string
    pub fn measure_text(&self, text: &str, font_size: f32) -> f32 {
        let scale = font_size / self.units_per_em() as f32;
        let mut width = 0.0;

        for c in text.chars() {
            if let Some(advance) = self.advance_width(c) {
                width += advance as f32 * scale;
            }
        }

        width
    }

    /// Get glyph outline data for a character
    pub fn glyph_outline(&self, c: char) -> Option<ttf_parser::GlyphId> {
        self.face.glyph_index(c)
    }

    /// Get the bounding box for a glyph
    pub fn glyph_bounding_box(&self, glyph_id: ttf_parser::GlyphId) -> Option<ttf_parser::Rect> {
        self.face.glyph_bounding_box(glyph_id)
    }

    /// Access the underlying font face
    pub fn face(&self) -> &ttf_parser::Face {
        &self.face
    }
}

/// System font paths for different platforms
pub struct SystemFonts;

impl SystemFonts {
    /// Get path to a default sans-serif font
    pub fn sans_serif() -> &'static str {
        #[cfg(target_os = "macos")]
        {
            "/System/Library/Fonts/Helvetica.ttc"
        }
        #[cfg(target_os = "linux")]
        {
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"
        }
        #[cfg(target_os = "windows")]
        {
            "C:\\Windows\\Fonts\\arial.ttf"
        }
    }

    /// Get path to a monospace font
    pub fn monospace() -> &'static str {
        #[cfg(target_os = "macos")]
        {
            "/System/Library/Fonts/Monaco.ttf"
        }
        #[cfg(target_os = "linux")]
        {
            "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf"
        }
        #[cfg(target_os = "windows")]
        {
            "C:\\Windows\\Fonts\\consola.ttf"
        }
    }
}

impl Clone for Font {
    fn clone(&self) -> Self {
        let data = Arc::clone(&self.data);

        let face = unsafe {
            let data_ptr = Arc::as_ptr(&data) as *const u8;
            let data_slice = std::slice::from_raw_parts(data_ptr, data.len());
            ttf_parser::Face::parse(data_slice, 0).expect("Font was already validated")
        };

        Self { data, face }
    }
}
