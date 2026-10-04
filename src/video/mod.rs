pub mod ffprobe;
pub mod info;
pub mod metadata;

use anyhow::Result;
use info::VideoInfo; //bring videoInfo in this scope

pub fn analyze_video(path: &str) -> Result<()> {
    let metadata = ffprobe::get_metadata(path)?;
    let info = VideoInfo::from_metadata(&metadata)?;

    // println!("analyze_video():\n{:#?}", metadata);
    println!("{:#?}", info);

    Ok(())
}
