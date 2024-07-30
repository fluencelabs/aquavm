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

#[tokio::test]
async fn embed_basic() {
    let init_peer_id = "init_peer_id";
    let mut vm = create_avm(unit_call_service(), init_peer_id).await;

    let script = r#"
        (seq
            (embed []
(#
"a string\nwith escape"
#)
                var)
            (call %init_peer_id% ("" "") [var] result_name))"#;

    let result = checked_call_vm!(vm, <_>::default(), script, "", "");
    assert!(result.next_peer_pks.is_empty());

    let expected_trace = vec![scalar!(
        json!("a string\nwith escape"),
        peer_name = init_peer_id,
        args = ["a string\nwith escape"]
    )];

    let trace = trace_from_result(&result);
    assert_eq!(trace, expected_trace);
}
