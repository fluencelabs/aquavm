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

use air_interpreter_signatures::KeyPair;
use air_test_utils::prelude::*;
use fluence_keypair::KeyFormat;

#[test]
fn call_generates_hop() {
    let client_peer_id = "client";
    let mut client_vm = create_avm(unit_call_service(), client_peer_id);

    let test_peer_id = "test_peer_id";

    let script = format!(
        r#"
        (call "{test_peer_id}" ("" "") [])
    "#
    );

    let client_result = checked_call_vm!(client_vm, <_>::default(), script, "", "");
    assert_next_pks!(&client_result.next_peer_pks, &[test_peer_id]);
}

#[test]
fn call_with_join_behaviour() {
    let relay_peer_id = "relay";
    let mut relay_vm = create_avm(unit_call_service(), relay_peer_id);
    let client_peer_id = "client";
    let mut client_vm = create_avm(unit_call_service(), client_peer_id);

    let friend_peer_id = "friend";
    let mut friend_vm = create_avm(unit_call_service(), friend_peer_id);
    let friend_relay_peer_id = "friend_relay";
    let mut friend_relay_vm = create_avm(unit_call_service(), friend_relay_peer_id);

    let script = format!(
        r#"
    (xor
     (seq
      (seq
       (seq
        (seq
         (seq
          (seq
           (seq
            (ap "{relay_peer_id}" -relay- )
            (ap "{client_peer_id}" me)
           )
           (ap "{relay_peer_id}" myRelay)
          )
          (ap "{friend_peer_id}" friend)
         )
         (ap "{friend_relay_peer_id}" friendRelay)
        )
        (par
         (seq
          (seq
           (call -relay- ("op" "noop") [])
           (call friendRelay ("op" "noop") [])
          )
          (xor
           (seq
            (seq
             (call friend ("testo" "getString") ["testo string via friend "] str)
             (call friendRelay ("op" "noop") [])
            )
            (call -relay- ("op" "noop") [])
           )
           (seq
            (seq
             (call friendRelay ("op" "noop") [])
             (call -relay- ("op" "noop") [])
            )
            (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 1])
           )
          )
         )
         (call %init_peer_id% ("lp" "print") ["string in par"])
        )
       )
       (call %init_peer_id% ("lp" "print") [str])
      )
      (xor
       (call %init_peer_id% ("callbackSrv" "response") ["finish"])
       (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 2])
      )
     )
     (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 3])
    )
"#
    );

    let keypair = KeyPair::generate(KeyFormat::Ed25519).unwrap();
    let run_parameters = TestRunParameters::new(client_peer_id.to_string(), 0, 0, "".to_string());

    let client_result = client_vm
        .runner
        .call(
            &script,
            "",
            "",
            client_peer_id,
            0,
            0,
            None,
            <_>::default(),
            &keypair,
            "".to_string(),
        )
        .unwrap();
    assert_next_pks!(&client_result.next_peer_pks, &[relay_peer_id]);

    let relay_result = checked_call_vm!(relay_vm, run_parameters.clone(), &script, "", client_result.data);
    assert_next_pks!(&relay_result.next_peer_pks, &[friend_relay_peer_id]);

    let friend_relay_result = checked_call_vm!(
        friend_relay_vm,
        run_parameters.clone(),
        &script,
        "",
        relay_result.data.clone()
    );
    assert_next_pks!(&friend_relay_result.next_peer_pks, &[friend_peer_id]);

    let friend_result = checked_call_vm!(
        friend_vm,
        run_parameters.clone(),
        &script,
        "",
        friend_relay_result.data.clone()
    );
    assert_next_pks!(&friend_result.next_peer_pks, &[friend_relay_peer_id]);

    let friend_relay_result = checked_call_vm!(
        friend_relay_vm,
        run_parameters.clone(),
        &script,
        friend_relay_result.data,
        friend_result.data
    );
    assert_next_pks!(&friend_relay_result.next_peer_pks, &[relay_peer_id]);

    let relay_result = checked_call_vm!(
        relay_vm,
        run_parameters.clone(),
        &script,
        relay_result.data,
        friend_relay_result.data
    );
    assert_next_pks!(&relay_result.next_peer_pks, &[client_peer_id]);

    let client_result = checked_call_vm!(client_vm, run_parameters.clone(), &script, "", "");
    let client_result = checked_call_vm!(
        client_vm,
        run_parameters.clone(),
        &script,
        client_result.data,
        relay_result.data
    );
    assert!(client_result.next_peer_pks.is_empty());
}
