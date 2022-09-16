#! /usr/bin/env bash

# if running in CI avm.wasm has been built already
if [[ -z $GITHUB_SHA ]] && [[ -f dist/avm.wasm ]]; then
  echo "air-interpreter wasm binary already present"
  exit 0
else
  (
    cd ../../air-interpreter
    marine build --release --features marine
  )
  mkdir -p dist/
  cp ../../target/wasm32-wasi/release/air_interpreter_server.wasm dist/avm.wasm
fi
