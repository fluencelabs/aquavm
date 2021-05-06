
New-Item -ItemType Directory -Force -Path ./wasm
wasm-pack build ../interpreter --no-typescript --release -d ../npm/wasm

$base64string = [Convert]::ToBase64String([IO.File]::ReadAllBytes('./npm/wasm/aquamarine_client_bg.wasm'))

$data = "// auto-generated

module.exports = `"${base64string}`"" 

$data | Out-File "./src/wasm.js"
