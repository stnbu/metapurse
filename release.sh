#!/bin/sh -ue

# This is getting redeekulous

cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --out-dir pkg --target web --reference-types --no-typescript --omit-default-module-path \
	     target/wasm32-unknown-unknown/release/rust_web3_meets_wasm.wasm

# TODO: add a "wasm-opt" 

