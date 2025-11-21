//! # Animation Effects
//!
//! Pre-built animation effects like FadeIn, FadeOut, Create, etc.
//! Inspired by Manim's animation library.

use crate::animation::property::{AnimationClip, AnimationTrack, Keyframe};
use crate::core::{TimeValue, Vector3};

/// Create a FadeIn animation that animates opacity from 0 to 1
pub fn fade_in(duration: f32) -> AnimationClip {
    let mut clip = AnimationClip::new("FadeIn".to_string());
    let mut track = AnimationTrack::new("opacity".to_string());

    track.add_keyframe(Keyframe::new(TimeValue::new(0.0), Vector3::new(0.0, 0.0, 0.0)));
    track.add_keyframe(Keyframe::new(TimeValue::new(duration), Vector3::new(1.0, 0.0, 0.0)));

    clip.add_track(track);
    clip.loop_animation = false;
    clip
}

/// Create a FadeOut animation that animates opacity from 1 to 0
pub fn fade_out(duration: f32) -> AnimationClip {
    let mut clip = AnimationClip::new("FadeOut".to_string());
    let mut track = AnimationTrack::new("opacity".to_string());

    track.add_keyframe(Keyframe::new(TimeValue::new(0.0), Vector3::new(1.0, 0.0, 0.0)));
    track.add_keyframe(Keyframe::new(TimeValue::new(duration), Vector3::new(0.0, 0.0, 0.0)));

    clip.add_track(track);
    clip.loop_animation = false;
    clip
}

/// Create a FadeInOut animation that fades in then out
pub fn fade_in_out(fade_in_duration: f32, hold_duration: f32, fade_out_duration: f32) -> AnimationClip {
    let mut clip = AnimationClip::new("FadeInOut".to_string());
    let mut track = AnimationTrack::new("opacity".to_string());

    let t1 = fade_in_duration;
    let t2 = t1 + hold_duration;
    let t3 = t2 + fade_out_duration;

    track.add_keyframe(Keyframe::new(TimeValue::new(0.0), Vector3::new(0.0, 0.0, 0.0)));
    track.add_keyframe(Keyframe::new(TimeValue::new(t1), Vector3::new(1.0, 0.0, 0.0)));
    track.add_keyframe(Keyframe::new(TimeValue::new(t2), Vector3::new(1.0, 0.0, 0.0)));
    track.add_keyframe(Keyframe::new(TimeValue::new(t3), Vector3::new(0.0, 0.0, 0.0)));

    clip.add_track(track);
    clip.loop_animation = false;
    clip
}

/// Create a GrowFromCenter animation that scales from 0 to 1
pub fn grow_from_center(duration: f32) -> AnimationClip {
    let mut clip = AnimationClip::new("GrowFromCenter".to_string());
    let mut track = AnimationTrack::new("scale".to_string());

    track.add_keyframe(Keyframe::new(TimeValue::new(0.0), Vector3::new(0.0, 0.0, 0.0)));
    track.add_keyframe(Keyframe::new(TimeValue::new(duration), Vector3::new(1.0, 1.0, 1.0)));

    clip.add_track(track);
    clip.loop_animation = false;
    clip
}

/// Create a ShrinkToCenter animation that scales from 1 to 0
pub fn shrink_to_center(duration: f32) -> AnimationClip {
    let mut clip = AnimationClip::new("ShrinkToCenter".to_string());
    let mut track = AnimationTrack::new("scale".to_string());

    track.add_keyframe(Keyframe::new(TimeValue::new(0.0), Vector3::new(1.0, 1.0, 1.0)));
    track.add_keyframe(Keyframe::new(TimeValue::new(duration), Vector3::new(0.0, 0.0, 0.0)));

    clip.add_track(track);
    clip.loop_animation = false;
    clip
}

/// Create a combined FadeIn + GrowFromCenter effect
pub fn create(duration: f32) -> AnimationClip {
    let mut clip = AnimationClip::new("Create".to_string());

    // Opacity track
    let mut opacity_track = AnimationTrack::new("opacity".to_string());
    opacity_track.add_keyframe(Keyframe::new(TimeValue::new(0.0), Vector3::new(0.0, 0.0, 0.0)));
    opacity_track.add_keyframe(Keyframe::new(TimeValue::new(duration), Vector3::new(1.0, 0.0, 0.0)));

    // Scale track
    let mut scale_track = AnimationTrack::new("scale".to_string());
    scale_track.add_keyframe(Keyframe::new(TimeValue::new(0.0), Vector3::new(0.0, 0.0, 0.0)));
    scale_track.add_keyframe(Keyframe::new(TimeValue::new(duration), Vector3::new(1.0, 1.0, 1.0)));

    clip.add_track(opacity_track);
    clip.add_track(scale_track);
    clip.loop_animation = false;
    clip
}

/// Create an Uncreate animation (reverse of Create)
pub fn uncreate(duration: f32) -> AnimationClip {
    let mut clip = AnimationClip::new("Uncreate".to_string());

    // Opacity track
    let mut opacity_track = AnimationTrack::new("opacity".to_string());
    opacity_track.add_keyframe(Keyframe::new(TimeValue::new(0.0), Vector3::new(1.0, 0.0, 0.0)));
    opacity_track.add_keyframe(Keyframe::new(TimeValue::new(duration), Vector3::new(0.0, 0.0, 0.0)));

    // Scale track
    let mut scale_track = AnimationTrack::new("scale".to_string());
    scale_track.add_keyframe(Keyframe::new(TimeValue::new(0.0), Vector3::new(1.0, 1.0, 1.0)));
    scale_track.add_keyframe(Keyframe::new(TimeValue::new(duration), Vector3::new(0.0, 0.0, 0.0)));

    clip.add_track(opacity_track);
    clip.add_track(scale_track);
    clip.loop_animation = false;
    clip
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fade_in() {
        let anim = fade_in(1.0);
        assert_eq!(anim.name, "FadeIn");
        assert_eq!(anim.tracks.len(), 1);
        assert!(!anim.loop_animation);
    }

    #[test]
    fn test_fade_out() {
        let anim = fade_out(1.0);
        assert_eq!(anim.name, "FadeOut");
        assert_eq!(anim.tracks.len(), 1);
    }

    #[test]
    fn test_create() {
        let anim = create(1.5);
        assert_eq!(anim.name, "Create");
        assert_eq!(anim.tracks.len(), 2); // opacity + scale
    }
}
