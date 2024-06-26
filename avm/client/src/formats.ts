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

import msgpack from "msgpack-lite"
import multicodec from "multicodec"

const decoder = new TextDecoder();
const encoder = new TextEncoder();

interface Representation {
    fromBinary(data: Uint8Array): object
    toBinary(obj: object): Uint8Array
}

interface Multiformatable {
    get_code(): multicodec.CodecCode
}

/**
 * Simple JSON representation.
 */
export class JsonRepr implements Representation, Multiformatable {
    fromBinary(data: Uint8Array): object {
        let dataStr = decoder.decode(data)
        return JSON.parse(dataStr);
    }

    toBinary(obj: object): Uint8Array {
        return encoder.encode(JSON.stringify(obj))
    }

    get_code(): multicodec.CodecCode {
        return multicodec.JSON
    }
}

/**
 * Simple MessagePack representation.
 */
export class MsgPackRepr implements Representation, Multiformatable {
    fromBinary(data: Uint8Array): object {
        return msgpack.decode(data)
    }

    toBinary(obj: object): Uint8Array {
        return msgpack.encode(obj)
    }

    get_code(): multicodec.CodecCode {
        return multicodec.MESSAGEPACK
    }
}

/**
 * Multicodec representation that supports both JSON and MsgPack, but uses only specific representation for encoding.
 */
export class MulticodecRepr implements Representation {
    serializer: Representation & Multiformatable

    constructor(serializer: Representation & Multiformatable) {
        this.serializer = serializer
    }

    fromBinary(data: Uint8Array): object {
        let code = multicodec.getCodeFromData(data)
        let repr: Representation | null = null;

        if (code == multicodec.JSON) {
            repr = new JsonRepr()
        } else if (code == multicodec.MESSAGEPACK) {
            repr = new MsgPackRepr()
        }

        if (repr === null) {
            throw "Unknown code " + code + "in multiformat data " + data
        }
        
        return repr.fromBinary(multicodec.rmPrefix(data))
    }

    toBinary(obj: object): Uint8Array {
        let bareData = this.serializer.toBinary(obj);
        let codeName = multicodec.getNameFromCode(this.serializer.get_code());
        return multicodec.addPrefix(codeName, bareData)
    }
}
