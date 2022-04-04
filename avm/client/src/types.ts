export type LogLevel = 'info' | 'trace' | 'debug' | 'info' | 'warn' | 'error' | 'off';

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

export type AvmRunner = {
    init: (logLevel: LogLevel) => Promise<void>;
    terminate: () => Promise<void>;
    run: (
        air: string,
        prevData: Uint8Array,
        data: Uint8Array,
        params: {
            initPeerId: string;
            currentPeerId: string;
        },
        callResults: CallResultsArray,
    ) => Promise<InterpreterResult>;
};
