#!/usr/bin/env bash
# WASM (スマホブラウザ) 版をビルドして dist/ に配信用一式を組み立てる
set -euo pipefail
cd "$(dirname "$0")"

cargo build --release --target wasm32-unknown-unknown

rm -rf dist
mkdir -p dist/assets/img dist/assets/se

cp web/index.html web/mq_js_bundle.js dist/
cp target/wasm32-unknown-unknown/release/RustOhkanGame.wasm dist/

cp assets/img/*.png dist/assets/img/

# Web版が参照する音声のみ配置（mp3 + mp3素材のないcrown_hitはwav）
for f in se_setsumei se_gamebgm se_catch se_damage se_gameclear se_beamgun; do
  cp "assets/se/${f}.mp3" dist/assets/se/
done
cp assets/se/se_crown_hit.wav dist/assets/se/

echo "dist/ ready: $(du -sh dist | cut -f1)"
