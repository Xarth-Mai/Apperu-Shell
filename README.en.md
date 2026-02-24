# Apperu Shell

> Other languages: [简体中文](README.md) | [日本語](README.ja.md) | [繁體中文](README.zh-TW.md)

Apperu Shell is a lightweight desktop wrapper that brings the Apple Music Web experience to Linux.

It runs on the official web version and enhances desktop integration and startup performance.

## ✨ Project Goals

- Provide a more native-like Apple Music desktop experience on Linux
- Offer a lighter and more controllable runtime than a full browser
- Optimize cold start speed and background residency
- Maintain maximum compatibility with the official web version

## 🎯 Features

- Linux desktop integration
  - MPRIS support
  - Media key control
  - System tray support
- Cold start optimization
  - WebView prewarming
  - Tray keep-alive
- Desktop notifications
- Lightweight wrapper architecture (based on Tauri)

## 🚀 Quick Start

### Runtime Requirements

- WebKitGTK (with EME support)
- GStreamer
- gst-plugins-base
- gst-plugins-good
- gst-libav
- Widevine component (for DRM)

> If the system WebView does not support DRM, the application will function only as a web container without audio playback.

## 🎵 Playback & Runtime Architecture

Apperu Shell runs on the system WebKitGTK and does not embed its own browser engine.

All playback behavior is handled by the system WebView.

This project does not implement audio decoding or modify playback protocols. It acts purely as a desktop wrapper.

Apple Music Web uses EME + Widevine DRM.

On Linux, playback capability depends on:

- Whether WebKitGTK is built with EME support
- Whether Widevine is correctly installed
- Whether required GStreamer plugins are available

DRM support varies across distributions.
In some environments, the application may function only as a web container.

Apperu Shell does not guarantee playback support on all distributions.

## 🛠 Development & Build

Required only for development or building:

- Rust
- Tauri CLI

### Development Mode

```bash
cargo tauri dev
```

### Build

```bash
cargo tauri build
```

## 🤝 Contributing

Issues and Pull Requests are welcome.

Before submitting a PR, please ensure:

- No DRM bypass behavior is introduced
- Official web logic is not modified
- The wrapper remains minimally invasive

## 📜 License

This project is licensed under Mozilla Public License Version 2.0 (MPL-2.0).

See LICENSE for details.

## ⚠ Disclaimer

Apperu Shell is not affiliated with Apple Inc.

Apple Music is a trademark and service of Apple Inc.

This project provides web wrapping and desktop integration only and does not include any DRM circumvention or modification.
