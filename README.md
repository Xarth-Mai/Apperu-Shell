# Apperu Shell

Apperu Shell 是一个基于 Apple Music Web 的 Linux 桌面客户端壳程序。  
项目目标是在不修改官方服务、不涉及逆向或 DRM 破解的前提下，为 Linux 提供更好的桌面集成体验与性能优化。

> 其他语言版本：[English](README.en.md) | [日本語](README.ja.md) | [繁體中文](README.zh-TW.md)

## 项目定位

Apperu Shell 不是独立播放器，不实现音频解码或协议逆向。

它基于 <https://music.apple.com> 的网页版本运行，并提供：

- Linux 桌面集成（MPRIS / 媒体键 / 托盘）
- 冷启动体感优化（预热 / 托盘保活）
- 桌面通知
- 轻量化封装（优先 Tauri）

## 项目目标

- 提供稳定的 Apple Music Web 运行容器
- 实现完整 MPRIS 支持（GNOME / KDE 可识别）
- 支持键盘媒体控制
- 支持托盘驻留与快速恢复
- 不实现 DRM 绕过或音频抓取功能

## 技术栈（规划）

- Tauri v2
- Rust
- WebKitGTK（Linux WebView）
- zbus（DBus / MPRIS）
- Tauri 插件（托盘 / 通知 / 快捷键）

## 性能策略

- 持久化 WebView profile（缓存与 cookie）
- 启动预热（WebView 先初始化再显示窗口）
- 默认托盘保活（避免频繁冷启动）
- 状态同步节流（避免高频轮询）

不实现自定义静态资源离线缓存，依赖 WebView 内核缓存机制。

## DRM 与播放说明

Apple Music Web 使用 DRM（EME/Widevine）。  
播放能力依赖系统 WebKitGTK 与相关组件支持情况。

Apperu Shell 不保证在所有 Linux 发行版环境中均可播放音频。  
若系统 WebView 不支持 DRM，应用将仅作为网页容器运行。

## 构建（计划）

```bash
# 依赖 Rust 与 Tauri CLI
cargo tauri dev
```

打包：

```bash
cargo tauri build
```

计划发布：

- AppImage
- AUR 包（后续）

## 开发阶段

- WebView 容器与登录持久化
- 播放状态探针
- JS 注入与状态桥接
- MPRIS 实现
- 媒体键支持
- 托盘与通知
- 启动预热与保活优化

## 许可证

本项目采用 Mozilla Public License Version 2.0 (MPL-2.0) 开源协议。

详见 [LICENSE](LICENSE) 文件。

---

## 免责声明

Apperu Shell 与 Apple Inc. 无关联。  
Apple Music 是 Apple Inc. 的商标与服务。  
本项目仅作为网页封装工具，不提供任何侵权或绕过授权的功能。
