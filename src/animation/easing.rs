//! # Easing Functions
//!
//! This module provides a comprehensive collection of easing functions for smooth
//! animations. Easing functions control the rate of change of a parameter over time,
//! creating more natural and appealing motion.
//!
//! Based on Robert Penner's easing equations (public domain).
//!
//! ## Usage
//!
//! ```rust
//! use diomanim::animation::easing::*;
//!
//! let t = 0.5; // 50% through animation
//! let value = ease_in_out_cubic(t); // Smooth acceleration and deceleration
//! ```

use std::f32::consts::PI;

/// Easing function type
pub type EasingFn = fn(f32) -> f32;

/// Enum of all available easing functions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EasingType {
    Linear,

    // Quadratic
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,

    // Cubic
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,

    // Quartic
    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,

    // Quintic
    EaseInQuint,
    EaseOutQuint,
    EaseInOutQuint,

    // Sinusoidal
    EaseInSine,
    EaseOutSine,
    EaseInOutSine,

    // Exponential
    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,

    // Circular
    EaseInCirc,
    EaseOutCirc,
    EaseInOutCirc,

    // Back
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,

    // Elastic
    EaseInElastic,
    EaseOutElastic,
    EaseInOutElastic,

    // Bounce
    EaseInBounce,
    EaseOutBounce,
    EaseInOutBounce,

    // Custom
    Custom(EasingFn),
}

impl EasingType {
    /// Apply the easing function to a time value (0.0 to 1.0)
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);

        match self {
            EasingType::Linear => linear(t),

            EasingType::EaseInQuad => ease_in_quad(t),
            EasingType::EaseOutQuad => ease_out_quad(t),
            EasingType::EaseInOutQuad => ease_in_out_quad(t),

            EasingType::EaseInCubic => ease_in_cubic(t),
            EasingType::EaseOutCubic => ease_out_cubic(t),
            EasingType::EaseInOutCubic => ease_in_out_cubic(t),

            EasingType::EaseInQuart => ease_in_quart(t),
            EasingType::EaseOutQuart => ease_out_quart(t),
            EasingType::EaseInOutQuart => ease_in_out_quart(t),

            EasingType::EaseInQuint => ease_in_quint(t),
            EasingType::EaseOutQuint => ease_out_quint(t),
            EasingType::EaseInOutQuint => ease_in_out_quint(t),

            EasingType::EaseInSine => ease_in_sine(t),
            EasingType::EaseOutSine => ease_out_sine(t),
            EasingType::EaseInOutSine => ease_in_out_sine(t),

            EasingType::EaseInExpo => ease_in_expo(t),
            EasingType::EaseOutExpo => ease_out_expo(t),
            EasingType::EaseInOutExpo => ease_in_out_expo(t),

            EasingType::EaseInCirc => ease_in_circ(t),
            EasingType::EaseOutCirc => ease_out_circ(t),
            EasingType::EaseInOutCirc => ease_in_out_circ(t),

            EasingType::EaseInBack => ease_in_back(t),
            EasingType::EaseOutBack => ease_out_back(t),
            EasingType::EaseInOutBack => ease_in_out_back(t),

            EasingType::EaseInElastic => ease_in_elastic(t),
            EasingType::EaseOutElastic => ease_out_elastic(t),
            EasingType::EaseInOutElastic => ease_in_out_elastic(t),

            EasingType::EaseInBounce => ease_in_bounce(t),
            EasingType::EaseOutBounce => ease_out_bounce(t),
            EasingType::EaseInOutBounce => ease_in_out_bounce(t),

            EasingType::Custom(f) => f(t),
        }
    }
}

impl Default for EasingType {
    fn default() -> Self {
        EasingType::Linear
    }
}

// ============================================================================
// LINEAR
// ============================================================================

/// No easing, linear interpolation
pub fn linear(t: f32) -> f32 {
    t
}

// ============================================================================
// QUADRATIC
// ============================================================================

/// Quadratic ease-in: accelerating from zero velocity
pub fn ease_in_quad(t: f32) -> f32 {
    t * t
}

/// Quadratic ease-out: decelerating to zero velocity
pub fn ease_out_quad(t: f32) -> f32 {
    t * (2.0 - t)
}

/// Quadratic ease-in-out: acceleration until halfway, then deceleration
pub fn ease_in_out_quad(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}

// ============================================================================
// CUBIC
// ============================================================================

/// Cubic ease-in: accelerating from zero velocity
pub fn ease_in_cubic(t: f32) -> f32 {
    t * t * t
}

/// Cubic ease-out: decelerating to zero velocity
pub fn ease_out_cubic(t: f32) -> f32 {
    let t = t - 1.0;
    t * t * t + 1.0
}

/// Cubic ease-in-out: acceleration until halfway, then deceleration
pub fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        let t = 2.0 * t - 2.0;
        1.0 + t * t * t / 2.0
    }
}

// ============================================================================
// QUARTIC
// ============================================================================

/// Quartic ease-in: accelerating from zero velocity
pub fn ease_in_quart(t: f32) -> f32 {
    t * t * t * t
}

/// Quartic ease-out: decelerating to zero velocity
pub fn ease_out_quart(t: f32) -> f32 {
    let t = t - 1.0;
    1.0 - t * t * t * t
}

/// Quartic ease-in-out: acceleration until halfway, then deceleration
pub fn ease_in_out_quart(t: f32) -> f32 {
    if t < 0.5 {
        8.0 * t * t * t * t
    } else {
        let t = t - 1.0;
        1.0 - 8.0 * t * t * t * t
    }
}

// ============================================================================
// QUINTIC
// ============================================================================

/// Quintic ease-in: accelerating from zero velocity
pub fn ease_in_quint(t: f32) -> f32 {
    t * t * t * t * t
}

/// Quintic ease-out: decelerating to zero velocity
pub fn ease_out_quint(t: f32) -> f32 {
    let t = t - 1.0;
    t * t * t * t * t + 1.0
}

/// Quintic ease-in-out: acceleration until halfway, then deceleration
pub fn ease_in_out_quint(t: f32) -> f32 {
    if t < 0.5 {
        16.0 * t * t * t * t * t
    } else {
        let t = 2.0 * t - 2.0;
        1.0 + t * t * t * t * t / 2.0
    }
}

// ============================================================================
// SINUSOIDAL
// ============================================================================

/// Sinusoidal ease-in: accelerating from zero velocity
pub fn ease_in_sine(t: f32) -> f32 {
    1.0 - ((t * PI) / 2.0).cos()
}

/// Sinusoidal ease-out: decelerating to zero velocity
pub fn ease_out_sine(t: f32) -> f32 {
    ((t * PI) / 2.0).sin()
}

/// Sinusoidal ease-in-out: acceleration until halfway, then deceleration
pub fn ease_in_out_sine(t: f32) -> f32 {
    -(((t * PI).cos() - 1.0) / 2.0)
}

// ============================================================================
// EXPONENTIAL
// ============================================================================

/// Exponential ease-in: accelerating from zero velocity
pub fn ease_in_expo(t: f32) -> f32 {
    if t == 0.0 {
        0.0
    } else {
        2.0_f32.powf(10.0 * (t - 1.0))
    }
}

/// Exponential ease-out: decelerating to zero velocity
pub fn ease_out_expo(t: f32) -> f32 {
    if t == 1.0 {
        1.0
    } else {
        1.0 - 2.0_f32.powf(-10.0 * t)
    }
}

/// Exponential ease-in-out: acceleration until halfway, then deceleration
pub fn ease_in_out_expo(t: f32) -> f32 {
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
        return 1.0;
    }

    if t < 0.5 {
        2.0_f32.powf(20.0 * t - 10.0) / 2.0
    } else {
        (2.0 - 2.0_f32.powf(-20.0 * t + 10.0)) / 2.0
    }
}

// ============================================================================
// CIRCULAR
// ============================================================================

/// Circular ease-in: accelerating from zero velocity
pub fn ease_in_circ(t: f32) -> f32 {
    1.0 - (1.0 - t * t).sqrt()
}

/// Circular ease-out: decelerating to zero velocity
pub fn ease_out_circ(t: f32) -> f32 {
    (1.0 - (t - 1.0).powi(2)).sqrt()
}

/// Circular ease-in-out: acceleration until halfway, then deceleration
pub fn ease_in_out_circ(t: f32) -> f32 {
    if t < 0.5 {
        (1.0 - (1.0 - 4.0 * t * t).sqrt()) / 2.0
    } else {
        ((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) / 2.0
    }
}

// ============================================================================
// BACK
// ============================================================================

const C1: f32 = 1.70158;
const C2: f32 = C1 * 1.525;
const C3: f32 = C1 + 1.0;

/// Back ease-in: overshoot then snap back
pub fn ease_in_back(t: f32) -> f32 {
    C3 * t * t * t - C1 * t * t
}

/// Back ease-out: overshoot then snap back
pub fn ease_out_back(t: f32) -> f32 {
    1.0 + C3 * (t - 1.0).powi(3) + C1 * (t - 1.0).powi(2)
}

/// Back ease-in-out: overshoot both directions
pub fn ease_in_out_back(t: f32) -> f32 {
    if t < 0.5 {
        ((2.0 * t).powi(2) * ((C2 + 1.0) * 2.0 * t - C2)) / 2.0
    } else {
        ((2.0 * t - 2.0).powi(2) * ((C2 + 1.0) * (t * 2.0 - 2.0) + C2) + 2.0) / 2.0
    }
}

// ============================================================================
// ELASTIC
// ============================================================================

const C4: f32 = (2.0 * PI) / 3.0;
const C5: f32 = (2.0 * PI) / 4.5;

/// Elastic ease-in: elastic snap with oscillation
pub fn ease_in_elastic(t: f32) -> f32 {
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
        return 1.0;
    }

    -2.0_f32.powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * C4).sin()
}

/// Elastic ease-out: elastic snap with oscillation
pub fn ease_out_elastic(t: f32) -> f32 {
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
        return 1.0;
    }

    2.0_f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * C4).sin() + 1.0
}

/// Elastic ease-in-out: elastic snap in both directions
pub fn ease_in_out_elastic(t: f32) -> f32 {
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
        return 1.0;
    }

    if t < 0.5 {
        -(2.0_f32.powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * C5).sin()) / 2.0
    } else {
        (2.0_f32.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * C5).sin()) / 2.0 + 1.0
    }
}

// ============================================================================
// BOUNCE
// ============================================================================

/// Bounce ease-out: bouncing effect (helper for bounce functions)
fn bounce_out(t: f32) -> f32 {
    const N1: f32 = 7.5625;
    const D1: f32 = 2.75;

    if t < 1.0 / D1 {
        N1 * t * t
    } else if t < 2.0 / D1 {
        let t = t - 1.5 / D1;
        N1 * t * t + 0.75
    } else if t < 2.5 / D1 {
        let t = t - 2.25 / D1;
        N1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / D1;
        N1 * t * t + 0.984375
    }
}

/// Bounce ease-in: bouncing effect at start
pub fn ease_in_bounce(t: f32) -> f32 {
    1.0 - bounce_out(1.0 - t)
}

/// Bounce ease-out: bouncing effect at end
pub fn ease_out_bounce(t: f32) -> f32 {
    bounce_out(t)
}

/// Bounce ease-in-out: bouncing effect at both ends
pub fn ease_in_out_bounce(t: f32) -> f32 {
    if t < 0.5 {
        (1.0 - bounce_out(1.0 - 2.0 * t)) / 2.0
    } else {
        (1.0 + bounce_out(2.0 * t - 1.0)) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear() {
        assert_eq!(linear(0.0), 0.0);
        assert_eq!(linear(0.5), 0.5);
        assert_eq!(linear(1.0), 1.0);
    }

    #[test]
    fn test_ease_in_out_cubic() {
        let start = ease_in_out_cubic(0.0);
        let mid = ease_in_out_cubic(0.5);
        let end = ease_in_out_cubic(1.0);

        assert!(start < 0.1);
        assert!((mid - 0.5).abs() < 0.1);
        assert!((end - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_bounce() {
        let value = ease_out_bounce(0.8);
        assert!(value >= 0.0 && value <= 1.0);
    }

    #[test]
    fn test_easing_type_apply() {
        let easing = EasingType::EaseInOutCubic;
        assert_eq!(easing.apply(0.5), ease_in_out_cubic(0.5));
    }
}
