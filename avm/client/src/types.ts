/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

export type LogLevel = 'info' | 'trace' | 'debug' | 'info' | 'warn' | 'error' | 'off';

/**
 * From fluence-keypair crate:
 * https://github.com/fluencelabs/trust-graph/blob/f7ef0f8da095fe1fef80faaa0b0c2d5ef854bd16/keypair/src/key_pair.rs#L79-L82
 *
 * We define here only the supported subset of formats.  This enum is used for future extention.
 */
export enum KeyPairFormat {
    Ed25519 = 0,
}

/**
 * Parameters that a host side should pass to an interpreter and that necessary for execution.
 */
export interface RunParameters {
    /**
     * Peer id of a peer that start this particle.
     */
    initPeerId: string;

    /**
     * Peer id of a current peer.
     */
    currentPeerId: string;

    /**
     * Key format of the current peer Ed25519.
     */
    keyFormat: KeyPairFormat;

    /**
     * They secret key itself serialized into 32 byte Uint8Array using libp2p marshal
     */
    secretKeyBytes: Uint8Array;

    /**
     * Unix timestamp from a particle in milliseconds.
     * It represents time when this particle was sent from the init peer id.
     */
    timestamp: number;

    /**
     * TTL set by init peer id in milliseconds.
     */
    ttl: number;

    /**
     * Unique particle ID
     */
    particleId: string;
}

/**
 * Represents an executed host function result.
 */
export interface CallServiceResult {
    /**
     * A error code service or builtin returned, where 0 represents success.
     */
    retCode: number;

    /**
     * Serialized return value from the service.
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
     * Arguments that should be passed to the service.
     */
    arguments: any[];

    /**
     * Security tetraplets that should be passed to the service.
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
     * Value was produced by applying this `lens` to the output from `call_service`.
     */
    lens: string;
}

export type JSONValue = string | number | boolean | { [x: string]: JSONValue } | Array<JSONValue>;
export type JSONArray = Array<JSONValue>;
export type JSONObject = { [x: string]: JSONValue };
