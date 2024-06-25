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

use std::rc::Rc;

use air_interpreter_cid::CidRef;
use air_interpreter_signatures::KeyError;
use air_interpreter_signatures::VerificationError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum DataVerifierError {
    #[error("malformed key: {key:?}: {error}")]
    MalformedKey { error: KeyError, key: String },

    #[error(transparent)]
    MalformedSignature(fluence_keypair::error::DecodingError),

    #[error("peer_id doens't match any available public key: {0:?}")]
    PeerIdNotFound(String),

    #[error("signature mismatch for {peer_id:?}: {error:?}, values: CIDS: {cids:?}")]
    SignatureMismatch {
        error: Box<VerificationError>,
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
