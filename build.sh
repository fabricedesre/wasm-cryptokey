#!/bin/bash

set -e

cargo build --release --target="wasm32-unknown-unknown"

export RUSTFLAGS="-Z macro-backtrace"

wasm-bindgen \
    --target web \
    --out-dir static \
    ./target/wasm32-unknown-unknown/release/wasm_cryptokey.wasm
