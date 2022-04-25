/*
 * Copyright 2022 Fluence Labs Limited
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

import { CallResultsArray, InterpreterResult, CallRequest, RunParameters } from './types';

const decoder = new TextDecoder();
const encoder = new TextEncoder();

/**
 * Serializes AVM arguments in JSON string which can be passed into marine-js
 * @param initPeerId - peer ID which initialized particle
 * @param currentPeerId - peer ID which is currently executing the particle
 * @param air - particle's air script as string
 * @param prevData - particle's prev data as raw byte array
 * @param data - particle's data as raw byte array
 * @param callResults - array of tuples [callResultKey, callResult]
 * @returns AVM call arguments as serialized JSON string
 */
export function serializeAvmArgs(
    runParams: RunParameters,
    air: string,
    prevData: Uint8Array,
    data: Uint8Array,
    callResults: CallResultsArray,
): string {
    const callResultsToPass: any = {};
    for (let [key, callResult] of callResults) {
        callResultsToPass[key] = {
            ret_code: callResult.retCode,
            result: callResult.result,
        };
    }

    const encoded = encoder.encode(JSON.stringify(callResultsToPass));

    const avmArg = JSON.stringify([
        // force new line
        air,
        Array.from(prevData),
        Array.from(data),
        {
            init_peer_id: runParams.initPeerId,
            current_peer_id: runParams.currentPeerId,
            timestamp: runParams.timestamp,
            ttl: runParams.ttl,
        },
        Array.from(encoded),
    ]);

    return avmArg;
}

/**
 * Deserializes raw result of AVM call obtained from marine-js into structured form
 * @param rawResult - string containing raw result of AVM call
 * @returns structured InterpreterResult
 */
export function deserializeAvmResult(rawResult: string): InterpreterResult {
    let result: any;
    try {
        result = JSON.parse(rawResult);
    } catch (ex) {
        throw 'call_module result parsing error: ' + ex + ', original text: ' + rawResult;
    }

    if (result.error !== '') {
        throw 'call_module returned error: ' + result.error;
    }

    result = result.result;

    const callRequestsStr = decoder.decode(new Uint8Array(result.call_requests));
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
    for (const key in parsedCallRequests) {
        const callRequest = parsedCallRequests[key];

        let arguments_;
        let tetraplets;
        try {
            arguments_ = JSON.parse(callRequest.arguments);
        } catch (e) {
            throw "Couldn't parse arguments: " + e + '. Original string is: ' + arguments_;
        }

        try {
            tetraplets = JSON.parse(callRequest.tetraplets);
        } catch (e) {
            throw "Couldn't parse tetraplets: " + e + '. Original string is: ' + tetraplets;
        }

        resultCallRequests.push([
            key as any,
            {
                serviceId: callRequest.service_id,
                functionName: callRequest.function_name,
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

type CallToAvm = ((args: string) => Promise<string>) | ((args: string) => string);

/**
 * Utility function which serializes AVM args and passed them into AVM returning interpreter result.
 * Call to AVM is delegated to a function which must be provided by user.
 * It might be either synchronous or asynchronous (returning a promise)
 * @param fn - delegated call to AVM
 * @param initPeerId - peer ID which initialized particle
 * @param currentPeerId - peer ID which is currently executing the particle
 * @param air - particle's air script as string
 * @param prevData - particle's prev data as raw byte array
 * @param data - particle's data as raw byte array
 * @param callResults - array of tuples [callResultKey, callResult]
 * @returns structured InterpreterResult
 */
export async function callAvm(
    fn: CallToAvm,
    runParams: RunParameters,
    air: string,
    prevData: Uint8Array,
    data: Uint8Array,
    callResults: CallResultsArray,
): Promise<InterpreterResult> {
    try {
        const avmArg = serializeAvmArgs(runParams, air, prevData, data, callResults);
        const rawResult = await fn(avmArg);
        return deserializeAvmResult(rawResult);
    } catch (e) {
        return {
            retCode: -1,
            errorMessage: 'marine-js call failed, ' + e,
        } as any;
    }
}
