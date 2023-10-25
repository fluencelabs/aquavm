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

use std::rc::Rc;

use air_interpreter_cid::CidRef;
use air_interpreter_signatures::KeyError;
use thiserror::Error as ThisError;
#[derive(Debug, ThisError)]
pub enum DataVerifierError {
    #[error("malformed signature: {0}")]
    MalformedKey(#[from] KeyError),

    #[error(transparent)]
    MalformedSignature(fluence_keypair::error::DecodingError),

    #[error("peer_id doens't match any available public key: {0:?}")]
    PeerIdNotFound(String),

    #[error("signature mismatch for {peer_id:?}: {error:?}, values: CIDS: {cids:?}")]
    SignatureMismatch {
        error: Box<fluence_keypair::error::VerificationError>,
        cids: Vec<Rc<CidRef>>,
        peer_id: String,
    },

    #[error(
        "inconsistent CID multisets on merge for peer {peer_id:?}, prev: {larger_cids:?}, current: {smaller_cids:?}"
    )]
    MergeMismatch {
        peer_id: String,
        larger_cids: Vec<Rc<CidRef>>,
        smaller_cids: Vec<Rc<CidRef>>,
    },
}
