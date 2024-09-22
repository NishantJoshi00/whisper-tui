# Liz - Real-time Speech-to-Text Transcription Tool

Liz is a Rust-based application that provides real-time speech-to-text transcription using the Whisper model. It features a terminal user interface for easy interaction and control.

## Features

- Real-time audio capture and transcription
- Terminal-based user interface using Ratatui
- Text copying to clipboard
- Start/stop functionality for audio capture
- Notifications for user actions

## Prerequisites

- Rust programming environment
- Whisper model file (see Installation section)
- Make (for using the provided Makefile)

## Installation

1. Clone the repository:
   ```
   git clone <repository-url>
   cd liz
   ```

2. Download the Whisper model:
   You can use the provided Makefile to download the model:
   ```
   make models/ggml-base.en.bin
   ```
   This will download the base English model. If you want a different model, you'll need to modify the Makefile.

3. Build the project:
   ```
   make build
   ```
   This will compile the project in release mode.

## Usage

You can run the application using the Makefile:

```
make run
```

This will run the application with the base English model, redirecting stderr to /dev/null.

Alternatively, you can run the application manually by providing the path to the Whisper model:

```
./target/release/liz ./models/ggml-base.en.bin
```

### Controls

- Space: Start/Stop audio capture and transcription
- C: Copy transcribed text to clipboard
- Q or Ctrl+C: Quit the application

## Dependencies

- `cpal`: Audio capture
- `whisper-rs`: Rust bindings for Whisper
- `ratatui`: Terminal user interface
- `crossterm`: Terminal manipulation
- `anyhow`: Error handling
- `arboard`: Clipboard operations

## Makefile Targets

- `make run`: Builds (if necessary) and runs the application
- `make build`: Compiles the project in release mode

