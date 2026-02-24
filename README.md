# Apperu Shell

> Other languages: [English](README.en.md) | [日本語](README.ja.md) | [繁體中文](README.zh-TW.md)

Apperu Shell 是一个为 Linux 提供 Apple Music Web 桌面体验的轻量封装应用。

基于官方网页运行，增强桌面集成与启动性能。

## ✨ 项目目标

- 为 Linux 提供更接近原生应用的 Apple Music 桌面体验
- 提供比浏览器更轻量、更可控的运行环境
- 优化冷启动速度与后台驻留体验
- 保持对官方 Web 版本的最大兼容性

## 🎯 功能特性

- Linux 桌面集成
  - MPRIS 支持
  - 媒体键控制
  - 系统托盘
- 冷启动优化
  - WebView 预热
  - 托盘保活
- 桌面通知
- 轻量封装架构（基于 Tauri）

## 🚀 快速开始

### 运行环境要求

- WebKitGTK（需支持 EME）
- GStreamer
- gst-plugins-base
- gst-plugins-good
- gst-libav
- Widevine 组件（用于 DRM）

> 如果系统 WebView 不支持 DRM，程序将仅作为网页容器运行，无法播放音频。

## 🎵 播放与运行机制说明

Apperu Shell 基于系统 WebKitGTK 运行，不内置浏览器引擎。

所有播放行为由系统 WebView 负责。

本项目不实现音频解码、不修改播放协议，仅作为桌面封装层。

Apple Music Web 使用 EME + Widevine DRM。

在 Linux 上，播放能力取决于：

- WebKitGTK 是否启用 EME
- 系统是否正确安装 Widevine
- GStreamer 插件是否完整

不同发行版对 DRM 的支持情况不同。
某些环境下可能仅能作为网页容器使用。

Apperu Shell 不对所有发行版提供播放保证。

## 🛠 开发与构建

仅在开发或构建时需要：

- Rust
- Tauri CLI

### 开发模式

```bash
cargo tauri dev
```

### 构建

```bash
cargo tauri build
```

## 🤝 贡献

欢迎提交 Issue 与 Pull Request。

在提交 PR 前，请确保：

- 不引入 DRM 绕过行为
- 不修改官方网页逻辑
- 保持最小侵入式封装原则

## 📜 许可证

本项目采用 Mozilla Public License Version 2.0 (MPL-2.0)。

详见 LICENSE。

## ⚠ 免责声明

Apperu Shell 与 Apple Inc. 无关联。

Apple Music 是 Apple Inc. 的商标与服务。

本项目仅提供网页封装与桌面集成功能，不包含任何绕过授权或修改 DRM 的实现。
