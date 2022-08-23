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

use air_test_utils::{test_runner::TestRunParameters, RawAVMOutcome};
use itertools::Itertools;

use crate::{
    ephemeral::{Network, Peer, PeerId},
    services::{results::ResultService, Service},
    transform::{walker::Transformer, Sexp},
};

use std::{borrow::Borrow, collections::HashMap, hash::Hash, rc::Rc, str::FromStr};

pub struct TestExecutor {
    pub air_script: String,
    pub network: Network,
}

impl TestExecutor {
    /// Create execution from the annotated air script
    pub fn new(
        test_parameters: TestRunParameters,
        common_services: Vec<Rc<dyn Service>>,
        extra_peers: impl IntoIterator<Item = PeerId>,
        annotated_air_script: &str,
    ) -> Result<Self, String> {
        let mut sexp = Sexp::from_str(annotated_air_script)?;
        let mut walker = Transformer::new();
        walker.transform(&mut sexp);

        let init_peer_id = test_parameters.init_peer_id.clone();
        let transformed_air_script = sexp.to_string();

        let peers = build_peers(
            common_services,
            walker.results,
            walker.peers,
            PeerId::new(init_peer_id.clone()),
            extra_peers,
        );

        let network = Network::from_peers(test_parameters, peers);
        // Seed execution
        network.distribute_to_peers(&[init_peer_id], &vec![]);

        Ok(TestExecutor {
            air_script: transformed_air_script,
            network,
        })
    }

    /// Return Iterator for handling all the queued datas
    /// for particular peer_id.
    pub fn iter_execution<'s, Id>(
        &'s self,
        peer_id: &Id,
    ) -> Option<impl Iterator<Item = RawAVMOutcome> + 's>
    where
        PeerId: Borrow<Id>,
        // TODO it's not clear why compiler requies + 's here, but not at Network::iter_execution
        Id: Eq + Hash + ?Sized + 's,
    {
        self.network.iter_execution(&self.air_script, peer_id)
    }

    /// Process all queued datas, panicing on error.
    pub fn execute_all<Id>(&self, peer_id: &Id) -> Option<Vec<RawAVMOutcome>>
    where
        PeerId: Borrow<Id>,
        Id: Eq + Hash + ?Sized,
    {
        self.iter_execution(peer_id).map(|it| it.collect())
    }

    /// Process one queued data, panicing if it is unavalable or on error.
    pub fn execute_one<Id>(&self, peer_id: &Id) -> Option<RawAVMOutcome>
    where
        PeerId: Borrow<Id>,
        Id: Eq + Hash + ?Sized,
    {
        self.iter_execution(peer_id)
            .map(|mut it| it.next().unwrap())
    }
}

fn build_peers(
    common_services: Vec<Rc<dyn Service>>,
    results: std::collections::HashMap<u32, serde_json::Value>,
    known_peers: std::collections::HashSet<PeerId>,
    init_peer_id: PeerId,
    extra_peers: impl IntoIterator<Item = PeerId>,
) -> Vec<Peer> {
    let mut result_services: Vec<Rc<dyn Service>> = Vec::with_capacity(1 + common_services.len());
    result_services.push(Rc::new(ResultService::new(results)));
    result_services.extend_from_slice(&common_services);
    let result_services = Rc::<[_]>::from(result_services);

    let common_services = Rc::<[_]>::from(common_services);

    let extra_peers_pairs = extra_peers
        .into_iter()
        .chain(std::iter::once(init_peer_id))
        .map(|peer_id| (peer_id.clone(), Peer::new(peer_id, common_services.clone())));
    let mut peers = extra_peers_pairs.collect::<HashMap<_, _>>();

    let known_peers_pairs = known_peers
        .into_iter()
        .map(|peer_id| (peer_id.clone(), Peer::new(peer_id, result_services.clone())));
    peers.extend(known_peers_pairs);

    peers.into_values().collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution() {
        let exec = TestExecutor::new(
            TestRunParameters::from_init_peer_id("init_peer_id"),
            vec![],
            std::iter::empty(),
            r#"(seq
(call "peer1" ("service" "func") [] arg) ; result=42
(call "peer2" ("service" "func") [arg]) ; result=43
)
"#,
        )
        .unwrap();

        let result_init: Vec<_> = exec.iter_execution("init_peer_id").unwrap().collect();

        assert_eq!(result_init.len(), 1);
        let outcome = &result_init[0];
        assert_eq!(outcome.next_peer_pks, vec!["peer1".to_owned()]);

        assert!(exec.iter_execution("peer2").unwrap().next().is_none());
        let results1: Vec<_> = exec.iter_execution("peer1").unwrap().collect();
        assert_eq!(results1.len(), 1);
        let outcome1 = &results1[0];
        assert_eq!(outcome1.ret_code, 0);
        assert!(exec.iter_execution("peer1").unwrap().next().is_none());

        let outcome2 = exec.execute_one("peer2").unwrap();
        assert_eq!(outcome2.ret_code, 0);
    }

    #[test]
    fn test_error() {
        let exec = TestExecutor::new(
            TestRunParameters::from_init_peer_id("init_peer_id"),
            vec![],
            std::iter::empty(),
            r#"(seq
(call "peer1" ("service" "func") [] arg) ; result = {"ret_code":12,"result":"ERROR MESSAGE"}
(call "peer2" ("service" "func") [arg]) ; result = 43
)
"#,
        )
        .unwrap();

        let result_init: Vec<_> = exec.iter_execution("init_peer_id").unwrap().collect();

        assert_eq!(result_init.len(), 1);
        let outcome1 = &result_init[0];
        assert_eq!(outcome1.ret_code, 0);
        assert_eq!(outcome1.error_message, "");

        assert!(exec.iter_execution("peer2").unwrap().next().is_none());
        let results1: Vec<_> = exec.iter_execution("peer1").unwrap().collect();
        assert_eq!(results1.len(), 1);
        let outcome1 = &results1[0];
        assert_eq!(outcome1.ret_code, 10000, "{:?}", outcome1);
        assert_eq!(
            outcome1.error_message,
            "Local service error, ret_code is 12, error message is '\"ERROR MESSAGE\"'",
            "{:?}",
            outcome1
        );
        assert!(exec.iter_execution("peer1").unwrap().next().is_none());

        let results2: Vec<_> = exec.iter_execution("peer2").unwrap().collect();
        assert_eq!(results2.len(), 0);
    }
}
