import fs from 'fs';
import path from 'path';

if (!process.argv[0]) {
    console.log('Specify destination directory');
    process.exit(1);
}

let destPath = process.argv[0];
if (!path.isAbsolute(destPath)) {
    destPath = path.join(__dirname, destPath);
}

const wasmName = 'avm.wasm';
const packageName = '@fluencelabs/avm';

const modulePath = require.resolve(packageName);
const source = path.join(path.dirname(modulePath), wasmName);
const dest = path.join(destPath, wasmName);

console.log('ensure directory exists: ', destPath);
fs.mkdirSync(destPath, { recursive: true });

console.log('copying AVM wasm');
console.log('from: ', source);
console.log('to: ', dest);
fs.copyFileSync(source, dest);

console.log('done!');
