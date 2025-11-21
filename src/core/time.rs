use instant::Instant;
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeValue {
    pub value: f32,
}

impl TimeValue {
    pub fn new(value: f32) -> Self {
        Self {
            value: value.max(0.0),
        }
    }

    pub fn seconds(&self) -> f32 {
        self.value
    }

    pub fn milliseconds(&self) -> f32 {
        self.value * 1000.0
    }

    pub fn as_ratio(&self, total: TimeValue) -> f32 {
        if total.value > 0.0 {
            (self.value / total.value).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone)]
pub struct Timer {
    duration: TimeValue,
    elapsed: TimeValue,
    is_running: bool,
    start_time: Option<Instant>,
    speed: f32,
}

impl Timer {
    pub fn new(duration: TimeValue) -> Self {
        Self {
            duration,
            elapsed: TimeValue::new(0.0),
            is_running: false,
            start_time: None,
            speed: 1.0,
        }
    }

    pub fn start(&mut self) {
        self.is_running = true;
        self.start_time = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        if self.is_running {
            if let Some(start) = self.start_time {
                let delta = Instant::now() - start;
                self.elapsed =
                    TimeValue::new(self.elapsed.value + delta.as_secs_f32() * self.speed);
            }
            self.is_running = false;
        }
    }

    pub fn reset(&mut self) {
        self.elapsed = TimeValue::new(0.0);
        self.is_running = false;
        self.start_time = None;
    }

    pub fn restart(&mut self) {
        self.reset();
        self.start();
    }

    pub fn update(&mut self) -> f32 {
        let old_elapsed = self.elapsed.value;

        if self.is_running {
            if let Some(start) = self.start_time {
                let delta = Instant::now() - start;
                self.elapsed = TimeValue::new(delta.as_secs_f32() * self.speed);
            }
        }

        self.elapsed.value - old_elapsed
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed.max(0.0);
    }

    pub fn is_finished(&self) -> bool {
        self.elapsed.value >= self.duration.value
    }

    pub fn progress(&self) -> f32 {
        self.elapsed.as_ratio(self.duration)
    }

    pub fn elapsed(&self) -> TimeValue {
        self.elapsed
    }

    pub fn remaining(&self) -> TimeValue {
        TimeValue::new((self.duration.value - self.elapsed.value).max(0.0))
    }
}

pub trait RateFunction: Send + Sync {
    fn evaluate(&self, t: f32) -> f32;
}

#[derive(Debug, Clone)]
pub struct LinearRate;

impl RateFunction for LinearRate {
    fn evaluate(&self, t: f32) -> f32 {
        t.clamp(0.0, 1.0)
    }
}

#[derive(Debug, Clone)]
pub struct SmoothRate;

impl RateFunction for SmoothRate {
    fn evaluate(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }
}

#[derive(Debug, Clone)]
pub struct EaseInOutCubicRate;

impl RateFunction for EaseInOutCubicRate {
    fn evaluate(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
        }
    }
}

#[derive(Debug, Clone)]
pub struct EaseInQuadraticRate;

impl RateFunction for EaseInQuadraticRate {
    fn evaluate(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        t * t
    }
}

#[derive(Debug, Clone)]
pub struct EaseOutQuadraticRate;

impl RateFunction for EaseOutQuadraticRate {
    fn evaluate(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        1.0 - (1.0 - t) * (1.0 - t)
    }
}

#[derive(Debug, Clone)]
pub struct EaseInExponentialRate {
    pub exponent: f32,
}

impl EaseInExponentialRate {
    pub fn new(exponent: f32) -> Self {
        Self { exponent }
    }
}

impl RateFunction for EaseInExponentialRate {
    fn evaluate(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        t.powf(self.exponent)
    }
}

#[derive(Debug, Clone)]
pub struct EaseOutExponentialRate {
    pub exponent: f32,
}

impl EaseOutExponentialRate {
    pub fn new(exponent: f32) -> Self {
        Self { exponent }
    }
}

impl RateFunction for EaseOutExponentialRate {
    fn evaluate(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        1.0 - (1.0 - t).powf(self.exponent)
    }
}

#[derive(Debug, Clone)]
pub struct WaveRate {
    pub amplitude: f32,
    pub frequency: f32,
}

impl WaveRate {
    pub fn new(amplitude: f32, frequency: f32) -> Self {
        Self {
            amplitude,
            frequency,
        }
    }
}

impl RateFunction for WaveRate {
    fn evaluate(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        t + self.amplitude * (t * self.frequency * 2.0 * PI).sin()
    }
}

#[derive(Debug, Clone)]
pub struct ThereAndBackRate;

impl RateFunction for ThereAndBackRate {
    fn evaluate(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        if t < 0.5 {
            2.0 * t
        } else {
            2.0 * (1.0 - t)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AnimationRateType;

impl AnimationRateType {
    pub fn linear(t: f32) -> f32 {
        LinearRate.evaluate(t)
    }

    pub fn smooth(t: f32) -> f32 {
        SmoothRate.evaluate(t)
    }

    pub fn ease_in_out_cubic(t: f32) -> f32 {
        EaseInOutCubicRate.evaluate(t)
    }

    pub fn ease_in_quad(t: f32) -> f32 {
        EaseInQuadraticRate.evaluate(t)
    }

    pub fn ease_out_quad(t: f32) -> f32 {
        EaseOutQuadraticRate.evaluate(t)
    }

    pub fn ease_in_exp(t: f32, exponent: f32) -> f32 {
        EaseInExponentialRate::new(exponent).evaluate(t)
    }

    pub fn ease_out_exp(t: f32, exponent: f32) -> f32 {
        EaseOutExponentialRate::new(exponent).evaluate(t)
    }

    pub fn wave(t: f32, amplitude: f32, frequency: f32) -> f32 {
        WaveRate::new(amplitude, frequency).evaluate(t)
    }

    pub fn there_and_back(t: f32) -> f32 {
        ThereAndBackRate.evaluate(t)
    }

    pub fn custom<T: RateFunction>(rate_function: &T, t: f32) -> f32 {
        rate_function.evaluate(t)
    }
}

pub struct Timeline {
    markers: Vec<(String, TimeValue)>,
    current_time: TimeValue,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            markers: Vec::new(),
            current_time: TimeValue::new(0.0),
        }
    }

    pub fn add_marker(&mut self, name: String, time: TimeValue) {
        self.markers.push((name, time));
        self.markers
            .sort_by(|a, b| a.1.value.partial_cmp(&b.1.value).unwrap());
    }

    pub fn add_marker_at_seconds(&mut self, name: String, seconds: f32) {
        self.add_marker(name, TimeValue::new(seconds));
    }

    pub fn get_markers_before(&self, time: TimeValue) -> Vec<&String> {
        self.markers
            .iter()
            .filter(|(_, marker_time)| marker_time.value <= time.value)
            .map(|(name, _)| name)
            .collect()
    }

    pub fn get_markers_after(&self, time: TimeValue) -> Vec<&String> {
        self.markers
            .iter()
            .filter(|(_, marker_time)| marker_time.value > time.value)
            .map(|(name, _)| name)
            .collect()
    }

    pub fn seek_to_marker(&mut self, name: &str) -> Option<TimeValue> {
        for (marker_name, time) in &self.markers {
            if marker_name == name {
                self.current_time = *time;
                return Some(*time);
            }
        }
        None
    }

    pub fn jump_by(&mut self, delta: TimeValue) {
        self.current_time.value = (self.current_time.value + delta.value).max(0.0);
    }

    pub fn jump_to(&mut self, time: TimeValue) {
        self.current_time = time;
    }

    pub fn current_time(&self) -> TimeValue {
        self.current_time
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}
// Trait implementations for TimeValue
impl PartialEq for TimeValue {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for TimeValue {}

impl PartialOrd for TimeValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for TimeValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value
            .partial_cmp(&other.value)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl std::ops::Add for TimeValue {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value)
    }
}

impl std::ops::Sub for TimeValue {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.value - other.value)
    }
}

impl std::ops::AddAssign for TimeValue {
    fn add_assign(&mut self, other: Self) {
        self.value += other.value;
    }
}

impl std::ops::SubAssign for TimeValue {
    fn sub_assign(&mut self, other: Self) {
        self.value -= other.value;
    }
}

impl std::ops::Rem for TimeValue {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        if other.value == 0.0 {
            Self::new(0.0)
        } else {
            Self::new(self.value % other.value)
        }
    }
}
