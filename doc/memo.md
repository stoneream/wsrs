---
title: 開発メモ
---

# ワークスペースを分ける

クライアントとサーバー間の通信を行う際のペイロードを共有するため、`shared-types` というワークスペースを作成している。  
クライアントはブラウザを想定しておりWASMを利用する。

```bash
# プロジェクトのルートディレクトリで実行

cargo new shared-types --lib
```

サーバー/クライアント側のワークスペースには以下のように `shared-types` を依存関係に追加する。

```toml
# Cargo.toml

[dependencies.shared-types]
path = "../shared-types"
```

## 参考

- [Cargoのワークスペース - The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/ch14-03-cargo-workspaces.html)

# wasm-bindgenとwasm-packについて

wasm-bindgen RustのコードをJavaScriptから呼び出せるようにバインディングを生成するクレート。また、DOM APIをRustから利用できたりもする。
wasm-pack WASMプロジェクトをパッケージングするツール、wasmファイル、JSラッパー、型定義ファイルなどを生成できたりする。

## プロジェクトの初期化

`wasm-pack new` コマンドでよしなに初期化してくれる。

```bash
# リポジトリのルートディレクトリで実行
wasm-pack new client
```

フロントエンド側はSvelteKitを使うことにした。

```bash
# リポジトリのルートディレクトリで実行

npx sv create frontend

cd frontend

npm install 
```

WASMをフロントエンドに組み込むために、`wasm-pack build` コマンドを実行し、生成物をフロントエンドの `src/pkg` に移動

```bash
cd client

wasm-pack build --target web --out-dir ../frontend/src/pkg

cd ../frontend

# WASMをimportするために依存に追加
npm install --save src/pkg
```

## ビルドフローを組む

tbd

## 参考

- [Introduction - The `wasm-bindgen` Guide](https://rustwasm.github.io/docs/wasm-bindgen/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)
