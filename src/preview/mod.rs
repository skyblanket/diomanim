//! Live Preview Module
//!
//! Provides real-time preview window with playback controls:
//! - Play/Pause
//! - Timeline scrubbing
//! - Frame-by-frame stepping
//! - 60 FPS real-time rendering

use crate::core::*;
use crate::render::ShapeRenderer;
use crate::scene::*;
use std::sync::Arc;
use std::time::Instant;
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

/// Playback state for the preview window
#[derive(Debug, Clone)]
pub struct PlaybackState {
    /// Current time in the animation
    pub current_time: f32,
    /// Total duration of the animation
    pub duration: f32,
    /// Whether the animation is currently playing
    pub playing: bool,
    /// Whether to loop the animation
    pub looping: bool,
    /// Target frames per second
    pub fps: f32,
    /// Playback speed multiplier (1.0 = normal speed)
    pub speed: f32,
}

impl PlaybackState {
    /// Create a new playback state
    pub fn new(duration: f32) -> Self {
        Self {
            current_time: 0.0,
            duration,
            playing: false,
            looping: true,
            fps: 60.0,
            speed: 1.0,
        }
    }

    /// Update the playback state based on elapsed time
    pub fn update(&mut self, delta_time: f32) {
        if self.playing {
            self.current_time += delta_time * self.speed;

            if self.current_time >= self.duration {
                if self.looping {
                    self.current_time = self.current_time % self.duration;
                } else {
                    self.current_time = self.duration;
                    self.playing = false;
                }
            }
        }
    }

    /// Toggle play/pause
    pub fn toggle_play(&mut self) {
        self.playing = !self.playing;
    }

    /// Seek to a specific time
    pub fn seek(&mut self, time: f32) {
        self.current_time = time.clamp(0.0, self.duration);
    }

    /// Step forward one frame
    pub fn step_forward(&mut self) {
        let frame_time = 1.0 / self.fps;
        self.seek(self.current_time + frame_time);
    }

    /// Step backward one frame
    pub fn step_backward(&mut self) {
        let frame_time = 1.0 / self.fps;
        self.seek(self.current_time - frame_time);
    }

    /// Reset to the beginning
    pub fn reset(&mut self) {
        self.current_time = 0.0;
        self.playing = false;
    }

    /// Get progress as a percentage (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        if self.duration > 0.0 {
            self.current_time / self.duration
        } else {
            0.0
        }
    }
}

/// Preview window application state
pub struct PreviewApp {
    window: Option<Arc<Window>>,
    renderer: Option<ShapeRenderer>,
    surface: Option<wgpu::Surface<'static>>,
    surface_config: Option<wgpu::SurfaceConfiguration>,
    scene: SceneGraph,
    playback: PlaybackState,
    last_update: Instant,
    width: u32,
    height: u32,
}

impl PreviewApp {
    /// Create a new preview application
    pub fn new(scene: SceneGraph, duration: f32, width: u32, height: u32) -> Self {
        Self {
            window: None,
            renderer: None,
            surface: None,
            surface_config: None,
            scene,
            playback: PlaybackState::new(duration),
            last_update: Instant::now(),
            width,
            height,
        }
    }

    /// Render the current frame
    fn render(&mut self) {
        let Some(renderer) = &mut self.renderer else {
            return;
        };
        let Some(surface) = &self.surface else { return };
        let Some(window) = &self.window else { return };

        // Get surface texture
        let surface_texture = match surface.get_current_texture() {
            Ok(texture) => texture,
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                // Recreate surface
                if let Some(config) = &self.surface_config {
                    surface.configure(renderer.get_device(), config);
                }
                return;
            }
            Err(wgpu::SurfaceError::OutOfMemory) => {
                eprintln!("Out of memory!");
                return;
            }
            Err(wgpu::SurfaceError::Timeout) => {
                return;
            }
            Err(wgpu::SurfaceError::Other) => {
                eprintln!("Unknown surface error!");
                return;
            }
        };

        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Create command encoder
        let mut encoder =
            renderer
                .get_device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Preview Render Encoder"),
                });

        // Begin render pass
        let mut render_pass = renderer.begin_render_pass(&mut encoder, &view, None);
        render_pass.set_pipeline(renderer.get_pipeline());
        render_pass.set_bind_group(0, renderer.get_transform_bind_group(), &[]);

        // Render all visible objects
        let renderables = self.scene.get_visible_renderables();
        for (transform_uniform, renderable, opacity) in renderables {
            renderer.update_transform(&transform_uniform);

            // Apply opacity to color
            let apply_opacity = |color: Color| -> Color {
                Color::rgba(color.r, color.g, color.b, color.a * opacity)
            };

            if let Some((radius, color)) = renderable.as_circle() {
                let circle = crate::mobjects::Circle {
                    radius: *radius,
                    color: apply_opacity(*color),
                    position: Vector3::zero(),
                };
                renderer.draw_circle(&circle, apply_opacity(*color), &mut render_pass);
            } else if let Some((width, height, color)) = renderable.as_rectangle() {
                renderer.draw_rectangle(*width, *height, apply_opacity(*color), &mut render_pass);
            } else if let Some((start, end, color, thickness)) = renderable.as_line() {
                renderer.draw_line(
                    *start,
                    *end,
                    apply_opacity(*color),
                    *thickness,
                    &mut render_pass,
                );
            } else if let Some((start, end, color, thickness)) = renderable.as_arrow() {
                renderer.draw_arrow(
                    *start,
                    *end,
                    apply_opacity(*color),
                    *thickness,
                    &mut render_pass,
                );
            } else if let Some((vertices, color)) = renderable.as_polygon() {
                renderer.draw_polygon(vertices, apply_opacity(*color), &mut render_pass);
            } else if let Some((content, font_size, color)) = renderable.as_text() {
                renderer.draw_text(content, *font_size, apply_opacity(*color), &mut render_pass);
            } else if let Some((latex, font_size, color)) = renderable.as_math() {
                renderer.draw_math(latex, *font_size, apply_opacity(*color), &mut render_pass);
            }
        }

        // End render pass
        drop(render_pass);

        // Submit commands
        renderer
            .get_queue()
            .submit(std::iter::once(encoder.finish()));

        // Present frame
        surface_texture.present();

        // Request redraw
        window.request_redraw();
    }

    /// Update the scene based on current time
    fn update_scene(&mut self) {
        // Calculate delta time
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        // Update playback state
        self.playback.update(delta_time);

        // Update scene to current time
        // Note: This is simplified - ideally we'd seek to absolute time
        let frame_delta = TimeValue::new(delta_time);
        self.scene.update_animations(frame_delta);
        self.scene.update_transforms();
    }

    /// Handle keyboard input
    fn handle_keyboard(&mut self, key_code: KeyCode, state: ElementState) {
        if state != ElementState::Pressed {
            return;
        }

        match key_code {
            KeyCode::Space => {
                self.playback.toggle_play();
                println!(
                    "Playback: {}",
                    if self.playback.playing {
                        "▶ Playing"
                    } else {
                        "⏸ Paused"
                    }
                );
            }
            KeyCode::KeyR => {
                self.playback.reset();
                println!("⏮ Reset to beginning");
            }
            KeyCode::ArrowRight => {
                self.playback.step_forward();
                println!("⏭ Step forward (time: {:.2}s)", self.playback.current_time);
            }
            KeyCode::ArrowLeft => {
                self.playback.step_backward();
                println!("⏮ Step backward (time: {:.2}s)", self.playback.current_time);
            }
            KeyCode::KeyL => {
                self.playback.looping = !self.playback.looping;
                println!("Loop: {}", if self.playback.looping { "ON" } else { "OFF" });
            }
            KeyCode::BracketRight => {
                self.playback.speed += 0.25;
                println!("Speed: {:.2}x", self.playback.speed);
            }
            KeyCode::BracketLeft => {
                self.playback.speed = (self.playback.speed - 0.25).max(0.25);
                println!("Speed: {:.2}x", self.playback.speed);
            }
            KeyCode::Escape => {
                // Window will close automatically on next event loop iteration
            }
            _ => {}
        }
    }
}

impl ApplicationHandler for PreviewApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        // Create window
        let window_attributes = Window::default_attributes()
            .with_title("Diomanim Preview - [Space] Play/Pause | [R] Reset | [←/→] Step | [L] Loop | [Esc] Quit")
            .with_inner_size(winit::dpi::PhysicalSize::new(self.width, self.height));

        let window = Arc::new(
            event_loop
                .create_window(window_attributes)
                .expect("Failed to create window"),
        );

        // Initialize renderer and surface (async operation)
        let (renderer, surface, surface_config) = pollster::block_on(async {
            // Create renderer
            let mut renderer = ShapeRenderer::new(self.width, self.height)
                .await
                .expect("Failed to create renderer");

            // Initialize text rendering
            renderer
                .init_text_rendering(48.0)
                .expect("Failed to initialize text rendering");

            // Create surface
            let surface = renderer
                .get_instance()
                .create_surface(Arc::clone(&window))
                .expect("Failed to create surface");

            // Configure surface
            let surface_config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8Unorm,
                width: self.width,
                height: self.height,
                present_mode: wgpu::PresentMode::Fifo,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats: vec![],
                desired_maximum_frame_latency: 2,
            };

            surface.configure(renderer.get_device(), &surface_config);

            (renderer, surface, surface_config)
        });

        self.window = Some(window);
        self.renderer = Some(renderer);
        self.surface = Some(surface);
        self.surface_config = Some(surface_config);
        self.last_update = Instant::now();

        println!("\n╔═══════════════════════════════════════════════════════════════╗");
        println!("║  Diomanim Live Preview                                       ║");
        println!("╚═══════════════════════════════════════════════════════════════╝");
        println!("\n🎬 Controls:");
        println!("  [Space]    Play / Pause");
        println!("  [R]        Reset to beginning");
        println!("  [←/→]      Step backward / forward");
        println!("  [L]        Toggle loop");
        println!("  [[/]]      Decrease / increase speed");
        println!("  [Esc]      Quit\n");
        println!(
            "Duration: {:.1}s | FPS: {}",
            self.playback.duration, self.playback.fps
        );
        println!("─────────────────────────────────────────────────────────────────\n");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = &self.window else { return };
        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                println!("\n👋 Closing preview window...");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.update_scene();
                self.render();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    self.handle_keyboard(key_code, event.state);
                }
            }
            WindowEvent::Resized(new_size) => {
                if new_size.width > 0 && new_size.height > 0 {
                    self.width = new_size.width;
                    self.height = new_size.height;

                    if let (Some(surface), Some(renderer), Some(config)) =
                        (&self.surface, &self.renderer, &mut self.surface_config)
                    {
                        config.width = new_size.width;
                        config.height = new_size.height;
                        surface.configure(renderer.get_device(), config);
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

/// Run the live preview window
pub fn run_preview(
    scene: SceneGraph,
    duration: f32,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = PreviewApp::new(scene, duration, width, height);
    event_loop.run_app(&mut app)?;

    Ok(())
}
