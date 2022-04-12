import { CallResultsArray, InterpreterResult, CallRequest } from './types';

const decoder = new TextDecoder();
const encoder = new TextEncoder();

/**
 * Serializes AVM arguments in JSON string which can be passed into marine-js
 * @param initPeerId - peer ID which initialized particle
 * @param currentPeerId - peer ID which is currently executing the particle
 * @param air - particle's air script as string
 * @param prevData - particle's prev data as raw byte array
 * @param data - particle's data as raw byte array
 * @param callResults - call results pro
 * @returns AVM call arguments as serialized JSON string
 */
export function serializeAvmArgs(
    initPeerId: string,
    currentPeerId: string,
    air: string,
    prevData: Uint8Array,
    data: Uint8Array,
    callResults: CallResultsArray,
): string {
    const callResultsToPass: any = {};
    for (let [k, v] of callResults) {
        callResultsToPass[k] = {
            ret_code: v.retCode,
            result: v.result,
        };
    }

    const paramsToPass = {
        init_peer_id: initPeerId,
        current_peer_id: currentPeerId,
    };

    const encoded = encoder.encode(JSON.stringify(callResultsToPass));

    const avmArg = JSON.stringify([
        // force new line
        air,
        Array.from(prevData),
        Array.from(data),
        paramsToPass,
        Array.from(encoded),
    ]);

    return avmArg;
}

interface MarineJsCallServiceResult {
    error: string;
    result: any;
}

/**
 * Deserializes raw result of AVM call obtained from marine-js into structured form
 * @param rawResult - string containing raw result of AVM call
 * @returns structured InterpreterResult
 */
export function deserializeAvmResult(rawResult: MarineJsCallServiceResult): InterpreterResult {
    if (rawResult.error !== '') {
        throw 'call_module returned error: ' + rawResult.error;
    }

    const result = rawResult.result;

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

type CallToAvm = ((args: string) => Promise<MarineJsCallServiceResult>) | ((args: string) => MarineJsCallServiceResult);

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
 * @param callResults - call results pro
 * @returns structured InterpreterResult
 */
export async function callAvm(
    fn: CallToAvm,
    initPeerId: string,
    currentPeerId: string,
    air: string,
    prevData: Uint8Array,
    data: Uint8Array,
    callResults: CallResultsArray,
): Promise<InterpreterResult> {
    try {
        const avmArg = serializeAvmArgs(initPeerId, currentPeerId, air, prevData, data, callResults);
        const rawResult = await fn(avmArg);
        return deserializeAvmResult(rawResult);
    } catch (e) {
        return {
            retCode: -1,
            errorMessage: 'marine-js call failed, ' + e,
        } as any;
    }
}
