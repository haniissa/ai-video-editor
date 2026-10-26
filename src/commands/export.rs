use anyhow::Result;

use crate::editor::{EditDecisionList, EditSegment};
use crate::video::ffmpeg;

pub fn run(input: &str, output: &str, cuts: &[String]) -> Result<()> {
    if cuts.is_empty() || cuts.len() % 2 != 0 {
        anyhow::bail!("Provide start/end pairs, for example: 0 5 10 15");
    }
    let mut segments = Vec::new();

    for pair in cuts.chunks_exact(2) {
        let start: f64 = pair[0].parse()?;
        let end: f64 = pair[1].parse()?;

        segments.push(EditSegment { start, end });
    }
    let edit_list = EditDecisionList { segments };

    ffmpeg::export_segments(input, output, &edit_list)?;

    println!("Export completed: {output}");
    println!(
        "Total selected duration: {:.2} seconds",
        edit_list.total_duration()
    );
    Ok(())
}
pub fn run_edl(input: &str, output: &str, edl_path: &str) -> Result<()> {
    let edit_list = crate::editor::edl::load_from_file(edl_path)?;

    ffmpeg::export_segments(input, output, &edit_list)?;

    println!("Export completed: {output}");
    println!(
        "Total selected duration: {:.2} seconds",
        edit_list.total_duration()
    );
    Ok(())
}

pub fn save_edl(edl_path: &str, cuts: &[String]) -> Result<()> {
    if cuts.is_empty() || cuts.len() % 2 != 0 {
        anyhow::bail!("Provide start/end pair, for examples: 0 5 10 15");
    }

    let mut segments = Vec::new();

    for pair in cuts.chunks_exact(2) {
        let start: f64 = pair[0].parse()?;
        let end: f64 = pair[1].parse()?;

        segments.push(EditSegment { start, end });
    }
    let edit_list = EditDecisionList { segments };

    crate::editor::edl::save_to_file(edl_path, &edit_list)?;

    println!("Edit list saved: {edl_path}");
    println!(
        "Total selected duration: {:.2} seconds",
        edit_list.total_duration()
    );
    Ok(())
}
