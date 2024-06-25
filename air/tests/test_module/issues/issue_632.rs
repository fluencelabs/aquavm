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

use air_test_utils::{key_utils::derive_dummy_keypair, prelude::*};
use std::cell::RefCell;

use std::rc::Rc;

#[tokio::test]
async fn issue_310() {
    let (key_pair, peer_id) = derive_dummy_keypair("init_peer_id");
    let particle_id = "particle_id";

    let air_script = r#"
      (xor
       (seq
        (par
         (call %init_peer_id% ("" "") [0])
         (call %init_peer_id% ("" "") [1] x)
        )
        (ap x $y)
       )
       (call %init_peer_id% ("" "") [2])
      )
    "#;

    let runner = Rc::new(RefCell::new(DefaultAirRunner::new(&peer_id, <_>::default()).await));
    let call = |prev_data, call_results| {
        let runner = runner.clone();
        let peer_id = peer_id.clone();
        let key_pair = key_pair.as_inner();
        let particle_id = particle_id.to_owned();
        async move {
            runner
                .borrow_mut()
                .call(
                    air_script,
                    prev_data,
                    "",
                    peer_id,
                    0,
                    0,
                    None,
                    call_results,
                    key_pair,
                    particle_id,
                )
                .await
                .unwrap()
        }
    };

    let res1 = call(&b""[..], <_>::default()).await;
    assert_eq!(res1.ret_code, 0);
    assert_eq!(res1.call_requests.len(), 2, "test invalid");

    let res2 = call(
        &res1.data[..],
        maplit::hashmap! {
            1u32 => CallServiceResult::ok(json!(0)),
        },
    )
    .await;
    assert_eq!(res2.ret_code, 0);
    // in the version without ap join behavior, it was 1.
    assert_eq!(res2.call_requests.len(), 0);

    let res3 = call(
        &res2.data[..],
        maplit::hashmap! {
            2u32 => CallServiceResult::ok(json!(0)),
        },
    )
    .await;

    // previously was an error:
    //   on instruction 'ap x $y' trace handler encountered an error: state from previous `Call(..)`
    //   is incompatible with expected ap"
    assert_eq!(res3.ret_code, 0, "{}", res3.error_message);
}
