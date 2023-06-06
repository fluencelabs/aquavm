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

use crate::{CanonResult, ExecutedState, InterpreterData};

use air_interpreter_signatures::{FullSignatureStore, PublicKey, Signature};
use thiserror::Error as ThisError;

use std::collections::HashMap;

const CANNOT_HAPPEN_IN_VERIFIED_CID_STORE: &str = "cannot happen in a checked CID store";

#[derive(Debug, ThisError)]
pub enum DataVerifierError {
    #[error(transparent)]
    MalformedKey(fluence_keypair::error::DecodingError),
    #[error(transparent)]
    MalformedSignature(fluence_keypair::error::DecodingError),

    #[error("peer_id doens't match any public key: {0:?}")]
    PeerIdNotFound(String),

    #[error("signature mismatch: {error:?}, values: CIDS: {cids:?}")]
    SignatureMismatch {
        error: fluence_keypair::error::VerificationError,
        cids: Vec<Box<str>>,
    },

    #[error(
        "inconsistent CID multisets on merge for pk {public_key:?}, prev: {larger_cids:?}, current: {smaller_cids:?}"
    )]
    MergeMismatch {
        public_key: Box<PublicKey>,
        larger_cids: Vec<Box<str>>,
        smaller_cids: Vec<Box<str>>,
    },
}

pub struct DataVerifier<'data> {
    grouped_cids: HashMap<Box<str>, PeerInfo<'data>>,
}

impl<'data> DataVerifier<'data> {
    // it can be further optimized if only required parts are passed
    // SignatureStore is not used elsewhere
    pub fn new(data: &'data InterpreterData) -> Result<Self, DataVerifierError> {
        use crate::CallResult::*;

        // it contains signature too; if we try to add a value to a peer w/o signature, it is an immediate error
        let mut grouped_cids: HashMap<Box<str>, PeerInfo<'data>> = data
            .signatures
            .iter()
            .map(|(public_key, signature)| {
                (
                    public_key.to_peer_id().to_string().into(),
                    PeerInfo::new(public_key, signature),
                )
            })
            .collect();

        for elt in &data.trace {
            match elt {
                ExecutedState::Call(ref call) => {
                    let cid = match call {
                        RequestSentBy(_) => None,
                        Executed(executed) => executed.get_cid(),
                        Failed(failed) => Some(failed),
                    };
                    if let Some(cid) = cid {
                        // TODO refactor
                        let service_result = data
                            .cid_info
                            .service_result_store
                            .get(cid)
                            .expect(CANNOT_HAPPEN_IN_VERIFIED_CID_STORE);
                        let tetraplet = data
                            .cid_info
                            .tetraplet_store
                            .get(&service_result.tetraplet_cid)
                            .expect(CANNOT_HAPPEN_IN_VERIFIED_CID_STORE);

                        let peer_pk = tetraplet.peer_pk.as_str();
                        match grouped_cids.get_mut(peer_pk) {
                            Some(peer_info) => {
                                peer_info.cids.push((**cid).clone().into_inner().into())
                            }
                            None => return Err(DataVerifierError::PeerIdNotFound(peer_pk.into())),
                        }
                    }
                }
                ExecutedState::Canon(CanonResult(ref cid)) => {
                    // TODO refactor
                    let canon_result = data
                        .cid_info
                        .canon_result_store
                        .get(cid)
                        .expect(CANNOT_HAPPEN_IN_VERIFIED_CID_STORE);
                    let tetraplet = data
                        .cid_info
                        .tetraplet_store
                        .get(&canon_result.tetraplet)
                        .expect(CANNOT_HAPPEN_IN_VERIFIED_CID_STORE);

                    let peer_pk = tetraplet.peer_pk.as_str();
                    match grouped_cids.get_mut(peer_pk) {
                        Some(peer_info) => peer_info.cids.push((**cid).clone().into_inner().into()),
                        None => return Err(DataVerifierError::PeerIdNotFound(peer_pk.into())),
                    }
                }
                _ => {}
            };
        }

        // sort cids for canonicalization
        // TODO wrapper type for sorted data
        for peer_info in grouped_cids.values_mut() {
            peer_info.cids.sort_unstable();
        }

        Ok(Self { grouped_cids })
    }

    pub fn verify(&self) -> Result<(), DataVerifierError> {
        for peer_info in self.grouped_cids.values() {
            peer_info
                .public_key
                .verify(&peer_info.cids, peer_info.signature)
                .map_err(|error| DataVerifierError::SignatureMismatch {
                    error,
                    cids: peer_info.cids.clone(),
                })?;
        }
        Ok(())
    }

    // TODO enforce merging only verified sets
    pub fn merge(mut self, other: Self) -> Result<FullSignatureStore, DataVerifierError> {
        use std::collections::hash_map::Entry::*;

        for (other_peer_pk, mut other_info) in other.grouped_cids {
            let our_info = self.grouped_cids.entry(other_peer_pk);
            match our_info {
                Occupied(mut our_info_ent) => {
                    debug_assert_eq!(other_info.public_key, our_info_ent.get().public_key);

                    if our_info_ent.get().cids.len() < other_info.cids.len() {
                        // the merged map countains largest set for each peer_id
                        //
                        // this code assumes that a peer only adds CIDs to its set, so CID multisets
                        //   are growing-only; but it is additionally checked below
                        // so, we get a largest set as merged one
                        std::mem::swap(our_info_ent.get_mut(), &mut other_info);
                    }

                    let larger_info = our_info_ent.get();
                    let smaller_info = &other_info;
                    check_cid_multiset_consistency(larger_info, smaller_info)?;
                }
                Vacant(ent) => {
                    ent.insert(other_info);
                }
            }
        }
        let mut store = FullSignatureStore::new();
        for peer_info in self.grouped_cids.into_values() {
            store.put(peer_info.public_key.clone(), peer_info.signature.clone())
        }
        Ok(store)
    }
}

// safety check for malicious peer that returns inconsistent CID multiset
fn check_cid_multiset_consistency(
    larger_pair: &PeerInfo<'_>,
    smaller_pair: &PeerInfo<'_>,
) -> Result<(), DataVerifierError> {
    let larger_cids = &larger_pair.cids;
    let smaller_cids = &smaller_pair.cids;
    use std::collections::HashSet;
    // TODO bug: it should be a multiset; hashset provides weaker check
    // TODO we may use the fact that cids are sorted and write the check manually
    let larger_set: HashSet<_> = larger_cids.iter().collect();
    let smaller_set: HashSet<_> = smaller_cids.iter().collect();

    if larger_set.is_superset(&smaller_set) {
        Ok(())
    } else {
        Err(DataVerifierError::MergeMismatch {
            public_key: smaller_pair.public_key.clone().into(),
            larger_cids: larger_cids.clone(),
            smaller_cids: smaller_cids.clone(),
        })
    }
}

struct PeerInfo<'data> {
    public_key: &'data PublicKey,
    signature: &'data Signature,
    cids: Vec<Box<str>>,
}

impl<'data> PeerInfo<'data> {
    fn new(public_key: &'data PublicKey, signature: &'data Signature) -> Self {
        Self {
            public_key,
            signature,
            cids: vec![],
        }
    }
}
