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

mod canon;

use air_interpreter_data::CidTracker;
use air_test_framework::AirScriptExecutor;
use air_test_utils::prelude::*;

#[test]
fn test_missing_cid() {
    let vm_peer_id = "vm_peer_id";
    let mut vm = create_avm(echo_call_service(), vm_peer_id);

    let air_script = r#"
       (seq
          (call "peer_id" ("service" "call1") [] x)
          (call "peer_id" ("service" "call2") []))"#;
    let trace = vec![scalar_number(42), scalar_number(43)];
    let mut tracker = CidTracker::<JValue>::new();
    tracker.record_value(json!(43)).unwrap();

    let cur_data = raw_data_from_trace(trace, tracker);
    let result = call_vm!(vm, <_>::default(), air_script, vec![], cur_data);
    assert_eq!(result.ret_code, 20011);
    assert_eq!(
        result.error_message,
        "value for CID \"bagaaieraondvznakk2hi3kfaixhnceatpykz7cikytniqo3lc7ogkgz2qbeq\" not found"
    );
}

#[test]
fn test_correct_cid() {
    let vm_peer_id = "vm_peer_id";
    let mut vm = create_avm(echo_call_service(), vm_peer_id);

    let air_script = r#"
       (seq
          (call "peer_id" ("service" "call1") [] x)
          (call "peer_id" ("service" "call2") [] y))"#;
    let trace = vec![scalar_number(42), scalar_number(43)];
    let mut tracker = CidTracker::<JValue>::new();
    tracker.record_value(json!(43)).unwrap();
    tracker.record_value(json!(42)).unwrap();

    let cur_data = raw_data_from_trace(trace, tracker);
    let result = call_vm!(vm, <_>::default(), air_script, vec![], cur_data);
    assert_eq!(result.ret_code, 0);
}

#[test]
fn test_scalar_cid() {
    let vm_peer_id = "vm_peer_id";

    let annotated_air_script = format!(
        r#"
       (seq
          (call "{vm_peer_id}" ("service" "call1") [] x) ; ok="hi"
          (call "{vm_peer_id}" ("service" "call2") [] y) ; ok="ipld"
       )"#
    );
    let executor = AirScriptExecutor::new(
        TestRunParameters::from_init_peer_id(vm_peer_id),
        vec![],
        std::iter::empty(),
        &annotated_air_script,
    )
    .unwrap();

    let result = executor.execute_one(vm_peer_id).unwrap();
    let data = data_from_result(&result);
    let mut tracker = CidTracker::<JValue>::new();
    let expected_trace = vec![scalar_tracked("hi", &mut tracker), scalar_tracked("ipld", &mut tracker)];

    assert_eq!(result.ret_code, 0);
    assert_eq!(data.trace, expected_trace);
    assert_eq!(data.cid_info.value_store, tracker.into());
}

#[test]
fn test_stream_cid() {
    let vm_peer_id = "vm_peer_id";

    let annotated_air_script = format!(
        r#"
       (seq
          (call "{vm_peer_id}" ("service" "call1") [] $x) ; ok="hi"
          (call "{vm_peer_id}" ("service" "call2") [] $x) ; ok="ipld"
       )"#
    );
    let executor = AirScriptExecutor::new(
        TestRunParameters::from_init_peer_id(vm_peer_id),
        vec![],
        std::iter::empty(),
        &annotated_air_script,
    )
    .unwrap();

    let result = executor.execute_one(vm_peer_id).unwrap();
    let data = data_from_result(&result);
    let mut tracker = CidTracker::<JValue>::new();
    let expected_trace = vec![
        stream_tracked("hi", 0, &mut tracker),
        stream_tracked("ipld", 1, &mut tracker),
    ];

    assert_eq!(result.ret_code, 0);
    assert_eq!(data.trace, expected_trace);
    assert_eq!(data.cid_info.value_store, tracker.into());
}
