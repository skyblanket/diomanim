use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r: r.clamp(0.0, 1.0), g: g.clamp(0.0, 1.0), b: b.clamp(0.0, 1.0), a: 1.0 }
    }

    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0),
            a: a.clamp(0.0, 1.0),
        }
    }

    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;
        Self::new(r, g, b)
    }

    pub fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        Self::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    }

    pub fn to_rgb8(&self) -> (u8, u8, u8) {
        (
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        )
    }

    pub fn lerp(&self, other: &Color, t: f32) -> Color {
        Color {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    pub fn with_opacity(&self, alpha: f32) -> Color {
        let mut c = *self;
        c.a = alpha.clamp(0.0, 1.0);
        c
    }

    pub fn invert(&self) -> Color {
        Color::new(1.0 - self.r, 1.0 - self.g, 1.0 - self.b)
    }

    pub fn mix(&self, other: &Color, ratio: f32) -> Color {
        let t = ratio.clamp(0.0, 1.0);
        self.lerp(other, t)
    }

    pub fn grayscale(&self) -> Color {
        let gray = self.r * 0.299 + self.g * 0.587 + self.b * 0.114;
        Color::new(gray, gray, gray)
    }

    pub fn brightness(&self, factor: f32) -> Color {
        Color::new(
            (self.r * factor).clamp(0.0, 1.0),
            (self.g * factor).clamp(0.0, 1.0),
            (self.b * factor).clamp(0.0, 1.0),
        )
    }

    pub fn saturate(&self, factor: f32) -> Color {
        let gray = self.grayscale().r;
        Color::new(
            (gray + (self.r - gray) * factor).clamp(0.0, 1.0),
            (gray + (self.g - gray) * factor).clamp(0.0, 1.0),
            (gray + (self.b - gray) * factor).clamp(0.0, 1.0),
        )
    }

    pub fn as_slice(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }

    pub fn as_slice_rgba(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// Convert color to f32 array for GPU rendering (already in 0.0-1.0 range)
    pub fn to_f32_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}

// Predefined colors
impl Color {
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const YELLOW: Color = Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const CYAN: Color = Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const MAGENTA: Color = Color { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const GRAY: Color = Color { r: 0.5, g: 0.5, b: 0.5, a: 1.0 };
    pub const LIGHT_GRAY: Color = Color { r: 0.75, g: 0.75, b: 0.75, a: 1.0 };
    pub const DARK_GRAY: Color = Color { r: 0.25, g: 0.25, b: 0.25, a: 1.0 };
    pub const ORANGE: Color = Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 };
    pub const PURPLE: Color = Color { r: 0.5, g: 0.0, b: 1.0, a: 1.0 };
    pub const BROWN: Color = Color { r: 0.5, g: 0.25, b: 0.0, a: 1.0 };
    pub const PINK: Color = Color { r: 1.0, g: 0.5, b: 1.0, a: 1.0 };
    
    pub const TEAL: Color = Color { r: 0.0, g: 0.5, b: 0.5, a: 1.0 };
    pub const MAROON: Color = Color { r: 0.5, g: 0.0, b: 0.0, a: 1.0 };
    pub const GOLD: Color = Color { r: 1.0, g: 0.843, b: 0.0, a: 1.0 };
    pub const LIME: Color = Color { r: 0.0, g: 1.0, b: 0.5, a: 1.0 };

    pub const TRANSPARENT: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorGradient {
    stops: Vec<(f32, Color)>,
}

impl ColorGradient {
    pub fn new(stops: Vec<(f32, Color)>) -> Self {
        let mut stops = stops;
        stops.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        Self { stops }
    }

    pub fn from_colors(colors: &[Color]) -> Self {
        let n = colors.len().max(2) as f32;
        let stops: Vec<(f32, Color)> = colors
            .iter()
            .enumerate()
            .map(|(i, color)| (i as f32 / (n - 1.0), *color))
            .collect();
        Self::new(stops)
    }

    pub fn evaluate(&self, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        
        if self.stops.is_empty() {
            return Color::BLACK;
        }

        if self.stops.len() == 1 || t <= self.stops[0].0 {
            return self.stops[0].1;
        }

        if t >= self.stops.last().unwrap().0 {
            return self.stops.last().unwrap().1;
        }

        for i in 0..self.stops.len() - 1 {
            let (t0, color0) = self.stops[i];
            let (t1, color1) = self.stops[i + 1];
            
            if t >= t0 && t <= t1 {
                let local_t = (t - t0) / (t1 - t0);
                return color0.lerp(&color1, local_t);
            }
        }

        self.stops.last().unwrap().1
    }

    pub fn add_stop(&mut self, position: f32, color: Color) {
        self.stops.push((position.clamp(0.0, 1.0), color));
        self.stops.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let c = Color::new(0.5, 0.6, 0.7);
        assert_eq!(c.r, 0.5);
        assert_eq!(c.g, 0.6);
        assert_eq!(c.b, 0.7);
        assert_eq!(c.a, 1.0);
    }

    #[test]
    fn test_color_clamping() {
        let c = Color::new(1.5, -0.5, 0.5);
        assert_eq!(c.r, 1.0); // Clamped to 1.0
        assert_eq!(c.g, 0.0); // Clamped to 0.0
        assert_eq!(c.b, 0.5);
    }

    #[test]
    fn test_color_rgba() {
        let c = Color::rgba(0.5, 0.6, 0.7, 0.8);
        assert_eq!(c.a, 0.8);
    }

    #[test]
    fn test_color_from_hex() {
        let c = Color::from_hex("#FF8000");
        assert_eq!(c.to_rgb8(), (255, 128, 0));
    }

    #[test]
    fn test_color_from_rgb8() {
        let c = Color::from_rgb8(255, 128, 0);
        assert!((c.r - 1.0).abs() < 0.01);
        assert!((c.g - 0.5).abs() < 0.01);
        assert!((c.b - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_color_to_rgb8() {
        let c = Color::new(1.0, 0.5, 0.0);
        assert_eq!(c.to_rgb8(), (255, 127, 0));
    }

    #[test]
    fn test_color_lerp() {
        let c1 = Color::new(0.0, 0.0, 0.0);
        let c2 = Color::new(1.0, 1.0, 1.0);
        let mid = c1.lerp(&c2, 0.5);
        assert!((mid.r - 0.5).abs() < 0.0001);
        assert!((mid.g - 0.5).abs() < 0.0001);
        assert!((mid.b - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_color_with_opacity() {
        let c = Color::RED.with_opacity(0.5);
        assert_eq!(c.a, 0.5);
    }

    #[test]
    fn test_color_invert() {
        let c = Color::new(0.2, 0.5, 0.8);
        let inv = c.invert();
        assert!((inv.r - 0.8).abs() < 0.0001);
        assert!((inv.g - 0.5).abs() < 0.0001);
        assert!((inv.b - 0.2).abs() < 0.0001);
    }

    #[test]
    fn test_color_grayscale() {
        let c = Color::RED;
        let gray = c.grayscale();
        // Grayscale of pure red should have equal RGB components
        assert_eq!(gray.r, gray.g);
        assert_eq!(gray.g, gray.b);
    }

    #[test]
    fn test_color_brightness() {
        let c = Color::new(0.5, 0.5, 0.5);
        let bright = c.brightness(2.0);
        assert_eq!(bright.r, 1.0); // Clamped to 1.0
    }

    #[test]
    fn test_color_constants() {
        assert_eq!(Color::RED.r, 1.0);
        assert_eq!(Color::RED.g, 0.0);
        assert_eq!(Color::RED.b, 0.0);

        assert_eq!(Color::GREEN.r, 0.0);
        assert_eq!(Color::GREEN.g, 1.0);
        assert_eq!(Color::GREEN.b, 0.0);

        assert_eq!(Color::BLUE.r, 0.0);
        assert_eq!(Color::BLUE.g, 0.0);
        assert_eq!(Color::BLUE.b, 1.0);
    }

    #[test]
    fn test_color_gradient_two_colors() {
        let gradient = ColorGradient::from_colors(&[Color::BLACK, Color::WHITE]);
        let mid = gradient.evaluate(0.5);
        assert!((mid.r - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_color_gradient_bounds() {
        let gradient = ColorGradient::from_colors(&[Color::BLACK, Color::WHITE]);
        let start = gradient.evaluate(0.0);
        let end = gradient.evaluate(1.0);
        assert_eq!(start, Color::BLACK);
        assert_eq!(end, Color::WHITE);
    }

    #[test]
    fn test_color_default() {
        let c = Color::default();
        assert_eq!(c, Color::BLACK);
    }

    #[test]
    fn test_color_to_f32_array() {
        let c = Color::from_rgb8(255, 128, 0);
        let arr = c.to_f32_array();
        assert!((arr[0] - 1.0).abs() < 0.01);
        assert!((arr[1] - 0.5).abs() < 0.01);
        assert!((arr[2] - 0.0).abs() < 0.01);
        assert!((arr[3] - 1.0).abs() < 0.01);
    }
}