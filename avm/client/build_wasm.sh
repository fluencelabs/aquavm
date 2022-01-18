#!/bin/sh

## requires wasm-pack
## > curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

(
    cd ../..;
    mkdir -p ./avm/client/wasm || exit;
    wasm-pack build ./air-interpreter --no-typescript --release -d ../avm/client/wasm
)

mkdir -p ./dist/
cp wasm/air_interpreter_client_bg.wasm dist/avm.wasm
cp dist/avm.wasm src/__test__/
