use serde::{Deserialize, Serialize};
use crate::core::{CameraSettings, Color};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    pub renderer: RendererConfig,
    pub animation: AnimationConfig,
    pub ui: UIConfig,
    pub export: ExportConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RendererConfig {
    pub backend: RendererBackend,
    pub quality: QualitySettings,
    pub multi_threading: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationConfig {
    pub fps: f32,
    pub enable_caching: bool,
    pub cache_size_mb: usize,
    pub enable_motion_blur: bool,
    pub motion_blur_samples: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIConfig {
    pub theme: String,
    pub show_fps: bool,
    pub auto_save: bool,
    pub auto_save_interval: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    pub default_format: ExportFormat,
    pub video_bitrate: u32,
    pub audio_bitrate: u32,
    pub enable_compression: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum RendererBackend {
    Wgpu,
    Vulkan,
    Metal,
    OpenGL,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum QualitySettings {
    Low,
    Medium,
    High,
    Ultra,
    Custom { samples: u32 },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ExportFormat {
    MP4,
    MOV,
    WebM,
    GIF,
    PNGSequence,
    SVGSequence,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            renderer: RendererConfig::default(),
            animation: AnimationConfig::default(),
            ui: UIConfig::default(),
            export: ExportConfig::default(),
        }
    }
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            backend: RendererBackend::Wgpu,
            quality: QualitySettings::High,
            multi_threading: true,
        }
    }
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            fps: 60.0,
            enable_caching: true,
            cache_size_mb: 512,
            enable_motion_blur: false,
            motion_blur_samples: 8,
        }
    }
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            show_fps: true,
            auto_save: true,
            auto_save_interval: 60,
        }
    }
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            default_format: ExportFormat::MP4,
            video_bitrate: 10000000,
            audio_bitrate: 320000,
            enable_compression: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneConfig {
    pub background_color: Color,
    pub ambient_light: Color,
    pub camera: CameraSettings,
    pub resolution: (u32, u32),
    pub enable_shadows: bool,
    pub shadow_quality: u32,
    pub max_ray_bounces: u32,
    pub samples_per_pixel: u32,
}

impl Default for SceneConfig {
    fn default() -> Self {
        Self {
            background_color: Color::from_hex("#1a1a1a"),
            ambient_light: Color::from_rgb8(20, 20, 20),
            camera: CameraSettings::hd(),
            resolution: (1920, 1080),
            enable_shadows: true,
            shadow_quality: 1024,
            max_ray_bounces: 3,
            samples_per_pixel: 64,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigManager {
    engine: EngineConfig,
    scene: SceneConfig,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self {
            engine: EngineConfig::default(),
            scene: SceneConfig::default(),
        }
    }

    pub fn engine(&self) -> &EngineConfig {
        &self.engine
    }

    pub fn scene(&self) -> &SceneConfig {
        &self.scene
    }

    pub fn engine_mut(&mut self) -> &mut EngineConfig {
        &mut self.engine
    }

    pub fn scene_mut(&mut self) -> &mut SceneConfig {
        &mut self.scene
    }

    pub fn save_to_file(&self, path: &str) -> anyhow::Result<()> {
        let config_str = serde_json::to_string_pretty(self)?;
        std::fs::write(path, config_str)?;
        Ok(())
    }

    pub fn load_from_file(path: &str) -> anyhow::Result<Self> {
        let config_str = std::fs::read_to_string(path)?;
        let config: ConfigManager = serde_json::from_str(&config_str)?;
        Ok(config)
    }

    pub fn apply_quality_preset(&mut self, quality: QualitySettings) {
        self.engine.renderer.quality = quality;
        
        match quality {
            QualitySettings::Low => {
                self.scene.samples_per_pixel = 16;
                self.scene.shadow_quality = 256;
                self.scene.max_ray_bounces = 1;
            }
            QualitySettings::Medium => {
                self.scene.samples_per_pixel = 32;
                self.scene.shadow_quality = 512;
                self.scene.max_ray_bounces = 2;
            }
            QualitySettings::High => {
                self.scene.samples_per_pixel = 64;
                self.scene.shadow_quality = 1024;
                self.scene.max_ray_bounces = 3;
            }
            QualitySettings::Ultra => {
                self.scene.samples_per_pixel = 128;
                self.scene.shadow_quality = 2048;
                self.scene.max_ray_bounces = 4;
            }
            QualitySettings::Custom { samples } => {
                self.scene.samples_per_pixel = samples;
            }
        }
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}