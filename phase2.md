## the next command 
cargo run -- analyze myvideo.mp4

### output:

Video Analysis:
File:
     myvideo.mp4
Duration:
      00:05:42
Resolution:
      1920x1080
FPS:
      30
Video Codec:
       h264
Audio:
       aac


  To do this professionally, we will use:
  rust
  |
  |
  V
  ffprobe
  |
  |
  V
  json output
  |
  |
  V
  Serde
  |
  |
  V
  Rust structs