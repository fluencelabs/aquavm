import {register_module, call_module} from 'marine-web2-runtime'
//import { toByteArray } from 'base64-js';
import { WASI } from '@wasmer/wasi'
import { WasmFs } from '@wasmer/wasmfs'
//import browserBindings from '@wasmer/wasi/lib/bindings/browser'
import nodeBindings from '@wasmer/wasi/lib/bindings/node'
//const wasmBs64 = require('./wasm')

export type LogLevel = 'info' | 'trace' | 'debug' | 'info' | 'warn' | 'error' | 'off';

export type LogFunction = (level: LogLevel, message: string) => void;

export interface CallServiceResult {
    retCode: number;
    result: string;
}

export interface CallRequest {
    serviceId: string;
    functionName: string;
    arguments: any[];
    tetraplets: SecurityTetraplet[][];
}

export type CallRequestsArray = Array<[key: number, callRequest: CallRequest]>;

export type CallResultsArray = Array<[key: number, callServiceResult: CallServiceResult]>;

export interface InterpreterResult {
    retCode: number;
    errorMessage: string;
    data: Uint8Array;
    nextPeerPks: Array<string>;
    callRequests: CallRequestsArray;
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

//const interpreter_wasm = toByteArray(wasmBs64);

/// Instantiates WebAssembly runtime with AIR interpreter module
async function interpreterInstance(module: WebAssembly.Module, cfg: HostImportsConfig, logFunction: LogFunction): Promise<Instance> {
    /// Create host imports that use module exports internally
    let imports = cfg.newImportObject();

    /// Instantiate interpreter
    let interpreter_module = module;
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

const lTextDecoder = typeof TextDecoder === 'undefined' ? module.require('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();
let cachegetUint8Memory0 = null;

function getUint8Memory0(wasm) {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

export function getStringFromWasm0(wasm, ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0(wasm).subarray(ptr, ptr + len));
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

const decoder = new TextDecoder();
const encoder = new TextEncoder();

export class AirInterpreter {
    private _interpreter;
    private _wasmFs;
    private _wasi;
    private logLevel: LogLevel;
    private logFunction;

    constructor(logFunction: LogFunction) {
        this._interpreter = null;
        this.logFunction = logFunction;
    }

    async init(module: WebAssembly.Module, logLevel: LogLevel): Promise<void> {
        if (this._interpreter !== null) {
            return;
        }

        this._wasmFs = new WasmFs()

        this._wasi = new WASI({
            // Arguments passed to the Wasm Module
            // The first argument is usually the filepath to the executable WASI module
            // we want to run.
            args: [],

            // Environment variables that are accesible to the WASI module
            env: {},

            // Bindings that are used by the WASI Instance (fs, path, etc...)
            bindings: {
                ...nodeBindings,
                fs: this._wasmFs.fs
            }
        })

        const cfg = new HostImportsConfig((cfg) => {
            return newImportObject(cfg, this.logFunction);
        });

        let wasmModule = module;//await WebAssembly.compile(interpreter_wasm);
        //console.log(wasmModule)

        let instance = await WebAssembly.instantiate(wasmModule, {
            ...this._wasi.getImports(wasmModule),
            ...cfg.newImportObject()
        });
        //console.log(instance)
        cfg.setExports(instance.exports)

        this._wasi.start(instance)                       // Start the WASI instance
        let custom_sections = WebAssembly.Module.customSections(wasmModule,"interface-types");
        //console.log(custom_sections)
        let it_custom_section = new Uint8Array(custom_sections[0])

        let result = register_module("avm", it_custom_section, instance);
        if (result.error.length > 0) {
            throw result.error
        }

        this._interpreter = instance
    }

    static async create(module: WebAssembly.Module, logLevel: LogLevel, logFunction: LogFunction) {
        /*
        const cfg = new HostImportsConfig((cfg) => {
            return newImportObject(cfg, logFunction);
        });
*/
        //const instance = await interpreterInstance(cfg, logFunction);
        const res = new AirInterpreter(logFunction);
        await res.init(module, logLevel);

        res.logLevel = logLevel;
        return res;
    }

    invoke(
        air: string,
        prevData: Uint8Array,
        data: Uint8Array,
        params: { initPeerId: string; currentPeerId: string },
        callResults: CallResultsArray,
    ): InterpreterResult {
        const callResultsToPass: any = {};
        for (let [k, v] of callResults) {
            callResultsToPass[k] = {
                ret_code: v.retCode,
                result: v.result,
            };
        }

        const paramsToPass = {
            init_peer_id: params.initPeerId,
            current_peer_id: params.currentPeerId,
        };

        //console.log(air)
        //console.log(call_module)
        const rawResult = call_module(
            "avm",
            "invoke",
            JSON.stringify([
                air,
                Array.from(prevData),
                Array.from(data),
                paramsToPass,
                Array.from(Buffer.from(JSON.stringify(callResultsToPass)))
            ])
        );

        if (rawResult.error.length > 0) {
            throw rawResult.error;
        }
        //console.log("end call_module")
        let result: any;
        try {
            result = JSON.parse(rawResult.result);
        } catch (e) {
            throw "Couldn't parse result of invoke call: " + e + '. Original string is: ' + rawResult.result;
        }

        const callRequestsStr = decoder.decode(Buffer.from(result.call_requests));

        let parsedCallRequests;
        try {
            if (callRequestsStr.length === 0) {
                parsedCallRequests = {};
            } else {
                parsedCallRequests = JSON.parse(callRequestsStr);
            }
        } catch (e) {
            throw "Couldn't parse call requests: " + e + '. Original string is: ' + callRequestsStr;
        }

        let resultCallRequests: Array<[key: number, callRequest: CallRequest]> = [];
        for (const k in parsedCallRequests) {
            const v = parsedCallRequests[k];

            let arguments_;
            let tetraplets;
            try {
                arguments_ = JSON.parse(v.arguments);
            } catch (e) {
                throw "Couldn't parse arguments: " + e + '. Original string is: ' + arguments_;
            }

            try {
                tetraplets = JSON.parse(v.tetraplets);
            } catch (e) {
                throw "Couldn't parse tetraplets: " + e + '. Original string is: ' + tetraplets;
            }

            resultCallRequests.push([
                k as any,
                {
                    serviceId: v.service_id,
                    functionName: v.function_name,
                    arguments: arguments_,
                    tetraplets: tetraplets,
                },
            ]);
        }
        return {
            retCode: result.ret_code,
            errorMessage: result.error_message,
            data: result.data,
            nextPeerPks: result.next_peer_pks,
            callRequests: resultCallRequests,
        };
    }
}
