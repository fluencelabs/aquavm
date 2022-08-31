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

use crate::{
    asserts::ServiceDefinition,
    ephemeral::{Network, Peer, PeerId},
    services::{results::ResultService, MarineService, MarineServiceHandle},
    transform::{walker::Transformer, Sexp},
};

use air_test_utils::{test_runner::TestRunParameters, RawAVMOutcome};

use std::{borrow::Borrow, collections::HashMap, hash::Hash, rc::Rc, str::FromStr};

pub struct TestExecutor {
    pub air_script: String,
    pub network: Network,
}

impl TestExecutor {
    /// Create execution from the annotated air script.
    pub fn new(
        test_parameters: TestRunParameters,
        common_services: Vec<MarineServiceHandle>,
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
        )?;

        let network = Network::from_peers(test_parameters, peers);
        // Seed execution
        network.distribute_to_peers(&[init_peer_id], &vec![]);

        Ok(TestExecutor {
            air_script: transformed_air_script,
            network,
        })
    }

    /// Simple constructor where everything is generated from the annotated_air_script.
    pub fn simple(
        test_parameters: TestRunParameters,
        annotated_air_script: &str,
    ) -> Result<Self, String> {
        Self::new(
            test_parameters,
            <_>::default(),
            std::iter::empty(),
            annotated_air_script,
        )
    }

    /// Return Iterator for handling all the queued datas
    /// for particular peer_id.
    pub fn execution_iter<'s, Id>(
        &'s self,
        peer_id: &Id,
    ) -> Option<impl Iterator<Item = RawAVMOutcome> + 's>
    where
        PeerId: Borrow<Id>,
        // TODO it's not clear why compiler requies + 's here, but not at Network::iter_execution
        Id: Eq + Hash + ?Sized + 's,
    {
        self.network.execution_iter(&self.air_script, peer_id)
    }

    /// Process all queued datas, panicing on error.
    pub fn execute_all<Id>(&self, peer_id: &Id) -> Option<Vec<RawAVMOutcome>>
    where
        PeerId: Borrow<Id>,
        Id: Eq + Hash + ?Sized,
    {
        self.execution_iter(peer_id).map(|it| it.collect())
    }

    /// Process one queued data, panicing if it is unavalable or on error.
    pub fn execute_one<Id>(&self, peer_id: &Id) -> Option<RawAVMOutcome>
    where
        PeerId: Borrow<Id>,
        Id: Eq + Hash + ?Sized,
    {
        self.execution_iter(peer_id)
            .map(|mut it| it.next().unwrap())
    }
}

fn build_peers(
    common_services: Vec<MarineServiceHandle>,
    results: std::collections::HashMap<u32, ServiceDefinition>,
    known_peers: std::collections::HashSet<PeerId>,
    init_peer_id: PeerId,
    extra_peers: impl IntoIterator<Item = PeerId>,
) -> Result<Vec<Peer>, String> {
    let mut result_services: Vec<MarineServiceHandle> =
        Vec::with_capacity(1 + common_services.len());
    result_services.push(ResultService::new(results)?.to_handle());
    result_services.extend(common_services);
    let result_services = Rc::<[_]>::from(result_services);

    let extra_peers_pairs = extra_peers
        .into_iter()
        .chain(std::iter::once(init_peer_id))
        .map(|peer_id| (peer_id.clone(), Peer::new(peer_id, result_services.clone())));
    let mut peers = extra_peers_pairs.collect::<HashMap<_, _>>();

    let known_peers_pairs = known_peers
        .into_iter()
        .map(|peer_id| (peer_id.clone(), Peer::new(peer_id, result_services.clone())));
    peers.extend(known_peers_pairs);

    Ok(peers.into_values().collect())
}

#[cfg(test)]
mod tests {
    use air_test_utils::prelude::*;

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

        let result_init: Vec<_> = exec.execution_iter("init_peer_id").unwrap().collect();

        assert_eq!(result_init.len(), 1);
        let outcome = &result_init[0];
        assert_eq!(outcome.next_peer_pks, vec!["peer1".to_owned()]);

        assert!(exec.execution_iter("peer2").unwrap().next().is_none());
        let results1: Vec<_> = exec.execution_iter("peer1").unwrap().collect();
        assert_eq!(results1.len(), 1);
        let outcome1 = &results1[0];
        assert_eq!(outcome1.ret_code, 0);
        assert!(exec.execution_iter("peer1").unwrap().next().is_none());

        let outcome2 = exec.execute_one("peer2").unwrap();
        assert_eq!(outcome2.ret_code, 0);
    }

    #[test]
    fn test_call_result_success() {
        let exec = TestExecutor::new(
            TestRunParameters::from_init_peer_id("init_peer_id"),
            vec![],
            std::iter::empty(),
            r#"(seq
(call "peer1" ("service" "func") [] arg) ; call_result = {"ret_code":0,"result":42}
(call "peer2" ("service" "func") [arg]) ; result = 43
)
"#,
        )
        .unwrap();

        let result_init: Vec<_> = exec.execution_iter("init_peer_id").unwrap().collect();

        assert_eq!(result_init.len(), 1);
        let outcome1 = &result_init[0];
        assert_eq!(outcome1.ret_code, 0);
        assert_eq!(outcome1.error_message, "");

        assert!(exec.execution_iter("peer2").unwrap().next().is_none());
        let results1: Vec<_> = exec.execution_iter("peer1").unwrap().collect();
        assert_eq!(results1.len(), 1);
        let outcome1 = &results1[0];
        assert_eq!(outcome1.ret_code, 0, "{:?}", outcome1);
        assert!(exec.execution_iter("peer1").unwrap().next().is_none());
    }

    #[test]
    fn test_call_result_error() {
        let exec = TestExecutor::new(
            TestRunParameters::from_init_peer_id("init_peer_id"),
            vec![],
            std::iter::empty(),
            r#"(seq
(call "peer1" ("service" "func") [] arg) ; call_result = {"ret_code":12,"result":"ERROR MESSAGE"}
(call "peer2" ("service" "func") [arg]) ; result = 43
)
"#,
        )
        .unwrap();

        let result_init: Vec<_> = exec.execution_iter("init_peer_id").unwrap().collect();

        assert_eq!(result_init.len(), 1);
        let outcome1 = &result_init[0];
        assert_eq!(outcome1.ret_code, 0);
        assert_eq!(outcome1.error_message, "");

        assert!(exec.execution_iter("peer2").unwrap().next().is_none());
        let results1: Vec<_> = exec.execution_iter("peer1").unwrap().collect();
        assert_eq!(results1.len(), 1);
        let outcome1 = &results1[0];
        assert_eq!(outcome1.ret_code, 10000, "{:?}", outcome1);
        assert_eq!(
            outcome1.error_message,
            "Local service error, ret_code is 12, error message is '\"ERROR MESSAGE\"'",
            "{:?}",
            outcome1
        );
        assert!(exec.execution_iter("peer1").unwrap().next().is_none());

        let results2: Vec<_> = exec.execution_iter("peer2").unwrap().collect();
        assert_eq!(results2.len(), 0);
    }

    #[test]
    fn test_seq_result() {
        let exec = TestExecutor::new(
            TestRunParameters::from_init_peer_id("init_peer_id"),
            vec![],
            IntoIterator::into_iter(["peer2", "peer3"]).map(Into::into),
            r#"(seq
  (seq
    (call "peer1" ("service" "func") [] var)  ; result = [{"p":"peer2","v":2},{"p":"peer3","v":3}]
    (seq
      (ap 1 k)
      (fold var i
        (seq
          (call i.$.p ("service" "func") [i k] k)  ; seq_result = {"0":12,"default":42}
          (next i)))))
  (call "init_peer_id" ("a" "b") []) ; result = 0
)"#,
        )
        .unwrap();

        let result_init: Vec<_> = exec.execution_iter("init_peer_id").unwrap().collect();

        assert_eq!(result_init.len(), 1);
        let outcome1 = &result_init[0];
        assert_eq!(outcome1.ret_code, 0);
        assert_eq!(outcome1.error_message, "");

        assert!(exec.execution_iter("peer2").unwrap().next().is_none());
        {
            let results1 = exec.execute_all("peer1").unwrap();
            assert_eq!(results1.len(), 1);
            let outcome1 = &results1[0];
            assert_eq!(outcome1.ret_code, 0, "{:?}", outcome1);
            assert!(exec.execution_iter("peer1").unwrap().next().is_none());
            assert_next_pks!(&outcome1.next_peer_pks, ["peer2"]);
        }

        {
            let results2: Vec<_> = exec.execute_all("peer2").unwrap();
            assert_eq!(results2.len(), 1);
            let outcome2 = &results2[0];
            assert_eq!(outcome2.ret_code, 0, "{:?}", outcome2);
            assert!(exec.execution_iter("peer2").unwrap().next().is_none());
            assert_next_pks!(&outcome2.next_peer_pks, ["peer3"]);

            let trace = trace_from_result(outcome2);
            assert_eq!(
                trace,
                ExecutionTrace::from(vec![
                    scalar(json!([{"p":"peer2","v":2},{"p":"peer3","v":3},])),
                    scalar_number(12),
                    request_sent_by("peer2"),
                ])
            );
        }

        {
            let results3: Vec<_> = exec.execute_all("peer3").unwrap();
            assert_eq!(results3.len(), 1);
            let outcome3 = &results3[0];
            assert_eq!(outcome3.ret_code, 0, "{:?}", outcome3);
            assert!(exec.execution_iter("peer3").unwrap().next().is_none());

            let trace = trace_from_result(outcome3);
            assert_eq!(
                trace,
                ExecutionTrace::from(vec![
                    scalar(json!([{"p":"peer2","v":2},{"p":"peer3","v":3},])),
                    scalar_number(12),
                    request_sent_by("peer2"),
                ])
            );
        }
    }

    #[test]
    fn test_echo() {
        let exec = TestExecutor::new(
            TestRunParameters::from_init_peer_id("init_peer_id"),
            vec![],
            std::iter::empty(),
            r#"(seq
(call "peer1" ("service" "func") [1 22] arg) ; behaviour=echo
(call "peer2" ("service" "func") [arg]) ; result = 43
)
"#,
        )
        .unwrap();

        let result_init: Vec<_> = exec.execution_iter("init_peer_id").unwrap().collect();

        assert_eq!(result_init.len(), 1);
        let outcome0 = &result_init[0];
        assert_eq!(outcome0.ret_code, 0);
        assert_eq!(outcome0.error_message, "");

        assert!(exec.execution_iter("peer2").unwrap().next().is_none());
        let results1: Vec<_> = exec.execution_iter("peer1").unwrap().collect();
        assert_eq!(results1.len(), 1);
        let outcome1 = &results1[0];
        assert_eq!(outcome1.ret_code, 0, "{:?}", outcome1);
        assert!(exec.execution_iter("peer1").unwrap().next().is_none());

        assert_eq!(
            trace_from_result(outcome1),
            ExecutionTrace::from(vec![scalar_number(1), request_sent_by("peer1"),]),
        )
    }
}
