#!/bin/sh

## requires wasm-pack
## > curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

(
    cd ../..;
    mkdir -p ./avm/client/wasm || exit;
    wasm-pack build ./air-interpreter --no-typescript --release -d ../avm/client/wasm
)

## base64 on MacOS doesn't have -w option
echo | base64 -w0 > /dev/null 2>&1
if [ $? -eq 0 ]; then
  BASE64=$(base64 -w0 wasm/air_interpreter_client_bg.wasm)
else
  BASE64=$(base64 wasm/air_interpreter_client_bg.wasm)
fi

cat << EOF > ./src/wasm.js
// auto-generated

module.exports = "$BASE64";
EOF

callserviceimpl=$(cat wasm/air_interpreter_client_bg.js | grep -o '__wbg_callserviceimpl_\w*')
getcurrentpeeridimpl=$(cat wasm/air_interpreter_client_bg.js | grep -o '__wbg_getcurrentpeeridimpl_\w*')

cat << EOF > ./src/importObject.ts
// auto-generated

export const __wbg_callserviceimpl = "$callserviceimpl";
export const __wbg_getcurrentpeeridimpl = "$getcurrentpeeridimpl";
EOF

callserviceimpl=$(cat wasm/air_interpreter_client_bg.js | grep -o '__wbg_callserviceimpl_\w*')
getcurrentpeeridimpl=$(cat wasm/air_interpreter_client_bg.js | grep -o '__wbg_getcurrentpeeridimpl_\w*')

echo $callserviceimpl
echo $getcurrentpeeridimpl

cat << EOF > ./src/importObject.ts
// auto-generated

export const __wbg_callserviceimpl = "$callserviceimpl";
export const __wbg_getcurrentpeeridimpl = "$getcurrentpeeridimpl";
EOF
