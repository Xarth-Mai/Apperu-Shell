# Apperu Shell

Apperu Shell は Apple Music Web をベースにした Linux デスクトップ向けシェルクライアントです。  
本プロジェクトは、公式サービスの改変・リバースエンジニアリング・DRM 回避を行わずに、Linux でのデスクトップ統合体験とパフォーマンス最適化を提供することを目的としています。

> 他言語: [简体中文](README.md) | [English](README.en.md) | [繁體中文](README.zh-TW.md)

## プロジェクトの位置づけ

Apperu Shell は独立したプレーヤーではなく、音声デコードやプロトコルのリバースエンジニアリングは実装しません。

<https://music.apple.com> の Web 版を基盤として動作し、次を提供します。

- Linux デスクトップ統合（MPRIS / メディアキー / トレイ）
- コールドスタート体感の最適化（プリウォーム / トレイ常駐）
- デスクトップ通知
- 軽量ラッピング（Tauri 優先）

## プロジェクト目標

- Apple Music Web の安定した実行コンテナを提供
- 完全な MPRIS 対応（GNOME / KDE で認識）
- キーボードのメディアコントロール対応
- トレイ常駐と高速復帰
- DRM 回避や音声抽出機能は実装しない

## 技術スタック（予定）

- Tauri v2
- Rust
- WebKitGTK（Linux WebView）
- zbus（DBus / MPRIS）
- Tauri プラグイン（トレイ / 通知 / ショートカット）

## パフォーマンス戦略

- 永続化 WebView プロファイル（キャッシュと Cookie）
- 起動プリウォーム（表示前に WebView を初期化）
- 既定でトレイ常駐（頻繁なコールドスタートを回避）
- 状態同期のスロットリング（高頻度ポーリングを回避）

独自の静的アセットのオフラインキャッシュは実装せず、WebView エンジンのキャッシュ機構に依存します。

## DRM と再生について

Apple Music Web は DRM（EME/Widevine）を使用します。  
再生可否はシステムの WebKitGTK と関連コンポーネントの対応状況に依存します。

Apperu Shell は、すべての Linux ディストリビューションでの音声再生を保証しません。  
システム WebView が DRM に対応していない場合、本アプリは Web コンテナとしてのみ動作します。

## ビルド（予定）

```bash
# Rust と Tauri CLI が必要
cargo tauri dev
```

パッケージ化:

```bash
cargo tauri build
```

予定している配布形式:

- AppImage
- AUR パッケージ（後日）

## 開発フェーズ

- WebView コンテナとログイン永続化
- 再生状態プローブ
- JS 注入と状態ブリッジ
- MPRIS 実装
- メディアキー対応
- トレイと通知
- 起動プリウォームと常駐最適化

## ライセンス

本プロジェクトは Mozilla Public License Version 2.0（MPL-2.0）の下で公開されています。

詳細は [LICENSE](LICENSE) を参照してください。

---

## 免責事項

Apperu Shell は Apple Inc. とは無関係です。  
Apple Music は Apple Inc. の商標およびサービスです。  
本プロジェクトは Web ラッパーツールとしてのみ提供され、権利侵害や認可回避機能は提供しません。
