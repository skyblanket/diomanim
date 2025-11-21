// Property animation system for animating object properties over time
use crate::core::TimeValue;
use std::any::Any;

/// Trait for types that can be animated/interpolated
pub trait Animatable: Clone + Send + Sync + 'static {
    /// Linear interpolation between self and other at time t (0.0 to 1.0)
    fn lerp(&self, other: &Self, t: f32) -> Self;

    /// Return a default/zero value for this type
    fn default_value() -> Self;
}

// Implement Animatable for Vector3
impl Animatable for crate::core::Vector3 {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            z: self.z + (other.z - self.z) * t,
        }
    }

    fn default_value() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

// Implement Animatable for Color
impl Animatable for crate::core::Color {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    fn default_value() -> Self {
        Self::BLACK
    }
}

/// A keyframe stores a value at a specific time point
#[derive(Debug, Clone)]
pub struct Keyframe<T: Animatable> {
    pub time: TimeValue,
    pub value: T,
    /// Interpolation type for this keyframe segment
    pub interpolation: InterpolationType,
}

impl<T: Animatable + std::fmt::Debug> Keyframe<T> {
    pub fn new(time: TimeValue, value: T) -> Self {
        Self {
            time,
            value,
            interpolation: InterpolationType::Linear,
        }
    }

    pub fn with_interpolation(mut self, interpolation: InterpolationType) -> Self {
        self.interpolation = interpolation;
        self
    }
}

/// Types of interpolation between keyframes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpolationType {
    Linear,
    Step,
    EaseIn,
    EaseOut,
    EaseInOut,
}

impl InterpolationType {
    /// Apply easing function to t
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            InterpolationType::Linear => t,
            InterpolationType::Step => {
                if t >= 1.0 {
                    1.0
                } else {
                    0.0
                }
            }
            InterpolationType::EaseIn => t * t,
            InterpolationType::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            InterpolationType::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
        }
    }
}

/// A track animates a single property over time using keyframes
#[derive(Debug, Clone)]
pub struct AnimationTrack<T: Animatable + std::fmt::Debug> {
    pub name: String,
    pub keyframes: Vec<Keyframe<T>>,
    /// Default value when no keyframes exist
    pub default_value: T,
}

impl<T: Animatable + std::fmt::Debug> AnimationTrack<T> {
    pub fn new(name: String) -> Self {
        let default_value = T::default_value();
        Self {
            name,
            keyframes: Vec::new(),
            default_value: default_value.clone(),
        }
    }

    pub fn with_default_value(name: String, default_value: T) -> Self {
        Self {
            name,
            keyframes: Vec::new(),
            default_value,
        }
    }

    /// Add a keyframe to this track
    pub fn add_keyframe(&mut self, keyframe: Keyframe<T>) {
        self.keyframes.push(keyframe);
        // Keep keyframes sorted by time
        self.keyframes
            .sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    }

    /// Sample the value at a given time
    pub fn sample(&self, time: TimeValue) -> T {
        if self.keyframes.is_empty() {
            return self.default_value.clone();
        }

        // Handle edge cases
        if time <= self.keyframes[0].time {
            return self.keyframes[0].value.clone();
        }
        if time >= self.keyframes.last().unwrap().time {
            return self.keyframes.last().unwrap().value.clone();
        }

        // Find surrounding keyframes
        let mut i = 0;
        while i < self.keyframes.len() - 1 && self.keyframes[i + 1].time <= time {
            i += 1;
        }

        let kf0 = &self.keyframes[i];
        let kf1 = &self.keyframes[i + 1];

        // Calculate interpolation factor
        let duration = (kf1.time - kf0.time).seconds();
        if duration <= 0.0 {
            return kf0.value.clone();
        }

        let t_raw = (time - kf0.time).seconds() / duration;
        let t = kf0.interpolation.apply(t_raw);

        // Interpolate
        kf0.value.lerp(&kf1.value, t)
    }

    /// Get the duration of this track
    pub fn duration(&self) -> TimeValue {
        if self.keyframes.is_empty() {
            return TimeValue::new(0.0);
        }
        self.keyframes.last().unwrap().time - self.keyframes.first().unwrap().time
    }

    /// Check if this track has any keyframes
    pub fn is_empty(&self) -> bool {
        self.keyframes.is_empty()
    }
}

/// An animation clip contains multiple tracks for animating different properties
#[derive(Debug)]
pub struct AnimationClip {
    pub name: String,
    /// Track storage using type erasure to support different property types
    pub tracks: Vec<Box<dyn AnyTrack>>,
    /// Playback configuration
    pub loop_animation: bool,
    pub speed: f32,
}

impl AnimationClip {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tracks: Vec::new(),
            loop_animation: false,
            speed: 1.0,
        }
    }

    /// Add a track to this animation
    pub fn add_track<T: Animatable + std::fmt::Debug + 'static>(
        &mut self,
        track: AnimationTrack<T>,
    ) {
        self.tracks.push(Box::new(track));
    }

    /// Sample the animation at a given time
    pub fn sample(&self, time: TimeValue) -> AnimationSample {
        let mut sample = AnimationSample::new();

        for track in &self.tracks {
            track.sample_to_sample(time, &mut sample);
        }

        sample
    }

    /// Get the duration of this animation (longest track)
    pub fn duration(&self) -> TimeValue {
        if self.tracks.is_empty() {
            return TimeValue::new(0.0);
        }

        self.tracks
            .iter()
            .map(|track| track.duration())
            .max()
            .unwrap_or(TimeValue::new(0.0))
    }
}

/// Trait for type-erased tracks
pub trait AnyTrack: Send + Sync + std::fmt::Debug {
    fn duration(&self) -> TimeValue;
    fn sample_to_sample(&self, time: TimeValue, sample: &mut AnimationSample);
    /// Get a reference to self as Any for downcasting
    fn as_any(&self) -> &dyn Any;
}

impl<T: Animatable + std::fmt::Debug + 'static> AnyTrack for AnimationTrack<T> {
    fn duration(&self) -> TimeValue {
        self.duration()
    }

    fn sample_to_sample(&self, _time: TimeValue, _sample: &mut AnimationSample) {
        // This would need a more sophisticated system for storing different types
        // For now, we'll skip type-erased sampling
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Result of sampling an animation at a time point
#[derive(Debug, Default)]
pub struct AnimationSample {
    /// Store sampled values for different properties
    /// In a real implementation, this would be more sophisticated
    pub values: std::collections::HashMap<String, Box<dyn Any + Send + Sync>>,
}

impl AnimationSample {
    pub fn new() -> Self {
        Self {
            values: std::collections::HashMap::new(),
        }
    }

    pub fn get<T: 'static>(&self, name: &str) -> Option<&T> {
        self.values.get(name).and_then(|v| v.downcast_ref::<T>())
    }
}

/// An animation instance is a running animation with state
pub struct AnimationInstance {
    pub clip: AnimationClip,
    pub start_time: TimeValue,
    pub is_playing: bool,
    pub current_time: TimeValue,
}

impl AnimationInstance {
    pub fn new(clip: AnimationClip, start_time: TimeValue) -> Self {
        Self {
            clip,
            start_time,
            is_playing: true,
            current_time: TimeValue::new(0.0),
        }
    }

    /// Update the animation to the current time
    pub fn update(&mut self, current_time: TimeValue) -> Option<AnimationSample> {
        if !self.is_playing {
            return None;
        }

        // Calculate local time in the animation
        let mut local_time = current_time - self.start_time;
        let duration = self.clip.duration();

        if duration > TimeValue::new(0.0) {
            if self.clip.loop_animation {
                // Loop the animation
                let loop_time = (local_time % duration).seconds();
                local_time = TimeValue::new(loop_time);
            } else if local_time >= duration {
                // Animation finished
                self.is_playing = false;
                local_time = duration;
            }
        }

        self.current_time = local_time;
        Some(self.clip.sample(local_time))
    }

    pub fn play(&mut self) {
        self.is_playing = true;
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.current_time = TimeValue::new(0.0);
    }
}
