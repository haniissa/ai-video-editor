use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::editor::EditDecisionList;

pub fn load_from_file(path: impl AsRef<Path>) -> Result<EditDecisionList> {
    let path = path.as_ref();

    let contents = fs::read_to_string(path)
        .with_context(|| format!("Failed to read edit list: {}", path.display()))?;

    let edit_list: EditDecisionList = serde_json::from_str(&contents)
        .with_context(|| format!("Invalid edit-list JSON: {}", path.display()))?;

    edit_list.validate()?;

    Ok(edit_list)
}

pub fn save_to_file(path: impl AsRef<Path>, edit_list: &EditDecisionList) -> Result<()> {
    edit_list.validate()?;

    let path = path.as_ref();

    let contents =
        serde_json::to_string_pretty(edit_list).context("Failed to serialize edit list")?;

    fs::write(path, contents)
        .with_context(|| format!("Failed to write edit list: {}", path.display()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::EditSegment;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_file(name: &str) -> std::path::PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        std::env::temp_dir().join(format!(
            "ai-video-editor-{name}-{}-{unique}.json",
            std::process::id()
        ))
    }
    fn sample_edl() -> EditDecisionList {
        EditDecisionList {
            segments: vec![
                EditSegment {
                    start: 0.0,
                    end: 5.0,
                },
                EditSegment {
                    start: 10.0,
                    end: 15.0,
                },
            ],
        }
    }
    #[test]
    fn save_and_load_round_trip() {
        let path = temp_file("round-trip");
        let original = sample_edl();

        save_to_file(&path, &original).unwrap();
        let loaded = load_from_file(&path).unwrap();

        assert_eq!(loaded.segments.len(), 2);
        assert_eq!(loaded.segments[0].start, 0.0);
        assert_eq!(loaded.segments[0].end, 5.0);
        assert_eq!(loaded.segments[1].start, 10.0);
        assert_eq!(loaded.segments[1].end, 15.0);

        fs::remove_file(path).unwrap();
    }
    #[test]
    fn saving_invalid_edl_fails() {
        let path = temp_file("invalid-save");
        let invalid = EditDecisionList {
            segments: vec![EditSegment {
                start: 5.0,
                end: 2.0,
            }],
        };
        assert!(save_to_file(&path, &invalid).is_err());
        assert!(!path.exists());
    }
    #[test]
    fn loading_invalid_json_fails() {
        let path = temp_file("invalid-json");
        fs::write(&path, "{ not valid json}").unwrap();

        assert!(load_from_file(&path).is_err());
        fs::remove_file(path).unwrap();
    }
    #[test]
    fn loading_missing_file_fails() {
        let path = temp_file("missing");

        assert!(load_from_file(&path).is_err());
    }
    #[test]
    fn loading_invalid_segments_fails() {
        let path = temp_file("invalid-segments");
        let json = r#"{
                "segments":[
                    {"start":5.0, "end":2.0}
                ]
            }"#;
        fs::write(&path, json).unwrap();
        assert!(load_from_file(&path).is_err());
        fs::remove_file(path).unwrap();
    }
}
