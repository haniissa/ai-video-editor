
## Now the relationship is:
Rust
 |
 |
ffprobe
 |
 |
JSON
 |
 |
serde_json
 |
 |
VideoMetadata struct

## to 
commands/analyze.rs
        │
        ▼
video/mod.rs
        │
        ▼
video/ffprobe.rs
        │
        ▼
     ffprobe
        │
        ▼
   JSON String
        │
        ▼
video/metadata.rs
        │
        ▼
 VideoMetadata


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


program is now successfully doing all of these:

Running ffprobe from Rust.
Receiving JSON from FFprobe.
Deserializing JSON with serde.
Finding the video stream.
Finding the audio stream.
Reading codec, width, and height.
Parsing the duration into f64.




note for me:
using it for:

boilerplate generation
explaining compiler errors
refactoring suggestions
