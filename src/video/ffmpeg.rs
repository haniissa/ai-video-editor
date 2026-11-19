use anyhow::{Context, Result, bail};
use std::process::Command;

use crate::editor::EditDecisionList;
use crate::video::ffprobe;

pub fn export_segments(input: &str, output: &str, edit_list: &EditDecisionList) -> Result<()> {
    edit_list.validate()?;

    if input == output {
        bail!("Input and output paths must be different");
    }

    let metadata = ffprobe::get_metadata(input)?;
    let has_audio = metadata.audio_stream_count() > 0;

    let timeline = edit_list.build_timeline()?;

    let mut filter = String::new();

    for (index, segment) in timeline.iter().enumerate() {
        filter.push_str(&format!(
            "[0:v:0]trim=start={}:end={},setpts=PTS-STARTPTS[v{index}];",
            segment.source_start, segment.source_end,
        ));
        if has_audio {
            filter.push_str(&format!(
                "[0:a:0]atrim=start={}:end={},asetpts=PTS-STARTPTS[a{index}];",
                segment.source_start, segment.source_end,
            ));
        }
    }
    for index in 0..timeline.len() {
        filter.push_str(&format!("[v{index}]"));
        if has_audio {
            filter.push_str(&format!("[a{index}]"));
        }
    }
    if has_audio {
        filter.push_str(&format!("concat=n={}:v=1:a=1[outv][outa]", timeline.len()));
    } else {
        filter.push_str(&format!("concat=n={}:v=1:a=0[outv]", timeline.len()));
    }

    let mut command = Command::new("ffmpeg");
    command.args(["-y", "-i", input]);
    command.args(["-filter_complex", &filter]);
    command.args(["-map", "[outv]"]);

    if has_audio {
        command.args(["-map", "[outa]"]);
    }

    command.args(["-c:v", "libx264"]);

    if has_audio {
        command.args(["-c:a", "aac"]);
    }

    tracing::debug!(filter = %filter, "FFmpeg filter graph");

    command.args(["-movflags", "+faststart", output]);

    let result = command.output().context("Failed to launch FFmpeg")?;

    if !result.status.success() {
        bail!(
            "FFmpeg export failed: \n{}",
            String::from_utf8_lossy(&result.stderr)
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::EditSegment;
    use anyhow::{Context, Result};
    use std::process::Command;

    fn sample_edl() -> EditDecisionList {
        EditDecisionList {
            segments: vec![
                EditSegment {
                    start: 0.0,
                    end: 5.0,
                },
                EditSegment {
                    start: 6.0,
                    end: 10.0,
                },
            ],
        }
    }
    fn create_test_video(path: &std::path::Path, with_audio: bool) -> Result<()> {
        let mut command = Command::new("ffmpeg");

        command.args([
            "-y",
            "-f",
            "lavfi",
            "-i",
            "testsrc=duration=10:size=640x360:rate=30",
        ]);
        if with_audio {
            command.args([
                "-f",
                "lavfi",
                "-i",
                "sine=frequency=1000:sample_rate=44100:duration=10",
            ]);
        }

        command.args(["-c:v", "libx264", "-pix_fmt", "yuv420p"]);

        if with_audio {
            command.args(["-c:a", "aac"]);
        }

        command.arg(path);

        let result = command
            .output()
            .context("Failed to launch FFmpeg while generating test video")?;

        if !result.status.success() {
            anyhow::bail!(
                "Failed to generate test video:\n{}",
                String::from_utf8_lossy(&result.stderr)
            );
        }
        Ok(())
    }
    fn assert_export(
        input: &std::path::Path,
        output: &std::path::Path,
        expected_audio_streams: usize,
    ) -> Result<()> {
        export_segments(
            input.to_str().context("Invalid input path")?,
            output.to_str().context("Invalid output path")?,
            &sample_edl(),
        )?;

        let metadata = ffprobe::get_metadata(output.to_str().context("Invalid output path")?)?;

        assert_eq!(
            metadata.video_stream_count(),
            1,
            "Expected exactly one video stream"
        );

        assert_eq!(
            metadata.audio_stream_count(),
            expected_audio_streams,
            "Unexpected number of audio streams",
        );

        let duration = metadata
            .format
            .as_ref()
            .and_then(|format| format.duration_seconds())
            .context("Output video has no readable duration")?;

        assert!(
            (duration - 9.0).abs() < 0.15,
            "Expected duration close to 9 seconds, got {duration}"
        );
        Ok(())
    }

    #[test]
    fn export_with_audio_preserves_both_streams() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;

        let input = temp_dir.path().join("input-with-audio.mp4");
        let output = temp_dir.path().join("output-with-audio.mp4");

        create_test_video(&input, true)?;

        assert_export(&input, &output, 1)?;

        Ok(())
    }
    #[test]
    fn export_without_audio_produces_video_only() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;

        let input = temp_dir.path().join("input-video-only.mp4");
        let output = temp_dir.path().join("output-video-only.mp4");

        create_test_video(&input, false)?;

        assert_export(&input, &output, 0)?;

        Ok(())
    }
}
