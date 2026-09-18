Project Roadmap

We'll build this in stages:

Phase 1 — Rust Video Engine (Weeks 1-3)

Learn:

Rust basics through real code
CLI applications
file handling
processes
FFmpeg integration
JSON
error handling

Build:
video-editor/
│
├── src/
│   ├── main.rs
│   ├── video.rs
│   ├── ffmpeg.rs
│   ├── error.rs
│
└── Cargo.toml

Features:

✅ Load video
✅ Extract metadata
✅ Run FFmpeg commands
✅ Create edit operations
✅ Render edited videos

Phase 2 — AI Analysis

Add:Video
 |
 +-- Extract audio
 |
 +-- Whisper transcription
 |
 +-- Word timestamps
 |
 +-- Detect:
       - ums
       - uhs
       - pauses
       - silence

      Output:
      {
  "cuts": [
    {
      "start": 12.3,
      "end": 13.8,
      "reason": "long_pause"
    }
  ]
}

Phase 3 — AI Chat Controller

Add:
User:

"Remove all pauses longer than half a second"

          ↓

Qwen / DeepSeek

          ↓

{
 operation:"remove_pause",
 threshold:500
}

          ↓

Rust EnginePhase 4 — SaaS

Add:

Axum backend
PostgreSQL
Authentication
S3 storage
Redis workers
React frontend

here is the stack we'll use:




Why these?

Crate	Purpose
clap	command line interface
serde	JSON handling
serde_json	edit plans
anyhow	easier errors
tracing	logging

