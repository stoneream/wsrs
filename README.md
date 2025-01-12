# wsrs

RustでWebSocketチャットサーバーを作ってみるよ〜

## 開発

```bash

# client

## ビルド
script/client-wasm-build.sh

## 開発ビルド（ファイル監視）
script/client-wasm-watch-build.sh

# server

cargo run

cargo build

# frontend
npm run dev

npm run build

# wasm-pack
# https://crates.io/crates/wasm-pack
cargo add wasm-pack

# cargo-watch
# https://crates.io/crates/cargo-watch
cargo install cargo-watch
```
