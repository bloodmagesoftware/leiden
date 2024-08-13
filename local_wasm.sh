#!/bin/sh

# cargo install wasm-bindgen-cli
mkdir -p wasm
cargo build --profile wasm-release --target wasm32-unknown-unknown --features "dev"
wasm-bindgen --no-typescript --out-name leiden --out-dir wasm --target web target/wasm32-unknown-unknown/wasm-release/leiden.wasm
wasm-opt -Os --output wasm/leiden_bg.wasm wasm/leiden_bg.wasm
cp -r assets wasm/
