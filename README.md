# OhkanPhone — OhkanGame スマホ版 (Rust)

2013年制作の Flash ゲーム「OhkanGame」Rust移植版（[RustOhkanGame](https://github.com/tobisako/RustOhkanGame)）を **スマートフォン対応** にしたもの。

**▶ ブラウザでプレイ: https://tobisako.github.io/OhkanPhone/** （スマホ/タブレット/PC対応）

![スクリーンショット](docs/screenshot.png)

## 概要

オリジナルは Adobe Flash（ActionScript 3）で開発・公開されたゲーム。  
本リポジトリはそれを **Rust** + [macroquad](https://github.com/not-fl3/macroquad) フレームワークで完全に作り直したもの。  
衝突判定・攻撃者アニメーション・スプライト挙動はAS3原典に準拠。

### スマホ対応の内容
- **WASM版**: モバイルブラウザで動作（GitHub Pages配信、タッチ操作・レターボックス表示・mp3音声）
- **Android APK版**: ネイティブアプリ（`cargo quad-apk build --release`、ビルド手順はCLAUDE.md参照）
- マルチタッチ対応（左右ボタン同時押し可）、任意の画面サイズ・DPIに自動スケール

## ゲームルール

- キャラクターを左右に動かして、落下する王冠を頭に乗せる
- 王冠を10個スタックするとゲームクリア
- 巨大こけし・自由の女神がスタック王冠を破壊しにくる
- 小こけしに当たるとライフ減少（3回でゲームオーバー）
- 制限時間90秒

## 操作方法

| 入力 | 動作 |
|------|------|
| ← / A | 左移動 |
| → / D | 右移動 |
| 画面下のボタン | 左右移動（マウス・タッチ） |
| Space / Enter | ゲーム開始（タイトル画面） |

## 技術スタック

- **言語**: Rust（100%）
- **フレームワーク**: [macroquad](https://github.com/not-fl3/macroquad) 0.4
- **対応プラットフォーム**: Webブラウザ（WASM / スマホ・タブレット・PC）、Android（APK）、macOS（arm64 / x86_64）
- **ビルドツール**: Cargo / cargo-quad-apk

## ビルド方法

```bash
cargo run
```

### macOS .appバンドル（arm64）ビルド

```bash
SDK=/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk
CLANG=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/clang
SDKROOT="$SDK" \
CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER="$CLANG" \
RUSTFLAGS="-C link-arg=-isysroot -C link-arg=$SDK -C link-arg=-mmacosx-version-min=13.0" \
cargo build --release --target aarch64-apple-darwin
cp target/aarch64-apple-darwin/release/RustOhkanGame RustOhkanGame.app/Contents/MacOS/RustOhkanGame
touch RustOhkanGame.app
```

## オリジナル

- 元リポジトリ: [TobiSyazaiFlash](https://github.com/tobisako/TobiSyazaiFlash)
- 元言語: ActionScript 3（Adobe Flash）
