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

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(transparent)]
pub struct CID(String);

impl CID {
    pub fn new(cid: impl Into<String>) -> Self {
        CID(cid.into())
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<CID> for String {
    fn from(value: CID) -> Self {
        value.0
    }
}

// TODO we might refactor this to `SerializationFormat` trait
// that both transform data to binary/text form (be it JSON, CBOR or something else)
// and produces CID too
pub fn json_data_cid(data: &[u8]) -> CID {
    use cid::Cid;
    use multihash::{Code, MultihashDigest};

    // the Sha2_256 is current IPFS default hash
    let digest = Code::Sha2_256.digest(data);
    // seems to be better than RAW_CODEC = 0x55
    const JSON_CODEC: u64 = 0x0200;

    let cid = Cid::new_v1(JSON_CODEC, digest);
    CID(cid.to_string())
}

/// Calculate a CID of JSON-serialized value.
pub fn value_to_json_cid<Val: Serialize>(value: &Val) -> Result<CID, serde_json::Error> {
    let data = serde_json::to_vec(value)?;
    Ok(json_data_cid(&data))
}
