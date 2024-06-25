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
use fluence_keypair::KeyFormat;
use fluence_keypair::KeyPair;

#[tokio::test]
async fn canon_generates_hop() {
    let client_peer_id = "client";
    let mut client_vm = create_avm(unit_call_service(), client_peer_id).await;

    let test_peer_id = "test_peer_id";

    let script = format!(
        r#"
        (canon "{test_peer_id}" $empty_stream #canon_stream)
    "#
    );

    let client_result = checked_call_vm!(client_vm, <_>::default(), script, "", "");
    assert_next_pks!(&client_result.next_peer_pks, &[test_peer_id]);
}

#[tokio::test]
async fn canon_with_join_behaviour() {
    let relay_peer_id = "relay";
    let mut relay_vm = create_avm(unit_call_service(), relay_peer_id).await;
    let client_peer_id = "client";
    let mut client_vm = create_avm(unit_call_service(), client_peer_id).await;

    let friend_peer_id = "friend";
    let mut friend_vm = create_avm(unit_call_service(), friend_peer_id).await;
    let friend_relay_peer_id = "friend_relay";
    let mut friend_relay_vm = create_avm(unit_call_service(), friend_relay_peer_id).await;

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
           (new $hop-stream-drop
            (new #hop-canon-drop
             (canon -relay- $hop-stream-drop  #hop-canon-drop)
            )
           )
           (new $hop-stream-drop
            (new #hop-canon-drop
             (canon friendRelay $hop-stream-drop  #hop-canon-drop)
            )
           )
          )
          (xor
           (seq
            (seq
             (call friend ("testo" "getString") ["testo string via friend "] str)
             (new $hop-stream-drop
              (new #hop-canon-drop
               (canon friendRelay $hop-stream-drop  #hop-canon-drop)
              )
             )
            )
            (new $hop-stream-drop
             (new #hop-canon-drop
              (canon -relay- $hop-stream-drop  #hop-canon-drop)
             )
            )
           )
           (seq
            (seq
             (new $hop-stream-drop
              (new #hop-canon-drop
               (canon friendRelay $hop-stream-drop  #hop-canon-drop)
              )
             )
             (new $hop-stream-drop
              (new #hop-canon-drop
               (canon -relay- $hop-stream-drop  #hop-canon-drop)
              )
             )
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

    let keypair = KeyPair::generate(KeyFormat::Ed25519);
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
        .await
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
