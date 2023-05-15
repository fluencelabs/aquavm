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

use air_test_utils::prelude::*;
use air::ExecutionCidState;

#[test]
fn test_attack_current_injection() {
    // Injecting a value that arrives to peer who does the next step
    let alice_peer = "alice_peer";
    let mallory_peer = "mallory_peer";

    let air_script = format!(r#"
    (seq
       (seq
          (call "{alice_peer}" ("" "") [] x)
          (call "{mallory_peer}" ("" "") [] y))
       (call "{alice_peer}" ("" "") [] x))
    "#);

    let mut alice_tracker = ExecutionCidState::default();

    let alice_call_1 = scalar_tracked!(
        "good result",
        &mut alice_tracker,
        peer = alice_peer
    );
    let alice_trace = vec![
        alice_call_1.clone(),
    ];

    let mut mallory_tracker = alice_tracker.clone();
    let mallory_call_2 = scalar_tracked!(
        "valid result",
        &mut mallory_tracker,
        peer = mallory_peer
    );
    let fake_call_3 = scalar_tracked!(
        "fake result",
        &mut mallory_tracker
    );
    let mallory_trace = vec![
        alice_call_1,
        mallory_call_2,
        fake_call_3,
    ];

    assert!(false);
}
