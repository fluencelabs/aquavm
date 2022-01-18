
New-Item -ItemType Directory -Force -Path ./wasm
wasm-pack build ../../air-interpreter --no-typescript --release -d ../avm/client/wasm

New-Item -ItemType Directory -Force -Path ./dist
cp wasm/air_interpreter_client_bg.wasm dist/avm.wasm
cp dist/avm.wasm src/__test__/
