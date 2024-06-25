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
async fn empty_array() {
    let vm_peer_id = "some_peer_id";
    let mut vm = create_avm(echo_call_service(), vm_peer_id).await;

    let script = format!(
        r#"
        (seq 
           (call "{vm_peer_id}" ("" "") [[]] result)
           (call "{vm_peer_id}" ("" "") [result])
        )"#
    );

    let result = checked_call_vm!(vm, <_>::default(), script, "", "");
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        scalar!(json!([]), peer = vm_peer_id, args = vec![json!([])]),
        unused!(json!([]), peer = vm_peer_id, args = vec![json!([])]),
    ];

    assert_eq!(actual_trace, expected_trace);
    assert!(result.next_peer_pks.is_empty());
}
