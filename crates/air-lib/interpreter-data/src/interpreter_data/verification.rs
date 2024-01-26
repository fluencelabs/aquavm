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

pub use super::errors::DataVerifierError;
use crate::CanonResult;
use crate::CidInfo;
use crate::ExecutedState;
use crate::ExecutionTrace;
use crate::InterpreterData;

use air_interpreter_cid::{CidRef, CID};
use air_interpreter_signatures::PublicKey;
use air_interpreter_signatures::Signature;
use air_interpreter_signatures::SignatureStore;

use std::collections::HashMap;
use std::rc::Rc;

const CANNOT_HAPPEN_IN_VERIFIED_CID_STORE: &str = "cannot happen in a checked CID store";

/// An util for verificating particular data's signatures.
pub struct DataVerifier<'data> {
    // a map from peer_id to peer's info (public key, signature, CIDS)
    grouped_cids: HashMap<Box<str>, PeerInfo<'data>>,
    salt: &'data str,
}

impl<'data> DataVerifier<'data> {
    // it can be further optimized if only required parts are passed;
    // SignatureStore is not used elsewhere
    pub fn new(data: &'data InterpreterData, salt: &'data str) -> Result<Self, DataVerifierError> {
        // validate key algoritms
        for (public_key, _) in data.signatures.iter() {
            public_key
                .validate()
                .map_err(|error| DataVerifierError::MalformedKey {
                    error,
                    key: public_key.to_string(),
                })?;
        }

        // it contains signature too; if we try to add a value to a peer w/o signature, it is an immediate error
        let mut grouped_cids: HashMap<Box<str>, PeerInfo<'data>> = data
            .signatures
            .iter()
            .map(|(public_key, signature)| {
                (
                    public_key
                        .to_peer_id()
                        .expect("cannot happen, was verifeid before")
                        .to_string()
                        .into(),
                    PeerInfo::new(public_key, signature),
                )
            })
            .collect();

        // fill PeerInfo's `cids` field, checking for peer IDs without a key
        collect_peers_cids_from_trace(&data.trace, &data.cid_info, &mut grouped_cids)?;

        // sort cids for canonicalization
        for peer_info in grouped_cids.values_mut() {
            peer_info.cids.sort_unstable();
        }

        Ok(Self { grouped_cids, salt })
    }

    /// Verify each peers' signatures.
    pub fn verify(&self) -> Result<(), DataVerifierError> {
        for peer_info in self.grouped_cids.values() {
            peer_info
                .public_key
                .verify(&peer_info.cids, self.salt, peer_info.signature)
                .map_err(|error| DataVerifierError::SignatureMismatch {
                    error: error.into(),
                    cids: peer_info.cids.clone(),
                    peer_id: peer_info
                        .public_key
                        .to_peer_id()
                        .expect("cannot happen, was verified before")
                        .to_string(),
                })?;
        }
        Ok(())
    }

    /// For each peer, merge previous and current CID multisets by determining the largest set.
    ///
    /// This code uses an invariant: peer's multiset of produced CIDs is always a superset of
    /// previous invocation's multiset:
    ///
    /// A_0 ⊆ A_1 ⊆ ... ⊆ A_n.
    ///
    /// So, the largest multiset is selected as the result of merging, the invariant is checked,
    /// and a error is returned if it is violated.
    ///
    /// If the multisets are of same size, they have to be equal.
    // TODO enforce merging only verified sets
    // The result is same regardless argument order, so "prevous/current" terminology
    // is not used deliberately.
    pub fn merge(mut self, other: Self) -> Result<SignatureStore, DataVerifierError> {
        use std::collections::hash_map::Entry::*;

        for (other_peer_pk, mut other_info) in other.grouped_cids {
            let our_info = self.grouped_cids.entry(other_peer_pk);
            match our_info {
                Occupied(mut our_info_ent) => {
                    debug_assert_eq!(other_info.public_key, our_info_ent.get().public_key);

                    if our_info_ent.get().cids.len() < other_info.cids.len() {
                        // the merged map contains the largest set for each peer_id
                        //
                        // this code assumes that a peer only adds CIDs to its set, so CID multisets
                        //   are growing-only; but it is additionally checked below
                        // so, we get a largest set as merged one
                        std::mem::swap(our_info_ent.get_mut(), &mut other_info);
                    }
                    // nb: if length are equal, sets should be equal, and any of them
                    // should be used; if they are not equal, check_cid_multiset_consistency
                    // will detect it.

                    let larger_info = our_info_ent.get();
                    let smaller_info = &other_info;
                    check_cid_multiset_invariant(larger_info, smaller_info)?;
                }
                Vacant(ent) => {
                    ent.insert(other_info);
                }
            }
        }
        let mut store = SignatureStore::new();
        for peer_info in self.grouped_cids.into_values() {
            store.put(peer_info.public_key.clone(), peer_info.signature.clone())
        }
        Ok(store)
    }
}

fn collect_peers_cids_from_trace<'data>(
    trace: &'data ExecutionTrace,
    cid_info: &'data CidInfo,
    grouped_cids: &mut HashMap<Box<str>, PeerInfo<'data>>,
) -> Result<(), DataVerifierError> {
    for elt in trace {
        match elt {
            ExecutedState::Call(ref call) => {
                let cid = call.get_cid();
                if let Some(cid) = cid {
                    // TODO refactor
                    let service_result = cid_info
                        .service_result_store
                        .get(cid)
                        .expect(CANNOT_HAPPEN_IN_VERIFIED_CID_STORE);
                    let tetraplet = cid_info
                        .tetraplet_store
                        .get(&service_result.tetraplet_cid)
                        .expect(CANNOT_HAPPEN_IN_VERIFIED_CID_STORE);

                    let peer_pk = tetraplet.peer_pk.as_str();
                    try_push_cid(grouped_cids, peer_pk, cid)?;
                }
            }
            ExecutedState::Canon(CanonResult::Executed(ref cid)) => {
                // TODO refactor
                let canon_result = cid_info
                    .canon_result_store
                    .get(cid)
                    .expect(CANNOT_HAPPEN_IN_VERIFIED_CID_STORE);
                let tetraplet = cid_info
                    .tetraplet_store
                    .get(&canon_result.tetraplet)
                    .expect(CANNOT_HAPPEN_IN_VERIFIED_CID_STORE);

                let peer_pk = tetraplet.peer_pk.as_str();
                try_push_cid(grouped_cids, peer_pk, cid)?;
            }
            _ => {}
        };
    }
    Ok(())
}

fn try_push_cid<T>(
    grouped_cids: &mut HashMap<Box<str>, PeerInfo<'_>>,
    peer_pk: &str,
    cid: &CID<T>,
) -> Result<(), DataVerifierError> {
    match grouped_cids.get_mut(peer_pk) {
        Some(peer_info) => {
            peer_info.cids.push(cid.get_inner());
            Ok(())
        }
        None => Err(DataVerifierError::PeerIdNotFound(peer_pk.into())),
    }
}

/// Safety check for malicious peer that returns inconsistent CID multisets,
/// i.e. non-increasing multisets.
fn check_cid_multiset_invariant(
    larger_pair: &PeerInfo<'_>,
    smaller_pair: &PeerInfo<'_>,
) -> Result<(), DataVerifierError> {
    let larger_cids = &larger_pair.cids;
    let smaller_cids = &smaller_pair.cids;

    let larger_count_map = to_count_map(larger_cids);
    let smaller_count_map = to_count_map(smaller_cids);

    if is_multisubset(larger_count_map, smaller_count_map) {
        Ok(())
    } else {
        let peer_id = smaller_pair
            .public_key
            .to_peer_id()
            .expect("cannot happen, was verified before")
            .to_string();
        Err(DataVerifierError::MergeMismatch {
            peer_id,
            larger_cids: larger_cids.clone(),
            smaller_cids: smaller_cids.clone(),
        })
    }
}

fn to_count_map(cids: &Vec<Rc<CidRef>>) -> HashMap<&str, usize> {
    let mut count_map = HashMap::<_, usize>::new();
    for cid in cids {
        // the counter can't overflow, the memory will overflow first
        *count_map.entry(&**cid).or_default() += 1;
    }
    count_map
}

fn is_multisubset(
    larger_count_set: HashMap<&str, usize>,
    smaller_count_set: HashMap<&str, usize>,
) -> bool {
    for (cid, &smaller_count) in &smaller_count_set {
        debug_assert!(smaller_count > 0);

        let larger_count = larger_count_set.get(cid).cloned().unwrap_or_default();
        if larger_count < smaller_count {
            return false;
        }
    }
    true
}

struct PeerInfo<'data> {
    /// A peer's public key.
    public_key: &'data PublicKey,
    /// A peer's signature.
    signature: &'data Signature,
    /// Sorted vector of CIDs that belong to the peer.
    cids: Vec<Rc<CidRef>>,
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
