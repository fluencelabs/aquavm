#!/bin/sh

## requires wasm-pack
## > curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

(
    cd ../..;
    mkdir -p ./avm/client/wasm || exit;
    wasm-pack build ./air-interpreter --no-typescript --release -d ../avm/client/wasm
)

## base64 on MacOS doesn't have -w option
if echo | base64 -w0 > /dev/null 2>&1;
then
  BASE64=$(base64 -w0 wasm/air_interpreter_client_bg.wasm)
else
  BASE64=$(base64 wasm/air_interpreter_client_bg.wasm)
fi

cat << EOF > ./src/wasm.js
// auto-generated

module.exports = "$BASE64";
EOF
