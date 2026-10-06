use crate::video::metadata::VideoMetadata;

#[derive(Debug)]
pub struct VideoInfo {
    pub duration: f64,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub has_audio: bool,
    pub sample_rate: Option<u32>,
    pub frame_rate: Option<f64>,
    pub bit_rate: Option<u64>,
    pub format: Option<String>,
    pub is_constant_frame_rate: bool,
}
fn parse_frame_rate(rate: &str) -> Option<f64> {
    let (numerator, denominator) = rate.split_once('/')?;

    let numerator = numerator.parse::<f64>().ok()?;
    let denominator = denominator.parse::<f64>().ok()?;

    if denominator == 0.0 {
        return None;
    }

    Some(numerator / denominator)
}

fn is_constant_frame_rate(r_frame_rate: Option<f64>, avg_frame_rate: Option<f64>) -> bool {
    match (r_frame_rate, avg_frame_rate) {
        (Some(r), Some(avg)) => (r - avg).abs() < 0.01,
        _ => false,
    }
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

        let has_audio = audio.is_some();

        // let sample_rate = audio.and_then(|stream| stream.sample_rate.clone());
        let sample_rate = audio.and_then(|stream| {
            stream
                .sample_rate
                .as_ref()
                .and_then(|s| s.parse::<u32>().ok())
        });
        let video = metadata.video_stream();

        let frame_rate = video.and_then(|frame_rate| {
            frame_rate
                .avg_frame_rate
                .as_deref()
                .and_then(parse_frame_rate)
        });

        let bit_rate = metadata
            .format
            .as_ref()
            .and_then(|format| format.bit_rate.as_deref())
            .and_then(|rate| rate.parse::<u64>().ok());

        let format = metadata
            .format
            .as_ref()
            .and_then(|format| format.format_name.clone());

        let r_frame_rate =
            video.and_then(|stream| stream.r_frame_rate.as_deref().and_then(parse_frame_rate));

        let constant_frame_rate = is_constant_frame_rate(r_frame_rate, frame_rate);

        Ok(VideoInfo {
            duration, // short hand just duration like js
            width,
            height,
            audio_codec,
            video_codec,
            has_audio,
            sample_rate,
            frame_rate,
            bit_rate,
            format,
            is_constant_frame_rate: constant_frame_rate,
        })
    }
    pub fn resolution_label(&self) -> &str {
        let resolution = self.height;
        if let Some(height) = resolution {
            if height >= 2160 {
                "4K"
            } else if height >= 1080 {
                "FHD"
            } else if height >= 720 {
                "HD"
            } else {
                "SD"
            }
        } else {
            "Unknown"
        }
    }
}
/*
 *
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test] // Normal frame rates like 30/1
    fn test_parse_frame_rate() {
        assert_eq!(parse_frame_rate("30/1"), Some(30.0));
        assert_eq!(parse_frame_rate("24/1"), Some(24.0));
    }

    #[test] // Fractional frame rate like 30000/1001
    fn test_parse_fractional_frame_rate() {
        let result = parse_frame_rate("30000/1001").unwrap();
        assert!((result - 29.97002997).abs() < 0.0001);
    }
    #[test] // Invalid string return none
    fn test_parse_invalid_frame_rate() {
        assert_eq!(parse_frame_rate("invalid"), None);
        assert_eq!(parse_frame_rate("30"), None);
        assert_eq!(parse_frame_rate("abc/1"), None);
    }
    #[test] // Division by zero rejected
    fn test_parse_zero_denominator() {
        assert_eq!(parse_frame_rate("30/0"), None);
    }
    #[test]
    fn test_parse_bit_rate() {
        let bit_rate = "3364471".parse::<u64>().ok();
        assert_eq!(bit_rate, Some(3364471));
    }
    #[test]
    fn test_parse_invalid_bit_rate() {
        let bit_rate = "invalid".parse::<u64>().ok();
        assert_eq!(bit_rate, None);
    }
    #[test]
    fn test_parse_empty_bit_rate() {
        let bit_rate = "".parse::<u64>().ok();
        assert_eq!(bit_rate, None);
    }
    #[test]
    fn test_constant_frame_rate() {
        let result = is_constant_frame_rate(Some(30.0), Some(30.0));
        assert!(result);
    }
    #[test]
    fn test_variable_frame_rate() {
        let result = is_constant_frame_rate(Some(30.0), Some(30.005));

        assert!(result);
    }
    #[test]
    fn test_missing_frame_rate() {
        assert!(!is_constant_frame_rate(None, Some(30.0)));
        assert!(!is_constant_frame_rate(Some(30.0), None));
    }
}
