
                 User
                  |
                  v
        cargo run -- analyze video.mp4
                  |
                  v
              main.rs
                  |
                  v
          commands/analyze.rs
                  |
                  v
            video/mod.rs
                  |
                  v
            ffprobe.rs
                  |
                  v
              FFprobe
                  |
                  v
              JSON
                  |
                  v
          metadata.rs
                  |
                  v
          VideoMetadata


program is now successfully doing all of these:

Running ffprobe from Rust.
Receiving JSON from FFprobe.
Deserializing JSON with serde.
Finding the video stream.
Finding the audio stream.
Reading codec, width, and height.
Parsing the duration into f64.


PHASE 1
──────────────────────────────

[✓] Rust project
[✓] CLI
[✓] analyze command
[✓] FFprobe integration
[✓] JSON output
[✓] Serde deserialization
[✓] VideoMetadata
[✓] Video stream detection
[✓] Audio stream detection
[✓] Duration parsing
[ ] Human-readable analysis
[ ] EditPlan
[ ] First actual edit


Phase 2
  │
  ├── FFprobe integration       ✅
  ├── Parse raw metadata        ✅
  ├── VideoMetadata model       ✅
  ├── VideoInfo model           ✅
  ├── Metadata → VideoInfo      ✅  ← COMMIT HERE
  │
  └── Human-readable analysis   ← next






note for me:
using it for:

boilerplate generation
explaining compiler errors
refactoring suggestions
