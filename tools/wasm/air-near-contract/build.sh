#!/bin/sh

echo ">> Building contract"

rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
