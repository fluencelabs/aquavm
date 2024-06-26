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

use air::{ExecutionCidState, UncatchableError};
use air_test_utils::prelude::*;
use air_trace_handler::merger::MergeError;
use air_trace_handler::TraceHandlerError;

#[tokio::test]
// test for github.com/fluencelabs/aquavm/issues/295
async fn issue_295() {
    let vm_peer_id = "vm_peer_id";
    let mut vm = create_avm(echo_call_service(), vm_peer_id).await;

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
