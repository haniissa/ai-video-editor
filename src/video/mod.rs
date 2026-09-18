pub mod ffprobe;
pub mod metadata;

use std::time::Duration;

use anyhow::Result;

pub fn analyze_video(path: &str) -> Result<()> {
    let metadata = ffprobe::get_metadata(path)?;
    println!("analyze_vide():\n{:#?}", metadata);

    if let Some(video) = metadata.video_stream() {
        println!("Video codec: {:?}", video.codec_name);
        println!("Width: {:?}", video.width);
        println!("Height: {:?}", video.height);
    }
    if let Some(audio) = metadata.audio_stream() {
        println!("Audio codec: {:?}", audio.codec_name);
    }

    if let Some(format) = &metadata.format {
        if let Some(duration) = format.duration_seconds() {
            println!("duration: {}", duration);
        }
    }

    Ok(())
}
