use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VideoMetadata {
    pub format: Option<Format>,
    pub streams: Vec<Stream>,
}

#[derive(Debug, Deserialize)]
pub struct Format {
    pub duration: Option<String>,
    pub bit_rate: Option<String>,
    pub format_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Stream {
    pub codec_name: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub codec_type: Option<String>,
    pub sample_rate: Option<String>,
    pub avg_frame_rate: Option<String>,
    pub r_frame_rate: Option<String>,
    pub disposition: Option<Disposition>,
}

#[derive(Debug, Deserialize)]
pub struct Disposition {
    pub attached_pic: Option<u8>,
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
    pub fn video_stream_count(&self) -> usize {
        self.streams
            .iter()
            .filter(|stream| {
                stream.codec_type.as_deref() == Some("video")
                    && !stream
                        .disposition
                        .as_ref()
                        .and_then(|d| d.attached_pic)
                        .is_some_and(|value| value == 1)
            })
            .count()
    }
    pub fn audio_stream_count(&self) -> usize {
        self.streams
            .iter()
            .filter(|s| s.codec_type.as_ref().is_some_and(|f| f == "audio"))
            .count()
    }
    pub fn subtitle_stream_count(&self) -> usize {
        self.streams
            .iter()
            .filter(|s| s.codec_type.as_ref().is_some_and(|f| f == "subtitle"))
            .count()
    }
    pub fn has_attached_picture(&self) -> bool {
        self.streams.iter().any(|stream| {
            stream
                .disposition
                .as_ref()
                .and_then(|d| d.attached_pic)
                .is_some_and(|flag| flag == 1)
        })
    }
}

impl Format {
    pub fn duration_seconds(&self) -> Option<f64> {
        let duration = self.duration.as_ref();
        duration.map(|s| s.parse::<f64>().ok())?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_video_stream_count() {
        let meta = VideoMetadata {
            format: None,
            streams: vec![Stream {
                codec_name: Some("h255".to_string()),
                width: Some(1233),
                height: Some(820),
                codec_type: Some("video".to_string()),
                sample_rate: Some("45000".to_string()),
                avg_frame_rate: Some("0/0".to_string()),
                r_frame_rate: Some("0/0".to_string()),
                disposition: None,
            }],
        };

        assert_eq!(meta.video_stream_count(), 1);
        assert_eq!(meta.audio_stream_count(), 0);
        assert_eq!(meta.subtitle_stream_count(), 0);
    }
    #[test]
    fn test_audio_stream_with_multiple_count() {
        let meta2 = VideoMetadata {
            format: None,
            streams: vec![
                Stream {
                    codec_name: Some("h265".to_string()),
                    width: Some(1233),
                    height: Some(820),
                    codec_type: Some("audio".to_string()),
                    sample_rate: Some("45000".to_string()),
                    avg_frame_rate: Some("0/0".to_string()),
                    r_frame_rate: Some("0/0".to_string()),
                    disposition: None,
                },
                Stream {
                    codec_name: Some("h232423".to_string()),
                    width: Some(1233),
                    height: Some(820),
                    codec_type: Some("audio".to_string()),
                    sample_rate: Some("452230".to_string()),
                    avg_frame_rate: Some("1/0".to_string()),
                    r_frame_rate: Some("25/0".to_string()),
                    disposition: None,
                },
                Stream {
                    codec_name: Some("h23423456".to_string()),
                    width: Some(1233),
                    height: Some(820),
                    codec_type: Some("video".to_string()),
                    sample_rate: Some("452230".to_string()),
                    avg_frame_rate: Some("5324/0".to_string()),
                    r_frame_rate: Some("25/0".to_string()),
                    disposition: None,
                },
                Stream {
                    codec_name: Some("hs2345254902345".to_string()),
                    width: Some(1233),
                    height: Some(820),
                    codec_type: Some("subtitle".to_string()),
                    sample_rate: Some("452230".to_string()),
                    avg_frame_rate: Some("5324/0".to_string()),
                    r_frame_rate: Some("25/0".to_string()),
                    disposition: None,
                },
            ],
        };
        assert_eq!(meta2.video_stream_count(), 1);
        assert_eq!(meta2.audio_stream_count(), 2);
        assert_eq!(meta2.subtitle_stream_count(), 1);
    }
    #[test]
    fn test_subtitl_stream_count() {
        let meta3 = VideoMetadata {
            format: None,
            streams: vec![Stream {
                codec_name: Some("h265".to_string()),
                width: Some(1233),
                height: Some(820),
                codec_type: Some("subtitle".to_string()),
                sample_rate: Some("45000".to_string()),
                avg_frame_rate: Some("0/0".to_string()),
                r_frame_rate: Some("0/0".to_string()),
                disposition: None,
            }],
        };

        assert_eq!(meta3.video_stream_count(), 0);
        assert_eq!(meta3.audio_stream_count(), 0);
        assert_eq!(meta3.subtitle_stream_count(), 1);
    }

    #[test]
    fn test_empty_stream_list() {
        let meta = VideoMetadata {
            format: None,
            streams: vec![],
        };

        assert_eq!(meta.video_stream_count(), 0);
        assert_eq!(meta.audio_stream_count(), 0);
        assert_eq!(meta.subtitle_stream_count(), 0);
    }
    #[test]
    fn test_has_attached_picture() {
        let meta = VideoMetadata {
            format: None,
            streams: vec![Stream {
                codec_name: Some("h255".to_string()),
                width: Some(1233),
                height: Some(820),
                codec_type: Some("video".to_string()),
                sample_rate: Some("45000".to_string()),
                avg_frame_rate: Some("0/0".to_string()),
                r_frame_rate: Some("0/0".to_string()),
                disposition: None,
            }],
        };
        let meta2 = VideoMetadata {
            format: None,
            streams: vec![Stream {
                codec_name: Some("h255".to_string()),
                width: Some(1233),
                height: Some(820),
                codec_type: Some("video".to_string()),
                sample_rate: Some("45000".to_string()),
                avg_frame_rate: Some("0/0".to_string()),
                r_frame_rate: Some("0/0".to_string()),
                disposition: Some(Disposition {
                    attached_pic: Some(1),
                }),
            }],
        };

        let meta3 = VideoMetadata {
            format: None,
            streams: vec![Stream {
                codec_name: Some("h255".to_string()),
                width: Some(1233),
                height: Some(820),
                codec_type: Some("video".to_string()),
                sample_rate: Some("45000".to_string()),
                avg_frame_rate: Some("0/0".to_string()),
                r_frame_rate: Some("0/0".to_string()),
                disposition: Some(Disposition {
                    attached_pic: Some(0),
                }),
            }],
        };

        assert!(!meta.has_attached_picture());
        assert!(meta2.has_attached_picture());
        assert!(!meta3.has_attached_picture());
    }
    #[test]
    fn test_video_stream_count2() {
        let meta = VideoMetadata {
            format: None,
            streams: vec![Stream {
                codec_name: Some("ahn".to_string()),
                width: Some(1233),
                height: Some(820),
                codec_type: Some("video".to_string()),
                sample_rate: Some("45000".to_string()),
                avg_frame_rate: Some("0/0".to_string()),
                r_frame_rate: Some("0/0".to_string()),
                disposition: Some(Disposition {
                    attached_pic: Some(1),
                }),
            }],
        };
        let meta2 = VideoMetadata {
            format: None,
            streams: vec![Stream {
                codec_name: Some("ahn".to_string()),
                width: Some(1233),
                height: Some(820),
                codec_type: Some("video".to_string()),
                sample_rate: Some("45000".to_string()),
                avg_frame_rate: Some("0/0".to_string()),
                r_frame_rate: Some("0/0".to_string()),
                disposition: Some(Disposition {
                    attached_pic: Some(0),
                }),
            }],
        };
        assert_eq!(meta.video_stream_count(), 0); // attached picture
        assert_eq!(meta2.video_stream_count(), 1); // normal video
    }
}

//note: ! with int => bitwise NOT
