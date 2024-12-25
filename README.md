# Liz - Real-time Speech Transcription TUI

Liz is a powerful Terminal User Interface (TUI) application for real-time speech-to-text transcription using the Whisper AI model. Built with Rust, it offers a seamless experience for capturing and transcribing audio with instant visual feedback.

## Description

Liz combines the power of OpenAI's Whisper model with an intuitive terminal interface to provide:
- Real-time audio transcription with visual progress indication
- Clean, organized display of transcribed text
- Easy-to-use controls for recording management
- Instant clipboard access to transcribed content
- Visual notifications for system status and actions

## Installation

### Prerequisites

1. **Rust Development Environment**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **System Dependencies**
   - For Linux:
     ```bash
     # Ubuntu/Debian
     sudo apt-get install libasound2-dev pkg-config make gcc
     
     # Fedora
     sudo dnf install alsa-lib-devel pkg-config make gcc
     ```
   - For macOS:
     ```bash
     brew install pkg-config make
     ```

### Build Instructions

1. Clone the repository
   ```bash
   git clone [repository-url]
   cd liz
   ```

2. Download the Whisper model
   ```bash
   chmod +x download-models.sh
   make models/ggml-base.en.bin
   ```

3. Build the application
   ```bash
   make build
   ```

## Usage

1. Start the application
   ```bash
   make run
   ```
   Or run directly with a specific model:
   ```bash
   ./target/release/liz ./models/ggml-base.en.bin
   ```

2. Interface Controls:
   - `Space`: Start/Stop audio recording and transcription
   - `c`: Copy all transcribed text to clipboard
   - `q` or `Ctrl+C`: Quit the application

3. Interface Elements:
   - Top panel: Displays transcribed text with current segment highlighted
   - Middle panel: Shows system notifications and status messages
   - Bottom panel: Lists available controls

## Features

- **Real-time Transcription**: Instant speech-to-text conversion using Whisper AI
- **Live Progress Display**: Visual indication of active recording and processing
- **Automatic Audio Capture**: Seamless integration with system microphone
- **Copy-to-Clipboard**: One-key access to transcribed content
- **Status Notifications**: Clear feedback for system actions and events
- **Clear Text Organization**: Timestamped text segments for easy reference

## Technical Details

- **Audio Specifications**
  - Sample Rate: 16 kHz
  - Format: 32-bit float mono / 16-bit stereo
  - Buffer Size: 1024 samples

- **Dependencies**
  - `cpal`: Audio capture system
  - `whisper-rs`: Rust bindings for Whisper
  - `ratatui`: Terminal UI framework
  - `crossterm`: Terminal control
  - `clipboard-rs`: System clipboard integration
  - Additional utilities: `anyhow`, `time`

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Troubleshooting

- **No audio input detected**: Ensure your microphone is properly connected and has system permissions
- **Build fails**: Verify all system dependencies are installed and up-to-date
- **Model download fails**: Check your internet connection and try downloading manually from the Whisper repository
