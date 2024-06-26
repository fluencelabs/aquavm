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
