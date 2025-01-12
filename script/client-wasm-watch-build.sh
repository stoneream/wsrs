#!/bin/bash

cd $(dirname $0)

cd ../client

cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build --target web --out-dir ../frontend/src/pkg"
