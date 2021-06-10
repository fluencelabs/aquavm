/*
 * Copyright 2020 Fluence Labs Limited
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

use super::*;

use serde_json::json;

fn test_value(id: usize) -> ExecutedState {
    scalar_jvalue(json!([id]))
}

#[test]
fn too_many_subtraces() {
    let lore = FoldSubTraceLore::default();
    let fold = fold(vec![vec![vec![lore.clone(), lore.clone(), lore.clone()]]]);
    let trace = vec![fold];

    let actual = merge_execution_traces(trace.clone().into(), trace.into());
    let expected: MergeResult<ExecutionTrace> = Err(DataMergingError::FoldIncorrectSubtracesCount(3));
    assert_eq!(actual, expected);
}

#[test]
fn fold_subtraces_overflows() {
    let lore = FoldSubTraceLore {
        value_pos: 0,
        begin_pos: 0,
        interval_len: usize::MAX,
    };
    let fold = fold(vec![vec![vec![lore.clone(), lore.clone()]]]);
    let trace = vec![scalar_jvalue(json!([])), fold];

    let actual = merge_execution_traces(trace.clone().into(), trace.into());
    let expected: MergeResult<ExecutionTrace> = Err(DataMergingError::FoldLenOverflow(FoldResult(vec![vec![vec![
        lore.clone(),
        lore,
    ]]])));
    assert_eq!(actual, expected);
}

#[test]
fn merge_fold_with_several_different_values() {
    let lore_v1_before = FoldSubTraceLore::new(1, 5, 2);
    let lore_v1_after = FoldSubTraceLore::new(1, 11, 2);
    let lore_v2_before = FoldSubTraceLore::new(2, 7, 2);
    let lore_v2_after = FoldSubTraceLore::new(2, 9, 2);

    let fold_state = fold(vec![vec![
        vec![lore_v1_before, lore_v1_after],
        vec![lore_v2_before, lore_v2_after],
    ]]);

    let prev_trace = vec![
        /*  0 */ par(2, 1),
        /*  1 */ scalar_jvalue(json!(1)), // v1
        /*  2 */ scalar_jvalue(json!(2)), // v2
        /*  3 */ request_sent_by(""),
        /*  4 */ fold_state,
        // v1 before trace
        /*  5 */ scalar_jvalue(json!([])),
        /*  6 */ scalar_jvalue(json!([])),
        // v2 before trace
        /*  7 */ scalar_jvalue(json!([])),
        /*  8 */ scalar_jvalue(json!([])),
        // v2 after trace
        /*  9 */ scalar_jvalue(json!([])),
        /* 10 */ scalar_jvalue(json!([])),
        // v1 after trace
        /* 11 */ scalar_jvalue(json!([])),
        /* 12 */ scalar_jvalue(json!([])),
    ];

    let lore_v2_before = FoldSubTraceLore::new(2, 5, 2);
    let lore_v2_after = FoldSubTraceLore::new(2, 11, 2);
    let lore_v3_before = FoldSubTraceLore::new(3, 7, 2);
    let lore_v3_after = FoldSubTraceLore::new(3, 9, 2);

    let fold_state = fold(vec![vec![
        vec![lore_v2_before, lore_v2_after],
        vec![lore_v3_before, lore_v3_after],
    ]]);

    let current_trace = vec![
        /*  0 */ par(2, 1),
        /*  1 */ request_sent_by(""),
        /*  2 */ scalar_jvalue(json!(2)), // v2
        /*  3 */ scalar_jvalue(json!(3)), // v3
        /*  4 */ fold_state,
        // v2 before trace
        /*  5 */ scalar_jvalue(json!([])),
        /*  6 */ scalar_jvalue(json!([])),
        // v3 before trace
        /*  7 */ scalar_jvalue(json!([])),
        /*  8 */ scalar_jvalue(json!([])),
        // v3 after trace
        /*  9 */ scalar_jvalue(json!([])),
        /* 10 */ scalar_jvalue(json!([])),
        // v2 after trace
        /* 11 */ scalar_jvalue(json!([])),
        /* 12 */ scalar_jvalue(json!([])),
    ];

    let actual_trace =
        merge_execution_traces(prev_trace.into(), current_trace.into()).expect("merging should be successful");

    let lore_v1_before = FoldSubTraceLore::new(1, 5, 2);
    let lore_v1_after = FoldSubTraceLore::new(1, 11, 2);
    let lore_v2_before = FoldSubTraceLore::new(2, 7, 2);
    let lore_v2_after = FoldSubTraceLore::new(2, 9, 2);
    let lore_v3_before = FoldSubTraceLore::new(3, 13, 2);
    let lore_v3_after = FoldSubTraceLore::new(3, 15, 2);

    let fold_state = fold(vec![
        vec![vec![lore_v1_before, lore_v1_after], vec![lore_v2_before, lore_v2_after]],
        vec![vec![lore_v3_before, lore_v3_after]],
    ]);

    let expected_trace = vec![
        /*  0 */ par(2, 1),
        /*  1 */ scalar_jvalue(json!(1)), // v1
        /*  2 */ scalar_jvalue(json!(2)), // v2
        /*  3 */ scalar_jvalue(json!(3)), // v3
        /*  4 */ fold_state,
        // v1 before trace
        /*  5 */ scalar_jvalue(json!([])),
        /*  6 */ scalar_jvalue(json!([])),
        // v2 before trace
        /*  7 */ scalar_jvalue(json!([])),
        /*  8 */ scalar_jvalue(json!([])),
        // v2 after trace
        /*  9 */ scalar_jvalue(json!([])),
        /* 10 */ scalar_jvalue(json!([])),
        // v1 after trace
        /* 11 */ scalar_jvalue(json!([])),
        /* 12 */ scalar_jvalue(json!([])),
        // v3 before trace
        /* 13 */ scalar_jvalue(json!([])),
        /* 14 */ scalar_jvalue(json!([])),
        // v3 after trace
        /* 15 */ scalar_jvalue(json!([])),
        /* 16 */ scalar_jvalue(json!([])),
    ];

    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn merge_fold_inside_par() {
    let lore_v1_before = FoldSubTraceLore::new(2, 6, 2);
    let lore_v1_after = FoldSubTraceLore::new(2, 12, 2);
    let lore_v2_before = FoldSubTraceLore::new(3, 8, 2);
    let lore_v2_after = FoldSubTraceLore::new(3, 10, 2);

    let fold_state = fold(vec![vec![
        vec![lore_v1_before, lore_v1_after],
        vec![lore_v2_before, lore_v2_after],
    ]]);

    let prev_trace = vec![
        /*  0 */ par(14, 2), // fold is located inside left Par subtree
        /*  1 */ par(2, 1),
        /*  2 */ test_value(1), // v1
        /*  3 */ test_value(2), // v2
        /*  4 */ request_sent_by(""),
        /*  5 */ fold_state,
        // v1 before trace
        /*  6 */ test_value(4),
        /*  7 */ test_value(5),
        // v2 before trace
        /*  8 */ test_value(6),
        /*  9 */ test_value(7),
        // v2 after trace
        /* 10 */ test_value(8),
        /* 11 */ test_value(9),
        // v1 after trace
        /* 12 */ test_value(10),
        /* 13 */ test_value(11),
        // other states from left par
        /* 14 */ test_value(16),
        /* 15 */ request_sent_by(""),
        /* 16 */ test_value(18),
        /* 17 */ request_sent_by(""),
        // right par subtree
        /* 18 */ test_value(20),
        /* 19 */ test_value(21),
    ];

    let lore_v2_before = FoldSubTraceLore::new(3, 6, 2);
    let lore_v2_after = FoldSubTraceLore::new(3, 12, 2);
    let lore_v3_before = FoldSubTraceLore::new(4, 8, 2);
    let lore_v3_after = FoldSubTraceLore::new(4, 10, 2);

    let fold_state = fold(vec![vec![
        vec![lore_v2_before, lore_v2_after],
        vec![lore_v3_before, lore_v3_after],
    ]]);

    let current_trace = vec![
        /*  0 */ par(14, 2), // fold is located inside left Par subtree
        /*  1 */ par(2, 1),
        /*  4 */ request_sent_by(""),
        /*  2 */ test_value(2), // v2
        /*  3 */ test_value(3), // v3
        /*  5 */ fold_state,
        // v2 before trace
        /*  6 */ test_value(6),
        /*  7 */ test_value(7),
        // v3 before trace
        /*  8 */ test_value(12),
        /*  9 */ test_value(13),
        // v3 after trace
        /* 10 */ test_value(14),
        /* 11 */ test_value(15),
        // v2 after trace
        /* 12 */ test_value(8),
        /* 13 */ test_value(9),
        // other states from left par
        /* 14 */ test_value(16),
        /* 15 */ test_value(17),
        /* 16 */ request_sent_by(""),
        /* 17 */ test_value(19),
        // right par subtree
        /* 18 */ request_sent_by(""),
        /* 19 */ test_value(21),
    ];

    let actual_trace =
        merge_execution_traces(prev_trace.into(), current_trace.into()).expect("merging should be successful");

    let lore_v1_before = FoldSubTraceLore::new(2, 6, 2);
    let lore_v1_after = FoldSubTraceLore::new(2, 12, 2);
    let lore_v2_before = FoldSubTraceLore::new(3, 8, 2);
    let lore_v2_after = FoldSubTraceLore::new(3, 10, 2);
    let lore_v3_before = FoldSubTraceLore::new(4, 14, 2);
    let lore_v3_after = FoldSubTraceLore::new(4, 16, 2);

    let fold_state = fold(vec![
        vec![vec![lore_v1_before, lore_v1_after], vec![lore_v2_before, lore_v2_after]],
        vec![vec![lore_v3_before, lore_v3_after]],
    ]);

    let expected_trace = vec![
        /*  0 */ par(14, 2), // fold is located inside left Par subtree
        /*  1 */ par(2, 1),
        /*  4 */ request_sent_by(""),
        /*  2 */ test_value(2), // v2
        /*  3 */ test_value(3), // v3
        /*  5 */ fold_state,
        // v1 before trace
        /*  6 */ test_value(4),
        /*  7 */ test_value(5),
        // v2 before trace
        /*  8 */ test_value(6),
        /*  9 */ test_value(7),
        // v2 after trace
        /* 10 */ test_value(8),
        /* 11 */ test_value(9),
        // v1 after trace
        /* 12 */ test_value(10),
        /* 13 */ test_value(11),
        // v3 before trace
        /* 14 */ test_value(12),
        /* 15 */ test_value(13),
        // v3 after trace
        /* 16 */ test_value(14),
        /* 17 */ test_value(15),
        /* 18 */ test_value(16),
        /* 19 */ test_value(18),
        /* 20 */ test_value(19),
        /* 21 */ test_value(20),
        /* 22 */ test_value(21),
        /* 23 */ test_value(22),
    ];

    assert_eq!(actual_trace, expected_trace);
}
