
New-Item -ItemType Directory -Force -Path ./wasm
wasm-pack build ../../air-interpreter --no-typescript --release -d ../avm/client/wasm

$base64string = [Convert]::ToBase64String([IO.File]::ReadAllBytes('./wasm/air_interpreter_client_bg.wasm'))

$data = "// auto-generated

module.exports = `"${base64string}`"" 

$data | Out-File "./src/wasm.js"

$__wbg_callserviceimpl = Get-Content wasm/air_interpreter_client_bg.js | Select-String -Pattern __wbg_callserviceimpl\w+
$__wbg_getcurrentpeeridimpl = Get-Content wasm/air_interpreter_client_bg.js | Select-String -Pattern __wbg_getcurrentpeeridimpl_\w+

$__wbg_callserviceimpl = $__wbg_callserviceimpl.matches[0].value
$__wbg_getcurrentpeeridimpl = $__wbg_getcurrentpeeridimpl.matches[0].value


$data = "// auto-generated

export const __wbg_callserviceimpl = '${__wbg_callserviceimpl}';
export const __wbg_getcurrentpeeridimpl = '${__wbg_getcurrentpeeridimpl}';"

$data | Out-File "./src/importObject.ts"
