//! LaTeX expression parsing
//!
//! Converts LaTeX math expressions into a renderable tree structure.

use super::MathNode;

/// Parse LaTeX math notation into a MathNode tree
pub struct MathParser {
    input: Vec<char>,
    pos: usize,
}

impl MathParser {
    /// Create a new parser for the given LaTeX string
    pub fn new(latex: &str) -> Self {
        // Remove surrounding $ if present
        let latex = latex.trim_matches('$').trim();

        MathParser {
            input: latex.chars().collect(),
            pos: 0,
        }
    }

    /// Parse the full expression
    pub fn parse(&mut self) -> MathNode {
        if self.is_eof() {
            return MathNode::Text(String::new());
        }

        let mut children = Vec::new();

        while !self.is_eof() {
            if let Some(node) = self.parse_node() {
                children.push(node);
            }
        }

        if children.is_empty() {
            MathNode::Text(String::new())
        } else if children.len() == 1 {
            children.into_iter().next().unwrap()
        } else {
            MathNode::Group { children }
        }
    }

    /// Parse a single node
    fn parse_node(&mut self) -> Option<MathNode> {
        self.skip_whitespace();

        if self.is_eof() {
            return None;
        }

        let ch = self.current();

        match ch {
            '\\' => self.parse_command(),
            '^' => self.parse_superscript(),
            '_' => self.parse_subscript(),
            '{' => self.parse_group(),
            '+' | '-' | '=' | '<' | '>' | '*' | '/' => {
                let op = ch.to_string();
                self.advance();
                Some(MathNode::Operator(op))
            }
            _ => self.parse_text(),
        }
    }

    /// Parse a LaTeX command (starts with \)
    fn parse_command(&mut self) -> Option<MathNode> {
        self.advance(); // skip '\'

        let cmd = self.read_identifier();

        match cmd.as_str() {
            "frac" => self.parse_fraction(),
            "sqrt" => self.parse_sqrt(),
            // Greek letters
            "alpha" => Some(MathNode::Symbol("α".to_string())),
            "beta" => Some(MathNode::Symbol("β".to_string())),
            "gamma" => Some(MathNode::Symbol("γ".to_string())),
            "delta" => Some(MathNode::Symbol("δ".to_string())),
            "epsilon" => Some(MathNode::Symbol("ε".to_string())),
            "theta" => Some(MathNode::Symbol("θ".to_string())),
            "lambda" => Some(MathNode::Symbol("λ".to_string())),
            "mu" => Some(MathNode::Symbol("μ".to_string())),
            "pi" => Some(MathNode::Symbol("π".to_string())),
            "sigma" => Some(MathNode::Symbol("σ".to_string())),
            "phi" => Some(MathNode::Symbol("φ".to_string())),
            "omega" => Some(MathNode::Symbol("ω".to_string())),
            // Special symbols
            "infty" => Some(MathNode::Symbol("∞".to_string())),
            "sum" => Some(MathNode::Symbol("Σ".to_string())),
            "prod" => Some(MathNode::Symbol("Π".to_string())),
            "int" => Some(MathNode::Symbol("∫".to_string())),
            "partial" => Some(MathNode::Symbol("∂".to_string())),
            "nabla" => Some(MathNode::Symbol("∇".to_string())),
            "times" => Some(MathNode::Symbol("×".to_string())),
            "cdot" => Some(MathNode::Symbol("·".to_string())),
            "pm" => Some(MathNode::Symbol("±".to_string())),
            "leq" => Some(MathNode::Symbol("≤".to_string())),
            "geq" => Some(MathNode::Symbol("≥".to_string())),
            "neq" => Some(MathNode::Symbol("≠".to_string())),
            "approx" => Some(MathNode::Symbol("≈".to_string())),
            _ => Some(MathNode::Text(format!("\\{}", cmd))),
        }
    }

    /// Parse \frac{numerator}{denominator}
    fn parse_fraction(&mut self) -> Option<MathNode> {
        let numerator = self.parse_braced_group()?;
        let denominator = self.parse_braced_group()?;

        Some(MathNode::Fraction {
            numerator: Box::new(numerator),
            denominator: Box::new(denominator),
        })
    }

    /// Parse \sqrt{content}
    fn parse_sqrt(&mut self) -> Option<MathNode> {
        let content = self.parse_braced_group()?;

        Some(MathNode::SquareRoot {
            content: Box::new(content),
        })
    }

    /// Parse ^{exponent} or ^x
    fn parse_superscript(&mut self) -> Option<MathNode> {
        self.advance(); // skip '^'

        // Get the base from the previous node (not implemented yet)
        // For now, we'll skip this and handle it in a second pass
        None
    }

    /// Parse _{index} or _x
    fn parse_subscript(&mut self) -> Option<MathNode> {
        self.advance(); // skip '_'

        // Get the base from the previous node (not implemented yet)
        None
    }

    /// Parse a group enclosed in braces: {content}
    fn parse_group(&mut self) -> Option<MathNode> {
        self.parse_braced_group()
    }

    /// Parse content within braces: {content}
    fn parse_braced_group(&mut self) -> Option<MathNode> {
        if !self.match_char('{') {
            return None;
        }

        let mut children = Vec::new();
        let mut depth = 1;

        while !self.is_eof() && depth > 0 {
            let ch = self.current();

            if ch == '{' {
                depth += 1;
                self.advance();
            } else if ch == '}' {
                depth -= 1;
                if depth > 0 {
                    self.advance();
                } else {
                    break;
                }
            } else if let Some(node) = self.parse_node() {
                children.push(node);
            }
        }

        self.match_char('}'); // consume closing brace

        if children.is_empty() {
            Some(MathNode::Text(String::new()))
        } else if children.len() == 1 {
            Some(children.into_iter().next().unwrap())
        } else {
            Some(MathNode::Group { children })
        }
    }

    /// Parse plain text (alphanumeric)
    fn parse_text(&mut self) -> Option<MathNode> {
        let mut text = String::new();

        while !self.is_eof() {
            let ch = self.current();

            if ch.is_alphanumeric() || ch == '.' || ch == ',' {
                text.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        if text.is_empty() {
            None
        } else {
            Some(MathNode::Text(text))
        }
    }

    /// Read an identifier (letters only)
    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();

        while !self.is_eof() && self.current().is_alphabetic() {
            ident.push(self.current());
            self.advance();
        }

        ident
    }

    /// Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while !self.is_eof() && self.current().is_whitespace() {
            self.advance();
        }
    }

    /// Get the current character
    fn current(&self) -> char {
        self.input[self.pos]
    }

    /// Check if we're at the end of input
    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    /// Advance to the next character
    fn advance(&mut self) {
        if !self.is_eof() {
            self.pos += 1;
        }
    }

    /// Match and consume a specific character
    fn match_char(&mut self, expected: char) -> bool {
        if !self.is_eof() && self.current() == expected {
            self.advance();
            true
        } else {
            false
        }
    }
}

/// Parse a LaTeX math expression into a MathNode tree
pub fn parse_latex(latex: &str) -> MathNode {
    let mut parser = MathParser::new(latex);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_text() {
        let node = parse_latex("abc");
        assert_eq!(node.to_text(), "abc");
    }

    #[test]
    fn test_parse_greek_letters() {
        let node = parse_latex("\\alpha \\beta");
        match node {
            MathNode::Group { children } => {
                assert_eq!(children.len(), 2);
            }
            _ => panic!("Expected group"),
        }
    }

    #[test]
    fn test_parse_fraction() {
        let node = parse_latex("\\frac{a}{b}");
        match node {
            MathNode::Fraction { .. } => {}
            _ => panic!("Expected fraction"),
        }
    }

    #[test]
    fn test_parse_sqrt() {
        let node = parse_latex("\\sqrt{x}");
        match node {
            MathNode::SquareRoot { .. } => {}
            _ => panic!("Expected square root"),
        }
    }
}
