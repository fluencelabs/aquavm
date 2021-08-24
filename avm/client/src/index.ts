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

import { toByteArray } from 'base64-js';
import { getStringFromWasm0, invoke } from './wrapper';
import wasmBs64 from './wasm';

export type LogLevel = 'info' | 'trace' | 'debug' | 'info' | 'warn' | 'error' | 'off';

export type LogFunction = (level: LogLevel, message: string) => void;

export interface CallServiceResult {
    ret_code: number;
    result: string;
}

export interface InterpreterResult {
    retCode: number;
    errorMessage: string;
    data: Uint8Array;
    nextPeerPks: Array<string>;
    callRequests: {
        [key: number]: {
            serviceName: string;
            functionName: string;
            arguments: string;
            tetraplets: string;
        };
    };
}

export interface ResolvedTriplet {
    peer_pk: string;
    service_id: string;
    function_name: string;
}

export interface SecurityTetraplet extends ResolvedTriplet {
    json_path: string;
}

type Exports = any;
type Instance = any;
type ExportValue = any;

type LogImport = {
    log_utf8_string: (level: any, target: any, offset: any, size: any) => void;
};

type ImportObject = {
    host: LogImport;
};

class HostImportsConfig {
    exports: Exports | undefined;
    newImportObject: () => ImportObject;

    constructor(create: (cfg: HostImportsConfig) => ImportObject) {
        this.exports = undefined;
        this.newImportObject = () => create(this);
    }

    setExports(exports: Exports) {
        this.exports = exports;
    }
}

const interpreter_wasm = toByteArray(wasmBs64);

/// Instantiates WebAssembly runtime with AIR interpreter module
async function interpreterInstance(cfg: HostImportsConfig, logFunction: LogFunction): Promise<Instance> {
    /// Create host imports that use module exports internally
    let imports = cfg.newImportObject();

    /// Instantiate interpreter
    let interpreter_module = await WebAssembly.compile(interpreter_wasm);
    let instance: Instance = await WebAssembly.instantiate(interpreter_module, imports);

    /// Set exports, so host imports can use them
    cfg.setExports(instance.exports);

    /// Trigger interpreter initialization (i.e., call main function)
    call_export(instance.exports.main, logFunction);

    return instance;
}

/// If export is a function, call it. Otherwise log a warning.
/// NOTE: any here is unavoidable, see Function interface definition
function call_export(f: ExportValue, logFunction: LogFunction): any {
    if (typeof f === 'function') {
        return f();
    } else {
        logFunction('error', `can't call export ${f}: it is not a function, but ${typeof f}`);
    }
}

function log_import(cfg: HostImportsConfig, logFunction: LogFunction): LogImport {
    return {
        log_utf8_string: (level: any, target: any, offset: any, size: any) => {
            let wasm = cfg.exports;

            try {
                let str = getStringFromWasm0(wasm, offset, size);
                let levelStr: LogLevel;
                switch (level) {
                    case 1:
                        levelStr = 'error';
                        break;
                    case 2:
                        levelStr = 'warn';
                        break;
                    case 3:
                        levelStr = 'info';
                        break;
                    case 4:
                        levelStr = 'debug';
                        break;
                    case 6:
                        levelStr = 'trace';
                        break;
                    default:
                        return;
                }
                logFunction(levelStr, str);
            } finally {
            }
        },
    };
}

/// Returns import object that describes host functions called by AIR interpreter
function newImportObject(cfg: HostImportsConfig, logFunction: LogFunction): ImportObject {
    return {
        host: log_import(cfg, logFunction),
    };
}

export class AirInterpreter {
    private wasmWrapper;
    private logLevel: LogLevel;

    constructor(wasmWrapper) {
        this.wasmWrapper = wasmWrapper;
    }

    static async create(logLevel: LogLevel, logFunction: LogFunction) {
        const cfg = new HostImportsConfig((cfg) => {
            return newImportObject(cfg, logFunction);
        });

        const instance = await interpreterInstance(cfg, logFunction);
        const res = new AirInterpreter(instance);
        res.logLevel = logLevel;
        return res;
    }

    invoke(
        air: string,
        prevData: Uint8Array,
        data: Uint8Array,
        params: { initPeerId: string; currentPeerId: string },
        callResults: { [key: number]: CallServiceResult },
    ): InterpreterResult {
        const resStr = invoke(
            // new line
            this.wasmWrapper.exports,
            air,
            prevData,
            data,
            Buffer.from(
                JSON.stringify({
                    init_peer_id: params.initPeerId,
                    current_peer_id: params.currentPeerId,
                }),
            ),
            Buffer.from(JSON.stringify(callResults)),
            this.logLevel,
        );
        const res = JSON.parse(resStr);
        res.call_requests = JSON.parse(new TextDecoder().decode(Buffer.from(res.call_requests)));

        for (const k in res.call_requests) {
            const v = res.call_requests[k];
            res.call_requests[k] = {
                serviceName: v.service_name,
                functionName: v.function_name,
                arguments: v.arguments,
                tetraplets: v.tetraplets,
            };
        }
        return {
            retCode: res.ret_code,
            errorMessage: res.error_message,
            data: res.data,
            nextPeerPks: res.next_peer_pks,
            callRequests: res.call_requests,
        };
    }
}
