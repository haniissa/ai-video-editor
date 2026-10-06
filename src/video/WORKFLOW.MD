
FFprobe
   ↓
VideoMetadata
   ↓
VideoInfo::from_metadata()
   ↓
VideoInfo



video.mp4
   ↓
FFprobe
   ↓
VideoMetadata
   ↓
VideoInfo
   ↓
print

we want 
video.mp4
   ↓
FFprobe
   ↓
VideoInfo
   ↓
Video Analysis
   ↓
Editor / AI pipeline





Notice the separation:
VideoMetadata → "What did FFprobe tell us?"
VideoInfo → "What does our application know about this video?" VideoAnalysis → "What can we calculate/derive from it?" analyze_video() → "How do we display it?"



as_deref() => because as_deref() turns Option<String> into Option<&str>.
