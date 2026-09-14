use anyhow::{Ok, Result};
use std::process::Command;


pub fn analyze(video: &str)-> Result<String>{
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