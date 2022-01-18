
New-Item -ItemType Directory -Force -Path ./wasm
marine build  --release --features marine

New-Item -ItemType Directory -Force -Path ./dist
cp ../../target/wasm32-wasi/release/air_interpreter_server.wasm dist/avm.wasm
cp dist/avm.wasm src/__test__/
