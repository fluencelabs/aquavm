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
    ephemeral::{Data, Network, PeerId},
    queue::ExecutionQueue,
    services::MarineServiceHandle,
    transform::walker::TransformedAirScript,
};

use air_test_utils::{
    test_runner::{AirRunner, DefaultAirRunner, TestInitParameters, TestRunParameters},
    RawAVMOutcome,
};

use std::{borrow::Borrow, hash::Hash, rc::Rc};

/// A executor for an AIR script. Several executors may share same TransformedAirScript
/// and its state.
pub struct AirScriptExecutor<R = DefaultAirRunner> {
    transformed_air_script: TransformedAirScript<R>,
    test_parameters: TestRunParameters,
    queue: ExecutionQueue,
}

// it is implemented only for the default runner for compatibility reasons
// Rust fails to deduce type for `AirScriptExecutor::simple()` without
//   extencive test code changes
impl AirScriptExecutor<DefaultAirRunner> {
    /// Simple constructor where everything is generated from the annotated_air_script.
    pub fn from_annotated(
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
}

impl<R: AirRunner> AirScriptExecutor<R> {
    pub fn from_transformed_air_script(
        mut test_parameters: TestRunParameters,
        test_init_parameters: TestInitParameters,
        transformed_air_script: TransformedAirScript<R>,
    ) -> Result<Self, String> {
        let network = transformed_air_script.get_network();
        let init_peer_id = test_parameters.init_peer_id.clone();
        let real_init_peer_id =
            network.ensure_named_peer(init_peer_id.as_str(), test_init_parameters);
        test_parameters.init_peer_id = real_init_peer_id.to_string();

        let queue = ExecutionQueue::new();
        // Seed execution
        queue.distribute_to_peers(&network, &[real_init_peer_id], &<_>::default());

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
        let network = Network::new(extra_peers.into_iter(), common_services, <_>::default());
        let transformed = TransformedAirScript::new(annotated_air_script, network, <_>::default())?;

        Self::from_transformed_air_script(test_parameters, <_>::default(), transformed)
    }

    pub fn from_network(
        test_parameters: TestRunParameters,
        test_init_parameters: TestInitParameters,
        network: Rc<Network<R>>,
        annotated_air_script: &str,
    ) -> Result<Self, String> {
        let transformed =
            TransformedAirScript::new(annotated_air_script, network, test_init_parameters)?;

        Self::from_transformed_air_script(test_parameters, test_init_parameters, transformed)
    }

    pub fn new_with_init_parameters(
        test_parameters: TestRunParameters,
        test_init_parameters: TestInitParameters,
        common_services: Vec<MarineServiceHandle>,
        extra_peers: impl IntoIterator<Item = PeerId>,
        annotated_air_script: &str,
    ) -> Result<Self, String> {
        let network = Network::new(
            extra_peers.into_iter(),
            common_services,
            test_init_parameters,
        );
        let transformed =
            TransformedAirScript::new(annotated_air_script, network, test_init_parameters)?;

        Self::from_transformed_air_script(test_parameters, test_init_parameters, transformed)
    }

    /// Return Iterator for handling all the queued datas
    /// for particular peer_id.
    pub fn execution_iter<'s, Id>(
        &'s self,
        peer_id: &Id,
    ) -> Option<impl Iterator<Item = RawAVMOutcome> + 's>
    where
        PeerId: Borrow<Id> + for<'a> From<&'a Id>,
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
        PeerId: Borrow<Id> + for<'a> From<&'a Id>,
        Id: Eq + Hash + ?Sized,
    {
        self.execution_iter(peer_id).map(|it| it.collect())
    }

    /// Process one queued data, panicing if it is unavalable or on error.
    pub fn execute_one<Id>(&self, peer_id: &Id) -> Option<RawAVMOutcome>
    where
        PeerId: Borrow<Id> + for<'a> From<&'a Id>,
        Id: Eq + Hash + ?Sized,
    {
        self.execution_iter(peer_id)
            .map(|mut it| it.next().expect("Nothing to execute"))
    }

    /// Push data into peer's queue.
    pub fn enqueue(&self, peer_id: impl Into<PeerId>, data: Data) {
        let queue_cell = self.queue.get_peer_queue_cell(peer_id.into());
        queue_cell.push_data(data);
    }

    pub fn get_prev_data(&self, peer_id: impl Into<PeerId>) -> Data {
        let queue_cell = self.queue.get_peer_queue_cell(peer_id.into());
        queue_cell.take_prev_data()
    }

    pub fn resolve_name(&self, name: &str) -> PeerId {
        self.transformed_air_script.get_network().resolve_peer(name)
    }

    pub fn get_network(&self) -> Rc<Network<R>> {
        self.transformed_air_script.get_network()
    }

    pub fn get_transformed_air_script(&self) -> &TransformedAirScript<R> {
        &self.transformed_air_script
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::MarineService;

    use air_test_utils::{key_utils::derive_dummy_keypair, prelude::*};
    use pretty_assertions::assert_eq;

    use std::cell::RefCell;

    #[test]
    fn test_execution() {
        let peer1_name = "peer1";
        let peer2_name = "peer2";
        let init_peer_name = "init_peer_id";

        let exec = AirScriptExecutor::<NativeAirRunner>::new(
            TestRunParameters::from_init_peer_id(init_peer_name),
            vec![],
            std::iter::empty(),
            &format!(
                r#"(seq
(call "{peer1_name}" ("service" "func") [] arg) ; ok=42
(call "{peer2_name}" ("service" "func") [arg]) ; ok=43
)
"#
            ),
        )
        .unwrap();

        let peer1_id = exec.resolve_name(peer1_name).to_string();

        let result_init: Vec<_> = exec.execution_iter(init_peer_name).unwrap().collect();

        assert_eq!(result_init.len(), 1);
        let outcome = &result_init[0];
        assert_eq!(outcome.next_peer_pks, vec![peer1_id.clone()]);

        assert!(exec.execution_iter(peer2_name).unwrap().next().is_none());
        let results1: Vec<_> = exec.execution_iter(peer1_name).unwrap().collect();
        assert_eq!(results1.len(), 1);
        let outcome1 = &results1[0];
        assert_eq!(outcome1.ret_code, 0);
        assert!(exec.execution_iter(peer1_name).unwrap().next().is_none());

        let outcome2 = exec.execute_one(peer2_name).unwrap();
        assert_eq!(outcome2.ret_code, 0);
    }

    #[test]
    fn test_call_result_success() {
        let exec = AirScriptExecutor::<NativeAirRunner>::new(
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
        let script = r#"
        (seq
            (call "peer1" ("service" "func") [] arg) ; err = {"ret_code":12,"result":"ERROR MESSAGE"}
            (call "peer2" ("service" "func") [arg]) ; ok = 43
        )
        "#;
        let exec = AirScriptExecutor::<NativeAirRunner>::new(
            TestRunParameters::from_init_peer_id("init_peer_id"),
            vec![],
            std::iter::empty(),
            script,
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
        let init_peer_name = "init_peer_id";
        let peer1_name = "peer1";
        let peer2_name = "peer2";
        let peer3_name = "peer3";
        let (_peer2_pk, peer2_id) = derive_dummy_keypair(peer2_name);
        let (_peer3_pk, peer3_id) = derive_dummy_keypair(peer3_name);

        let exec = AirScriptExecutor::<NativeAirRunner>::new(
            TestRunParameters::from_init_peer_id(init_peer_name),
            vec![],
            IntoIterator::into_iter([peer2_name, peer3_name]).map(Into::into),
            &format!(r#"(seq
  (seq
    (call "{peer1_name}" ("service" "func") [] var)  ; ok = [{{"p":"{peer2_id}","v":2}},{{"p":"{peer3_id}","v":3}}]
    (seq
      (ap 1 k)
      (fold var i
        (seq
          (call i.$.p ("service" "func") [i k] k)  ; seq_ok = {{"0":12,"default":42}}
          (next i)))))
  (call "init_peer_id" ("a" "b") []) ; ok = 0
)"#),
        )
        .unwrap();

        let result_init: Vec<_> = exec.execution_iter(init_peer_name).unwrap().collect();

        assert_eq!(result_init.len(), 1);
        let outcome1 = &result_init[0];
        assert_eq!(outcome1.ret_code, 0);
        assert_eq!(outcome1.error_message, "");

        assert!(exec.execution_iter(peer2_name).unwrap().next().is_none());
        {
            let results1 = exec.execute_all(peer1_name).unwrap();
            assert_eq!(results1.len(), 1);
            let outcome1 = &results1[0];
            assert_eq!(outcome1.ret_code, 0, "{:?}", outcome1);
            assert!(exec.execution_iter(peer1_name).unwrap().next().is_none());
            assert_next_pks!(&outcome1.next_peer_pks, [peer2_id.as_str()]);
        }

        {
            let results2: Vec<_> = exec.execute_all(peer2_name).unwrap();
            assert_eq!(results2.len(), 1);
            let outcome2 = &results2[0];
            assert_eq!(outcome2.ret_code, 0, "{:?}", outcome2);
            assert!(exec.execution_iter(peer2_name).unwrap().next().is_none());
            assert_next_pks!(&outcome2.next_peer_pks, [peer3_id.as_str()]);

            let trace = trace_from_result(outcome2);
            assert_eq!(
                trace,
                ExecutionTrace::from(vec![
                    scalar!(
                        json!([{"p":peer2_id,"v":2},{"p":peer3_id,"v":3}]),
                        peer_name = &peer1_name,
                        service = "service..0",
                        function = "func"
                    ),
                    scalar!(
                        12,
                        peer_name = &peer2_name,
                        service = "service..1",
                        function = "func",
                        args = vec![json!({"p":peer2_id,"v":2}), json!(1)]
                    ),
                    request_sent_by(peer2_id.clone()),
                ])
            );
        }

        {
            let results3: Vec<_> = exec.execute_all(peer3_name).unwrap();
            assert_eq!(results3.len(), 1);
            let outcome3 = &results3[0];
            assert_eq!(outcome3.ret_code, 0, "{:?}", outcome3);
            assert!(exec.execution_iter(peer3_name).unwrap().next().is_none());

            let trace = trace_from_result(outcome3);
            assert_eq!(
                trace,
                ExecutionTrace::from(vec![
                    scalar!(
                        json!([{"p":peer2_id,"v":2},{"p":peer3_id,"v":3}]),
                        peer_name = &peer1_name,
                        service = "service..0",
                        function = "func"
                    ),
                    scalar!(
                        12,
                        peer_name = &peer2_name,
                        service = "service..1",
                        function = "func",
                        args = vec![json!({"p":peer2_id,"v":2}), json!(1)]
                    ),
                    request_sent_by(peer2_id),
                ])
            );
        }
    }

    #[test]
    fn test_map() {
        let peer1_name = "peer1";
        let peer2_name = "peer2";
        let peer3_name = "peer3";
        let (_peer2_pk, peer2_id) = derive_dummy_keypair(peer2_name);
        let (_peer3_pk, peer3_id) = derive_dummy_keypair(peer3_name);

        let exec = AirScriptExecutor::<NativeAirRunner>::new(
            TestRunParameters::from_init_peer_id(peer1_name),
            vec![],
            IntoIterator::into_iter([peer2_name, peer3_name]).map(Into::into),
            &format!(
                r#"
(seq
  (call "{peer1_name}" ("" "") [] peers) ; ok = ["{peer2_id}", "{peer3_id}"]
  (fold peers p
    (seq
      (call p ("" "") [p]) ; map = {{"{peer2_id}": 42, "{peer3_id}": 43}}
      (next p)
)))
"#
            ),
        )
        .unwrap();

        let result_init: Vec<_> = exec.execution_iter("peer1").unwrap().collect();

        assert_eq!(result_init.len(), 1);
        let outcome1 = &result_init[0];
        assert_eq!(outcome1.ret_code, 0);
        assert_eq!(outcome1.error_message, "");
        assert_next_pks!(&outcome1.next_peer_pks, [peer2_id.as_str()]);

        {
            let results2 = exec.execute_all("peer2").unwrap();
            assert_eq!(results2.len(), 1);
            let outcome2 = &results2[0];
            assert_eq!(outcome2.ret_code, 0, "{:?}", outcome2);
            assert!(exec.execution_iter("peer2").unwrap().next().is_none());
            assert_next_pks!(&outcome2.next_peer_pks, [peer3_id.as_str()]);
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
                    scalar!(
                        json!([peer2_id, peer3_id]),
                        peer_name = peer1_name,
                        service = "..0"
                    ),
                    unused!(
                        42,
                        peer_name = &peer2_name,
                        service = "..1",
                        args = vec![peer2_id]
                    ),
                    unused!(
                        43,
                        peer_name = &peer3_name,
                        service = "..1",
                        args = vec![peer3_id]
                    ),
                ]
            );
        }
    }

    #[test]
    #[should_panic]
    fn test_map_no_arg() {
        let peer1_name = "peer1";

        let exec = AirScriptExecutor::<NativeAirRunner>::new(
            TestRunParameters::from_init_peer_id(peer1_name),
            vec![],
            std::iter::empty(),
            &format!(
                r#"
(call "{peer1_name}" ("" "") [] p) ; map = {{"any": "key"}}
"#
            ),
        )
        .unwrap();
        let _result_init: Vec<_> = exec.execution_iter(peer1_name).unwrap().collect();
    }

    #[test]
    fn test_seq_error() {
        let init_peer_name = "init_peer_id";
        let peer1_name = "peer1";
        let peer2_name = "peer2";
        let peer3_name = "peer3";
        let peer4_name = "peer4";
        let (_peer2_pk, peer2_id) = derive_dummy_keypair(peer2_name);
        let (_peer3_pk, peer3_id) = derive_dummy_keypair(peer3_name);
        let (_peer4_pk, peer4_id) = derive_dummy_keypair(peer4_name);

        let exec = AirScriptExecutor::<NativeAirRunner>::new(
            TestRunParameters::from_init_peer_id(init_peer_name),
            vec![],
            IntoIterator::into_iter([peer2_name, peer3_name, peer4_name]).map(Into::into),
            &format!(r#"(seq
  (seq
    (call "peer1" ("service" "func") [] var)  ; ok = [{{"p":"{peer2_id}","v":2}},{{"p":"{peer3_id}","v":3}}, {{"p":"{peer4_id}"}}]
    (seq
      (ap 1 k)
      (fold var i
        (seq
          (call i.$.p ("service" "func") [i.$.v k] k)  ; seq_error = {{"0":{{"ret_code":0,"result":12}},"default":{{"ret_code":1,"result":42}}}}
          (next i)))))
  (call "init_peer_id" ("a" "b") []) ; ok = 0
)"#),
        )
        .unwrap();

        let result_init: Vec<_> = exec.execution_iter(init_peer_name).unwrap().collect();

        assert_eq!(result_init.len(), 1);
        let outcome1 = &result_init[0];
        assert_eq!(outcome1.ret_code, 0);
        assert_eq!(outcome1.error_message, "");

        assert!(exec.execution_iter(peer2_name).unwrap().next().is_none());
        {
            let results1 = exec.execute_all(peer1_name).unwrap();
            assert_eq!(results1.len(), 1);
            let outcome1 = &results1[0];
            assert_eq!(outcome1.ret_code, 0, "{:?}", outcome1);
            assert!(exec.execution_iter(peer1_name).unwrap().next().is_none());
            assert_next_pks!(&outcome1.next_peer_pks, [peer2_id.as_str()]);
        }

        {
            let results2: Vec<_> = exec.execute_all(peer2_name).unwrap();
            assert_eq!(results2.len(), 1);
            let outcome2 = &results2[0];
            assert_eq!(outcome2.ret_code, 0, "{:?}", outcome2);
            assert!(exec.execution_iter(peer2_name).unwrap().next().is_none());
            assert_next_pks!(&outcome2.next_peer_pks, [peer3_id.as_str()]);

            let trace = trace_from_result(outcome2);
            assert_eq!(
                trace,
                ExecutionTrace::from(vec![
                    scalar!(
                        json!([{"p":peer2_id,"v":2},{"p":peer3_id,"v":3},{"p":peer4_id}]),
                        peer_name = &peer1_name,
                        service = "service..0",
                        function = "func"
                    ),
                    scalar!(
                        12,
                        peer_name = &peer2_name,
                        service = "service..1",
                        function = "func",
                        args = vec![2, 1]
                    ),
                    request_sent_by(peer2_id.clone()),
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
                    scalar!(
                        json!([{"p":peer2_id,"v":2},{"p":peer3_id,"v":3},{"p":peer4_id}]),
                        peer_name = peer1_name,
                        service = "service..0",
                        function = "func"
                    ),
                    scalar!(
                        12,
                        peer_name = peer2_name,
                        service = "service..1",
                        function = "func",
                        args = vec![2, 1]
                    ),
                    request_sent_by(peer2_id),
                ])
            );
        }
    }

    #[test]
    fn test_echo() {
        let init_peer_name = "init_peer_id";
        let peer1_name = "peer1";
        let peer2_name = "peer2";

        let exec = AirScriptExecutor::<NativeAirRunner>::new(
            TestRunParameters::from_init_peer_id(init_peer_name),
            vec![],
            std::iter::empty(),
            &format!(
                r#"(seq
(call "{peer1_name}" ("service" "func") [1 22] arg) ; behaviour=echo
(call "{peer2_name}" ("service" "func") [arg]) ; ok = 43
)
"#
            ),
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

        let peer1_id = exec.resolve_name(peer1_name).to_string();
        assert_eq!(
            trace_from_result(outcome1),
            ExecutionTrace::from(vec![
                scalar!(
                    1,
                    peer_name = &peer1_name,
                    service = "service..0",
                    function = "func",
                    args = vec![1, 22]
                ),
                request_sent_by(&peer1_id),
            ]),
        )
    }

    #[test]
    fn test_transformed_distinct() {
        let peer_name = "peer1";
        let network =
            Network::<NativeAirRunner>::new(std::iter::empty::<PeerId>(), vec![], <_>::default());

        let transformed1 = TransformedAirScript::new(
            &format!(r#"(call "{peer_name}" ("service" "function") []) ; ok = 42"#),
            network.clone(),
            <_>::default(),
        )
        .unwrap();
        let exectution1 = AirScriptExecutor::from_transformed_air_script(
            TestRunParameters::from_init_peer_id(peer_name),
            <_>::default(),
            transformed1,
        )
        .unwrap();

        let transformed2 = TransformedAirScript::new(
            &format!(r#"(call "{peer_name}" ("service" "function") []) ; ok = 24"#,),
            network,
            <_>::default(),
        )
        .unwrap();
        let exectution2 = AirScriptExecutor::from_transformed_air_script(
            TestRunParameters::from_init_peer_id(peer_name),
            <_>::default(),
            transformed2,
        )
        .unwrap();

        let trace1 = exectution1.execute_one(peer_name).unwrap();
        let trace2 = exectution2.execute_one(peer_name).unwrap();

        assert_eq!(
            trace_from_result(&trace1),
            ExecutionTrace::from(vec![unused!(
                42,
                peer_name = peer_name,
                service = "service..0",
                function = "function"
            )]),
        );
        assert_eq!(
            trace_from_result(&trace2),
            ExecutionTrace::from(vec![unused!(
                24,
                peer_name = peer_name,
                service = "service..1",
                function = "function"
            )]),
        );
    }

    #[test]
    fn test_transformed_shared() {
        struct Service {
            state: RefCell<std::vec::IntoIter<serde_json::Value>>,
        }

        impl MarineService for Service {
            fn call(&self, _params: CallRequestParams) -> crate::services::FunctionOutcome {
                let mut cell = self.state.borrow_mut();
                crate::services::FunctionOutcome::from_value(cell.next().unwrap())
            }
        }
        let service = Service {
            state: vec![json!(42), json!(24)].into_iter().into(),
        };
        let network = Network::<NativeAirRunner>::new(
            std::iter::empty::<PeerId>(),
            vec![service.to_handle()],
            <_>::default(),
        );

        let peer_name = "peer1";
        let air_script = format!(r#"(call "{peer_name}" ("service" "function") [])"#);
        let transformed1 =
            TransformedAirScript::new(&air_script, network.clone(), <_>::default()).unwrap();
        let exectution1 = AirScriptExecutor::from_transformed_air_script(
            TestRunParameters::from_init_peer_id(peer_name),
            <_>::default(),
            transformed1,
        )
        .unwrap();

        let transformed2 = TransformedAirScript::new(&air_script, network, <_>::default()).unwrap();
        let exectution2 = AirScriptExecutor::from_transformed_air_script(
            TestRunParameters::from_init_peer_id(peer_name),
            <_>::default(),
            transformed2,
        )
        .unwrap();

        let trace1 = exectution1.execute_one(peer_name).unwrap();
        let trace2 = exectution2.execute_one(peer_name).unwrap();

        assert_eq!(
            trace_from_result(&trace1),
            ExecutionTrace::from(vec![unused!(
                42,
                peer_name = peer_name,
                service = "service",
                function = "function"
            ),]),
        );
        assert_eq!(
            trace_from_result(&trace2),
            ExecutionTrace::from(vec![unused!(
                24,
                peer_name = peer_name,
                service = "service",
                function = "function"
            ),]),
        );
    }

    #[test]
    fn test_invalid_air() {
        let res = AirScriptExecutor::<NativeAirRunner>::new(
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

    fn run_behaviour_service(peer_name: &str, air_script: &str) {
        let exec = AirScriptExecutor::<NativeAirRunner>::new(
            TestRunParameters::from_init_peer_id(peer_name),
            vec![],
            std::iter::empty(),
            air_script,
        )
        .unwrap();

        let result_init: Vec<_> = exec.execution_iter(peer_name).unwrap().collect();

        assert_eq!(result_init.len(), 1);
        let outcome = &result_init[0];
        assert_eq!(outcome.ret_code, 0);
        assert_eq!(outcome.error_message, "");

        assert_eq!(
            trace_from_result(outcome),
            ExecutionTrace::from(vec![scalar!(
                "service",
                peer_name = peer_name,
                service = "service..0",
                function = "func",
                args = vec![1, 22]
            ),]),
        )
    }

    #[test]
    fn test_behaviour_service() {
        let peer_name = "peer1";
        let air_script =
            &format!(r#"(call "{peer_name}" ("service" "func") [1 22] arg) ; behaviour=service"#);
        run_behaviour_service(peer_name, air_script)
    }

    #[test]
    fn test_dbg_behaviour_service() {
        let peer_name = "peer1";
        let air_script = &format!(
            r#"(call "{peer_name}" ("service" "func") [1 22] arg) ; dbg_behaviour=service"#
        );
        run_behaviour_service(peer_name, air_script)
    }

    fn run_behaviour_function(peer_name: &str, air_script: &str) {
        let exec = AirScriptExecutor::<NativeAirRunner>::new(
            TestRunParameters::from_init_peer_id(peer_name),
            vec![],
            std::iter::empty(),
            air_script,
        )
        .unwrap();

        let result_init: Vec<_> = exec.execution_iter(peer_name).unwrap().collect();

        assert_eq!(result_init.len(), 1);
        let outcome = &result_init[0];
        assert_eq!(outcome.ret_code, 0);
        assert_eq!(outcome.error_message, "");

        assert_eq!(
            trace_from_result(outcome),
            ExecutionTrace::from(vec![scalar!(
                "func",
                peer_name = peer_name,
                service = "service..0",
                function = "func",
                args = vec![1, 22]
            ),]),
        )
    }

    #[test]
    fn test_behaviour_function() {
        let peer_name = "peer1";
        let air_script =
            &format!(r#"(call "{peer_name}" ("service" "func") [1 22] arg) ; behaviour=function"#);
        run_behaviour_function(peer_name, air_script)
    }

    #[test]
    fn test_dbg_behaviour_function() {
        let peer_name = "peer1";
        let air_script = &format!(
            r#"(call "{peer_name}" ("service" "func") [1 22] arg) ; dbg_behaviour=function"#
        );
        run_behaviour_function(peer_name, air_script)
    }

    fn run_behaviour_arg(peer_name: &str, air_script: &str) {
        let exec = AirScriptExecutor::<NativeAirRunner>::new(
            TestRunParameters::from_init_peer_id(peer_name),
            vec![],
            std::iter::empty(),
            air_script,
        )
        .unwrap();

        let result_init: Vec<_> = exec.execution_iter(peer_name).unwrap().collect();

        assert_eq!(result_init.len(), 1);
        let outcome = &result_init[0];
        assert_eq!(outcome.ret_code, 0);
        assert_eq!(outcome.error_message, "");

        assert_eq!(
            trace_from_result(outcome),
            ExecutionTrace::from(vec![scalar!(
                22,
                peer_name = peer_name,
                service = "service..0",
                function = "func",
                args = vec![1, 22]
            ),]),
        )
    }

    #[test]
    fn test_behaviour_arg() {
        let peer_name = "peer1";
        let air_script =
            &format!(r#"(call "{peer_name}" ("service" "func") [1 22] arg) ; behaviour=arg.1"#);

        run_behaviour_arg(peer_name, air_script)
    }

    #[test]
    fn test_dbg_behaviour_arg() {
        let peer_name = "peer1";
        let air_script =
            &format!(r#"(call "{peer_name}" ("service" "func") [1 22] arg) ; dbg_behaviour=arg.1"#);

        run_behaviour_arg(peer_name, air_script)
    }

    fn run_behaviour_tetraplet(peer_name: &str, air_script: &str) {
        let (_peer_pk, peer_id) = derive_dummy_keypair(peer_name);

        let exec = AirScriptExecutor::<NativeAirRunner>::new(
            TestRunParameters::from_init_peer_id(peer_name),
            vec![],
            std::iter::empty(),
            air_script,
        )
        .unwrap();

        let result_init: Vec<_> = exec.execution_iter(peer_name).unwrap().collect();

        assert_eq!(result_init.len(), 1);
        let outcome = &result_init[0];
        assert_eq!(outcome.ret_code, 0);
        assert_eq!(outcome.error_message, "");

        assert_eq!(
            trace_from_result(outcome),
            ExecutionTrace::from(vec![scalar!(
                json!([[{
                    "function_name": "",
                    "json_path": "",
                    "peer_pk": &peer_id,
                    "service_id": "",
                }], [{
                    "function_name": "",
                    "json_path": "",
                    "peer_pk": &peer_id,
                    "service_id": "",
                }]]),
                peer_name = peer_name,
                service = "service..0",
                function = "func",
                args = vec![1, 22]
            )]),
        )
    }

    #[test]
    fn test_behaviour_tetraplet() {
        let peer_name = "peer1";
        let air_script =
            &format!(r#"(call "{peer_name}" ("service" "func") [1 22] arg) ; behaviour=tetraplet"#);
        run_behaviour_tetraplet(peer_name, air_script)
    }

    #[test]
    fn test_dbg_behaviour_tetraplet() {
        let peer_name = "peer1";
        let air_script = &format!(
            r#"(call "{peer_name}" ("service" "func") [1 22] arg) ; dbg_behaviour=tetraplet"#
        );
        run_behaviour_tetraplet(peer_name, air_script)
    }
}
