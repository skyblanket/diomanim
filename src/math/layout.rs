//! Mathematical expression layout engine
//!
//! Handles positioning and sizing of mathematical components.

use super::MathNode;
use crate::core::Vector3;

/// Layout information for a rendered math component
#[derive(Debug, Clone)]
pub struct MathLayout {
    /// Position of this component (relative to parent)
    pub position: Vector3,
    /// Width of this component
    pub width: f32,
    /// Height of this component
    pub height: f32,
    /// Baseline offset (distance from top to baseline)
    pub baseline: f32,
    /// Text content (if leaf node)
    pub text: Option<String>,
    /// Child layouts (if group/fraction/etc.)
    pub children: Vec<MathLayout>,
}

impl MathLayout {
    /// Create a simple text layout
    pub fn text(content: String, font_size: f32) -> Self {
        let width = content.len() as f32 * font_size * 0.6;
        let height = font_size;
        let baseline = font_size * 0.8;

        MathLayout {
            position: Vector3::zero(),
            width,
            height,
            baseline,
            text: Some(content),
            children: Vec::new(),
        }
    }

    /// Layout a math node tree
    pub fn layout_node(node: &MathNode, font_size: f32) -> Self {
        match node {
            MathNode::Text(text) => MathLayout::text(text.clone(), font_size),

            MathNode::Symbol(symbol) => MathLayout::text(symbol.clone(), font_size),

            MathNode::Operator(op) => {
                // Operators get extra spacing
                let mut layout = MathLayout::text(op.clone(), font_size);
                layout.width += font_size * 0.4; // Add padding
                layout
            }

            MathNode::Fraction {
                numerator,
                denominator,
            } => Self::layout_fraction(numerator, denominator, font_size),

            MathNode::Superscript { base, exponent } => {
                Self::layout_superscript(base, exponent, font_size)
            }

            MathNode::Subscript { base, index } => Self::layout_subscript(base, index, font_size),

            MathNode::SquareRoot { content } => Self::layout_sqrt(content, font_size),

            MathNode::Group { children } => Self::layout_group(children, font_size),
        }
    }

    /// Layout a fraction: numerator over denominator
    fn layout_fraction(numerator: &MathNode, denominator: &MathNode, font_size: f32) -> MathLayout {
        let small_size = font_size * 0.7; // Smaller font for fractions

        let mut num_layout = Self::layout_node(numerator, small_size);
        let mut den_layout = Self::layout_node(denominator, small_size);

        // Position numerator above baseline
        num_layout.position = Vector3::new(0.0, -small_size * 0.5, 0.0);

        // Position denominator below baseline
        den_layout.position = Vector3::new(0.0, small_size * 0.5, 0.0);

        // Width is max of numerator and denominator plus padding
        let width = num_layout.width.max(den_layout.width) + font_size * 0.2;

        // Center components horizontally
        let num_offset = (width - num_layout.width) * 0.5;
        let den_offset = (width - den_layout.width) * 0.5;
        num_layout.position.x = num_offset;
        den_layout.position.x = den_offset;

        // Total height is sum of both parts
        let height = num_layout.height + den_layout.height + small_size * 0.2;
        let baseline = height * 0.5;

        MathLayout {
            position: Vector3::zero(),
            width,
            height,
            baseline,
            text: None,
            children: vec![num_layout, den_layout],
        }
    }

    /// Layout a superscript: base with exponent raised
    fn layout_superscript(base: &MathNode, exponent: &MathNode, font_size: f32) -> MathLayout {
        let base_layout = Self::layout_node(base, font_size);
        let mut exp_layout = Self::layout_node(exponent, font_size * 0.6);

        // Position exponent above and to the right of base
        exp_layout.position = Vector3::new(
            base_layout.width,
            -font_size * 0.4, // Raise up
            0.0,
        );

        let width = base_layout.width + exp_layout.width;
        let height = base_layout.height.max(font_size * 1.2);
        let baseline = base_layout.baseline;

        MathLayout {
            position: Vector3::zero(),
            width,
            height,
            baseline,
            text: None,
            children: vec![base_layout, exp_layout],
        }
    }

    /// Layout a subscript: base with index lowered
    fn layout_subscript(base: &MathNode, index: &MathNode, font_size: f32) -> MathLayout {
        let base_layout = Self::layout_node(base, font_size);
        let mut idx_layout = Self::layout_node(index, font_size * 0.6);

        // Position index below and to the right of base
        idx_layout.position = Vector3::new(
            base_layout.width,
            font_size * 0.3, // Lower down
            0.0,
        );

        let width = base_layout.width + idx_layout.width;
        let height = base_layout.height.max(font_size * 1.2);
        let baseline = base_layout.baseline;

        MathLayout {
            position: Vector3::zero(),
            width,
            height,
            baseline,
            text: None,
            children: vec![base_layout, idx_layout],
        }
    }

    /// Layout a square root
    fn layout_sqrt(content: &MathNode, font_size: f32) -> MathLayout {
        let mut content_layout = Self::layout_node(content, font_size);

        // Add padding for the radical symbol
        let symbol_width = font_size * 0.5;
        content_layout.position.x = symbol_width + font_size * 0.1;

        let width = symbol_width + content_layout.width + font_size * 0.1;
        let height = content_layout.height + font_size * 0.1;
        let baseline = content_layout.baseline;

        // Create a symbol layout for the √
        let mut symbol_layout = MathLayout::text("√".to_string(), font_size);
        symbol_layout.position = Vector3::zero();

        MathLayout {
            position: Vector3::zero(),
            width,
            height,
            baseline,
            text: None,
            children: vec![symbol_layout, content_layout],
        }
    }

    /// Layout a group of nodes horizontally
    fn layout_group(children: &[MathNode], font_size: f32) -> MathLayout {
        let mut layouts = Vec::new();
        let mut cursor_x = 0.0;
        let mut max_height = 0.0f32;
        let mut max_baseline = 0.0f32;

        for child in children {
            let mut layout = Self::layout_node(child, font_size);
            layout.position.x = cursor_x;
            cursor_x += layout.width;
            max_height = max_height.max(layout.height);
            max_baseline = max_baseline.max(layout.baseline);
            layouts.push(layout);
        }

        MathLayout {
            position: Vector3::zero(),
            width: cursor_x,
            height: max_height,
            baseline: max_baseline,
            text: None,
            children: layouts,
        }
    }

    /// Flatten the layout tree into a list of positioned text elements
    pub fn flatten(&self) -> Vec<(Vector3, String, f32)> {
        let mut result = Vec::new();
        self.flatten_recursive(Vector3::zero(), &mut result);
        result
    }

    fn flatten_recursive(&self, offset: Vector3, result: &mut Vec<(Vector3, String, f32)>) {
        if let Some(text) = &self.text {
            let pos = offset + self.position;
            let size = self.height; // Approximate font size from height
            result.push((pos, text.clone(), size));
        }

        for child in &self.children {
            child.flatten_recursive(offset + self.position, result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_layout() {
        let layout = MathLayout::text("hello".to_string(), 48.0);
        assert!(layout.width > 0.0);
        assert_eq!(layout.height, 48.0);
    }

    #[test]
    fn test_flatten() {
        let layout = MathLayout::text("test".to_string(), 48.0);
        let flattened = layout.flatten();
        assert_eq!(flattened.len(), 1);
        assert_eq!(flattened[0].1, "test");
    }
}
