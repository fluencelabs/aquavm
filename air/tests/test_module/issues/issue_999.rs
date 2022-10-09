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

const PREV_DATA: &str = r#"{"trace":[{"call":{"executed":{"scalar":"12D3KooWAKNos2KogexTXhrkMZzFYpLHuWJ4PgoAhurSAv7o5CWA"}}},{"call":{"executed":{"stream":{"value":{"air_version":"js-0.24.2","external_addresses":[],"node_version":"js-0.23.0"},"generation":0}}}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"canon":{"ids":[2,3]}},{"par":[6,0]},{"par":[3,2]},{"call":{"executed":{"scalar":{"air_version":"0.30.11","external_addresses":["/ip4/134.209.186.43/tcp/7770","/ip4/134.209.186.43/tcp/9990/ws"],"node_version":"0.0.347-bump-aquavm-to-0-30-0"}}}},{"ap":{"gens":[0]}},{"call":{"sent_by":"12D3KooWDcpWuyrMTDinqNgmXAuRdfd2mTdY9VoXZSAet2pDzh6r"}},{"par":[1,0]},{"call":{"sent_by":"12D3KooWAKNos2KogexTXhrkMZzFYpLHuWJ4PgoAhurSAv7o5CWA"}},{"fold":{"lore":[{"pos":8,"desc":[{"pos":13,"len":1},{"pos":14,"len":0}]}]}},{"call":{"sent_by":"12D3KooWDcpWuyrMTDinqNgmXAuRdfd2mTdY9VoXZSAet2pDzh6r"}}],"streams":{"$identify_res":1},"version":"0.2.2","lcid":1,"r_streams":{"$successful":{"164":[1]},"$success":{"191":[0]},"$status":{"142":[0]},"$array-inline":{"451":[1]},"$successful_test":{"1675":[0]}}}"#;
const CURRENT_DATA:&str = r#"{"trace":[{"call":{"executed":{"scalar":"12D3KooWAKNos2KogexTXhrkMZzFYpLHuWJ4PgoAhurSAv7o5CWA"}}},{"call":{"executed":{"stream":{"value":{"air_version":"js-0.24.2","external_addresses":[],"node_version":"js-0.23.0"},"generation":0}}}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"canon":{"ids":[2,3]}},{"par":[8,0]},{"par":[3,4]},{"call":{"executed":{"scalar":{"air_version":"0.30.11","external_addresses":["/ip4/134.209.186.43/tcp/7770","/ip4/134.209.186.43/tcp/9990/ws"],"node_version":"0.0.347-bump-aquavm-to-0-30-0"}}}},{"ap":{"gens":[1]}},{"call":{"executed":{"scalar":""}}},{"par":[3,0]},{"call":{"executed":{"scalar":{"air_version":"0.30.11","external_addresses":["/ip4/134.209.186.43/tcp/7001","/ip4/134.209.186.43/tcp/9001/ws"],"node_version":"0.0.347-bump-aquavm-to-0-30-0"}}}},{"ap":{"gens":[0]}},{"call":{"executed":{"scalar":""}}},{"fold":{"lore":[{"pos":12,"desc":[{"pos":15,"len":3},{"pos":18,"len":0}]},{"pos":8,"desc":[{"pos":18,"len":3},{"pos":21,"len":0}]}]}},{"call":{"executed":{"scalar":1}}},{"ap":{"gens":[0]}},{"canon":{"ids":[16]}},{"call":{"executed":{"scalar":1}}},{"ap":{"gens":[1]}},{"canon":{"ids":[16,19]}},{"canon":{"ids":[16]}},{"ap":{"gens":[0]}},{"fold":{"lore":[{"pos":22,"desc":[{"pos":24,"len":3},{"pos":27,"len":0}]}]}},{"call":{"executed":{"scalar":1}}},{"ap":{"gens":[0]}},{"canon":{"ids":[25]}},{"canon":{"ids":[25]}},{"ap":{"gens":[0]}},{"call":{"sent_by":"12D3KooWAKNos2KogexTXhrkMZzFYpLHuWJ4PgoAhurSAv7o5CWA"}}],"streams":{"$identify_res":1},"version":"0.2.2","lcid":5,"r_streams":{"$status":{"142":[1]},"$success":{"191":[1]},"$successful":{"164":[2]},"$successful_test":{"1675":[2]},"$array-inline":{"451":[1]},"$status_test":{"2578":[1]}}}"#;

#[test]
fn issue_999() {
    let peer_id = "peer_id";
    let mut vm = create_avm(echo_call_service(), peer_id);

    let script = "
(xor
 (seq
  (seq
   (seq
    (seq
     (seq
      (seq
       (seq
        (call %init_peer_id% (\"getDataSrv\" \"-relay-\") [] -relay-)
        (new $status
         (new $successful
          (new $success
           (seq
            (call %init_peer_id% (\"peer\" \"identify\") [] $identify_res)
            (xor
             (seq
              (seq
               (seq
                (seq
                 (seq
                  (seq
                   (new $array-inline
                    (seq
                     (seq
                      (ap \"12D3KooWDcpWuyrMTDinqNgmXAuRdfd2mTdY9VoXZSAet2pDzh6r\" $array-inline)
                      (ap \"12D3KooWHCJbJKGDfCgHSoCuK9q4STyRnVveqLoXAPBbXHTZx9Cv\" $array-inline)
                     )
                     (canon -relay- $array-inline  #array-inline-0)
                    )
                   )
                   (par
                    (fold #array-inline-0 n-0
                     (par
                      (seq
                       (xor
                        (seq
                         (call n-0 (\"peer\" \"identify\") [] indentify_res)
                         (ap true $successful)
                        )
                        (seq
                         (call -relay- (\"op\" \"noop\") [])
                         (call %init_peer_id% (\"errorHandlingSrv\" \"error\") [%last_error% 1])
                        )
                       )
                       (call -relay- (\"op\" \"noop\") [])
                      )
                      (next n-0)
                     )
                     (never)
                    )
                    (null)
                   )
                  )
                  (new $successful_test
                   (seq
                    (fold $successful s
                     (seq
                      (seq
                       (seq
                        (call -relay- (\"math\" \"add\") [0 1] incr_idx)
                        (ap s $successful_test)
                       )
                       (canon -relay- $successful_test  #successful_iter_canon)
                      )
                      (xor
                       (match #successful_iter_canon.length incr_idx
                        (null)
                       )
                       (next s)
                      )
                     )
                     (never)
                    )
                    (canon -relay- $successful_test  #successful_result_canon)
                   )
                  )
                 )
                 (ap \"ok\" $status)
                )
                (new $status_test
                 (seq
                  (fold $status s
                   (seq
                    (seq
                     (seq
                      (call -relay- (\"math\" \"add\") [0 1] incr_idx)
                      (ap s $status_test)
                     )
                     (canon -relay- $status_test  #status_iter_canon)
                    )
                    (xor
                     (match #status_iter_canon.length incr_idx
                      (null)
                     )
                     (next s)
                    )
                   )
                   (never)
                  )
                  (canon -relay- $status_test  #status_result_canon)
                 )
                )
               )
               (ap true $success)
              )
              (xor
               (seq
                (call \"12D3KooWDcpWuyrMTDinqNgmXAuRdfd2mTdY9VoXZSAet2pDzh6r\" (\"op\" \"noop\") [])
                (call -relay- (\"op\" \"noop\") [])
               )
               (seq
                (call -relay- (\"op\" \"noop\") [])
                (call %init_peer_id% (\"errorHandlingSrv\" \"error\") [%last_error% 2])
               )
              )
             )
             (call %init_peer_id% (\"errorHandlingSrv\" \"error\") [%last_error% 3])
            )
           )
          )
         )
        )
       )
       (canon %init_peer_id% $identify_res  #push-to-stream-14)
      )
      (ap #push-to-stream-14 $-some-unique-res-name-0)
     )
     (canon %init_peer_id% $-some-unique-res-name-0  #-some-unique-res-name-0_canon)
    )
    (call %init_peer_id% (\"--after-callback-srv-service--\" \"console-log\") [#-some-unique-res-name-0_canon])
   )
   (call %init_peer_id% (\"--finisher--\" \"--finish-execution--\") [])
  )
  (xor
   (call %init_peer_id% (\"callbackSrv\" \"response\") [\"ok\"])
   (call %init_peer_id% (\"errorHandlingSrv\" \"error\") [%last_error% 4])
  )
 )
 (call %init_peer_id% (\"errorHandlingSrv\" \"error\") [%last_error% 5])
)

    ";

    let prev_data: InterpreterData = serde_json::from_str(PREV_DATA).unwrap();
    let prev_data = serde_json::to_vec(&prev_data).unwrap();

    let current_data: InterpreterData = serde_json::from_str(CURRENT_DATA).unwrap();
    let current_data = serde_json::to_vec(&current_data).unwrap();
    let result = checked_call_vm!(vm, <_>::default(), script, prev_data, current_data);
    print_trace(&result, "");
}
