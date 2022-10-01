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

use air_test_framework::TestExecutor;
use air_test_utils::prelude::*;

pub fn join_stream(stream: &str, relay: &str, length: &str, result: &str) -> String {
    f!(r#"
        (new $monotonic_stream
            (seq
                (fold ${stream} elem
                    (seq
                        (ap elem $monotonic_stream)
                        (seq
                            (canon {relay} $monotonic_stream #result)
                            (xor
                                (match #result.length {length}
                                    (null)
                                )
                                (next elem)
                            )
                        )
                    )
                )
                (canon {relay} ${stream} #{result})
            )
        )
    "#)
}

#[test]
fn folex_bug_999() {
    env_logger::init();
    let script = format!(
        r#"
        (seq
            (seq
                (call "relay" ("kad" "neighborhood") ["relay"] neighs_top) ; ok = ["p1"]
                (seq
                    (call "p1" ("kad" "neighborhood") ["p1"] neighs_inner) ; ok =["p1"]
                    (par
                        (call "relay" ("peer" "identify") ["relay"] $external_addresses) ; behaviour = echo
                        (call "p1" ("peer" "identify") ["p1"] $external_addresses) ; behaviour = echo
                    )
                )
            )
            (seq
                (new $monotonic_stream
                    (fold $external_addresses elem
                        (seq
                            (ap "asd" $monotonic_stream)
                            (seq
                                (canon "relay" $monotonic_stream #result)
                                (null)
                            )
                        )
                    )
                )
                (call "client" ("return" "") [$external_addresses neighs_inner] x) ; ok = null
            )
        )
        "#,
       // join_stream("external_addresses", "\"relay\"", "2", "joined_addresses"),
    );

    let engine = TestExecutor::new(
        TestRunParameters::from_init_peer_id("client"),
        vec![],
        vec!["p1","p2","p3"].into_iter().map(Into::into),
        &script,
    ).unwrap();

    for _ in 0..7 {
        for peer in ["client", "relay", "p1", "p2"] {
            let it = engine.execution_iter(peer).unwrap();
            for v in it {
                assert_eq!(v.ret_code, 0, "{:?}", v);
            }
        }
    }
    assert!(false);
}
