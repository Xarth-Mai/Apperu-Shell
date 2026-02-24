# Apperu Shell

Apperu Shell is a Linux desktop shell client based on Apple Music Web.  
The project aims to provide better desktop integration and performance optimization on Linux without modifying official services, reverse engineering, or bypassing DRM.

> Other languages: [简体中文](README.md) | [日本語](README.ja.md) | [繁體中文](README.zh-TW.md)

## Project Positioning

Apperu Shell is not a standalone player and does not implement audio decoding or protocol reverse engineering.

It runs on top of the web version at <https://music.apple.com> and provides:

- Linux desktop integration (MPRIS / media keys / tray)
- Cold-start experience optimization (prewarm / tray keep-alive)
- Desktop notifications
- Lightweight packaging (Tauri-first)

## Project Goals

- Provide a stable runtime container for Apple Music Web
- Deliver complete MPRIS support (recognizable in GNOME / KDE)
- Support keyboard media controls
- Support tray residency and quick restore
- Do not implement DRM bypass or audio extraction

## Tech Stack (Planned)

- Tauri v2
- Rust
- WebKitGTK (Linux WebView)
- zbus (DBus / MPRIS)
- Tauri plugins (tray / notifications / shortcuts)

## Performance Strategy

- Persistent WebView profile (cache & cookies)
- Startup prewarm (initialize WebView before showing the window)
- Tray keep-alive by default (avoid frequent cold starts)
- State sync throttling (avoid high-frequency polling)

No custom offline cache for static assets is planned; it relies on WebView engine caching.

## DRM and Playback Notes

Apple Music Web uses DRM (EME/Widevine).  
Playback capability depends on system WebKitGTK and related components.

Apperu Shell does not guarantee audio playback on all Linux distributions.  
If the system WebView does not support DRM, the app will only function as a web container.

## Build (Planned)

```bash
# Requires Rust and Tauri CLI
cargo tauri dev
```

Package:

```bash
cargo tauri build
```

Planned releases:

- AppImage
- AUR package (later)

## Development Stages

- WebView container and login persistence
- Playback state probe
- JS injection and state bridge
- MPRIS implementation
- Media key support
- Tray and notifications
- Startup prewarm and keep-alive optimization

## License

This project is licensed under Mozilla Public License Version 2.0 (MPL-2.0).

See [LICENSE](LICENSE) for details.

---

## Disclaimer

Apperu Shell is not affiliated with Apple Inc.  
Apple Music is a trademark and service of Apple Inc.  
This project is a web wrapper tool only and does not provide any infringing or authorization-bypassing functionality.
