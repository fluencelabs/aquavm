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
use std::collections::HashMap;

use serde_json::json;

#[test]
// https://github.com/fluencelabs/aquavm/issues/177
fn issue_177() {
    let client_peer_id = "12D3KooWMMcNVt5AsiisAHbkfyZWKHufB2dkHCY5pUqZ6AQgEVK6";
    let relay_peer_id = "12D3KooWSD5PToNiLQwKDXsu8JSysCwUt8BVUJEqCHcDe7P5h45e";
    let variables = maplit::hashmap! {
        "-relay-".to_string() => json!(relay_peer_id),
        "noop".to_string() => json!([]),
        "string_to_parse_iter".to_string() => json!(
            "CovLVG4fQcqVPcweSGV5ANQ8NQ2hJnVQrFJJPyQvdKmMDDNDuYYveDy4ncnmDbsvRFA5FcG"
        ),
        "neighborhood".to_string() => json!([
            "12D3KooWGzNvhSDsgFoHwpWHAyPf1kcTYCGeRBPfznL8J6qdyu2H",
            "12D3KooWJbJFaZ3k5sNd8DjQgg3aERoKtBAnirEvPV8yp76kEXHB",
            "12D3KooWBSdm6TkqnEFrgBuSkpVE3dR1kr6952DsWQRNwJZjFZBv",
            "12D3KooWKnRcsTpYx9axkJ6d69LPfpPXrkVLe96skuPTAo76LLVH",
            "12D3KooWHCJbJKGDfCgHSoCuK9q4STyRnVveqLoXAPBbXHTZx9Cv",
            "12D3KooWMhVpgfQxBLkQkJed8VFNvgN4iE6MD7xCybb1ZYWW2Gtz",
            "12D3KooWF7gjXhQ4LaKj6j7ntxsPpGk34psdQicN2KNfBi9bFKXg",
            "12D3KooWBUJifCTgaxAUrcM9JysqCcS4CS8tiYH5hExbdWCAoNwb",
            "12D3KooWEXNUbCXooUwHrHBbrmjsrpHXoEphPwbjQXEGyzbqKnE9",
            "12D3KooWHk9BjDQBUqnavciRPhAYFvqKBe4ZiPPvde7vDaqgn5er",
            "12D3KooWDcpWuyrMTDinqNgmXAuRdfd2mTdY9VoXZSAet2pDzh6r",
            "12D3KooWCKCeqLPSgMnDjyFsJuWqREDtKNHx1JEBiwaMXhCLNTRb",
            "12D3KooWMigkP4jkVyufq5JnDJL6nXvyjeaDNpRfEZqQhsG3sYCU",
            "12D3KooWB9P1xmV3c7ZPpBemovbwCiRRTKd3Kq2jsVPQN4ZukDfy",
            "12D3KooWAKNos2KogexTXhrkMZzFYpLHuWJ4PgoAhurSAv7o5CWA",
            "12D3KooWDUszU2NeWyUVjCXhGEt1MoZrhvdmaQQwtZUriuGN1jTr",
            "12D3KooWKnEqMfYo9zvfHmqTLpLdiHXPe4SVqUWcWHDJdFGrSmcA",
            "12D3KooWEFFCZnar1cUJQ3rMWjvPQg6yMV2aXWs2DkJNSRbduBWn",
            "12D3KooWHBG9oaVx4i3vi6c1rSBUm7MLBmyGmmbHoZ23pmjDCnvK",
            "12D3KooWFpQ7LHxcC9FEBUh3k4nSCC12jBhijJv3gJbi7wsNYzJ5"
        ]),
    };

    let mut client = create_avm(
        set_variables_call_service(variables.clone(), VariableOptionSource::FunctionName),
        client_peer_id,
    );
    let mut relay = create_avm(
        set_variables_call_service(variables, VariableOptionSource::FunctionName),
        relay_peer_id,
    );

    let script = include_str!("scripts/issue_177.air");

    // client 1: demand result for (call %init_peer_id% ("getDataSrv" "-relay-") [] -relay-)
    let client_result_1 = client
        .runner
        .call(script, "", "", client_peer_id, 0, 0, None, HashMap::new())
        .expect("call should be success");
    let expected_call_requests = maplit::hashmap! {
        1 => CallRequestParams::new("getDataSrv", "-relay-", vec![], vec![]),
    };
    assert_eq!(client_result_1.call_requests, expected_call_requests);

    let call_results = maplit::hashmap! {
        1 => CallServiceResult::ok(json!("12D3KooWSD5PToNiLQwKDXsu8JSysCwUt8BVUJEqCHcDe7P5h45e"))
    };

    // client 2: send result to the specified relay
    let client_result_2 = client
        .runner
        .call(
            script,
            client_result_1.data,
            "",
            client_peer_id,
            0,
            0,
            None,
            call_results,
        )
        .expect("call should be success");
    assert!(client_result_2.call_requests.is_empty());
    assert_eq!(client_result_2.next_peer_pks, vec![relay_peer_id.to_string()]);

    // relay 1: execute one time (without providing call results) on the relay and them send back to the client
    let relay_result_1 = relay
        .runner
        .call(
            script,
            "",
            client_result_2.data.clone(),
            client_peer_id,
            0,
            0,
            None,
            HashMap::new(),
        )
        .expect("call should be success");
    let expected_call_requests = maplit::hashmap! {
        1 => CallRequestParams::new("op", "noop", vec![], vec![]),
    };
    assert_eq!(relay_result_1.call_requests, expected_call_requests);
    assert!(relay_result_1.next_peer_pks.is_empty());

    // relay 2:
    let call_results = maplit::hashmap! {
        1 => CallServiceResult::ok(json!(""))
    };
    let relay_result_2 = relay
        .runner
        .call(
            script,
            relay_result_1.data,
            "",
            client_peer_id,
            0,
            0,
            None,
            call_results,
        )
        .expect("call should be success");
    assert!(relay_result_2.next_peer_pks.is_empty());

    // relay 3:
    let call_results = maplit::hashmap! {
        2 => CallServiceResult::ok(json!("CovLVG4fQcqVPcweSGV5ANQ8NQ2hJnVQrFJJPyQvdKmMDDNDuYYveDy4ncnmDbsvRFA5FcG"))
    };
    let relay_result_3 = relay
        .runner
        .call(
            script,
            relay_result_2.data,
            "",
            client_peer_id,
            0,
            0,
            None,
            call_results,
        )
        .expect("call should be success");
    assert!(relay_result_3.next_peer_pks.is_empty());

    // relay 4:
    let call_results = maplit::hashmap! {
        3 => CallServiceResult::ok(json!(["12D3KooWBUJifCTgaxAUrcM9JysqCcS4CS8tiYH5hExbdWCAoNwb","12D3KooWF7gjXhQ4LaKj6j7ntxsPpGk34psdQicN2KNfBi9bFKXg","12D3KooWBSdm6TkqnEFrgBuSkpVE3dR1kr6952DsWQRNwJZjFZBv","12D3KooWKnRcsTpYx9axkJ6d69LPfpPXrkVLe96skuPTAo76LLVH","12D3KooWEFFCZnar1cUJQ3rMWjvPQg6yMV2aXWs2DkJNSRbduBWn","12D3KooWMhVpgfQxBLkQkJed8VFNvgN4iE6MD7xCybb1ZYWW2Gtz","12D3KooWGzNvhSDsgFoHwpWHAyPf1kcTYCGeRBPfznL8J6qdyu2H","12D3KooWJbJFaZ3k5sNd8DjQgg3aERoKtBAnirEvPV8yp76kEXHB","12D3KooWCKCeqLPSgMnDjyFsJuWqREDtKNHx1JEBiwaMXhCLNTRb","12D3KooWHBG9oaVx4i3vi6c1rSBUm7MLBmyGmmbHoZ23pmjDCnvK","12D3KooWB9P1xmV3c7ZPpBemovbwCiRRTKd3Kq2jsVPQN4ZukDfy","12D3KooWAKNos2KogexTXhrkMZzFYpLHuWJ4PgoAhurSAv7o5CWA","12D3KooWEXNUbCXooUwHrHBbrmjsrpHXoEphPwbjQXEGyzbqKnE9","12D3KooWHk9BjDQBUqnavciRPhAYFvqKBe4ZiPPvde7vDaqgn5er","12D3KooWDUszU2NeWyUVjCXhGEt1MoZrhvdmaQQwtZUriuGN1jTr","12D3KooWKnEqMfYo9zvfHmqTLpLdiHXPe4SVqUWcWHDJdFGrSmcA","12D3KooWHCJbJKGDfCgHSoCuK9q4STyRnVveqLoXAPBbXHTZx9Cv","12D3KooWMigkP4jkVyufq5JnDJL6nXvyjeaDNpRfEZqQhsG3sYCU","12D3KooWDcpWuyrMTDinqNgmXAuRdfd2mTdY9VoXZSAet2pDzh6r","12D3KooWJd3HaMJ1rpLY1kQvcjRPEvnDwcXrH8mJvk7ypcZXqXGE"]))
    };
    let relay_result_4 = relay
        .runner
        .call(
            script,
            relay_result_3.data,
            "",
            client_peer_id,
            0,
            0,
            None,
            call_results,
        )
        .expect("call should be success");

    // client 4: receive result from the relay
    // demand result for (call %init_peer_id% ("op" "noop") [])
    let client_result_3 = client
        .runner
        .call(
            script,
            client_result_2.data,
            relay_result_4.data,
            client_peer_id,
            0,
            0,
            None,
            HashMap::new(),
        )
        .expect("call should be success");
    let expected_call_requests = maplit::hashmap! {
        2 => CallRequestParams::new("op", "noop", vec![], vec![])
    };
    assert_eq!(client_result_3.call_requests, expected_call_requests);

    let call_results = maplit::hashmap! {
        2 => CallServiceResult::ok(json!(""))
    };

    // client 5: (call %init_peer_id% ("op" "identity") [$res.$.[19]!]) joined
    // demand a result for (call %init_peer_id% ("peer" "timeout") [1000 "timeout"])
    let client_result_4 = client
        .runner
        .call(
            script,
            client_result_3.data,
            "",
            client_peer_id,
            0,
            0,
            None,
            call_results,
        )
        .expect("call should be success");
    let expected_call_requests = maplit::hashmap! {
        3 => CallRequestParams::new("peer", "timeout", vec![json!(1000u64), json!("timeout")], vec![
            vec![SecurityTetraplet::new("12D3KooWMMcNVt5AsiisAHbkfyZWKHufB2dkHCY5pUqZ6AQgEVK6", "", "", "")],
            vec![SecurityTetraplet::new("12D3KooWMMcNVt5AsiisAHbkfyZWKHufB2dkHCY5pUqZ6AQgEVK6", "", "", "")],
        ])
    };
    assert_eq!(client_result_4.call_requests, expected_call_requests);

    let call_results = maplit::hashmap! {
        3 => CallServiceResult::ok(json!(""))
    };

    // timeout requests provided
    let client_result_5 = client.runner.call(
        script,
        client_result_4.data,
        "",
        client_peer_id,
        0,
        0,
        None,
        call_results,
    );
    // before patch the interpreter crashed here
    assert!(client_result_5.is_ok());
}
