
FFprobe
   ↓
VideoMetadata
   ↓
VideoInfo::from_metadata()
   ↓
VideoInfo



analyze_video()
    |
    +-- get metadata from FFprobe
    |
    +-- convert metadata -> VideoInfo
    |
    +-- display/use VideoInfo
