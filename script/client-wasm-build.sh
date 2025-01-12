#!/bin/bash

cd $(dirname $0)

cd ../client

wasm-pack build --target web --out-dir ../frontend/src/pkg
