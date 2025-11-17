use crate::core::{Camera, Color};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;

pub trait SceneNode: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn node_type(&self) -> &'static str;
    fn update(&mut self, delta_time: f32);
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SceneState {
    None,
    Playing,
    Paused,
    Recording,
    Finished,
}

pub struct Scene {
    pub name: String,
    pub start_time: f64,
    pub duration: f64,
    pub current_time: f64,
    pub state: SceneState,
    pub camera: Camera,
    pub background_color: Color,
    pub nodes: Vec<Box<dyn SceneNode>>,
    pub metadata: HashMap<String, String>,
    pub frame_rate: f32,
    pub samples_per_frame: u32,
}

impl Scene {
    pub fn new(name: String) -> Self {
        let mut camera = Camera::new();
        camera.transform.position = crate::core::Vector3::new(0.0, 0.0, 10.0);
        camera.transform.look_at(crate::core::Vector3::zero(), crate::core::Vector3::up());

        Self {
            name,
            start_time: 0.0,
            duration: 10.0,
            current_time: 0.0,
            state: SceneState::None,
            camera,
            background_color: Color::from_hex("#1a1a1a"),
            nodes: Vec::new(),
            metadata: HashMap::new(),
            frame_rate: 60.0,
            samples_per_frame: 64,
        }
    }

    pub fn set_duration(&mut self, duration: f64) {
        self.duration = duration.max(0.0);
    }

    pub fn set_fps(&mut self, fps: f32) {
        self.frame_rate = fps.max(1.0);
    }

    pub fn add_node<T: SceneNode + 'static>(&mut self, node: T) -> usize {
        self.nodes.push(Box::new(node));
        self.nodes.len() - 1
    }

    pub fn remove_node(&mut self, index: usize) -> Option<Box<dyn SceneNode>> {
        if index < self.nodes.len() {
            Some(self.nodes.remove(index))
        } else {
            None
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        if matches!(self.state, SceneState::Playing | SceneState::Recording) {
            self.current_time += delta_time;
            
            if self.current_time >= self.duration {
                if self.state == SceneState::Recording {
                    self.state = SceneState::Finished;
                } else {
                    self.current_time = self.duration;
                    self.state = SceneState::Paused;
                }
            }

            let delta_time_f32 = delta_time as f32;
            for node in &mut self.nodes {
                node.update(delta_time_f32);
            }
        }
    }

    pub fn play(&mut self) {
        self.state = SceneState::Playing;
    }

    pub fn pause(&mut self) {
        self.state = SceneState::Paused;
    }

    pub fn record(&mut self) {
        self.state = SceneState::Recording;
    }

    pub fn stop(&mut self) {
        self.state = SceneState::None;
        self.current_time = 0.0;
    }

    pub fn seek(&mut self, time: f64) {
        self.current_time = time.clamp(0.0, self.duration);
    }

    pub fn is_playing(&self) -> bool {
        matches!(self.state, SceneState::Playing | SceneState::Recording)
    }

    pub fn progress(&self) -> f64 {
        if self.duration > 0.0 {
            (self.current_time / self.duration).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    pub fn remaining_time(&self) -> f64 {
        (self.duration - self.current_time).max(0.0)
    }

    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    pub fn frame_count(&self) -> usize {
        ((self.duration * self.frame_rate as f64).ceil() as usize) + 1
    }

    pub fn current_frame(&self) -> usize {
        (self.current_time * self.frame_rate as f64).floor() as usize
    }

    pub fn frame_time(&self, frame: usize) -> f64 {
        (frame as f64) / self.frame_rate as f64
    }

    pub fn seek_to_frame(&mut self, frame: usize) {
        let time = self.frame_time(frame);
        self.seek(time);
    }

    pub fn world_to_screen(&self, point: crate::core::Vector3, screen_width: u32, screen_height: u32) -> Option<(f32, f32)> {
        self.camera.world_to_screen_point(point, screen_width, screen_height)
    }

    pub fn screen_to_world(&self, x: f32, y: f32, screen_width: u32, screen_height: u32) -> (crate::core::Vector3, crate::core::Vector3) {
        self.camera.screen_point_to_ray(x, y, screen_width as f32, screen_height as f32)
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new("Default Scene".to_string())
    }
}

pub struct SceneGraph {
    pub scenes: Vec<Scene>,
    pub active_scene: usize,
}

impl SceneGraph {
    pub fn new() -> Self {
        Self {
            scenes: vec![Scene::default()],
            active_scene: 0,
        }
    }

    pub fn add_scene(&mut self, scene: Scene) -> usize {
        self.scenes.push(scene);
        self.scenes.len() - 1
    }

    pub fn active(&self) -> Option<&Scene> {
        self.scenes.get(self.active_scene)
    }

    pub fn active_mut(&mut self) -> Option<&mut Scene> {
        self.scenes.get_mut(self.active_scene)
    }

    pub fn set_active(&mut self, index: usize) -> bool {
        if index < self.scenes.len() {
            self.active_scene = index;
            true
        } else {
            false
        }
    }

    pub fn remove_scene(&mut self, index: usize) -> Option<Scene> {
        if index < self.scenes.len() {
            let removed = self.scenes.remove(index);
            if self.active_scene >= self.scenes.len() {
                self.active_scene = self.scenes.len().saturating_sub(1);
            }
            Some(removed)
        } else {
            None
        }
    }

    pub fn update_all(&mut self, delta_time: f64) {
        for scene in &mut self.scenes {
            scene.update(delta_time);
        }
    }

    pub fn current_time(&self) -> f64 {
        self.active().map(|s| s.current_time).unwrap_or(0.0)
    }

    pub fn current_frame(&self) -> usize {
        self.active().map(|s| s.current_frame()).unwrap_or(0)
    }

    pub fn total_duration(&self) -> f64 {
        self.active().map(|s| s.duration).unwrap_or(0.0)
    }
}

impl Default for SceneGraph {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RenderGraph {
    pub output_node: Option<String>,
    pub nodes: HashMap<String, Box<dyn RenderNode>>,
    pub connections: Vec<(String, String)>,
}

pub trait RenderNode: Send + Sync {
    fn render(&self, context: &mut RenderContext);
    fn dependencies(&self) -> &[String];
}

pub struct RenderContext<'a> {
    pub scene: &'a Scene,
    pub camera: &'a Camera,
    pub frame_time: f64,
    pub settings: &'a RenderSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderSettings {
    pub enable_shadows: bool,
    pub enable_anti_aliasing: bool,
    pub enable_post_processing: bool,
    pub max_ray_bounces: u32,
    pub samples_per_pixel: u32,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            enable_shadows: true,
            enable_anti_aliasing: true,
            enable_post_processing: true,
            max_ray_bounces: 3,
            samples_per_pixel: 64,
        }
    }
}

impl RenderGraph {
    pub fn new() -> Self {
        Self {
            output_node: None,
            nodes: HashMap::new(),
            connections: Vec::new(),
        }
    }

    pub fn add_node(&mut self, name: String, node: Box<dyn RenderNode>) {
        self.nodes.insert(name, node);
    }

    pub fn connect(&mut self, from: String, to: String) {
        self.connections.push((from, to));
    }

    pub fn set_output(&mut self, node_name: String) {
        self.output_node = Some(node_name);
    }

    pub fn render(&self, context: &mut RenderContext) {
        if let Some(output_name) = &self.output_node {
            if let Some(node) = self.nodes.get(output_name) {
                node.render(context);
            }
        }
    }
}

impl Default for RenderGraph {
    fn default() -> Self {
        Self::new()
    }
}