use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::editor::project::EditorProject;

pub fn save_project(path: &Path, project: &EditorProject) -> Result<()> {
    project.validate()?;

    let json = serde_json::to_string_pretty(project).context("Failed to serialize project")?;

    fs::write(path, json)
        .with_context(|| format!("Failed to write project file: {}", path.display()))?;

    Ok(())
}

pub fn load_project(path: &Path) -> Result<EditorProject> {
    let json = fs::read_to_string(path)
        .with_context(|| format!("Failed to read project file: {}", path.display()))?;

    let project: EditorProject =
        serde_json::from_str(&json).context("Failed to parse project file")?;

    project.validate()?;

    Ok(project)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::project::{Clip, MediaAsset, Track, TrackKind};

    fn create_test_project() -> EditorProject {
        EditorProject {
            id: "project1".to_string(),
            name: "test".to_string(),
            media: vec![MediaAsset {
                id: "first-bun".to_string(),
                name: "bu.mp4".to_string(),
                path: "bu.mp4".to_string(),
                duration_seconds: 80.0,
            }],
            tracks: vec![Track {
                id: "track-1".to_string(),
                name: "track".to_string(),
                kind: TrackKind::Video,
                clips: vec![Clip {
                    id: "clip-1".to_string(),
                    media_asset_id: "first-bun".to_string(),
                    timeline_start: 0.0,
                    source_start: 5.0,
                    source_end: 15.0,
                }],
            }],
        }
    }

    #[test]
    fn save_and_load_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("ai_video_editor_test.json");

        let original = create_test_project();

        //save project
        save_project(&path, &original).expect("Failed to save project");

        //Verify file exists
        assert!(path.exists());

        //load project
        let loaded = load_project(&path).expect("Failed to load project");

        //Verify project data
        assert_eq!(loaded.id, original.id);
        assert_eq!(loaded.name, original.name);
        assert_eq!(loaded.media.len(), original.media.len());
        assert_eq!(loaded.tracks.len(), original.tracks.len());

        assert_eq!(
            loaded.tracks[0].clips[0].source_start,
            original.tracks[0].clips[0].source_start
        );
    }

    #[test]
    fn load_missing_project_returns_errors() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("missing.json");

        let result = load_project(&path);
        assert!(result.is_err());
    }

    #[test]
    fn load_invalid_json_returns_error() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("invalid.json");

        std::fs::write(&path, "{ invlaid json }").expect("Failed to write invalid JSON");

        let result = load_project(&path);
        assert!(result.is_err());
    }
    #[test]
    fn save_invalid_project_returns_error() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("invalid_project.json");

        let mut project = create_test_project();

        //An empty project name violates our validation rules
        project.name = "  ".to_string();

        let result = save_project(&path, &project);

        assert!(result.is_err());

        //Invalid project should not be written.
        assert!(!path.exists());
    }
    #[test]
    fn load_project_with_invalid_data_returns_error() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("invalid_project.json");

        let mut project = create_test_project();
        project.name.clear();

        //Write JSON directly, bypassing save_project validation
        let json = serde_json::to_string(&project).unwrap();

        std::fs::write(&path, json).unwrap();

        let result = load_project(&path);

        assert!(result.is_err());
    }

    #[test]
    fn timeline_duration_returns_latest_clip_end() {
        let mut project = create_test_project();

        //Second clip on another track:
        //Starts at 4 seconds and lasts 30 seconds
        project.tracks.push(Track {
            id: "track-2".to_string(),
            name: "Second video track".to_string(),
            kind: TrackKind::Video,
            clips: vec![Clip {
                id: "clip-3".to_string(),
                media_asset_id: "first-bun".to_string(),
                timeline_start: 4.0,
                source_start: 0.0,
                source_end: 30.0,
            }],
        });
        assert_eq!(project.timeline_duration(), 34.0);
    }
    #[test]
    fn empty_timeline_duration_is_zero() {
        let mut project = create_test_project();

        project.tracks.clear();
        assert_eq!(project.timeline_duration(), 0.0);
    }
}
