#!/bin/sh
(
  cd ../../air-interpreter;
  marine build  --release --features marine
)

mkdir -p ./dist/
cp ../../target/wasm32-wasi/release/air_interpreter_server.wasm dist/avm.wasm
cp dist/avm.wasm src/__test__/
