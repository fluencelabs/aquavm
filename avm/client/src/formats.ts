/*
 * Copyright 2023 Fluence Labs Limited
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

import msgpackr from "msgpackr"
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
        return msgpackr.unpack(data)
    }

    toBinary(obj: object): Uint8Array {
        return msgpackr.pack(obj)
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
        var repr = null;

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
        let varintCode = multicodec.getVarintFromCode(this.serializer.get_code());
        return multicodec.addPrefix(varintCode, bareData)
    }
}
