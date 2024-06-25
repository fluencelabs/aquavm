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

use air_test_framework::AirScriptExecutor;
use air_test_utils::prelude::*;

use futures::StreamExt;

#[tokio::test]
async fn issue_356() {
    let script = r#"
        (seq
            (seq
                (call "relay" ("kad" "neighborhood") ["relay"] neighs_top) ; ok = [@"p1"]
                (seq
                    (call "p1" ("kad" "neighborhood") ["p1"] neighs_inner) ; ok =[@"p1"]
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
                (seq
                    (canon "client" $external_addresses #external_addresses)
                    (call "client" ("return" "") [#external_addresses neighs_inner] x) ; ok = null
                )
            )
        )
        "#;

    let engine = <AirScriptExecutor>::new(
        TestRunParameters::from_init_peer_id("client"),
        vec![],
        vec!["p1", "p2", "p3"].into_iter().map(Into::into),
        script,
    )
    .await
    .unwrap();

    for _ in 0..7 {
        for peer in ["client", "relay", "p1", "p2"] {
            for outcome in engine.execution_iter(peer).unwrap().collect::<Vec<_>>().await {
                assert_eq!(outcome.ret_code, 0, "{outcome:?}");
            }
        }
    }
}
