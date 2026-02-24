# Apperu Shell

> Other languages: [简体中文](README.md) | [English](README.en.md) | [繁體中文](README.zh-TW.md)

Apperu Shell は、Linux 向けに Apple Music Web のデスクトップ体験を提供する軽量ラッパーアプリケーションです。

公式 Web 版を基盤とし、デスクトップ統合と起動性能を強化します。

## ✨ プロジェクト目標

- Linux 上でよりネイティブに近い Apple Music デスクトップ体験を提供
- フルブラウザより軽量で制御しやすい実行環境を提供
- コールドスタート速度とバックグラウンド常駐を最適化
- 公式 Web 版との最大限の互換性を維持

## 🎯 機能

- Linux デスクトップ統合
  - MPRIS 対応
  - メディアキー制御
  - システムトレイ対応
- コールドスタート最適化
  - WebView プリウォーム
  - トレイ常駐
- デスクトップ通知
- 軽量ラッパー構成（Tauri ベース）

## 🚀 クイックスタート

### 実行環境要件

- WebKitGTK（EME 対応版）
- GStreamer
- gst-plugins-base
- gst-plugins-good
- gst-libav
- Widevine コンポーネント（DRM 用）

> システム WebView が DRM に対応していない場合、音声再生は行えず、Web コンテナとしてのみ動作します。

## 🎵 再生および実行構造について

Apperu Shell はシステムの WebKitGTK 上で動作し、独自のブラウザエンジンは内蔵していません。

再生処理はすべてシステム WebView によって行われます。

本プロジェクトは音声デコードや再生プロトコルの改変を行わず、デスクトップ用ラッパーとして機能します。

Apple Music Web は EME + Widevine DRM を使用しています。

Linux 上での再生可否は以下に依存します：

- WebKitGTK が EME 対応でビルドされているか
- Widevine が正しくインストールされているか
- 必要な GStreamer プラグインが揃っているか

DRM サポートはディストリビューションごとに異なります。
環境によっては Web コンテナとしてのみ動作する場合があります。

すべてのディストリビューションでの再生を保証するものではありません。

## 🛠 開発およびビルド

開発またはビルド時のみ必要：

- Rust
- Tauri CLI

### 開発モード

```bash
cargo tauri dev
```

### ビルド

```bash
cargo tauri build
```

## 🤝 コントリビュート

Issue や Pull Request を歓迎します。

PR 提出前に以下を確認してください：

- DRM 回避動作を追加しないこと
- 公式 Web ロジックを変更しないこと
- 最小限のラッパー設計を維持すること

## 📜 ライセンス

Mozilla Public License Version 2.0 (MPL-2.0) の下で公開されています。

詳細は LICENSE を参照してください。

## ⚠ 免責事項

Apperu Shell は Apple Inc. とは関係ありません。

Apple Music は Apple Inc. の商標およびサービスです。

本プロジェクトは Web ラッピングおよびデスクトップ統合のみを提供し、DRM の回避や改変は含みません。
