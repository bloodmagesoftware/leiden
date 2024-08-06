#!/bin/sh

# cargo install wasm-bindgen-cli
mkdir -p wasm
cargo build --release --target wasm32-unknown-unknown --features "dev"
wasm-bindgen --no-typescript --out-name leiden --out-dir wasm --target web target/wasm32-unknown-unknown/release/leiden.wasm
cp -r assets wasm/
