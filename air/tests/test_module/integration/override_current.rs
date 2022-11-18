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

use air_test_utils::prelude::*;
use fstrings::f;
use fstrings::format_args_f;

use std::collections::HashMap;

#[test]
fn test_override_current_peer_id() {
    let spell_id = "spell_id";
    let host_peer_id = "host_peer_id";
    let script = f!(r#"(call "{}" ("service" "func") [])"#, spell_id);

    let variables = maplit::hashmap! {
        "func".to_owned() => json!("success"),
    };

    let mut client = create_avm(
        set_variables_call_service(variables, VariableOptionSource::FunctionName),
        host_peer_id,
    );

    let current_result_1 = client
        .runner
        .call(&script, "", "", spell_id, 0, 0, None, HashMap::new())
        .expect("call should be success");

    let expected_current_call_requests = HashMap::new();
    let expected_current_next_peers_pks = vec![spell_id.to_owned()];

    assert_eq!(current_result_1.call_requests, expected_current_call_requests);
    assert_eq!(current_result_1.next_peer_pks, expected_current_next_peers_pks);

    let spell_result_1 = client
        .runner
        .call(
            script,
            "",
            "",
            spell_id,
            0,
            0,
            Some(spell_id.to_owned()),
            HashMap::new(),
        )
        .expect("call should be success");

    let expected_spell_call_requests = maplit::hashmap! {
        1 => CallRequestParams::new("service", "func", vec![], vec![]),
    };
    let expected_spell_next_peers_pks = Vec::<String>::new();

    assert_eq!(spell_result_1.call_requests, expected_spell_call_requests);
    assert_eq!(spell_result_1.next_peer_pks, expected_spell_next_peers_pks);
}
