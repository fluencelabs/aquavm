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

use air::{ExecutionCidState, UncatchableError};
use air_test_utils::prelude::*;
use air_trace_handler::merger::MergeError;
use air_trace_handler::TraceHandlerError;

#[test]
// test for github.com/fluencelabs/aquavm/issues/295
fn issue_295() {
    let vm_peer_id = "vm_peer_id";
    let mut vm = create_avm(echo_call_service(), vm_peer_id);

    let script = format!(
        r#"
        (seq
            (call "{vm_peer_id}" ("" "") [] scalar)
            (ap scalar $stream)
        )
    "#
    );

    let mut cid_state = ExecutionCidState::new();

    let scalar = scalar_tracked!("", cid_state, peer = vm_peer_id);
    let prev_trace = vec![scalar.clone(), executed_state::ap(1)];
    let current_trace = vec![scalar.clone(), scalar];
    let prev_data = raw_data_from_trace(prev_trace, cid_state.clone().into());
    let current_data = raw_data_from_trace(current_trace, cid_state.clone().into());
    let result = call_vm!(vm, <_>::default(), &script, prev_data, current_data);

    let cid = value_aggregate_cid(
        json!(""),
        SecurityTetraplet::new(vm_peer_id, "", "", ""),
        vec![],
        &mut cid_state,
    );
    let expected_error = UncatchableError::TraceError {
        trace_error: TraceHandlerError::MergeError(MergeError::IncompatibleExecutedStates(
            ExecutedState::Ap(ApResult::new(1.into())),
            ExecutedState::Call(CallResult::Executed(ValueRef::Scalar(cid))),
        )),
        instruction: "ap scalar $stream".to_string(),
    };

    assert!(check_error(&result, expected_error));
}
