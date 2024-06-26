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

use air_interpreter_interface::INTERPRETER_SUCCESS;
use air_test_utils::prelude::*;

#[tokio::test]
// test for github.com/fluencelabs/aquavm/issues/306
async fn issue_306() {
    let peer_id_1 = "peer_id_1";
    let mut peer_vm_1 = create_avm(echo_call_service(), peer_id_1).await;

    let script = format!(
        r#"
        (new $stream
            (seq
                (canon "{peer_id_1}" $stream #canon_stream)
                (fold #canon_stream iterator
                    (ap iterator $stream))))
    "#
    );

    let result = call_vm!(peer_vm_1, <_>::default(), &script, "", "");
    assert_eq!(result.ret_code, INTERPRETER_SUCCESS)
}
