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
use pretty_assertions::assert_eq;

#[tokio::test]
// test for github.com/fluencelabs/aquavm/issues/222
async fn issue_222() {
    let other_id = "other_id";
    let other_1_id = "other_1";
    let other_2_id = "other_2";

    let air_script = format!(
        r#"
        (new $stream
            (par
                (par
                    (call "{other_1_id}" ("" "") [] $stream)
                    (call "{other_2_id}" ("" "") [] $stream))
                (fold $stream j
                    (seq
                        (call "{other_id}" ("" "") [j])
                        (next j)))))
    "#
    );

    let mut other_id_vm = create_avm(echo_call_service(), "other_id").await;
    let mut other_1_vm = create_avm(set_variable_call_service(json!([1])), "other_1").await;
    let mut other_2_vm = create_avm(set_variable_call_service(json!([2])), "other_2").await;

    let result = checked_call_vm!(other_id_vm, <_>::default(), &air_script, "", "");
    let other_1_result = checked_call_vm!(other_1_vm, <_>::default(), &air_script, "", result.data.clone());
    let other_2_result = checked_call_vm!(other_2_vm, <_>::default(), &air_script, "", result.data.clone());

    // the bug is triggered when (call "other_2" ...) result arrives to "other_id"
    // before the "other_1" result.
    let result_from_2 = checked_call_vm!(
        other_id_vm,
        <_>::default(),
        &air_script,
        result.data,
        other_2_result.data
    );
    let final_result = checked_call_vm!(
        other_id_vm,
        <_>::default(),
        &air_script,
        result_from_2.data,
        other_1_result.data
    );

    let actual_trace = trace_from_result(&final_result);

    let expected_trace = vec![
        executed_state::par(3, 3),
        executed_state::par(1, 1),
        stream!(json!([1]), 1, peer = other_1_id),
        stream!(json!([2]), 0, peer = other_2_id),
        executed_state::fold(vec![
            executed_state::subtrace_lore(3, SubTraceDesc::new(5.into(), 1), SubTraceDesc::new(6.into(), 0)),
            executed_state::subtrace_lore(2, SubTraceDesc::new(6.into(), 1), SubTraceDesc::new(7.into(), 0)),
        ]),
        unused!(json!([2]), peer = other_id, args = vec![vec![2]]),
        unused!(json!([1]), peer = other_id, args = vec![vec![1]]),
    ];

    assert_eq!(&*actual_trace, expected_trace);
}
