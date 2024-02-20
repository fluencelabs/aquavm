/*
 * Copyright 2023 Fluence Labs Limited
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

use air_test_framework::AirScriptExecutor;
use air_test_utils::{key_utils::at, prelude::*};

#[test]
fn call_result() {
    let init_peer_name = "B";

    let air_script = r#"
        (seq
            (call "B" ("service" "func") [] $s) ; ok = "some_data"
            (canon "B" $s #c))
    "#;
    let runner =
        AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), air_script).unwrap();

    let result = runner.execute_one(init_peer_name).unwrap();
    assert_eq!(result.ret_code, 0, "{:?}", result.error_message);

    let data = data_from_result(&result);
    let last_state = data.trace.last().unwrap();

    let val = scalar!(
        "some_data",
        peer_name = init_peer_name,
        service = "service..0",
        function = "func"
    );
    let val_cid = extract_service_result_cid(&val);

    let init_peer_id = at(init_peer_name);

    let expected_state = canon(json!({
        "tetraplet": {
            "peer_pk": init_peer_id,
            "service_id": "",
            "function_name": "",
            "lambda": "",
        },
        "values": [{
            "result": "some_data",
            "tetraplet": {
                "peer_pk": init_peer_id,
                "service_id": "service..0",
                "function_name": "func",
                "lambda": "",
            },
            "provenance": Provenance::service_result(val_cid.clone()),
        }]
    }));

    assert_eq!(last_state, &expected_state);
}

#[test]
fn call_result_iteration() {
    let init_peer_name = "A";

    let air_script = r#"
        (seq
            (seq
                (call "A" ("service" "func") [] x) ; ok = [10, 11, 12]
                (fold x a
                    (seq
                        (ap a $s)
                        (next a))))
            (canon "A" $s #c))
    "#;
    let runner =
        AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), air_script).unwrap();

    let result = runner.execute_one(init_peer_name).unwrap();
    assert_eq!(result.ret_code, 0, "{:?}", result.error_message);

    let data = data_from_result(&result);
    let last_state = data.trace.last().unwrap();

    let init_peer_id = at(init_peer_name);

    let val = scalar!(
        json!([10, 11, 12]),
        peer_name = init_peer_name,
        service = "service..0",
        function = "func"
    );
    let val_cid = extract_service_result_cid(&val);

    let expected_state = canon(json!({
        "tetraplet": {
            "peer_pk": init_peer_id,
            "service_id": "",
            "function_name": "",
            "lambda": "",
        },
        "values": [{
            "result": 10,
            "tetraplet": {
                "peer_pk": init_peer_id,
                "service_id": "service..0",
                "function_name": "func",
                "lambda": ".$.[0]",
            },
            "provenance": Provenance::service_result(val_cid.clone()),
        }, {
            "result": 11,
            "tetraplet": {
                "peer_pk": init_peer_id,
                "service_id": "service..0",
                "function_name": "func",
                "lambda": ".$.[1]",
            },
            "provenance": Provenance::service_result(val_cid.clone()),
        }, {
            "result": 12,
            "tetraplet": {
                "peer_pk": init_peer_id,
                "service_id": "service..0",
                "function_name": "func",
                "lambda": ".$.[2]",
            },
            "provenance": Provenance::service_result(val_cid),
        }]
    }));

    assert_eq!(last_state, &expected_state);
}

#[test]
fn literal() {
    let init_peer_name = "B";

    let air_script = r#"
        (seq
            (ap 1 $s)
            (canon "B" $s #c))
    "#;
    let runner =
        AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), air_script).unwrap();

    let result = runner.execute_one(init_peer_name).unwrap();
    assert_eq!(result.ret_code, 0, "{:?}", result.error_message);

    let data = data_from_result(&result);
    let last_state = data.trace.last().unwrap();

    let init_peer_id = at(init_peer_name);

    let expected_state = canon(json!({
        "tetraplet": {
            "peer_pk": init_peer_id,
            "service_id": "",
            "function_name": "",
            "lambda": "",
        },
        "values": [{
            "result": 1,
            "tetraplet": {
                "peer_pk": init_peer_id,
                "service_id": "",
                "function_name": "",
                "lambda": "",
            },
            "provenance": Provenance::literal(),
        }]
    }));

    assert_eq!(last_state, &expected_state);
}

#[test]
fn canon_in_canon() {
    let init_peer_name = "B";

    let air_script = r#"
        (seq
            (seq
                (call "B" ("service" "func") [] $s) ; ok = 1
                (canon "B" $s #c))
            (seq
                (ap #c $s)
                (canon "B" $s #d)))
    "#;
    let runner =
        AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), air_script).unwrap();

    let result = runner.execute_one(init_peer_name).unwrap();
    assert_eq!(result.ret_code, 0, "{:?}", result.error_message);

    let trace = trace_from_result(&result);
    let last_state = trace.last().unwrap();

    let init_peer_id = at(init_peer_name);

    let val = scalar!(1, peer_name = init_peer_name, service = "service..0", function = "func");
    let val_cid = extract_service_result_cid(&val);
    let value_tetraplet = json!({
        "peer_pk": init_peer_id,
        "service_id": "service..0",
        "function_name": "func",
        "lambda": "",
    });

    let canon_val = canon(json!({
        "tetraplet": {
            "peer_pk": init_peer_id,
            "service_id": "",
            "function_name": "",
            "lambda": "",
        },
        "values": [{
            "result": 1,
            "tetraplet": value_tetraplet,
            "provenance": Provenance::service_result(val_cid.clone()),
        }]
    }));
    let canon_cid = extract_canon_result_cid(&canon_val);

    let expected_state = canon(json!({
        "tetraplet": {
            "peer_pk": init_peer_id,
            "service_id": "",
            "function_name": "",
            "lambda": "",
        },
        "values": [{
            "result": 1,
            "tetraplet": value_tetraplet,
            "provenance": Provenance::service_result(val_cid),
        }, {
            "result": [1],
            "tetraplet": {
                "peer_pk": init_peer_id,
                "service_id": "",
                "function_name": "",
                "lambda": "",
            },
            "provenance": Provenance::canon(canon_cid),
        }]
    }));

    assert_eq!(last_state, &expected_state,);
}

#[test]
fn lambda_result_iteration() {
    let init_peer_name = "A";

    let air_script = r#"
        (seq
            (seq
                (call "A" ("service" "func") [] x) ; ok = [{"field": [10, 11, 12]}]
                (fold x y
                    (seq
                        (fold y.$.field a
                        (seq
                            (ap a $s)
                            (next a)))
                        (next y))))
            (canon "A" $s #c))
    "#;
    let runner =
        AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), air_script).unwrap();

    let result = runner.execute_one(init_peer_name).unwrap();
    assert_eq!(result.ret_code, 0, "{:?}", result.error_message);

    let data = data_from_result(&result);
    let last_state = data.trace.last().unwrap();

    let init_peer_id = at(init_peer_name);

    let val = scalar!(
        json!([{"field": [10, 11, 12]}]),
        peer_name = init_peer_name,
        service = "service..0",
        function = "func"
    );
    let val_cid = extract_service_result_cid(&val);

    let expected_state = canon(json!({
        "tetraplet": {
            "peer_pk": init_peer_id,
            "service_id": "",
            "function_name": "",
            "lambda": "",
        },
        "values": [{
            "result": 10,
            "tetraplet": {
                "peer_pk": init_peer_id,
                "service_id": "service..0",
                "function_name": "func",
                "lambda": ".$.[0].$.field.$.[0]",
            },
            "provenance": Provenance::service_result(val_cid.clone()),
        }, {
            "result": 11,
            "tetraplet": {
                "peer_pk": init_peer_id,
                "service_id": "service..0",
                "function_name": "func",
                "lambda": ".$.[0].$.field.$.[1]",
            },
            "provenance": Provenance::service_result(val_cid.clone()),
        }, {
            "result": 12,
            "tetraplet": {
                "peer_pk": init_peer_id,
                "service_id": "service..0",
                "function_name": "func",
                "lambda": ".$.[0].$.field.$.[2]",
            },
            "provenance": Provenance::service_result(val_cid),
        }]
    }));

    assert_eq!(last_state, &expected_state);
}
