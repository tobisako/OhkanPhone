# RustOhkanGame

## .appビルド後の必須手順

バイナリを `.app` にコピーした後、**必ず** `touch` でディレクトリタイムスタンプを更新する。
Finderに表示される時刻がユーザーの確認基準。これを怠らない。

```bash
cp target/aarch64-apple-darwin/release/RustOhkanGame RustOhkanGame.app/Contents/MacOS/RustOhkanGame
touch RustOhkanGame.app
```

## ゲーム起動方法

`cargo run` をBashツールで `run_in_background: true` で実行する。これが唯一の成功方式。

```
cargo run  # run_in_background=true で実行
```

Terminal.appを開く方式（osascript）や `cargo run &` では起動しない。
