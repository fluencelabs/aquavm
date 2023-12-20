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

import { CallResultsArray, InterpreterResult, CallRequest, RunParameters, JSONArray, JSONObject } from './types';
import { JsonRepr } from './formats'

const decoder = new TextDecoder();
const encoder = new TextEncoder();

// Have to match the air-interpreter-interface.
const callRequestsRepr = new JsonRepr();
// Have to match the air-interpreter-interface.
const argumentRepr = new JsonRepr();
// Have to match the air-interpreter-interface.
const tetrapletRepr = new JsonRepr();

const callResultsRepr = new JsonRepr();

/**
 * Encodes arguments into JSON array suitable for marine-js
 * @param initPeerId - peer ID which initialized particle
 * @param currentPeerId - peer ID which is currently executing the particle
 * @param air - particle's air script as string
 * @param prevData - particle's prev data as raw byte array
 * @param data - particle's data as raw byte array
 * @param callResults - array of tuples [callResultKey, callResult]
 * @returns AVM call arguments suitable for marine-js
 */
export function serializeAvmArgs(
    runParams: RunParameters,
    air: string,
    prevData: Uint8Array,
    data: Uint8Array,
    callResults: CallResultsArray,
): JSONArray {
    const callResultsToPass: any = {};
    for (let [key, callResult] of callResults) {
        callResultsToPass[key] = {
            ret_code: callResult.retCode,
            result: callResult.result,
        };
    }

    const encodedCallResults = callResultsRepr.toBinary(callResultsToPass)
    const runParamsSnakeCase = {
        init_peer_id: runParams.initPeerId,
        current_peer_id: runParams.currentPeerId,
        key_format: runParams.keyFormat,
        secret_key_bytes: Array.from(runParams.secretKeyBytes),
        timestamp: runParams.timestamp,
        ttl: runParams.ttl,
        particle_id: runParams.particleId,
    };

    return [air, Array.from(prevData), Array.from(data), runParamsSnakeCase, Array.from(encodedCallResults)];
}

/**
 * Deserializes raw result of AVM call obtained from marine-js into structured form
 * @param rawResult - string containing raw result of AVM call
 * @returns structured InterpreterResult
 */
export function deserializeAvmResult(result: any): InterpreterResult {
    const callRequestsStr = decoder.decode(new Uint8Array(result.call_requests));
    let parsedCallRequests: object;
    try {
        if (callRequestsStr.length === 0) {
            parsedCallRequests = {};
        } else {
            parsedCallRequests = callRequestsRepr.fromBinary(new Uint8Array(result.call_requests));
        }
    } catch (e) {
        throw "Couldn't parse call requests: " + e + '. Original data is: ' + result.call_requests;
    }

    let resultCallRequests: Array<[key: number, callRequest: CallRequest]> = [];
    for (const key in parsedCallRequests) {
        const callRequest = parsedCallRequests[key];

        let arguments_;
        let tetraplets;
        try {
            arguments_ = argumentRepr.fromBinary(callRequest.arguments)
        } catch (e) {
            throw "Couldn't parse arguments: " + e + '. Original data is: ' + callRequest.arguments;
        }

        try {
            tetraplets = tetrapletRepr.fromBinary(callRequest.tetraplets)
        } catch (e) {
            throw "Couldn't parse tetraplets: " + e + '. Original data is: ' + callRequest.tetraplets;
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

type CallToAvm = ((args: JSONArray | JSONObject) => Promise<unknown>) | ((args: JSONArray | JSONObject) => unknown);

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
    const avmArg = serializeAvmArgs(runParams, air, prevData, data, callResults);
    const rawResult = await fn(avmArg);
    return deserializeAvmResult(rawResult);
}
