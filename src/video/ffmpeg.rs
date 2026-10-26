use anyhow::{Context, Result, bail};
use std::process::Command;

use crate::editor::EditDecisionList;

pub fn export_segments(input: &str, output: &str, edit_list: &EditDecisionList) -> Result<()> {
    edit_list.validate()?;

    if input == output {
        bail!("Input and output paths must be different");
    }
    let timeline = edit_list.build_timeline()?;

    let mut filter = String::new();

    for (index, segment) in timeline.iter().enumerate() {
        filter.push_str(&format!(
            "[0:v:0]trim=start={}:end={},setpts=PTS-STARTPTS[v{index}];",
            segment.source_start, segment.source_end,
        ));
        filter.push_str(&format!(
            "[0:a:0]atrim=start={}:end={},asetpts=PTS-STARTPTS[a{index}];",
            segment.source_start, segment.source_end,
        ));
    }
    for index in 0..timeline.len() {
        filter.push_str(&format!("[v{index}][a{index}]"));
    }
    filter.push_str(&format!("concat=n={}:v=1:a=1[outv][outa]", timeline.len()));

    let result = Command::new("ffmpeg")
        .args(["-y", "-i", input])
        .args(["-filter_complex", &filter])
        .args([
            "-map",
            "[outv]",
            "-map",
            "[outa]",
            "-c:v",
            "libx264",
            "-c:a",
            "aac",
            "-movflags",
            "+faststart",
            output,
        ])
        .output()
        .context("Failed to launch FFmpeg")?;

    if !result.status.success() {
        bail!(
            "FFmpeg export failed:\n{}",
            String::from_utf8_lossy(&result.stderr)
        );
    }

    Ok(())
}
