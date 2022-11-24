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
    ephemeral::{Network, PeerId},
    queue::ExecutionQueue,
    services::MarineServiceHandle,
    transform::walker::TransformedAirScript,
};

use air_test_utils::{test_runner::TestRunParameters, RawAVMOutcome};

use std::{borrow::Borrow, hash::Hash, rc::Rc};

/// A particle execution. Several executors (particles) may share same TransformedAirScript
/// and its state.
pub struct ParticleExecutor {
    transformed_air_script: TransformedAirScript,
    test_parameters: TestRunParameters,
    queue: ExecutionQueue,
}

impl ParticleExecutor {
    pub fn from_transformed_air_script(
        test_parameters: TestRunParameters,
        transformed_air_script: TransformedAirScript,
    ) -> Result<Self, String> {
        let network = transformed_air_script.get_network();
        let init_peer_id = test_parameters.init_peer_id.as_str();
        network.ensure_peer(init_peer_id);

        let queue = ExecutionQueue::new();
        // Seed execution
        queue.distribute_to_peers(&network, &[init_peer_id], &<_>::default());

        Ok(Self {
            transformed_air_script,
            test_parameters,
            queue,
        })
    }

    /// Create execution from the annotated air script.
    ///
    /// `extra_peers` allows you to define peers that are not mentioned in the annotated script
    /// explicitly, but are used, e.g. if their names are returned from a call.
    pub fn new(
        test_parameters: TestRunParameters,
        common_services: Vec<MarineServiceHandle>,
        extra_peers: impl IntoIterator<Item = PeerId>,
        annotated_air_script: &str,
    ) -> Result<Self, String> {
        let network = Network::new(extra_peers.into_iter(), common_services);
        let transformed = TransformedAirScript::new(annotated_air_script, network)?;

        Self::from_transformed_air_script(test_parameters, transformed)
    }

    /// Simple constructor where everything is generated from the annotated_air_script.
    pub fn simple(
        test_parameters: TestRunParameters,
        annotated_air_script: &str,
    ) -> Result<Self, String> {
        Self::new(
            test_parameters,
            vec![],
            std::iter::empty(),
            annotated_air_script,
        )
    }

    pub fn from_network(
        test_parameters: TestRunParameters,
        network: Rc<Network>,
        annotated_air_script: &str,
    ) -> Result<Self, String> {
        let transformed = TransformedAirScript::new(annotated_air_script, network)?;

        Self::from_transformed_air_script(test_parameters, transformed)
    }

    /// Return Iterator for handling all the queued datas
    /// for particular peer_id.
    pub fn execution_iter<'s, Id>(
        &'s self,
        peer_id: &Id,
    ) -> Option<impl Iterator<Item = RawAVMOutcome> + 's>
    where
        PeerId: Borrow<Id>,
        Id: Eq + Hash + ?Sized,
    {
        self.queue.execution_iter(
            &self.transformed_air_script,
            self.transformed_air_script.get_network(),
            &self.test_parameters,
            peer_id,
        )
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
            .map(|mut it| it.next().expect("Nothing to execute"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::MarineService;

    use air_test_utils::prelude::*;
    use pretty_assertions::assert_eq;

    use std::cell::RefCell;

    #[test]
    fn test_execution() {
        let exec = ParticleExecutor::new(
            TestRunParameters::from_init_peer_id("init_peer_id"),
            vec![],
            std::iter::empty(),
            r#"(seq
(call "peer1" ("service" "func") [] arg) ; ok=42
(call "peer2" ("service" "func") [arg]) ; ok=43
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
        let exec = ParticleExecutor::new(
            TestRunParameters::from_init_peer_id("init_peer_id"),
            vec![],
            std::iter::empty(),
            r#"(seq
(call "peer1" ("service" "func") [] arg) ; err = {"ret_code":0,"result":42}
(call "peer2" ("service" "func") [arg]) ; ok = 43
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
        let exec = ParticleExecutor::new(
            TestRunParameters::from_init_peer_id("init_peer_id"),
            vec![],
            std::iter::empty(),
            r#"(seq
(call "peer1" ("service" "func") [] arg) ; err = {"ret_code":12,"result":"ERROR MESSAGE"}
(call "peer2" ("service" "func") [arg]) ; ok = 43
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
    fn test_seq_ok() {
        let exec = ParticleExecutor::new(
            TestRunParameters::from_init_peer_id("init_peer_id"),
            vec![],
            IntoIterator::into_iter(["peer2", "peer3"]).map(Into::into),
            r#"(seq
  (seq
    (call "peer1" ("service" "func") [] var)  ; ok = [{"p":"peer2","v":2},{"p":"peer3","v":3}]
    (seq
      (ap 1 k)
      (fold var i
        (seq
          (call i.$.p ("service" "func") [i k] k)  ; seq_ok = {"0":12,"default":42}
          (next i)))))
  (call "init_peer_id" ("a" "b") []) ; ok = 0
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
    fn test_map() {
        let exec = ParticleExecutor::new(
            TestRunParameters::from_init_peer_id("peer1"),
            vec![],
            IntoIterator::into_iter(["peer2", "peer3"]).map(Into::into),
            r#"
(seq
  (call "peer1" ("" "") [] peers) ; ok = ["peer2", "peer3"]
  (fold peers p
    (seq
      (call p ("" "") [p]) ; map = {"peer2": 42, "peer3": 43}
      (next p)
)))
"#,
        )
        .unwrap();

        let result_init: Vec<_> = exec.execution_iter("peer1").unwrap().collect();

        assert_eq!(result_init.len(), 1);
        let outcome1 = &result_init[0];
        assert_eq!(outcome1.ret_code, 0);
        assert_eq!(outcome1.error_message, "");
        assert_next_pks!(&outcome1.next_peer_pks, ["peer2"]);

        {
            let results2 = exec.execute_all("peer2").unwrap();
            assert_eq!(results2.len(), 1);
            let outcome2 = &results2[0];
            assert_eq!(outcome2.ret_code, 0, "{:?}", outcome2);
            assert!(exec.execution_iter("peer2").unwrap().next().is_none());
            assert_next_pks!(&outcome2.next_peer_pks, ["peer3"]);
        }

        {
            let results3 = exec.execute_all("peer3").unwrap();
            assert_eq!(results3.len(), 1);
            let outcome3 = &results3[0];
            assert_eq!(outcome3.ret_code, 0, "{:?}", outcome3);
            assert_next_pks!(&outcome3.next_peer_pks, []);

            let trace = trace_from_result(outcome3);

            assert_eq!(
                &*trace,
                vec![
                    executed_state::scalar(json!(["peer2", "peer3"])),
                    executed_state::scalar(json!(42)),
                    executed_state::scalar(json!(43)),
                ]
            );
        }
    }

    #[test]
    #[should_panic]
    fn test_map_no_arg() {
        let exec = ParticleExecutor::new(
            TestRunParameters::from_init_peer_id("peer1"),
            vec![],
            IntoIterator::into_iter(["peer2", "peer3"]).map(Into::into),
            r#"
(call "peer1" ("" "") [] p) ; map = {"any": "key"}
"#,
        )
        .unwrap();
        let _result_init: Vec<_> = exec.execution_iter("peer1").unwrap().collect();
    }

    #[test]
    fn test_seq_error() {
        let exec = ParticleExecutor::new(
            TestRunParameters::from_init_peer_id("init_peer_id"),
            vec![],
            IntoIterator::into_iter(["peer2", "peer3"]).map(Into::into),
            r#"(seq
  (seq
    (call "peer1" ("service" "func") [] var)  ; ok = [{"p":"peer2","v":2},{"p":"peer3","v":3}, {"p":"peer4"}]
    (seq
      (ap 1 k)
      (fold var i
        (seq
          (call i.$.p ("service" "func") [i.$.v k] k)  ; seq_error = {"0":{"ret_code":0,"result":12},"default":{"ret_code":1,"result":42}}
          (next i)))))
  (call "init_peer_id" ("a" "b") []) ; ok = 0
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
                    scalar(json!([{"p":"peer2","v":2},{"p":"peer3","v":3},{"p":"peer4"}])),
                    scalar_number(12),
                    request_sent_by("peer2"),
                ])
            );
        }

        {
            let results3: Vec<_> = exec.execute_all("peer3").unwrap();
            assert_eq!(results3.len(), 1);
            // TODO why doesn't it fail?
            let outcome3 = &results3[0];
            assert_eq!(outcome3.ret_code, 0, "{:?}", outcome3);
            assert!(exec.execution_iter("peer3").unwrap().next().is_none());

            let trace = trace_from_result(outcome3);
            assert_eq!(
                trace,
                ExecutionTrace::from(vec![
                    scalar(json!([{"p":"peer2","v":2},{"p":"peer3","v":3},{"p":"peer4"}])),
                    scalar_number(12),
                    request_sent_by("peer2"),
                ])
            );
        }
    }

    #[test]
    fn test_echo() {
        let exec = ParticleExecutor::new(
            TestRunParameters::from_init_peer_id("init_peer_id"),
            vec![],
            std::iter::empty(),
            r#"(seq
(call "peer1" ("service" "func") [1 22] arg) ; behaviour=echo
(call "peer2" ("service" "func") [arg]) ; ok = 43
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

    #[test]
    fn test_transformed_distinct() {
        let peer = "peer1";
        let network = Network::empty();

        let transformed1 = TransformedAirScript::new(
            &f!(r#"(call "{}" ("service" "function") []) ; ok = 42"#, peer),
            network.clone(),
        )
        .unwrap();
        let exectution1 = ParticleExecutor::from_transformed_air_script(
            TestRunParameters::from_init_peer_id(peer),
            transformed1,
        )
        .unwrap();

        let transformed2 = TransformedAirScript::new(
            &f!(r#"(call "{}" ("service" "function") []) ; ok = 24"#, peer),
            network.clone(),
        )
        .unwrap();
        let exectution2 = ParticleExecutor::from_transformed_air_script(
            TestRunParameters::from_init_peer_id(peer),
            transformed2,
        )
        .unwrap();

        let trace1 = exectution1.execute_one(peer).unwrap();
        let trace2 = exectution2.execute_one(peer).unwrap();

        assert_eq!(
            trace_from_result(&trace1),
            ExecutionTrace::from(vec![scalar_number(42)]),
        );
        assert_eq!(
            trace_from_result(&trace2),
            ExecutionTrace::from(vec![scalar_number(24)]),
        );
    }

    #[test]
    fn test_transformed_shared() {
        struct Service {
            state: RefCell<std::vec::IntoIter<JValue>>,
        }

        impl MarineService for Service {
            fn call(&self, _params: CallRequestParams) -> crate::services::FunctionOutcome {
                let mut cell = self.state.borrow_mut();
                crate::services::FunctionOutcome::ServiceResult(
                    CallServiceResult::ok(cell.next().unwrap()),
                    <_>::default(),
                )
            }
        }
        let service = Service {
            state: vec![json!(42), json!(24)].into_iter().into(),
        };
        let network = Network::new(std::iter::empty::<PeerId>(), vec![service.to_handle()]);

        let peer = "peer1";
        let air_script = f!(r#"(call "{}" ("service" "function") [])"#, peer);
        let transformed1 = TransformedAirScript::new(&air_script, network.clone()).unwrap();
        let exectution1 = ParticleExecutor::from_transformed_air_script(
            TestRunParameters::from_init_peer_id(peer),
            transformed1,
        )
        .unwrap();

        let transformed2 = TransformedAirScript::new(&air_script, network.clone()).unwrap();
        let exectution2 = ParticleExecutor::from_transformed_air_script(
            TestRunParameters::from_init_peer_id(peer),
            transformed2,
        )
        .unwrap();

        let trace1 = exectution1.execute_one(peer).unwrap();
        let trace2 = exectution2.execute_one(peer).unwrap();

        assert_eq!(
            trace_from_result(&trace1),
            ExecutionTrace::from(vec![scalar_number(42)]),
        );
        assert_eq!(
            trace_from_result(&trace2),
            ExecutionTrace::from(vec![scalar_number(24)]),
        );
    }

    #[test]
    fn test_invalid_air() {
        let res = ParticleExecutor::new(
            TestRunParameters::from_init_peer_id("init_peer_id"),
            vec![],
            std::iter::empty(),
            r#"(seq
(call "peer1" ("service" "func") [1 22] arg) ; behaviour=echo
)
"#,
        );

        match &res {
            Ok(_) => {
                assert!(res.is_err());
            }
            Err(err) => {
                assert_eq!(
                    err,
                    "error: \n  ┌─ script.air:3:1\n  │\n3 │ )\n  │ ^ expected \"(\"\n\n"
                );
            }
        }
    }
}
