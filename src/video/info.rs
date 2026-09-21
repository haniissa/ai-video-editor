use anyhow::Ok;

use crate::video::metadata::VideoMetadata;

#[derive(Debug)]
pub struct VideoInfo {
    pub duration: f64,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
}

impl VideoInfo {
    pub fn from_metadata(metadata: &VideoMetadata) -> anyhow::Result<Self> {
        let duration = metadata
            .format
            .as_ref()
            .and_then(|format| format.duration_seconds())
            .ok_or_else(|| anyhow::anyhow!("Video duration is missing or invalid"))?;

        let video = metadata
            .video_stream()
            .ok_or_else(|| anyhow::anyhow!("Video stream is missing"))?;

        let width = video.width;
        let height = video.height;
        let video_codec = video.codec_name.clone();

        //We do not want to make audio mandatory , so we keep as its
        // not like video , which want an error if not exists
        let audio = metadata.audio_stream();
        let audio_codec = audio.and_then(|stream| stream.codec_name.clone());

        Ok(VideoInfo {
            duration: duration, // short hand just duration like js
            width,
            height: height,
            audio_codec,
            video_codec,
        })
    }
}
