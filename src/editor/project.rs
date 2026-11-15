use serde::{Deserialize, Serialize};

/// A video or audio file imported into the project
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaAsset {
    pub id: String,
    pub name: String,
    pub path: String,
    pub duration_seconds: f64,
}

///The type of content a track can contain
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrackKind {
    Video,
    Audio,
}

/// A clip placed on a track,
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Clip {
    pub id: String,

    ///References a MediaAsset by its ID
    pub media_asset_id: String,

    ///Position on this clip on the edited timeline
    pub timeline_start: f64,

    ///Start and end points inside the original media
    pub source_start: f64,
    pub source_end: f64,
}

impl Clip {
    ///Duration of the portion taken from the original media
    pub fn duration(&self) -> f64 {
        self.source_end - self.source_start
    }

    ///End position of this clip on the edited timeline
    pub fn timeline_end(&self) -> f64 {
        self.timeline_start + self.duration()
    }
}

///A timeline track containing clips.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub kind: TrackKind,
    pub clips: Vec<Clip>,
}

///The complete editing project
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditorProject {
    pub id: String,
    pub name: String,
    pub media: Vec<MediaAsset>,
    pub tracks: Vec<Track>,
}

impl EditorProject {
    ///Validate the project's basic structure and clip references
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.id.trim().is_empty() {
            anyhow::bail!("Project ID cannot be empty");
        }

        if self.name.trim().is_empty() {
            anyhow::bail!("Project name cannot be empty");
        }
        //Check that media IDs are unique and durations are valid
        for (index, asset) in self.media.iter().enumerate() {
            if asset.id.trim().is_empty() {
                anyhow::bail!("Media asset ID cannot be empty");
            }

            if self.media[..index].iter().any(|other| other.id == asset.id) {
                anyhow::bail!("Duplicate media asset ID: {}", asset.id);
            }

            if !asset.duration_seconds.is_finite() || asset.duration_seconds <= 0.0 {
                anyhow::bail!("Invalid duration for media asset: {}", asset.id);
            }
        }
        //Check track ID' and their clips
        for (index, track) in self.tracks.iter().enumerate() {
            if track.id.trim().is_empty() {
                anyhow::bail!("Track ID cannot be empty");
            }

            if self.tracks[..index]
                .iter()
                .any(|other| other.id == track.id)
            {
                anyhow::bail!("Duplicate track ID: {}", track.id);
            }

            for (clip_index, clip) in track.clips.iter().enumerate() {
                if clip.id.trim().is_empty() {
                    anyhow::bail!("Clip ID cannot be empty");
                }

                if track.clips[..clip_index]
                    .iter()
                    .any(|other| other.id == clip.id)
                {
                    anyhow::bail!("Duplicate clip ID: {}", clip.id);
                }
                let asset = self
                    .media
                    .iter()
                    .find(|asset| asset.id == clip.media_asset_id)
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "Clip reference unknown media asset: {}",
                            clip.media_asset_id
                        )
                    })?;
                if !clip.timeline_start.is_finite() || clip.timeline_start < 0.0 {
                    anyhow::bail!("Invalid timeline position for clip: {}", clip.id);
                }
                if !clip.source_start.is_finite()
                    || !clip.source_end.is_finite()
                    || clip.source_start < 0.0
                    || clip.source_end <= clip.source_start
                    || clip.source_end > asset.duration_seconds
                {
                    anyhow::bail!("Invalid source range for clip: {}", clip.id);
                }
            }
        }
        Ok(())
    }

    ///Calculate the total length of the edited timeline
    pub fn timeline_duration(&self) -> f64 {
        self.tracks
            .iter()
            .flat_map(|track| track.clips.iter())
            .map(Clip::timeline_end)
            .fold(0.0_f64, f64::max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_project() -> EditorProject {
        EditorProject {
            id: "project-1".into(),
            name: "My first edit".into(),
            media: vec![MediaAsset {
                id: "media-1".into(),
                name: "video.mp4".into(),
                path: "video.mp4".into(),
                duration_seconds: 120.0,
            }],
            tracks: vec![Track {
                id: "track-1".into(),
                name: "Video 1".into(),
                kind: TrackKind::Video,
                clips: vec![Clip {
                    id: "clip-1".into(),
                    media_asset_id: "media-1".into(),
                    timeline_start: 0.0,
                    source_start: 10.0,
                    source_end: 30.0,
                }],
            }],
        }
    }
    #[test]
    fn valid_project_passes_validation() {
        assert!(sample_project().validate().is_ok());
    }

    #[test]
    fn unknown_media_reference_fails_validation() {
        let mut project = sample_project();
        project.tracks[0].clips[0].media_asset_id = "missing-media".into();

        assert!(project.validate().is_err());
    }

    #[test]
    fn invalid_source_range_fails_validation() {
        let mut project = sample_project();
        project.tracks[0].clips[0].source_end = 150.0;

        assert!(project.validate().is_err());
    }

    #[test]
    fn timeline_duration_uses_latest_clip_end() {
        let project = sample_project();

        assert_eq!(project.timeline_duration(), 20.0);
    }
}
