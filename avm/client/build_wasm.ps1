
New-Item -ItemType Directory -Force -Path ./wasm
wasm-pack build ../../air-interpreter --no-typescript --release -d ../avm/client/wasm

$base64string = [Convert]::ToBase64String([IO.File]::ReadAllBytes('./avm/client/wasm/air_interpreter_client_bg.wasm'))

$data = "// auto-generated

module.exports = `"${base64string}`"" 

$data | Out-File "./src/wasm.js"
