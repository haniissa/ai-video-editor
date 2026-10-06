use crate::video::analysis::VideoAnalysis;
use crate::video::ffprobe;
use crate::video::info;

use info::VideoInfo;

pub fn analyze_video(path: &str) -> anyhow::Result<()> {
    let metadata = ffprobe::get_metadata(path)?;
    let info = VideoInfo::from_metadata(&metadata)?;
    // let frame_rate = video
    // println!("info: {:#?}", info);
    let duration_minutes = info.duration / 60.0;

    let analysis = VideoAnalysis {
        video: info,
        duration_minutes,
    };
    println!("------------------------------");
    println!("Video analysis");
    println!("------------------------------");
    println!("Duration: {:.2} seconds", analysis.video.duration);
    println!("Duration: {:.2} minutes", analysis.duration_minutes);
    println!("Width:  {}", analysis.video.width.unwrap_or(0));
    println!("Height:  {}", analysis.video.height.unwrap_or(0));

    println!(
        "Video codec {}",
        analysis.video.video_codec.as_deref().unwrap_or("unknown")
    );
    println!(
        "Audio codec {}",
        analysis.video.audio_codec.as_deref().unwrap_or("none")
    );
    println!("Resolution: {}", analysis.video.resolution_label());

    println!("Has audio: {}", analysis.video.has_audio);

    println!(
        "Sample rate: {} Hz",
        analysis.video.sample_rate.unwrap_or(0)
    );
    // println!("frame rate: {}", analysis.video.frame_rate.unwrap_or(0.0));
    //Or
    match analysis.video.frame_rate {
        Some(fps) => println!("Frame rate: {fps:.2} FPS"),
        None => println!("Frame rate: Unknown"),
    }
    match analysis.video.bit_rate {
        // ffprob gives bitrate in bit/second => Mbps
        Some(rate) => println!("Bit rate: {} Mbps", rate as f64 / 1_000_000.0),
        None => println!("Bit rate: Unknown"),
    }
    match analysis.video.format.as_deref() {
        Some(format) => println!("Format: {}", format),
        None => println!("Format: Unknown"),
    }
    println!(
        "Frame rate type: {}",
        if analysis.video.is_constant_frame_rate {
            "Constant (CFR)"
        } else {
            "Variable/Unknown (VFR)"
        }
    );
    Ok(())
}
