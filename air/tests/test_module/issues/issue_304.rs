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

use air_test_framework::AirScriptExecutor;
use air_test_utils::prelude::*;

#[tokio::test]
// test for github.com/fluencelabs/aquavm/issues/304
async fn issue_304() {
    let script = r#"
        (par
           (seq
              (fail 1 "error")
              (seq
                 (call "peer_id1" ("" "") [] $stream)
                 (canon "peer_id1" $stream #can)))
           (fold #can i
              (null))
        )
    "#;

    let init_peer_id = "init_peer_id";
    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_id), script)
        .await
        .expect("invalid test AIR script");

    let res = executor.execute_one(init_peer_id).await.unwrap();
    assert_eq!(res.ret_code, air_interpreter_interface::INTERPRETER_SUCCESS);
}
