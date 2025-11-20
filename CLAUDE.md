# CLAUDE.md - AI Assistant Guide for Diomanim

## Project Overview

**Diomanim** (also known as DioxManim) is a next-generation animation engine built with Rust and Dioxus, designed to outperform Python-based Manim with 60x faster rendering, GPU acceleration, and real-time preview capabilities. This document provides AI assistants with comprehensive guidance on the codebase structure, development workflows, and key conventions.

### Key Project Goals
- **Performance**: 60 FPS real-time preview, 60x faster rendering than Manim
- **GPU Acceleration**: WebGPU-based rendering (Vulkan/Metal/DirectX 12)
- **Modern UI**: Dioxus-based editor with React-like immediate mode interface
- **Type Safety**: Leverage Rust's compile-time guarantees
- **Memory Efficiency**: 20x less memory usage compared to Python alternatives

### Technology Stack
- **Language**: Rust 1.70+ (Edition 2021)
- **UI Framework**: Dioxus 0.7.1 (desktop)
- **Rendering**: WebGPU (wgpu 27.0.1)
- **Math Library**: glam 0.30.9 (SIMD-accelerated)
- **Async Runtime**: Tokio 1.48.0
- **Serialization**: serde + serde_json

---

## Repository Structure

```
diomanim/
├── src/
│   ├── core/               # Foundation systems
│   │   ├── vector.rs       # Vector math with SIMD
│   │   ├── color.rs        # Color management
│   │   ├── transform.rs    # Position, rotation, scale
│   │   ├── camera.rs       # Camera system
│   │   ├── time.rs         # Timing utilities
│   │   ├── config.rs       # Configuration
│   │   ├── scene.rs        # Core scene types
│   │   └── mod.rs          # Core module exports
│   ├── animation/          # Animation system
│   │   ├── property.rs     # Keyframe animations
│   │   └── mod.rs          # Animation exports
│   ├── scene/              # Scene graph
│   │   └── mod.rs          # Hierarchical node system
│   ├── mobjects/           # Scene objects
│   │   └── mod.rs          # Shapes and geometry
│   ├── render/             # GPU rendering
│   │   ├── mod.rs          # WebGPU renderer
│   │   └── shaders/        # WGSL shaders
│   │       ├── basic.wgsl
│   │       └── transform.wgsl
│   ├── lib.rs              # Library entry point with prelude
│   └── main.rs             # Demo application
├── examples/
│   └── simple_circle.rs    # Example animations
├── Cargo.toml              # Dependencies and metadata
├── build.sh                # Build and setup script
├── README.md               # User-facing documentation
├── PHASE1_ROADMAP.md       # Current development plan
├── CONTRIBUTING.md         # Contribution guidelines
└── styles.css              # UI styles

Ignored directories (see .gitignore):
├── target/                 # Cargo build artifacts
├── frames/                 # Rendered frame outputs
├── output/                 # Video export directory
└── .dioxus/                # Dioxus cache
```

---

## Development Workflows

### Initial Setup

```bash
# Prerequisites: Rust 1.70+, Git
# Install Rust: https://rustup.rs/

# Clone and setup
git clone <repository-url>
cd diomanim

# Quick build (runs build.sh)
./build.sh

# The build script:
# - Checks Rust installation
# - Installs Dioxus CLI if needed
# - Builds in release mode
# - Shows project status
```

### Daily Development

```bash
# Development build
cd diomanim
cargo build

# Run demo application
cargo run --release

# Run with Dioxus hot-reload
dx serve --desktop --hot-reload

# Run tests
cargo test

# Run specific test
cargo test test_name

# Run example
cargo run --example simple_circle --release
```

### Code Quality

```bash
# Format code (ALWAYS run before committing)
cargo fmt

# Lint with Clippy (address all warnings)
cargo clippy -- -D warnings

# Check without building
cargo check

# Full quality check before PR
cargo fmt && cargo clippy -- -D warnings && cargo test && cargo build --release
```

### Performance Profiling

```bash
# Release build with optimizations
cargo build --release

# Profile with perf (Linux)
perf record -g ./target/release/diomanim
perf report

# Benchmark (when benches/ directory exists)
cargo bench
```

---

## Git Conventions

### Branch Strategy
- Main branch: `main`
- Feature branches: `feature/<description>` or `claude/<session-id>`
- Current working branch: `claude/claude-md-mi7ot2nq3xa5zfsv-01EJdrdijrrJJsTdUZdNV6dE`

### Commit Message Format

Use **conventional commits** format:

```
<type>: <description>

[optional body]
[optional footer]
```

**Types:**
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `test:` - Test additions or modifications
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `style:` - Code style changes (formatting)
- `chore:` - Build process or auxiliary tool changes

**Examples:**
```bash
git commit -m "feat: add rectangle mobject with GPU rendering"
git commit -m "fix: correct transform matrix multiplication order"
git commit -m "docs: update CLAUDE.md with animation system details"
git commit -m "refactor: extract color conversion to helper method"
```

### Git Push Strategy

**IMPORTANT:** Always push to the designated branch with `-u` flag:

```bash
# Push to feature branch
git push -u origin claude/claude-md-mi7ot2nq3xa5zfsv-01EJdrdijrrJJsTdUZdNV6dE

# If network errors occur, retry up to 4 times with exponential backoff:
# - Wait 2s, retry
# - Wait 4s, retry
# - Wait 8s, retry
# - Wait 16s, retry
```

---

## Code Architecture & Conventions

### Module Organization

#### `src/core/` - Foundation Systems
- **vector.rs**: 3D vector math with SIMD optimizations (Vector3)
- **color.rs**: Color management, gradients, color spaces (Color)
- **transform.rs**: Position, rotation, scale transformations (Transform)
- **camera.rs**: Camera system with perspective/orthographic projections
- **time.rs**: Timing utilities, frame-rate independent animation (TimeValue)
- **config.rs**: Application configuration
- **scene.rs**: Core scene types and utilities

#### `src/animation/` - Animation System
- **property.rs**: Keyframe-based property animation system
  - `AnimationClip`: Container for keyframes
  - `AnimationInstance`: Playback state for clips
  - Currently linear interpolation (easing functions in Phase 1 roadmap)

#### `src/scene/` - Scene Graph
- Hierarchical node system (ECS-inspired)
- Transform inheritance from parent to child
- Node management: create, parent, update, traverse
- Each node can have a `Renderable` (visual representation)

#### `src/mobjects/` - Scene Objects
- **Circle**: Primary shape (fully implemented)
- **Square**: Declared but not yet rendered
- Renderables are GPU-primitive representations

#### `src/render/` - GPU Rendering Pipeline
- WebGPU-based renderer (ShapeRenderer)
- Vertex/fragment shaders in WGSL
- **Current limitation**: Multi-object rendering needs optimization (see Phase 1 roadmap)
- Frame export to PNG/video formats

### Prelude Pattern

The library uses a prelude pattern for common imports:

```rust
use diomanim::prelude::*;
// Exports: Timer, Camera, Color, TimeValue, Transform, Vector3, Circle, ShapeRenderer, Vertex
```

### Type Conventions

```rust
// Vectors: glam-based SIMD types
use glam::Vec3;
pub type Vector3 = Vec3;

// Colors: RGBA floating point (0.0-1.0)
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

// Time: Type-safe wrapper
pub struct TimeValue(f32); // seconds

// Transforms: 4x4 matrices
pub struct Transform {
    pub position: Vector3,
    pub rotation: Vector3,
    pub scale: Vector3,
}
```

### Rendering Pipeline Pattern

```rust
// 1. Initialize renderer (async)
let renderer = ShapeRenderer::new(width, height).await?;

// 2. Build scene
let mut scene = SceneGraph::new();
let node_id = scene.create_node("name".to_string());

// 3. Add renderable
scene.get_node_mut(node_id).unwrap().set_renderable(
    Renderable::Circle { radius: 1.0, color: Color::RED }
);

// 4. Animate (per frame)
scene.update_animations(delta_time);
scene.update_transforms();

// 5. Render
renderer.render_circle(&node, color, &output_view);
```

---

## Current Development Phase

### Phase 1: Make Diomanim Usable (In Progress)

**Timeline**: 2-3 weeks
**Status**: Early development, core systems completed

#### Completed ✅
- WebGPU rendering pipeline
- Scene graph with hierarchy
- Basic keyframe animations
- Circle rendering
- Video export (MP4)

#### In Progress / Blockers ⚠️
1. **CRITICAL (P0)**: Multi-object rendering optimization
   - Current: Each object creates new render pass (inefficient)
   - Target: Batch all objects in single pass
   - Files: `src/render/mod.rs`, `src/main.rs`

2. **HIGH (P1)**: Additional shapes
   - Rectangle, Line, Arrow, Polygon
   - Each needs vertex generation + GPU rendering

3. **HIGH (P1)**: Live preview window
   - Real-time 60 FPS preview with playback controls
   - Eliminates slow video render iteration

4. **HIGH (P1)**: Easing functions
   - Currently only linear interpolation
   - Need: ease-in, ease-out, ease-in-out, bounce, elastic

5. **MEDIUM (P2)**: More animation types
   - FadeIn/FadeOut, Create/Write, Transform animations

See **PHASE1_ROADMAP.md** for detailed implementation plan.

---

## Key Conventions for AI Assistants

### When Writing Code

1. **Always run `cargo fmt` before suggesting code**
   - Rust has strict formatting conventions
   - Use 4-space indentation, standard Rust style

2. **Handle errors properly**
   - Avoid `.unwrap()` in production code
   - Use `Result<T, E>` with proper error types
   - Consider adding `DiomanimError` enum (Phase 1 task)

3. **Follow existing patterns**
   - Copy Circle implementation for new shapes
   - Use Transform for all position/rotation/scale
   - Follow prelude export pattern for public APIs

4. **Performance considerations**
   - Minimize GPU state changes
   - Batch rendering operations
   - Use SIMD-friendly data layouts
   - Pre-compute when possible (e.g., triangulations)

5. **Type safety**
   - Leverage Rust's type system
   - Use newtype pattern (e.g., `TimeValue`) for domain concepts
   - Prefer compile-time checks over runtime validations

6. **Documentation**
   - Add `///` doc comments for public APIs
   - Include usage examples in doc comments
   - Update lib.rs module docs when adding new modules

### When Reading/Analyzing Code

1. **Check the roadmap first** (PHASE1_ROADMAP.md)
   - Understand current priorities
   - Known issues and planned solutions

2. **Understand the rendering pipeline**
   - WebGPU is async-first
   - Coordinate spaces: world → camera → clip → screen
   - Shader communication via uniforms and vertex buffers

3. **Scene graph hierarchy matters**
   - Transforms inherit from parent to child
   - Update order: animations → transforms → rendering

4. **Be aware of current limitations**
   - Only Circle fully works
   - Multi-object rendering is inefficient
   - No easing functions yet
   - No live preview yet

### When Making Changes

1. **Test thoroughly**
   ```bash
   cargo test                    # Run all tests
   cargo test --test integration # Integration tests
   cargo run --example simple_circle # Visual test
   ```

2. **Check for regressions**
   - Does the demo still work?
   - Are existing tests passing?
   - Does it build with `--release`?

3. **Update documentation**
   - README.md for user-facing changes
   - CLAUDE.md for architectural changes
   - Doc comments for API changes
   - PHASE1_ROADMAP.md if completing tasks

4. **Commit atomically**
   - One logical change per commit
   - Working state at each commit
   - Clear commit messages

---

## Common Tasks for AI Assistants

### Adding a New Shape

1. **Define struct** in `src/mobjects/mod.rs`:
   ```rust
   pub struct Rectangle {
       pub width: f32,
       pub height: f32,
       pub color: Color,
       pub position: Vector3,
   }
   ```

2. **Add to Renderable enum**:
   ```rust
   pub enum Renderable {
       Circle { radius: f32, color: Color },
       Rectangle { width: f32, height: f32, color: Color },
   }
   ```

3. **Generate vertices** (triangulation):
   ```rust
   impl Rectangle {
       pub fn vertices(&self) -> Vec<Vertex> {
           // 6 vertices = 2 triangles for quad
       }
   }
   ```

4. **Add renderer method** in `src/render/mod.rs`:
   ```rust
   pub fn render_rectangle(&self, rect: &Rectangle, ...) { ... }
   ```

5. **Test it**:
   ```rust
   #[test]
   fn test_rectangle_vertices() {
       let rect = Rectangle::new(2.0, 1.0, Color::RED);
       assert_eq!(rect.vertices().len(), 6);
   }
   ```

### Adding Animation Type

1. **Define animation struct** in `src/animation/`:
   ```rust
   pub struct FadeAnimation {
       pub duration: TimeValue,
       pub fade_in: bool,
   }
   ```

2. **Implement animation trait/interface**:
   ```rust
   impl Animation for FadeAnimation {
       fn update(&mut self, time: TimeValue) -> f32 {
           // Return alpha value 0.0 - 1.0
       }
   }
   ```

3. **Integrate with scene graph**:
   - Add animation instance to node
   - Update opacity in render pass

4. **Test edge cases**:
   - Time = 0.0, time = duration, time > duration

### Debugging Rendering Issues

1. **Check GPU output**:
   - Use RenderDoc or similar GPU debugger
   - Verify shader compilation
   - Check vertex buffer contents

2. **Validate transforms**:
   ```rust
   println!("Transform: {:?}", node.transform);
   println!("World pos: {:?}", node.world_position());
   ```

3. **Frame dumps**:
   - Export individual frames to PNG
   - Compare with expected output
   - Check `frames/` directory

4. **Common issues**:
   - Wrong winding order (front face culling)
   - Incorrect projection matrix
   - Transform not updated
   - Z-fighting (overlapping geometry)

---

## Performance Guidelines

### Optimization Priorities

1. **GPU efficiency** (highest impact)
   - Minimize state changes
   - Batch draw calls
   - Use instancing for repeated geometry
   - Optimize shader complexity

2. **Memory layout** (important)
   - Use SIMD-friendly structures (Vec3A instead of Vec3)
   - Cache-friendly data access patterns
   - Pre-allocate buffers

3. **Algorithmic** (case-by-case)
   - Pre-compute static geometry
   - Use spatial data structures for large scenes
   - LOD (Level of Detail) for complex shapes

### When to Optimize

- **Don't optimize prematurely** - Correct first, fast second
- **Profile first** - Measure before changing
- **Focus on bottlenecks** - 80/20 rule applies

### Benchmarking

```rust
// Add to benches/ directory (when created)
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_transform_multiply(c: &mut Criterion) {
    c.bench_function("transform_multiply", |b| {
        let t1 = Transform::identity();
        let t2 = Transform::from_translation(1.0, 2.0, 3.0);
        b.iter(|| black_box(t1.multiply(&t2)));
    });
}
```

---

## Testing Strategy

### Unit Tests

```rust
// In same file as implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_addition() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let result = v1 + v2;
        assert_eq!(result, Vector3::new(5.0, 7.0, 9.0));
    }
}
```

### Integration Tests

```rust
// In tests/ directory (when created)
use diomanim::prelude::*;

#[test]
fn test_scene_animation_pipeline() {
    let mut scene = SceneGraph::new();
    let node = scene.create_node("test".to_string());
    // Test full pipeline
}
```

### Visual Tests

- Render known scenes
- Compare output images (pixel-by-pixel or perceptual)
- Check against reference frames in `tests/fixtures/`

---

## Troubleshooting

### Build Errors

```bash
# Clean build
cargo clean
cargo build

# Update dependencies
cargo update

# Check for issues
cargo check --all-targets
```

### Runtime Errors

- **"Failed to create renderer"**: GPU/WebGPU driver issue
  - Check GPU support: `cargo run --features wgpu/vulkan`
  - Try different backend: DX12 on Windows, Metal on macOS

- **Panic in unwrap()**: Common in current codebase
  - Check PHASE1_ROADMAP.md Task 7 (Better Error Handling)
  - Replace with proper error handling

- **Animation not playing**:
  - Verify `scene.update_animations(delta_time)` called
  - Check animation duration and current time
  - Ensure transforms updated after animations

### Rendering Issues

- **Black screen**: Shader compilation failure
  - Check `src/render/shaders/*.wgsl`
  - Validate WGSL syntax

- **Wrong position**: Transform issue
  - Verify world transform calculation
  - Check parent-child hierarchy

- **Performance degradation**:
  - Profile with release build
  - Check for unnecessary allocations
  - Review GPU frame time

---

## Resources & References

### Internal Documentation
- **README.md**: User-facing project overview
- **PHASE1_ROADMAP.md**: Current development priorities
- **CONTRIBUTING.md**: Contribution guidelines
- **lib.rs**: Module documentation and API examples

### External Resources
- [Rust Book](https://doc.rust-lang.org/book/)
- [Dioxus Documentation](https://dioxuslabs.com/learn)
- [WebGPU Specification](https://www.w3.org/TR/webgpu/)
- [wgpu Tutorial](https://sotrh.github.io/learn-wgpu/)
- [glam Documentation](https://docs.rs/glam/latest/glam/)

### Manim Comparison
- [Manim Community](https://www.manim.community/)
- Understanding Manim's API helps maintain conceptual compatibility

---

## AI Assistant Best Practices

### DO ✅

- **Read PHASE1_ROADMAP.md** before suggesting major changes
- **Run `cargo fmt`** before showing code
- **Follow existing patterns** in the codebase
- **Add tests** for new functionality
- **Update documentation** when changing APIs
- **Ask clarifying questions** when requirements are ambiguous
- **Consider performance** implications
- **Use proper error handling** (Result types)
- **Write idiomatic Rust** (ownership, borrowing, lifetimes)
- **Commit frequently** with clear messages

### DON'T ❌

- **Don't add dependencies** without justification
- **Don't ignore Clippy warnings**
- **Don't use `.unwrap()`** in production code without reason
- **Don't break existing tests**
- **Don't optimize prematurely**
- **Don't add TODO comments** without tracking in roadmap
- **Don't assume GPU capabilities** - handle fallbacks
- **Don't mix concerns** - keep modules focused
- **Don't commit generated files** (see .gitignore)

### When Uncertain

1. **Check existing code** for patterns
2. **Consult PHASE1_ROADMAP.md** for planned approach
3. **Run tests** to validate assumptions
4. **Ask the user** for clarification on requirements
5. **Start simple** and iterate

---

## Version History

- **v0.1.0** (Current): Initial release, Phase 1 in progress
  - Core systems functional
  - Circle rendering works
  - Animation system operational
  - Multi-object rendering needs work

---

## Contact & Support

- **Issues**: Use GitHub Issues for bugs and feature requests
- **Discussions**: Use GitHub Discussions for questions
- **Contributing**: See CONTRIBUTING.md for guidelines

---

**Last Updated**: 2025-11-20
**Maintained by**: Diomanim Development Team
**For**: AI Assistants (Claude, etc.)
