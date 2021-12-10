#!/bin/sh
(
  cd ../../air-interpreter;
  marine build  --release --features marine
)

## base64 on MacOS doesn't have -w option
if echo | base64 -w0 > /dev/null 2>&1;
then
  BASE64=$(base64 -w0 ../../target/wasm32-wasi/release/air_interpreter_server.wasm)
else
  BASE64=$(base64 ../../target/wasm32-wasi/release/air_interpreter_server.wasm)
fi

cat << EOF > ./src/wasm.js
// auto-generated

module.exports = "$BASE64";
EOF
