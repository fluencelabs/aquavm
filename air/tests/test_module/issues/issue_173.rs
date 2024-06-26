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

use air_test_utils::prelude::*;

#[tokio::test]
#[ignore]
// test for github.com/fluencelabs/aquavm/issues/173
async fn issue_173() {
    let set_variable_peer_id = "set_variable_peer_id";
    let local_vm_peer_id_1 = "local_vm_peer_id_1";
    let local_vm_peer_id_2 = "local_vm_peer_id_2";

    let mut local_vm_1 = create_avm(echo_call_service(), local_vm_peer_id_1).await;
    let mut local_vm_2 = create_avm(echo_call_service(), local_vm_peer_id_2).await;

    let variables_mapping = maplit::hashmap! {
        "1".to_string() => json!(1),
        "2".to_string() => json!(2),
    };
    let mut set_variable_vm = create_avm(
        set_variables_call_service(variables_mapping, VariableOptionSource::Argument(0)),
        set_variable_peer_id,
    )
    .await;

    let script = format!(
        r#"
            (seq
                (seq
                    (call "{set_variable_peer_id}" ("" "") ["1"] $stream)
                    (call "{set_variable_peer_id}" ("" "") ["2"] $stream)
                )
                (fold $stream i
                    (par
                        (new $stream
                            (seq
                                (seq
                                    (call "{local_vm_peer_id_1}" ("" "") [i] $stream)
                                    (next i)
                                )
                                (call "{local_vm_peer_id_1}" ("" "") [$stream])
                            )
                        )
                        (call "{local_vm_peer_id_2}" ("" "") [$stream])
                    )
                )
            )"#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let vm_1_result = checked_call_vm!(local_vm_1, <_>::default(), &script, "", result.data);
    let vm_2_result = checked_call_vm!(local_vm_2, <_>::default(), &script, "", vm_1_result.data.clone());

    let vm_1_result = checked_call_vm!(
        local_vm_1,
        <_>::default(),
        &script,
        vm_1_result.data,
        vm_2_result.data.clone()
    );
    let vm_2_result = checked_call_vm!(local_vm_2, <_>::default(), script, vm_2_result.data, vm_1_result.data);

    let actual_trace = trace_from_result(&vm_2_result);
    let expected_trace = vec![
        stream!(1, 0),
        stream!(2, 0),
        executed_state::fold(vec![
            executed_state::subtrace_lore(0, SubTraceDesc::new(3.into(), 2), SubTraceDesc::new(9.into(), 2)),
            executed_state::subtrace_lore(1, SubTraceDesc::new(5.into(), 2), SubTraceDesc::new(7.into(), 2)),
        ]),
        executed_state::par(6, 1),
        stream!(1, 0),
        executed_state::par(2, 1),
        stream!(2, 0),
        scalar!(json!([2])),
        scalar!(json!([1, 2])),
        scalar!(json!([1])),
        scalar!(json!([1, 2])),
    ];
    assert_eq!(actual_trace, expected_trace);
}
