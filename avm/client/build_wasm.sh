#!/bin/sh

## requires wasm-pack
## > curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

(
    cd ../..;
    mkdir -p ./avm/client/wasm || exit;
    wasm-pack build ./air-interpreter --no-typescript --release -d ../avm/client/wasm
)

cat << EOF > ./src/wasm.js
// auto-generated

module.exports = "$(base64 -w0 wasm/air_interpreter_client_bg.wasm)";
EOF

__wbg_callserviceimpl=$(cat wasm/air_interpreter_client_bg.js | grep -o __wbg_callserviceimpl\w+)
__wbg_getcurrentpeeridimpl=$(cat wasm/air_interpreter_client_bg.js | grep -o __wbg_getcurrentpeeridimpl_\w+)

cat << EOF > ./src/importObject.ts
// auto-generated

export const __wbg_callserviceimpl = '${__wbg_callserviceimpl}';
export const __wbg_getcurrentpeeridimpl = '${__wbg_getcurrentpeeridimpl}';
EOF

cat ./src/importObject.ts