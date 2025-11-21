//! Mathematical notation rendering
//!
//! Provides support for rendering mathematical expressions with LaTeX syntax.
//! Uses our text rendering system with a custom layout engine for math-specific formatting.

pub mod expression;
pub mod layout;

pub use expression::*;
pub use layout::*;

use crate::core::Color;
use std::fmt;

/// A mathematical expression that can be rendered
#[derive(Debug, Clone)]
pub struct MathExpression {
    /// LaTeX source code
    pub latex: String,
    /// Parsed expression tree
    pub root: MathNode,
    /// Font size in points
    pub font_size: f32,
    /// Color
    pub color: Color,
}

/// A node in the mathematical expression tree
#[derive(Debug, Clone)]
pub enum MathNode {
    /// Plain text (numbers, variables)
    Text(String),
    /// Fraction: numerator / denominator
    Fraction {
        numerator: Box<MathNode>,
        denominator: Box<MathNode>,
    },
    /// Superscript: base ^ exponent
    Superscript {
        base: Box<MathNode>,
        exponent: Box<MathNode>,
    },
    /// Subscript: base _ index
    Subscript {
        base: Box<MathNode>,
        index: Box<MathNode>,
    },
    /// Square root: √(content)
    SquareRoot { content: Box<MathNode> },
    /// Group of nodes (for parentheses, etc.)
    Group { children: Vec<MathNode> },
    /// Operator (+, -, =, etc.)
    Operator(String),
    /// Greek letter or special symbol
    Symbol(String),
}

impl MathExpression {
    /// Create a new math expression from LaTeX
    pub fn new(latex: impl Into<String>, font_size: f32, color: Color) -> Self {
        let latex = latex.into();
        let root = expression::parse_latex(&latex);

        MathExpression {
            latex,
            root,
            font_size,
            color,
        }
    }

    /// Get the width of this expression in pixels
    pub fn width(&self) -> f32 {
        // Placeholder: estimate based on text length
        self.latex.len() as f32 * self.font_size * 0.6
    }

    /// Get the height of this expression in pixels
    pub fn height(&self) -> f32 {
        // Placeholder: use font size
        self.font_size * 1.2
    }
}

impl MathNode {
    /// Get a simple text representation (for debugging)
    pub fn to_text(&self) -> String {
        match self {
            MathNode::Text(s) => s.clone(),
            MathNode::Fraction {
                numerator,
                denominator,
            } => format!("({}) / ({})", numerator.to_text(), denominator.to_text()),
            MathNode::Superscript { base, exponent } => {
                format!("{}^{}", base.to_text(), exponent.to_text())
            }
            MathNode::Subscript { base, index } => {
                format!("{}_{}", base.to_text(), index.to_text())
            }
            MathNode::SquareRoot { content } => format!("√({})", content.to_text()),
            MathNode::Group { children } => {
                let parts: Vec<_> = children.iter().map(|c| c.to_text()).collect();
                format!("({})", parts.join(" "))
            }
            MathNode::Operator(op) => format!(" {} ", op),
            MathNode::Symbol(sym) => sym.clone(),
        }
    }
}

impl fmt::Display for MathExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MathExpression[{}]", self.latex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_expression_creation() {
        let expr = MathExpression::new("x^2 + y^2 = r^2", 48.0, Color::WHITE);
        assert_eq!(expr.latex, "x^2 + y^2 = r^2");
        assert_eq!(expr.font_size, 48.0);
    }

    #[test]
    fn test_node_to_text() {
        let node = MathNode::Text("hello".to_string());
        assert_eq!(node.to_text(), "hello");

        let frac = MathNode::Fraction {
            numerator: Box::new(MathNode::Text("a".to_string())),
            denominator: Box::new(MathNode::Text("b".to_string())),
        };
        assert_eq!(frac.to_text(), "(a) / (b)");
    }
}
