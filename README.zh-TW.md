# Apperu Shell

Apperu Shell 是一個基於 Apple Music Web 的 Linux 桌面客戶端殼程式。  
本專案目標是在不修改官方服務、不涉及逆向或 DRM 破解的前提下，為 Linux 提供更好的桌面整合體驗與效能優化。

> 其他語言版本：[简体中文](README.md) | [English](README.en.md) | [日本語](README.ja.md)

## 專案定位

Apperu Shell 不是獨立播放器，不實作音訊解碼或協定逆向。

它基於 <https://music.apple.com> 的網頁版本運行，並提供：

- Linux 桌面整合（MPRIS / 媒體鍵 / 系統匣）
- 冷啟動體感優化（預熱 / 系統匣保活）
- 桌面通知
- 輕量化封裝（優先 Tauri）

## 專案目標

- 提供穩定的 Apple Music Web 運行容器
- 實作完整 MPRIS 支援（GNOME / KDE 可識別）
- 支援鍵盤媒體控制
- 支援系統匣駐留與快速恢復
- 不實作 DRM 繞過或音訊擷取功能

## 技術棧（規劃）

- Tauri v2
- Rust
- WebKitGTK（Linux WebView）
- zbus（DBus / MPRIS）
- Tauri 外掛（系統匣 / 通知 / 快捷鍵）

## 效能策略

- 持久化 WebView profile（快取與 cookie）
- 啟動預熱（WebView 先初始化再顯示視窗）
- 預設系統匣保活（避免頻繁冷啟動）
- 狀態同步節流（避免高頻輪詢）

不實作自訂靜態資源離線快取，依賴 WebView 核心快取機制。

## DRM 與播放說明

Apple Music Web 使用 DRM（EME/Widevine）。  
播放能力依賴系統 WebKitGTK 與相關元件支援情況。

Apperu Shell 不保證在所有 Linux 發行版環境中均可播放音訊。  
若系統 WebView 不支援 DRM，應用將僅作為網頁容器運行。

## 建置（規劃）

```bash
# 依賴 Rust 與 Tauri CLI
cargo tauri dev
```

打包：

```bash
cargo tauri build
```

計畫發布：

- AppImage
- AUR 套件（後續）

## 開發階段

- WebView 容器與登入持久化
- 播放狀態探針
- JS 注入與狀態橋接
- MPRIS 實作
- 媒體鍵支援
- 系統匣與通知
- 啟動預熱與保活優化

## 授權

本專案採用 Mozilla Public License Version 2.0（MPL-2.0）開源協議。

詳見 [LICENSE](LICENSE) 檔案。

---

## 免責聲明

Apperu Shell 與 Apple Inc. 無關聯。  
Apple Music 是 Apple Inc. 的商標與服務。  
本專案僅作為網頁封裝工具，不提供任何侵權或繞過授權的功能。
