/*
 * Copyright 2021 Fluence Labs Limited
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

use air_test_utils::prelude::*;

use fstrings::f;
use fstrings::format_args_f;

#[test]
fn fail_with_last_error() {
    let local_peer_id = "local_peer_id";
    let fallible_service_id = "service_id_1";
    let mut vm = create_avm(fallible_call_service(fallible_service_id), local_peer_id);

    let script = f!(r#"
            (xor
                (call "{local_peer_id}" ("service_id_1" "local_fn_name") [] result_1)
                (fail %last_error%)
            )"#);

    let result = call_vm!(vm, "", script, "", "");
    assert_eq!(result.ret_code, 1000);
    assert_eq!(
        result.error_message,
        r#"Local service error, ret_code is 1, error message is '"failed result from fallible_call_service"'"#
    );
}

#[test]
fn fail_with_literals() {
    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_call_service(), local_peer_id);

    let script = format!(
        r#"
            (xor
                (fail 1337 "error message")
                (fail %last_error%)
            )"#,
    );

    let result = call_vm!(vm, "", script, "", "");
    assert_eq!(result.ret_code, 1012);
    assert_eq!(
        result.error_message,
        "fail with ret_code '1337' and error_message 'error message' is used without corresponding xor"
    );
}

#[test]
fn fail_with_last_error_tetraplets() {
    let local_peer_id = "local_peer_id";
    let fallible_service_id = "service_id_1";
    let (host_closure, tetraplet_anchor) = tetraplet_host_function(fallible_call_service(fallible_service_id));
    let mut vm = create_avm(host_closure, local_peer_id);

    let local_fn_name = "local_fn_name";
    let script = f!(r#"
        (xor
            (xor
                (call "{local_peer_id}" ("{fallible_service_id}" "{local_fn_name}") [] result_1)
                (fail %last_error%)
            )
            (call "{local_peer_id}" ("" "") [%last_error%])
        )
          "#);

    let result = checked_call_vm!(vm, local_peer_id, script, "", "");
    assert_eq!(
        tetraplet_anchor.borrow()[0][0],
        SecurityTetraplet::new(local_peer_id, fallible_service_id, local_fn_name, "")
    );
}

#[test]
fn fail_with_literals_tetraplets() {
    let local_peer_id = "local_peer_id";
    let (host_closure, tetraplet_anchor) = tetraplet_host_function(echo_call_service());
    let mut vm = create_avm(host_closure, local_peer_id);

    let script = f!(r#"
            (xor
                (xor
                    (fail 1337 "error message")
                    (fail %last_error%)
                )
                (call "{local_peer_id}" ("" "") [%last_error%])
            )"#);

    let _ = checked_call_vm!(vm, local_peer_id, script, "", "");
    assert_eq!(
        tetraplet_anchor.borrow()[0][0],
        SecurityTetraplet::literal_tetraplet(local_peer_id)
    );
}
