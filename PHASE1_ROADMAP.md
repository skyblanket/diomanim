# Phase 1 Roadmap: Make Diomanim Usable

**Goal:** Transform Diomanim from a proof-of-concept into a usable animation tool
**Timeline:** 2-3 weeks of focused work
**Priority:** High-impact features that unlock real-world usage

---

## Current State

✅ **What Works:**
- WebGPU rendering pipeline
- Scene graph with hierarchy
- Basic keyframe animations
- Circle rendering
- Video export (MP4)

❌ **What's Blocking Usage:**
- Only 2 shapes (Circle, Square - and Square isn't even rendered!)
- Can't render multiple objects efficiently in one frame
- No live preview (must wait for full video render)
- Limited animation types (only keyframe interpolation)
- No text rendering

---

## Phase 1 Tasks

### 1. Fix Multi-Object Rendering 🔴 CRITICAL

**Problem:** Currently creates a new render pass for each object, which is inefficient and causes rendering issues.

**Solution:**
- Batch all objects into a single render pass
- Clear background only once per frame
- Render all objects sequentially within one pass

**Files to modify:**
- `src/render/mod.rs`: Refactor `render_circle()` to support batching
- `src/main.rs`: Update rendering loop to collect all objects before rendering

**Implementation Steps:**
```rust
// Current (bad):
for object in objects {
    renderer.render_circle(object);  // Each creates new render pass
}

// Target (good):
renderer.begin_frame(&output_view);  // Clear once
for object in objects {
    renderer.draw_circle(object);     // Add to batch
}
renderer.end_frame();                 // Submit once
```

**Time estimate:** 1 day
**Priority:** P0 - Blocks everything else

---

### 2. Add Essential Shapes 🟡 HIGH

Implement the most commonly used shapes for animations.

#### 2.1 Rectangle
```rust
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub position: Vector3,
}
```

**Implementation:**
- Add to `src/mobjects/mod.rs`
- Create quad vertices (6 vertices = 2 triangles)
- Add `Renderable::Rectangle` variant
- Implement rendering in `src/render/mod.rs`

**Time estimate:** 0.5 days

#### 2.2 Line
```rust
pub struct Line {
    pub start: Vector3,
    pub end: Vector3,
    pub color: Color,
    pub thickness: f32,
}
```

**Implementation:**
- Render as thick line (4 vertices forming rectangle)
- Calculate perpendicular direction for thickness
- Support end caps (rounded, square)

**Time estimate:** 0.5 days

#### 2.3 Arrow
```rust
pub struct Arrow {
    pub start: Vector3,
    pub end: Vector3,
    pub color: Color,
    pub tip_size: f32,
}
```

**Implementation:**
- Extends Line with triangle tip
- Calculate tip vertices at end point
- Support different arrow styles

**Time estimate:** 0.5 days

#### 2.4 Polygon
```rust
pub struct Polygon {
    pub vertices: Vec<Vector3>,
    pub color: Color,
}
```

**Implementation:**
- Triangulate using ear clipping algorithm
- Support both filled and outline modes
- Helper methods: `regular_polygon(sides, radius)`

**Time estimate:** 1 day

**Total shapes time:** 2.5 days
**Priority:** P1

---

### 3. More Animation Types 🟢 MEDIUM

Currently only have keyframe interpolation. Add common animation patterns.

#### 3.1 FadeIn / FadeOut
```rust
pub struct FadeAnimation {
    pub duration: TimeValue,
    pub fade_in: bool,  // true = fade in, false = fade out
}
```

**Implementation:**
- Animate alpha channel from 0→1 (fade in) or 1→0 (fade out)
- Add `opacity` field to all Renderables
- Modify shaders to support alpha

**Time estimate:** 0.5 days

#### 3.2 Create / Write Animation
```rust
pub struct CreateAnimation {
    pub duration: TimeValue,
    pub direction: CreateDirection,  // LeftToRight, RightToLeft, etc.
}
```

**Implementation:**
- Animate a "reveal" parameter from 0→1
- For shapes: grow from center or sweep direction
- For lines: draw from start to end

**Time estimate:** 1 day

#### 3.3 Transform Animation
```rust
pub struct TransformAnimation {
    pub target_shape: Box<dyn Renderable>,
    pub duration: TimeValue,
}
```

**Implementation:**
- Morph one shape into another
- Interpolate vertices between shapes
- Handle different vertex counts (triangle subdivision)

**Time estimate:** 1.5 days

**Total animation time:** 3 days
**Priority:** P2

---

### 4. Live Preview Window 🟢 MEDIUM

**Problem:** Must render full video to see results (slow iteration).

**Solution:** Real-time preview window with playback controls.

**Implementation:**
```rust
// Use existing dioxus for window
// Create preview mode
struct PreviewWindow {
    scene: SceneGraph,
    current_time: TimeValue,
    playing: bool,
}

// Add controls
- Play/Pause
- Scrub timeline
- Frame-by-frame stepping
- Real-time rendering at 60 FPS
```

**Files to create:**
- `src/preview/mod.rs`: Preview window logic
- `src/preview/controls.rs`: Playback controls UI

**Time estimate:** 2 days
**Priority:** P1 (huge DX improvement)

---

### 5. Easing Functions 🟡 HIGH

Current interpolation is linear only. Add smooth easing.

**Implementation:**
```rust
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
    Elastic,
    Custom(fn(f32) -> f32),
}

impl EasingFunction {
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            Linear => t,
            EaseIn => t * t,
            EaseOut => 1.0 - (1.0 - t).powi(2),
            EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            },
            // ... more functions
        }
    }
}
```

**Files to modify:**
- `src/animation/property.rs`: Add easing to AnimationTrack
- Use easing in interpolation calculations

**Time estimate:** 1 day
**Priority:** P1

---

### 6. Scene API Improvements 🟢 MEDIUM

Make it easier to create animations programmatically.

**Current (verbose):**
```rust
let circle_id = scene.create_node("circle".to_string());
let node = scene.get_node_mut(circle_id).unwrap();
node.set_renderable(Renderable::Circle { radius: 1.0, color: Color::RED });
```

**Target (fluent):**
```rust
let circle = scene
    .add_circle(1.0, Color::RED)
    .at(0.0, 0.0)
    .animate_to(5.0, 0.0, 2.0)
    .with_easing(EaseInOut)
    .build();
```

**Implementation:**
- Create builder pattern for shapes
- Add convenience methods to SceneGraph
- Chain-able API for common operations

**Files to create:**
- `src/scene/builder.rs`: Builder API

**Time estimate:** 1.5 days
**Priority:** P2

---

### 7. Better Error Handling 🟢 MEDIUM

**Current:** Lots of `.unwrap()` calls that panic.

**Target:** Proper error handling with helpful messages.

**Implementation:**
```rust
// Create error types
#[derive(Debug)]
pub enum DiomanimError {
    RenderError(String),
    SceneError(String),
    AnimationError(String),
    IoError(std::io::Error),
}

// Replace unwraps
// Before: scene.get_node_mut(id).unwrap()
// After:  scene.get_node_mut(id)
//              .ok_or(DiomanimError::SceneError(format!("Node {:?} not found", id)))?
```

**Files to create:**
- `src/error.rs`: Error types

**Time estimate:** 1 day
**Priority:** P2

---

## Implementation Order

### Week 1: Core Rendering
1. **Day 1:** Fix multi-object rendering (P0)
2. **Day 2:** Add Rectangle and Line shapes
3. **Day 3:** Add Arrow and Polygon shapes
4. **Day 4:** Test and debug shape rendering
5. **Day 5:** Add easing functions

### Week 2: Animation & Preview
6. **Day 6-7:** Live preview window with controls
7. **Day 8:** FadeIn/FadeOut animations
8. **Day 9:** Create/Write animations
9. **Day 10:** Scene API improvements

### Week 3: Polish
10. **Day 11-12:** Transform animations
11. **Day 13:** Better error handling
12. **Day 14:** Integration testing, bug fixes
13. **Day 15:** Documentation updates

---

## Testing Strategy

For each feature, write:
1. **Unit tests**: Test shape creation, vertex generation
2. **Integration tests**: Test full render pipeline
3. **Visual tests**: Render sample frames, verify output

**Example:**
```rust
#[test]
fn test_rectangle_vertices() {
    let rect = Rectangle::new(2.0, 1.0, Color::RED);
    let vertices = rect.generate_vertices();
    assert_eq!(vertices.len(), 6); // 2 triangles
}

#[test]
fn test_fade_animation() {
    let mut anim = FadeAnimation::new(1.0, true);
    assert_eq!(anim.get_alpha(0.0), 0.0);
    assert_eq!(anim.get_alpha(0.5), 0.5);
    assert_eq!(anim.get_alpha(1.0), 1.0);
}
```

---

## Success Criteria

Phase 1 is complete when you can:

✅ **Create a simple animation like this:**
```rust
use diomanim::prelude::*;

fn main() {
    let mut scene = Scene::new();

    // Add shapes
    let circle = scene.add_circle(1.0, Color::BLUE)
        .at(-5.0, 0.0);

    let rect = scene.add_rectangle(2.0, 1.0, Color::RED)
        .at(5.0, 0.0);

    let arrow = scene.add_arrow((-3.0, -2.0), (3.0, 2.0), Color::GREEN);

    // Animate them
    circle.fade_in(0.0, 1.0);
    circle.move_to(5.0, 0.0, 1.0, 3.0).with_easing(EaseInOut);

    rect.create(1.0, 2.0);

    arrow.fade_out(3.0, 4.0);

    // Preview or export
    scene.preview();  // Live preview
    // OR
    scene.export_video("output.mp4", 60.0);  // 60 FPS
}
```

✅ **See results in real-time** via preview window
✅ **All shapes render correctly** in a single frame
✅ **Smooth animations** with easing functions
✅ **No panics** - proper error messages

---

## Notes for Mally

### Quick Wins First
Start with **Task 1 (multi-object rendering)**. This unblocks everything else and you'll immediately see all 6 circles rendering properly in the demo.

### Don't Overthink Text Yet
Phase 1 intentionally skips text/LaTeX. Get the foundation solid first. Text is Phase 2 because it's complex and requires font rendering infrastructure.

### Use Existing Code Patterns
- Follow the Circle implementation for new shapes
- Copy the keyframe system for new animation types
- Maintain the existing test coverage approach

### Keep Performance in Mind
- Batch rendering is key
- Minimize GPU state changes
- Pre-compute triangulations where possible

### Reference Implementations
- **Shapes:** Look at how SVG renderers triangulate polygons
- **Easing:** Use Robert Penner's easing equations (public domain)
- **Preview:** Winit + wgpu surface (you already have dioxus setup)

---

## After Phase 1

You'll have a **usable animation tool** that can create:
- Multi-shape scenes
- Smooth animations with easing
- Live preview for fast iteration
- Real animations people can use

**Then move to Phase 2:** Text rendering, LaTeX, and math-specific features.

---

## Questions?

- **Stuck on rendering?** Check the wgpu examples repo
- **Animation math unclear?** Look at CSS animation specs (same easing functions)
- **Need design feedback?** The API should feel like Manim but more Rust-like

**Most important:** Get Task 1 done first. Everything else builds on it.

Good luck! 🚀
