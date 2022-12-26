/*
 * Copyright 2020 Fluence Labs Limited
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

use air_interpreter_data::CidTracker;
use air_test_utils::prelude::*;

use std::rc::Rc;

#[test]
fn seq_remote_remote() {
    let mut vm = create_avm(unit_call_service(), "");
    let mut cid_tracker = CidTracker::new();
    cid_tracker.record_value(Rc::new("".into())).unwrap();

    let script = r#"
            (seq
                (call "remote_peer_id_1" ("local_service_id" "local_fn_name") [] result_name)
                (call "remote_peer_id_2" ("service_id" "fn_name") [] g)
            )"#;

    let result = checked_call_vm!(vm, <_>::default(), script, "", "");
    assert_eq!(result.next_peer_pks, vec![String::from("remote_peer_id_1")]);

    let initial_trace = vec![executed_state::scalar_string("")];
    let initial_data = raw_data_from_trace(initial_trace, cid_tracker.into());

    let result = checked_call_vm!(vm, <_>::default(), script, "", initial_data);

    assert_eq!(result.next_peer_pks, vec![String::from("remote_peer_id_2")]);
}

#[test]
fn seq_local_remote() {
    let local_peer_id = "local_peer_id";
    let remote_peer_id = String::from("remote_peer_id");
    let mut vm = create_avm(unit_call_service(), local_peer_id);

    let script = f!(r#"
            (seq
                (call "{local_peer_id}" ("local_service_id" "local_fn_name") [] result_name)
                (call "{remote_peer_id}" ("service_id" "fn_name") [] g)
            )"#);

    let result = checked_call_vm!(vm, <_>::default(), script, "", "");
    assert_eq!(result.next_peer_pks, vec![remote_peer_id]);
}
