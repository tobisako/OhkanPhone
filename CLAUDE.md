# OhkanPhone (RustOhkanGame スマホ版)

## スマホ版ビルド

### Web (WASM) 版
```bash
./build_web.sh   # wasm32ビルド + dist/ に配信一式を組み立て
python3 -m http.server 8765 --directory dist   # ローカル確認
```
- アセットパスは `set_pc_assets_folder("assets")` 前提で `img/...` `se/...` と書く（`assets/` プレフィックス禁止 — Android AssetManagerはassetsフォルダ自体がルート）
- Web版音声はmp3（iOS Safariはogg不可）、ネイティブはwav（quad-sndネイティブにmp3デコーダなし）
- `Camera2D.viewport` はglViewport直渡し=物理px。`screen_width()`/`mouse_position()`は論理px。`touches()`だけ物理px格納 — viewport.rs参照

### Android APK 版
```bash
export ANDROID_HOME=~/Library/Android/sdk
export NDK_HOME=~/Library/Android/sdk/ndk/28.2.13676358
export RUSTFLAGS="-L $NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/lib/clang/19/lib/linux/aarch64"   # libunwind用
export JAVA_HOME=~/dev/jdk8/jdk8u492-b09/Contents/Home   # quad-apkはrt.jar必須=JDK8
export PATH="$JAVA_HOME/bin:$PATH"
cargo quad-apk build --release
# 出力: target/android-artifacts/release/apk/RustOhkanGame.apk
adb install -r target/android-artifacts/release/apk/RustOhkanGame.apk
adb shell am start -n com.tobisako.RustOhkanGame/.MainActivity
```

#### APKビルドの前提（このMacに設定済み）
- `Cargo.lock` は **version = 3 を維持**（quad-apk内蔵の旧cargoがv4を読めない。cargoが4に書き換えたら3に戻す）
- NDK 28に旧binutils名のシンボリックリンク作成済み（`aarch64-linux-android-{ar,ld,readelf,strip,objcopy,objdump,nm}` → llvm-*）
- `build-tools/36.0.0/dx` = d8変換シム設置済み（dx廃止対応、シム内でJDK17使用）
- `package_name` の末尾セグメントは **クレート名と完全一致必須**（quad-apkがloadLibrary名に使う）
- `sample_count: 0`（Android時）— エミュレーターにMSAA EGL configが無くpanicするため
- エミュレーターは `-gpu swiftshader_indirect` で起動（`-gpu host` はApple SiliconでWebGL/GLES壊れる）

## .appビルド後の必須手順

バイナリを `.app` にコピーした後、**必ず** `touch` でディレクトリタイムスタンプを更新する。
Finderに表示される時刻がユーザーの確認基準。これを怠らない。

```bash
cp target/aarch64-apple-darwin/release/RustOhkanGame RustOhkanGame.app/Contents/MacOS/RustOhkanGame
touch RustOhkanGame.app
```

## リリース用DMG作成手順（毎回この手順で行う）

### 1. arm64リリースビルド
```bash
SDK=/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk
CLANG=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/clang
SDKROOT="$SDK" CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER="$CLANG" \
RUSTFLAGS="-C link-arg=-isysroot -C link-arg=$SDK -C link-arg=-mmacosx-version-min=13.0" \
cargo build --release --target aarch64-apple-darwin
```

### 2. .appバイナリ更新 + touch（Finderタイムスタンプ更新必須）
```bash
cp target/aarch64-apple-darwin/release/RustOhkanGame RustOhkanGame.app/Contents/MacOS/RustOhkanGame
touch RustOhkanGame.app
```

### 3. ad-hoc署名
```bash
codesign --force --deep -s - RustOhkanGame.app
```

### 4. create-dmg でDMG生成（`--volicon`は使わない。SetFileがxcrun破損で失敗するため）
```bash
rm -f /tmp/OhkanGame-*.dmg
create-dmg \
  --volname "RustOhkanGame" \
  --window-pos 200 120 \
  --window-size 600 380 \
  --icon-size 100 \
  --icon "RustOhkanGame.app" 180 190 \
  --hide-extension "RustOhkanGame.app" \
  --app-drop-link 420 190 \
  /tmp/OhkanGame-vX.Y.Z-macos-arm64.dmg \
  RustOhkanGame.app
```

### 5. GitHubリリースにアップロード
```bash
gh release upload vX.Y.Z /tmp/OhkanGame-vX.Y.Z-macos-arm64.dmg --clobber
```

---

## ゲーム起動方法

`cargo run` をBashツールで `run_in_background: true` で実行する。これが唯一の成功方式。

```
cargo run  # run_in_background=true で実行
```

Terminal.appを開く方式（osascript）や `cargo run &` では起動しない。
