//! # Video Export Module
//!
//! Provides functionality to export rendered PNG frames to video files (MP4/H.264)
//! using ffmpeg subprocess

use std::path::Path;
use std::process::Command;

/// Video export settings
pub struct VideoExportSettings {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub output_path: String,
    pub input_pattern: String,
}

impl VideoExportSettings {
    pub fn new(width: u32, height: u32, fps: u32, output_path: String, input_pattern: String) -> Self {
        Self {
            width,
            height,
            fps,
            output_path,
            input_pattern,
        }
    }
}

/// Export PNG frames to MP4 video using ffmpeg
///
/// # Arguments
/// * `settings` - Video export settings
///
/// # Returns
/// * `Ok(())` if export succeeded
/// * `Err` with error message if ffmpeg failed
///
/// # Example
/// ```no_run
/// use diomanim::export::{VideoExportSettings, export_video_ffmpeg};
///
/// let settings = VideoExportSettings::new(
///     1920, 1080, 30,
///     "output/video.mp4".to_string(),
///     "output/frames/frame_%04d.png".to_string()
/// );
/// export_video_ffmpeg(&settings).unwrap();
/// ```
pub fn export_video_ffmpeg(settings: &VideoExportSettings) -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Exporting Video with FFmpeg                                  ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
    println!("  Input:  {}", settings.input_pattern);
    println!("  Output: {}", settings.output_path);
    println!("  Resolution: {}x{}", settings.width, settings.height);
    println!("  FPS: {}", settings.fps);
    println!();

    // Check if ffmpeg is available
    let ffmpeg_check = Command::new("ffmpeg")
        .arg("-version")
        .output();

    if ffmpeg_check.is_err() {
        return Err("ffmpeg not found. Please install ffmpeg to export videos.".into());
    }

    // Ensure output directory exists
    if let Some(parent) = Path::new(&settings.output_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Build ffmpeg command
    // ffmpeg -framerate 30 -i frames/frame_%04d.png -c:v libx264 -pix_fmt yuv420p -crf 18 output.mp4
    let output = Command::new("ffmpeg")
        .arg("-y") // Overwrite output file without asking
        .arg("-framerate")
        .arg(settings.fps.to_string())
        .arg("-i")
        .arg(&settings.input_pattern)
        .arg("-c:v")
        .arg("libx264") // H.264 codec
        .arg("-pix_fmt")
        .arg("yuv420p") // Standard pixel format for compatibility
        .arg("-crf")
        .arg("18") // Quality (0-51, lower is better, 18 is visually lossless)
        .arg("-preset")
        .arg("slow") // Encoding speed vs compression (slow = better compression)
        .arg(&settings.output_path)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("ffmpeg failed: {}", stderr).into());
    }

    // Get output file size
    let metadata = std::fs::metadata(&settings.output_path)?;
    let file_size_mb = metadata.len() as f64 / (1024.0 * 1024.0);

    println!("✅ Video export complete!");
    println!("   Output: {}", settings.output_path);
    println!("   Size: {:.2} MB", file_size_mb);

    Ok(())
}

/// Simple helper to export frames with default pattern
///
/// Assumes frames are named: `frame_0000.png`, `frame_0001.png`, etc.
pub fn export_video(
    frames_dir: &str,
    output_path: &str,
    width: u32,
    height: u32,
    fps: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let input_pattern = format!("{}/frame_%04d.png", frames_dir);
    let settings = VideoExportSettings::new(
        width,
        height,
        fps,
        output_path.to_string(),
        input_pattern,
    );
    export_video_ffmpeg(&settings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_export_settings() {
        let settings = VideoExportSettings::new(
            1920,
            1080,
            30,
            "test.mp4".to_string(),
            "frames/frame_%04d.png".to_string(),
        );
        assert_eq!(settings.width, 1920);
        assert_eq!(settings.height, 1080);
        assert_eq!(settings.fps, 30);
        assert_eq!(settings.output_path, "test.mp4");
        assert_eq!(settings.input_pattern, "frames/frame_%04d.png");
    }
}
