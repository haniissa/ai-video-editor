use crate::video::metadata::VideoMetadata;
use anyhow::Result;
use std::process::Command;

pub fn analyze(video: &str) -> Result<String> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "quiet",
            "-print_format",
            "json",
            "-show_format",
            "-show_streams",
            video,
        ])
        .output()?;
    let json = String::from_utf8(output.stdout)?;
    Ok(json)
}

pub fn get_metadata(video: &str) -> Result<VideoMetadata> {
    let json = analyze(video)?;
    println!("json: {json}");

    let metadata: VideoMetadata = serde_json::from_str(&json)?;
    // println!("metadata: {:#?}", metadata);
    Ok(metadata)
}
