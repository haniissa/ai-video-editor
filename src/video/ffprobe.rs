use crate::video::metadata::VideoMetadata;
use anyhow::{Context, Result, bail};
use std::process::Command;

pub fn analyze(video: &str) -> Result<String> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-print_format",
            "json",
            "-show_format",
            "-show_streams",
            video,
        ])
        .output()
        .context("Failed to launch FFprobe. Is it installed?")?;
    if !output.status.success() {
        bail!(
            "FFprobe failed for  '{}': {}",
            video,
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let json = String::from_utf8(output.stdout).context("FFprobe returned invalid UTF-8 output")?;
    Ok(json)
}

pub fn get_metadata(video: &str) -> Result<VideoMetadata> {
    let json = analyze(video)?;

    let metadata: VideoMetadata =
        serde_json::from_str(&json).context("Failed to parse video metadata")?;

    // println!("metadata: {:#?}", metadata);
    Ok(metadata)
}
