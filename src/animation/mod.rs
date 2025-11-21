//! # Animation System
//!
//! This module provides a powerful keyframe-based animation system with support for
//! multiple interpolation methods and property animation.
//!
//! ## Core Concepts
//!
//! - **AnimationClip**: A collection of animation tracks defining object behavior over time
//! - **AnimationTrack**: A single animated property (e.g., position, rotation, scale)
//! - **Keyframe**: A specific value at a specific time point
//! - **InterpolationType**: How values are interpolated between keyframes (Linear, Ease, etc.)
//! - **AnimationController**: Manages multiple concurrent animations
//! - **Timer**: Utility for timing and progress tracking
//!
//! ## Example
//!
//! ```rust
//! use diomanim::animation::*;
//! use diomanim::core::*;
//!
//! // Create an animation clip
//! let mut clip = property::AnimationClip::new("my_animation".to_string());
//!
//! // Create a position animation track
//! let mut track = property::AnimationTrack::new("position".to_string());
//! track.add_keyframe(property::Keyframe::new(
//!     TimeValue::new(0.0),
//!     Vector3::new(0.0, 0.0, 0.0),
//! ));
//! track.add_keyframe(property::Keyframe::new(
//!     TimeValue::new(1.0),
//!     Vector3::new(10.0, 0.0, 0.0),
//! ));
//!
//! clip.add_track(track);
//! ```

pub mod property;
pub mod easing;
pub mod effects;

use crate::core::TimeValue;
use property::{AnimationClip, AnimationInstance};

// Re-export key types
pub use property::{Keyframe, AnimationTrack, InterpolationType, AnimationSample};
pub use effects::*;

// Timer for animation control
pub struct Timer {
    duration: TimeValue,
    elapsed: TimeValue,
    is_running: bool,
    start_time: Option<std::time::Instant>,
}

impl Timer {
    pub fn new(duration: TimeValue) -> Self {
        Self {
            duration,
            elapsed: TimeValue::new(0.0),
            is_running: false,
            start_time: None,
        }
    }

    pub fn start(&mut self) {
        self.is_running = true;
        self.start_time = Some(std::time::Instant::now());
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn reset(&mut self) {
        self.elapsed = TimeValue::new(0.0);
        self.is_running = false;
        self.start_time = None;
    }

    pub fn update(&mut self) -> f32 {
        if self.is_running {
            if let Some(start) = self.start_time {
                let delta = start.elapsed().as_secs_f32();
                self.elapsed = TimeValue::new(delta);
            }
        }
        self.progress()
    }

    pub fn progress(&self) -> f32 {
        if self.duration.seconds() > 0.0 {
            (self.elapsed.seconds() / self.duration.seconds()).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    pub fn is_finished(&self) -> bool {
        self.progress() >= 1.0
    }
}

/// Animation controller manages multiple running animations
pub struct AnimationController {
    animations: Vec<AnimationInstance>,
    global_time: TimeValue,
}

impl AnimationController {
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
            global_time: TimeValue::new(0.0),
        }
    }
    
    /// Add an animation to be managed
    pub fn add_animation(&mut self, animation: AnimationInstance) {
        self.animations.push(animation);
    }
    
    /// Update all animations to the current global time
    pub fn update(&mut self, delta_time: TimeValue) {
        self.global_time = self.global_time + delta_time;
        
        // Update all animations and remove finished ones
        self.animations.retain_mut(|anim| {
            anim.update(self.global_time);
            anim.is_playing
        });
    }
    
    /// Play a new animation
    pub fn play(&mut self, clip: AnimationClip) {
        let instance = AnimationInstance::new(clip, self.global_time);
        self.add_animation(instance);
    }
    
    /// Get the current global time
    pub fn global_time(&self) -> TimeValue {
        self.global_time
    }
}