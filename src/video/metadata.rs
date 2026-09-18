use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VideoMetadata {
    pub format: Option<Format>,
    pub streams: Vec<Stream>,
}

#[derive(Debug, Deserialize)]
pub struct Format {
    pub duration: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Stream {
    pub codec_name: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub codec_type: Option<String>,
}

impl VideoMetadata {
    pub fn video_stream(&self) -> Option<&Stream> {
        self.streams
            .iter()
            .find(|stream| stream.codec_type.as_deref() == Some("video"))
    }
    pub fn audio_stream(&self) -> Option<&Stream> {
        self.streams
            .iter()
            .find(|stream| stream.codec_type.as_deref() == Some("audio"))
    }
}

impl Format {
    pub fn duration_seconds(&self) -> Option<f64>{
        let duration = self.duration.as_ref();
        let dur = duration.map(|s| s.parse::<f64>().ok())?;
        dur        
    }
}
