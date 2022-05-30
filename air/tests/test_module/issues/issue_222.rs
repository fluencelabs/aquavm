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

use air_test_utils::prelude::*;

use pretty_assertions::assert_eq;

#[test]
// test for github.com/fluencelabs/aquavm/issues/222
fn issue_222_possibly() {
    let air_script = r#"
(new $stream
  (par
   (par
      (call "other_1" ("" "") [] $stream)
      (call "other_2" ("" "") [] $stream))
   (fold $stream j
       (seq (call "other_id" ("" "") [j])
            (next j)))))
"#;

    let mut other_id = create_avm(echo_call_service(), "other_id");

    // The bug is triggered when (call "other_2" ...) result arrives to "other_id"
    // before the "other_1" result.
    let cur_data = br#"
{"trace":[
  {"par":[3,2]},
  {"par":[1,1]},
  {"call":{"executed":{"stream":{"value":[3],"generation":0}}}},
  {"call":{"sent_by":"init_id"}},
  {"fold":{"lore":[{"pos":2,"desc":[{"pos":5,"len":1},{"pos":6,"len":0}]}]}},
  {"call":{"sent_by":"other_1"}}
  ],
  "streams":{},"version":"0.2.2","lcid":1,"r_streams":{"$stream":{"0":[1]}}}
"#;

    let prev_data = br#"
{"trace":[
  {"par":[3,2]},
  {"par":[1,1]},
  {"call":{"sent_by":"init_id"}},
  {"call":{"executed":{"stream":{"value":[1],"generation":0}}}},
  {"fold":{"lore":[{"pos":3,"desc":[{"pos":5,"len":1},{"pos":6,"len":0}]}]}},
  {"call":{"executed":{"scalar":[1]}}}
],"streams":{},"version":"0.2.2","lcid":1,"r_streams":{"$stream":{"0":[1]}}}
"#;

    let other_result = checked_call_vm!(
        other_id,
        <_>::default(),
        air_script,
        prev_data.to_vec(),
        cur_data.to_vec()
    );

    let actual_trace = trace_from_result(&other_result);
    let expected_trace = vec![
        executed_state::par(3, 3),
        executed_state::par(1, 1),
        executed_state::stream(json!([3]), 0),
        executed_state::stream(json!([1]), 0),
        executed_state::fold(vec![
            executed_state::subtrace_lore(2, SubTraceDesc::new(5, 1), SubTraceDesc::new(7, 0)),
            executed_state::subtrace_lore(3, SubTraceDesc::new(6, 1), SubTraceDesc::new(7, 0)),
        ]),
        executed_state::scalar(json!([3])),
        executed_state::scalar(json!([1])),
    ];

    assert_eq!(actual_trace, expected_trace);
}
