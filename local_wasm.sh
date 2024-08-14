#!/bin/sh

# cargo install wasm-bindgen-cli
cargo update -p wasm-bindgen
cargo build --profile wasm-release --target wasm32-unknown-unknown --features "dev"
wasm-bindgen --no-typescript --out-name leiden --out-dir wasm --target web target/wasm32-unknown-unknown/wasm-release/leiden.wasm
echo 'export { __wbg_get_imports, __wbg_init_memory, __wbg_load, __wbg_finalize_init };' >> wasm/leiden.js
wasm-opt -Os --output wasm/leiden_bg.wasm wasm/leiden_bg.wasm
cp -r assets wasm/
