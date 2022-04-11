export type LogLevel = 'info' | 'trace' | 'debug' | 'info' | 'warn' | 'error' | 'off';

/**
 * Represents an executed host function result.
 */
export interface CallServiceResult {
    /**
     * A error code service or builtin returned, where 0 represents success.
     */
    retCode: number;

    /**
     * Serialized return value from the service
     */
    result: string;
}

/**
 * Contains arguments of a call instruction and all other necessary information required for calling a service.
 */
export interface CallRequest {
    /**
     * Id of a service that should be called.
     */
    serviceId: string;

    /**
     * Name of a function from service identified by service_id that should be called.
     */
    functionName: string;

    /**
     * Arguments that should be passed to the service
     */
    arguments: any[];

    /**
     * Security tetraplets that should be passed to the service
     */
    tetraplets: SecurityTetraplet[][];
}

export type CallRequestsArray = Array<[key: number, callRequest: CallRequest]>;

export type CallResultsArray = Array<[key: number, callServiceResult: CallServiceResult]>;

/**
 * Describes a result returned at the end of the interpreter execution_step.
 */
export interface InterpreterResult {
    /**
     * A return code, where 0 means success.
     */
    retCode: number;

    /**
     * Contains error message if ret_code != 0
     */
    errorMessage: string;

    /**
     * Contains script data that should be preserved in an executor of this interpreter regardless of ret_code value.
     */
    data: Uint8Array;

    /**
     * Public keys of peers that should receive data.
     */
    nextPeerPks: Array<string>;

    /**
     * Collected parameters of all met call instructions that could be executed on a current peer.
     */
    callRequests: CallRequestsArray;
}

/**
 * ResolvedTriplet represents peer network location with all variables, literals and etc resolved into final string.
 * This structure contains a subset of values that SecurityTetraplet consists of.
 */
export interface ResolvedTriplet {
    /**
     * Id of a peer where corresponding value was set.
     */
    peer_pk: string;

    /**
     *  Id of a service that set corresponding value.
     */
    service_id: string;

    /**
     * Name of a function that returned corresponding value.
     */
    function_name: string;
}

/**
 *  Describes an origin that set corresponding value.
 */
export interface SecurityTetraplet extends ResolvedTriplet {
    /**
     * Value was produced by applying this `json_path` to the output from `call_service`.
     */
    json_path: string;
}
