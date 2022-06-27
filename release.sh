#!/bin/sh -ue

cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --out-dir pkg --target web --reference-types --no-typescript --omit-default-module-path \
	     target/wasm32-unknown-unknown/release/metapurse.wasm

rsync -xva index.html pkg assets pu:/var/www/unintuitive.org/s/s/tmp/metapurse/

