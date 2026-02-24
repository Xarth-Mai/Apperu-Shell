# Apperu Shell

> Other languages: [简体中文](README.md) | [English](README.en.md) | [日本語](README.ja.md)

Apperu Shell 是一個為 Linux 提供 Apple Music Web 桌面體驗的輕量封裝應用。

基於官方網頁版本運行，強化桌面整合與啟動效能。

## ✨ 專案目標

- 為 Linux 提供更接近原生應用的 Apple Music 桌面體驗
- 提供比瀏覽器更輕量且可控的執行環境
- 優化冷啟動速度與背景常駐體驗
- 維持對官方 Web 版本的最大相容性

## 🎯 功能特色

- Linux 桌面整合
  - 支援 MPRIS
  - 媒體鍵控制
  - 系統匣支援
- 冷啟動最佳化
  - WebView 預熱
  - 系統匣保活
- 桌面通知
- 輕量封裝架構（基於 Tauri）

## 🚀 快速開始

### 執行環境需求

- WebKitGTK（需支援 EME）
- GStreamer
- gst-plugins-base
- gst-plugins-good
- gst-libav
- Widevine 元件（用於 DRM）

> 若系統 WebView 不支援 DRM，應用程式將僅作為網頁容器運行，無法播放音訊。

## 🎵 播放與執行機制說明

Apperu Shell 基於系統 WebKitGTK 運行，並不內建瀏覽器引擎。

所有播放行為皆由系統 WebView 負責。

本專案不實作音訊解碼，也不修改播放協議，僅作為桌面封裝層。

Apple Music Web 使用 EME + Widevine DRM。

在 Linux 上，播放能力取決於：

- WebKitGTK 是否啟用 EME
- 系統是否正確安裝 Widevine
- 是否具備完整的 GStreamer 插件

不同發行版對 DRM 的支援程度不同。
在某些環境下可能僅能作為網頁容器使用。

Apperu Shell 不保證在所有發行版上皆可正常播放。

## 🛠 開發與建置

僅於開發或建置時需要：

- Rust
- Tauri CLI

### 開發模式

```bash
cargo tauri dev
```

### 建置

```bash
cargo tauri build
```

## 🤝 貢獻

歡迎提交 Issue 與 Pull Request。

提交 PR 前請確保：

- 不引入 DRM 繞過行為
- 不修改官方網頁邏輯
- 維持最小侵入式封裝原則

## 📜 授權

本專案採用 Mozilla Public License Version 2.0 (MPL-2.0)。

詳見 LICENSE。

## ⚠ 免責聲明

Apperu Shell 與 Apple Inc. 無任何關聯。

Apple Music 為 Apple Inc. 的商標與服務。

本專案僅提供網頁封裝與桌面整合功能，不包含任何 DRM 繞過或修改行為。
