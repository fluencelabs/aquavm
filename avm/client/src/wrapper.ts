/*
 * Copyright 2021 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

/**
 *
 * This is generated and patched code. All functions are using local wasm as an argument for now, not a global wasm file.
 *
 */

function main(wasm) {
    wasm.main();
}

let WASM_VECTOR_LEN = 0;

let cachegetUint8Memory0 = null;
function getUint8Memory0(wasm) {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

const lTextEncoder = typeof TextEncoder === 'undefined' ? module.require('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

const encodeString =
    typeof cachedTextEncoder.encodeInto === 'function'
        ? function (arg, view) {
              return cachedTextEncoder.encodeInto(arg, view);
          }
        : function (arg, view) {
              const buf = cachedTextEncoder.encode(arg);
              view.set(buf);
              return {
                  read: arg.length,
                  written: buf.length,
              };
          };

function passStringToWasm0(wasm, arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0(wasm)
            .subarray(ptr, ptr + buf.length)
            .set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0(wasm);

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7f) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, (len = offset + arg.length * 3));
        const view = getUint8Memory0(wasm).subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;

    return ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0(wasm) {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? module.require('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

export function getStringFromWasm0(wasm, ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0(wasm).subarray(ptr, ptr + len));
}

function passArray8ToWasm0(wasm, arg, malloc) {
    const ptr = malloc(arg.length * 1);
    getUint8Memory0(wasm).set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

/**
 * @param {any} wasm
 * @param {string} init_peer_id
 * @param {string} air
 * @param {string} prev_data
 * @param {string} data
 * @param {string} log_level
 * @returns {string}
 */
export function invoke(wasm, init_peer_id, air, prev_data, data, log_level) {
    try {
        var ptr0 = passStringToWasm0(wasm, init_peer_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = passStringToWasm0(wasm, air, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        var ptr2 = passArray8ToWasm0(wasm, prev_data, wasm.__wbindgen_malloc);
        var len2 = WASM_VECTOR_LEN;
        var ptr3 = passArray8ToWasm0(wasm, data, wasm.__wbindgen_malloc);
        var len3 = WASM_VECTOR_LEN;
        var ptr4 = passStringToWasm0(wasm, log_level, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len4 = WASM_VECTOR_LEN;
        wasm.invoke(8, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3, ptr4, len4);
        var r0 = getInt32Memory0(wasm)[8 / 4 + 0];
        var r1 = getInt32Memory0(wasm)[8 / 4 + 1];
        return getStringFromWasm0(wasm, r0, r1);
    } finally {
        wasm.__wbindgen_free(r0, r1);
    }
}

export function ast(wasm, script) {
    try {
        var ptr0 = passStringToWasm0(wasm, script, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.ast(8, ptr0, len0);
        var r0 = getInt32Memory0(wasm)[8 / 4 + 0];
        var r1 = getInt32Memory0(wasm)[8 / 4 + 1];
        return getStringFromWasm0(wasm, r0, r1);
    } finally {
        wasm.__wbindgen_free(r0, r1);
    }
}

export function return_current_peer_id(wasm, peerId, arg0) {
    var ptr0 = passStringToWasm0(wasm, peerId, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0(wasm)[arg0 / 4 + 1] = len0;
    getInt32Memory0(wasm)[arg0 / 4 + 0] = ptr0;
}

export function return_call_service_result(wasm, ret, arg0) {
    var ptr1 = passStringToWasm0(wasm, ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getInt32Memory0(wasm)[arg0 / 4 + 1] = len1;
    getInt32Memory0(wasm)[arg0 / 4 + 0] = ptr1;
}

export function free(wasm, ptr, len) {
    wasm.__wbindgen_free(ptr, len);
}
