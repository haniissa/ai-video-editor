use anyhow::Result;

use crate::ffprobe;

pub fn analyze_video(path: &str) -> Result<()>{
    let data = ffprobe::analyze(path)?;

    println!("{}", data);

    Ok(())

}