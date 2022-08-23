#!/bin/bash

cargo build --release --target="wasm32-unknown-unknown"

wasm-bindgen \
    --target web \
    --out-dir static \
    ./target/wasm32-unknown-unknown/release/wasm_cryptokey.wasm
