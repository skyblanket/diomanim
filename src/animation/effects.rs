//! # Animation Effects
//!
//! Pre-built animation effects like FadeIn, FadeOut, Create, etc.
//! Inspired by Manim's animation library.
//!
//! ## Phase 1 Effects
//! - Opacity animations (FadeIn, FadeOut)
//! - Scale animations (GrowFromCenter, ShrinkToCenter)
//! - Combined effects (Create, Uncreate)
//!
//! ## Phase 2 Effects
//! - Transform animations (MoveTo, Shift, Rotate)
//! - Path animations (Write)

use crate::animation::property::{AnimationClip, AnimationTrack, Keyframe};
use crate::core::{TimeValue, Vector3};

/// Create a FadeIn animation that animates opacity from 0 to 1
pub fn fade_in(duration: f32) -> AnimationClip {
    let mut clip = AnimationClip::new("FadeIn".to_string());
    let mut track = AnimationTrack::new("opacity".to_string());

    track.add_keyframe(Keyframe::new(
        TimeValue::new(0.0),
        Vector3::new(0.0, 0.0, 0.0),
    ));
    track.add_keyframe(Keyframe::new(
        TimeValue::new(duration),
        Vector3::new(1.0, 0.0, 0.0),
    ));

    clip.add_track(track);
    clip.loop_animation = false;
    clip
}

/// Create a FadeOut animation that animates opacity from 1 to 0
pub fn fade_out(duration: f32) -> AnimationClip {
    let mut clip = AnimationClip::new("FadeOut".to_string());
    let mut track = AnimationTrack::new("opacity".to_string());

    track.add_keyframe(Keyframe::new(
        TimeValue::new(0.0),
        Vector3::new(1.0, 0.0, 0.0),
    ));
    track.add_keyframe(Keyframe::new(
        TimeValue::new(duration),
        Vector3::new(0.0, 0.0, 0.0),
    ));

    clip.add_track(track);
    clip.loop_animation = false;
    clip
}

/// Create a FadeInOut animation that fades in then out
pub fn fade_in_out(
    fade_in_duration: f32,
    hold_duration: f32,
    fade_out_duration: f32,
) -> AnimationClip {
    let mut clip = AnimationClip::new("FadeInOut".to_string());
    let mut track = AnimationTrack::new("opacity".to_string());

    let t1 = fade_in_duration;
    let t2 = t1 + hold_duration;
    let t3 = t2 + fade_out_duration;

    track.add_keyframe(Keyframe::new(
        TimeValue::new(0.0),
        Vector3::new(0.0, 0.0, 0.0),
    ));
    track.add_keyframe(Keyframe::new(
        TimeValue::new(t1),
        Vector3::new(1.0, 0.0, 0.0),
    ));
    track.add_keyframe(Keyframe::new(
        TimeValue::new(t2),
        Vector3::new(1.0, 0.0, 0.0),
    ));
    track.add_keyframe(Keyframe::new(
        TimeValue::new(t3),
        Vector3::new(0.0, 0.0, 0.0),
    ));

    clip.add_track(track);
    clip.loop_animation = false;
    clip
}

/// Create a GrowFromCenter animation that scales from 0 to 1
pub fn grow_from_center(duration: f32) -> AnimationClip {
    let mut clip = AnimationClip::new("GrowFromCenter".to_string());
    let mut track = AnimationTrack::new("scale".to_string());

    track.add_keyframe(Keyframe::new(
        TimeValue::new(0.0),
        Vector3::new(0.0, 0.0, 0.0),
    ));
    track.add_keyframe(Keyframe::new(
        TimeValue::new(duration),
        Vector3::new(1.0, 1.0, 1.0),
    ));

    clip.add_track(track);
    clip.loop_animation = false;
    clip
}

/// Create a ShrinkToCenter animation that scales from 1 to 0
pub fn shrink_to_center(duration: f32) -> AnimationClip {
    let mut clip = AnimationClip::new("ShrinkToCenter".to_string());
    let mut track = AnimationTrack::new("scale".to_string());

    track.add_keyframe(Keyframe::new(
        TimeValue::new(0.0),
        Vector3::new(1.0, 1.0, 1.0),
    ));
    track.add_keyframe(Keyframe::new(
        TimeValue::new(duration),
        Vector3::new(0.0, 0.0, 0.0),
    ));

    clip.add_track(track);
    clip.loop_animation = false;
    clip
}

/// Create a combined FadeIn + GrowFromCenter effect
pub fn create(duration: f32) -> AnimationClip {
    let mut clip = AnimationClip::new("Create".to_string());

    // Opacity track
    let mut opacity_track = AnimationTrack::new("opacity".to_string());
    opacity_track.add_keyframe(Keyframe::new(
        TimeValue::new(0.0),
        Vector3::new(0.0, 0.0, 0.0),
    ));
    opacity_track.add_keyframe(Keyframe::new(
        TimeValue::new(duration),
        Vector3::new(1.0, 0.0, 0.0),
    ));

    // Scale track
    let mut scale_track = AnimationTrack::new("scale".to_string());
    scale_track.add_keyframe(Keyframe::new(
        TimeValue::new(0.0),
        Vector3::new(0.0, 0.0, 0.0),
    ));
    scale_track.add_keyframe(Keyframe::new(
        TimeValue::new(duration),
        Vector3::new(1.0, 1.0, 1.0),
    ));

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
    opacity_track.add_keyframe(Keyframe::new(
        TimeValue::new(0.0),
        Vector3::new(1.0, 0.0, 0.0),
    ));
    opacity_track.add_keyframe(Keyframe::new(
        TimeValue::new(duration),
        Vector3::new(0.0, 0.0, 0.0),
    ));

    // Scale track
    let mut scale_track = AnimationTrack::new("scale".to_string());
    scale_track.add_keyframe(Keyframe::new(
        TimeValue::new(0.0),
        Vector3::new(1.0, 1.0, 1.0),
    ));
    scale_track.add_keyframe(Keyframe::new(
        TimeValue::new(duration),
        Vector3::new(0.0, 0.0, 0.0),
    ));

    clip.add_track(opacity_track);
    clip.add_track(scale_track);
    clip.loop_animation = false;
    clip
}

// ============================================================================
// PHASE 2 EFFECTS - Transform Animations
// ============================================================================

/// Move object to a specific position
///
/// # Arguments
/// * `from` - Starting position
/// * `to` - Target position
/// * `duration` - Animation duration in seconds
///
/// # Example
/// ```
/// use diomanim::animation::effects::move_to;
/// use diomanim::core::Vector3;
///
/// let anim = move_to(
///     Vector3::new(-1.0, 0.0, 0.0),
///     Vector3::new(1.0, 0.0, 0.0),
///     2.0
/// );
/// ```
pub fn move_to(from: Vector3, to: Vector3, duration: f32) -> AnimationClip {
    let mut clip = AnimationClip::new("MoveTo".to_string());
    let mut track = AnimationTrack::new("position".to_string());

    track.add_keyframe(Keyframe::new(TimeValue::new(0.0), from));
    track.add_keyframe(Keyframe::new(TimeValue::new(duration), to));

    clip.add_track(track);
    clip.loop_animation = false;
    clip
}

/// Shift object by an offset
///
/// # Arguments
/// * `current_pos` - Current position
/// * `offset` - Offset to add
/// * `duration` - Animation duration in seconds
pub fn shift(current_pos: Vector3, offset: Vector3, duration: f32) -> AnimationClip {
    move_to(current_pos, current_pos + offset, duration)
}

/// Rotate object to a specific angle (Z-axis rotation for 2D)
///
/// # Arguments
/// * `from_angle` - Starting angle in radians
/// * `to_angle` - Target angle in radians
/// * `duration` - Animation duration in seconds
pub fn rotate(from_angle: f32, to_angle: f32, duration: f32) -> AnimationClip {
    let mut clip = AnimationClip::new("Rotate".to_string());
    let mut track = AnimationTrack::new("rotation".to_string());

    track.add_keyframe(Keyframe::new(
        TimeValue::new(0.0),
        Vector3::new(0.0, 0.0, from_angle),
    ));
    track.add_keyframe(Keyframe::new(
        TimeValue::new(duration),
        Vector3::new(0.0, 0.0, to_angle),
    ));

    clip.add_track(track);
    clip.loop_animation = false;
    clip
}

/// Rotate by an offset angle
pub fn rotate_by(current_angle: f32, angle_offset: f32, duration: f32) -> AnimationClip {
    rotate(current_angle, current_angle + angle_offset, duration)
}

/// General transform animation
///
/// # Arguments
/// * `from_pos` - Starting position
/// * `to_pos` - Target position
/// * `from_scale` - Starting scale
/// * `to_scale` - Target scale
/// * `from_rotation` - Starting rotation (Z-axis)
/// * `to_rotation` - Target rotation (Z-axis)
/// * `duration` - Animation duration in seconds
pub fn transform(
    from_pos: Vector3,
    to_pos: Vector3,
    from_scale: Vector3,
    to_scale: Vector3,
    from_rotation: f32,
    to_rotation: f32,
    duration: f32,
) -> AnimationClip {
    let mut clip = AnimationClip::new("Transform".to_string());

    // Position track
    let mut pos_track = AnimationTrack::new("position".to_string());
    pos_track.add_keyframe(Keyframe::new(TimeValue::new(0.0), from_pos));
    pos_track.add_keyframe(Keyframe::new(TimeValue::new(duration), to_pos));

    // Scale track
    let mut scale_track = AnimationTrack::new("scale".to_string());
    scale_track.add_keyframe(Keyframe::new(TimeValue::new(0.0), from_scale));
    scale_track.add_keyframe(Keyframe::new(TimeValue::new(duration), to_scale));

    // Rotation track
    let mut rot_track = AnimationTrack::new("rotation".to_string());
    rot_track.add_keyframe(Keyframe::new(
        TimeValue::new(0.0),
        Vector3::new(0.0, 0.0, from_rotation),
    ));
    rot_track.add_keyframe(Keyframe::new(
        TimeValue::new(duration),
        Vector3::new(0.0, 0.0, to_rotation),
    ));

    clip.add_track(pos_track);
    clip.add_track(scale_track);
    clip.add_track(rot_track);
    clip.loop_animation = false;
    clip
}

/// Spin animation - continuous rotation
///
/// # Arguments
/// * `rotations` - Number of full rotations
/// * `duration` - Animation duration in seconds
pub fn spin(rotations: f32, duration: f32) -> AnimationClip {
    let end_angle = rotations * 2.0 * std::f32::consts::PI;
    rotate(0.0, end_angle, duration)
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

    #[test]
    fn test_move_to() {
        let from = Vector3::new(-1.0, 0.0, 0.0);
        let to = Vector3::new(1.0, 0.0, 0.0);
        let anim = move_to(from, to, 2.0);
        assert_eq!(anim.name, "MoveTo");
        assert_eq!(anim.tracks.len(), 1);
    }

    #[test]
    fn test_rotate() {
        let anim = rotate(0.0, std::f32::consts::PI, 1.0);
        assert_eq!(anim.name, "Rotate");
        assert_eq!(anim.tracks.len(), 1);
    }

    #[test]
    fn test_spin() {
        let anim = spin(2.0, 3.0);
        assert_eq!(anim.name, "Rotate");
        assert_eq!(anim.tracks.len(), 1);
    }

    #[test]
    fn test_transform() {
        let anim = transform(
            Vector3::zero(),
            Vector3::new(1.0, 1.0, 0.0),
            Vector3::one(),
            Vector3::new(2.0, 2.0, 1.0),
            0.0,
            std::f32::consts::PI,
            2.0,
        );
        assert_eq!(anim.name, "Transform");
        assert_eq!(anim.tracks.len(), 3); // position + scale + rotation
    }
}
