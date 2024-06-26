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

use air::CatchableError;
use air_test_framework::AirScriptExecutor;
use air_test_utils::prelude::*;

use pretty_assertions::assert_eq;
use std::cell::RefCell;

#[tokio::test]
async fn length_functor_for_array_scalar() {
    let script = r#"
        (seq
            (call %init_peer_id% ("" "") [] variable) ; ok = [1,1,1]
            (call %init_peer_id% ("" "") [variable.length]) ; behaviour = echo
        )
        "#;

    let init_peer_name = "init_peer_id";
    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), script)
        .await
        .expect("invalid test AIR script");

    let result = executor.execute_one(init_peer_name).await.unwrap();
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        scalar!(json!([1, 1, 1]), peer_name = init_peer_name, service = "..0"),
        unused!(3, peer_name = init_peer_name, args = vec![3], service = "..1"),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[tokio::test]
async fn length_functor_for_non_array_scalar() {
    let result_jvalue = "string_jvalue";
    let script = format!(
        r#"
        (seq
            (call %init_peer_id% ("" "") [] variable) ; ok = "{result_jvalue}"
            (call %init_peer_id% ("" "") [variable.length]) ; behaviour = echo
        )
        "#
    );

    let init_peer_name = "init_peer_id";
    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), &script)
        .await
        .expect("invalid test AIR script");

    let result = executor.execute_one(init_peer_name).await.unwrap();
    check_error(
        &result,
        CatchableError::LengthFunctorAppliedToNotArray(result_jvalue.into()),
    );
}

#[tokio::test]
async fn length_functor_for_stream() {
    let script = r#"
        (seq
            (seq
                (ap 1 $stream)
                (ap 1 $stream))
            (seq
                (canon %init_peer_id% $stream #stream)
                (call %init_peer_id% ("" "") [#stream.length]) ; behaviour = echo
            )
        )
        "#;

    let init_peer_name = "init_peer_id";
    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), script)
        .await
        .expect("invalid test AIR script");

    let result = executor.execute_one(init_peer_name).await.unwrap();
    let actual_trace = trace_from_result(&result);

    let init_peer_id = executor.resolve_name(init_peer_name).to_string();

    let expected_trace = vec![
        executed_state::ap(0),
        executed_state::ap(0),
        executed_state::canon(json!({
            "tetraplet": {"function_name": "", "lens": "", "peer_pk": init_peer_id, "service_id": ""},
            "values": [
                {
                    "result": 1,
                    "tetraplet": {"function_name": "", "lens": "", "peer_pk": init_peer_id, "service_id": ""},
                    "trace_pos": 0,
                },
                {
                    "result": 1,
                    "tetraplet": {"function_name": "", "lens": "", "peer_pk": init_peer_id, "service_id": ""},
                    "trace_pos": 1,
                },
            ]
        })),
        unused!(2, peer = init_peer_id, service = "..0", args = vec![2]),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[tokio::test]
async fn length_functor_for_empty_stream() {
    let script = r#"
        (new $stream
            (seq
                (canon %init_peer_id% $stream #canon_stream)
                (call %init_peer_id% ("" "") [#canon_stream.length]) ; behaviour = echo
            )
        )
        "#;

    let init_peer_name = "init_peer_id";
    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), script)
        .await
        .expect("invalid test AIR script");

    let result = executor.execute_one(init_peer_name).await.unwrap();
    let actual_trace = trace_from_result(&result);

    let init_peer_id = executor.resolve_name(init_peer_name).to_string();

    let expected_trace = vec![
        executed_state::canon(
            json!({"tetraplet": {"function_name": "", "lens": "", "peer_pk": init_peer_id, "service_id": ""},
                "values": []} ),
        ),
        unused!(0, peer = init_peer_id, service = "..0", args = vec![0]),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[tokio::test]
async fn length_functor_for_canon_stream() {
    let script = r#"
        (seq
            (seq
                (ap 1 $stream)
                (ap 1 $stream))
            (seq
                (canon %init_peer_id% $stream #canon_stream)
                (call %init_peer_id% ("" "") [#canon_stream.length]) ; behaviour = echo
            )
        )
        "#;

    let init_peer_name = "init_peer_id";
    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), script)
        .await
        .expect("invalid test AIR script");

    let result = executor.execute_one(init_peer_name).await.unwrap();
    let actual_trace = trace_from_result(&result);

    let init_peer_id = executor.resolve_name(init_peer_name).to_string();

    let expected_trace = vec![
        executed_state::ap(0),
        executed_state::ap(0),
        executed_state::canon(
            json!({"tetraplet": {"function_name": "", "lens": "", "peer_pk": init_peer_id, "service_id": ""},
                "values": [{"result": 1, "tetraplet": {"function_name": "", "lens": "", "peer_pk": init_peer_id, "service_id": ""}, "trace_pos": 0},
                           {"result": 1, "tetraplet": {"function_name": "", "lens": "", "peer_pk": init_peer_id, "service_id": ""}, "trace_pos": 1}
                ]} ),
        ),
        unused!(2, peer = init_peer_id, service = "..0", args = vec![2]),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[tokio::test]
async fn length_functor_for_empty_canon_stream() {
    let script = r#"
        (new $stream
            (seq
                (canon %init_peer_id% $stream #canon_stream)
                (call %init_peer_id% ("" "") [#canon_stream.length]) ; behaviour = echo
            )
        )
        "#;

    let init_peer_name = "init_peer_id";
    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), script)
        .await
        .expect("invalid test AIR script");

    let result = executor.execute_one(init_peer_name).await.unwrap();
    let actual_trace = trace_from_result(&result);

    let init_peer_id = executor.resolve_name(init_peer_name).to_string();

    let expected_trace = vec![
        executed_state::canon(
            json!({"tetraplet": {"function_name": "", "lens": "", "peer_pk": init_peer_id, "service_id": ""}, "values": []} ),
        ),
        unused!(0, peer = init_peer_id, service = "..0", args = vec![0]),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[tokio::test]
async fn functor_dont_influence_tetraplet() {
    let set_variable_peer_id = "set_variable_peer_id";
    let set_variable_peer_result = json!({"field": [1,2,3]});
    let mut set_variable_vm = create_avm(
        set_variable_call_service(set_variable_peer_result.clone()),
        set_variable_peer_id,
    )
    .await;

    let tetraplet_catcher_peer_id = "tetraplet_catcher_peer_id";
    let (call_service, actual_tetraplet) = tetraplet_host_function(echo_call_service());
    let mut tetraplet_catcher_vm = create_avm(call_service, tetraplet_catcher_peer_id).await;

    let script = format!(
        r#"
        (seq
            (call "{set_variable_peer_id}" ("" "") [] scalar)
            (seq
                (ap scalar.$.field field)
                (seq
                    (ap field.length length)
                    (call "{tetraplet_catcher_peer_id}" ("" "") [length])
                )
            )
        )
        "#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(tetraplet_catcher_vm, <_>::default(), &script, "", result.data);
    let actual_trace = trace_from_result(&result);

    let expected_tetraplet = RefCell::new(vec![vec![SecurityTetraplet::new("", "", "", ".length")]]);
    assert_eq!(actual_tetraplet.as_ref(), &expected_tetraplet);

    let expected_trace = vec![
        scalar!(set_variable_peer_result, peer = set_variable_peer_id),
        unused!(3, peer = tetraplet_catcher_peer_id, args = vec![3]),
    ];
    assert_eq!(actual_trace, expected_trace);
}
