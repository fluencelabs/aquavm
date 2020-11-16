const fs = require('fs')

// read wasm as base64
let wasmFile = fs.readFileSync('./pkg/aquamarine_client_bg.wasm', {encoding: 'base64'})

let name = 'aquamarine.base64.js'

// create js file with wasm
let str = `const WASM = '${wasmFile}'`
fs.writeFileSync(`./pkg/${name}`, str)

// edit generated package.json
let packageFile = fs.readFileSync('./pkg/package.json', 'utf8')
let json = JSON.parse(packageFile)
json.files = [name]
json.name = 'aquamarine-base64'

// rewrite package.json
fs.writeFileSync('./pkg/package.json', JSON.stringify(json))
