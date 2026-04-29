# RustOhkanGame

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
