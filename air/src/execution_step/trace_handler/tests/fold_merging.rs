/*
 * Copyright 2021 Fluence Labs Limited
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

fn test_state(id: usize) -> ExecutedState {
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
        subtrace_len: usize::MAX,
    };
    let fold = fold(vec![vec![vec![lore.clone(), lore.clone()]]]);
    let trace = vec![test_state(1), fold];

    let actual = merge_execution_traces(trace.clone().into(), trace.into());
    let expected: MergeResult<ExecutionTrace> = Err(DataMergingError::FoldLenOverflow(FoldResult(vec![vec![vec![
        lore.clone(),
        lore,
    ]]])));
    assert_eq!(actual, expected);
}

#[test]
fn merge_folds_with_only_before_part() {
    let lore_v1_before = FoldSubTraceLore::new(1, 5, 2);
    let lore_v1_after = FoldSubTraceLore::new(1, 9, 0);
    let lore_v2_before = FoldSubTraceLore::new(2, 7, 2);
    let lore_v2_after = FoldSubTraceLore::new(2, 11, 0);

    let fold_state = fold(vec![vec![
        vec![lore_v1_before, lore_v1_after],
        vec![lore_v2_before, lore_v2_after],
    ]]);

    let prev_trace = vec![
        /*  0 */ par(2, 1),
        /*  1 */ test_state(1), // v1
        /*  2 */ test_state(2), // v2
        /*  3 */ request_sent_by(""),
        /*  4 */ fold_state,
        // v1 before next
        /*  5 */ test_state(4),
        /*  6 */ test_state(5),
        // v2 before next
        /*  7 */ test_state(6),
        /*  8 */ test_state(7),
    ];

    let lore_v2_before = FoldSubTraceLore::new(2, 5, 2);
    let lore_v2_after = FoldSubTraceLore::new(2, 9, 0);
    let lore_v3_before = FoldSubTraceLore::new(3, 7, 2);
    let lore_v3_after = FoldSubTraceLore::new(3, 11, 0);

    let fold_state = fold(vec![vec![
        vec![lore_v2_before, lore_v2_after],
        vec![lore_v3_before, lore_v3_after],
    ]]);

    let current_trace = vec![
        /*  0 */ par(2, 1),
        /*  1 */ request_sent_by(""),
        /*  2 */ test_state(2), // v2
        /*  3 */ test_state(3), // v3
        /*  4 */ fold_state,
        // v2 before next
        /*  5 */ test_state(6),
        /*  6 */ test_state(7),
        // v3 before next
        /*  7 */ test_state(8),
        /*  8 */ test_state(9),
    ];

    let actual_trace =
        merge_execution_traces(prev_trace.into(), current_trace.into()).expect("merging should be successful");

    let lore_v1_before = FoldSubTraceLore::new(1, 5, 2);
    let lore_v1_after = FoldSubTraceLore::new(1, 9, 0);
    let lore_v2_before = FoldSubTraceLore::new(2, 7, 2);
    let lore_v2_after = FoldSubTraceLore::new(2, 9, 0);
    let lore_v3_before = FoldSubTraceLore::new(3, 9, 2);
    let lore_v3_after = FoldSubTraceLore::new(3, 11, 0);

    let fold_state = fold(vec![
        vec![vec![lore_v1_before, lore_v1_after], vec![lore_v2_before, lore_v2_after]],
        vec![vec![lore_v3_before, lore_v3_after]],
    ]);

    let expected_trace = vec![
        /*  0 */ par(2, 1),
        /*  1 */ test_state(1), // v1
        /*  2 */ test_state(2), // v2
        /*  3 */ test_state(3), // v3
        /*  4 */ fold_state,
        // v1 before next
        /*  5 */ test_state(4),
        /*  6 */ test_state(5),
        // v2 before next
        /*  7 */ test_state(6),
        /*  8 */ test_state(7),
        // v3 before next
        /*  9 */ test_state(8),
        /* 10 */ test_state(9),
    ];

    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn merge_folds_with_only_after_part() {
    let lore_v1_before = FoldSubTraceLore::new(1, 7, 0);
    let lore_v1_after = FoldSubTraceLore::new(1, 7, 2);
    let lore_v2_before = FoldSubTraceLore::new(2, 9, 0);
    let lore_v2_after = FoldSubTraceLore::new(2, 9, 2);

    let fold_state = fold(vec![vec![
        vec![lore_v1_before, lore_v1_after],
        vec![lore_v2_before, lore_v2_after],
    ]]);

    let prev_trace = vec![
        /*  0 */ par(2, 1),
        /*  1 */ test_state(1), // v1
        /*  2 */ test_state(2), // v2
        /*  3 */ request_sent_by(""),
        /*  4 */ fold_state,
        // v2 after next
        /*  5 */ test_state(6),
        /*  6 */ test_state(7),
        // v1 after next
        /*  7 */ test_state(4),
        /*  8 */ test_state(5),
    ];

    let lore_v2_before = FoldSubTraceLore::new(2, 7, 0);
    let lore_v2_after = FoldSubTraceLore::new(2, 7, 2);
    let lore_v3_before = FoldSubTraceLore::new(3, 5, 0);
    let lore_v3_after = FoldSubTraceLore::new(3, 5, 2);

    let fold_state = fold(vec![vec![
        vec![lore_v2_before, lore_v2_after],
        vec![lore_v3_before, lore_v3_after],
    ]]);

    let current_trace = vec![
        /*  0 */ par(2, 1),
        /*  1 */ request_sent_by(""),
        /*  2 */ test_state(2), // v2
        /*  3 */ test_state(3), // v3
        /*  4 */ fold_state,
        // v3 before next
        /*  5 */ test_state(8),
        /*  6 */ test_state(9),
        // v2 before next
        /*  7 */ test_state(6),
        /*  8 */ test_state(7),
    ];

    let actual_trace =
        merge_execution_traces(prev_trace.into(), current_trace.into()).expect("merging should be successful");

    let lore_v1_before = FoldSubTraceLore::new(1, 5, 0);
    let lore_v1_after = FoldSubTraceLore::new(1, 7, 2);
    let lore_v2_before = FoldSubTraceLore::new(2, 5, 0);
    let lore_v2_after = FoldSubTraceLore::new(2, 5, 2);
    let lore_v3_before = FoldSubTraceLore::new(3, 9, 0);
    let lore_v3_after = FoldSubTraceLore::new(3, 9, 2);

    let fold_state = fold(vec![
        vec![vec![lore_v1_before, lore_v1_after], vec![lore_v2_before, lore_v2_after]],
        vec![vec![lore_v3_before, lore_v3_after]],
    ]);

    let expected_trace = vec![
        /*  0 */ par(2, 1),
        /*  1 */ test_state(1), // v1
        /*  2 */ test_state(2), // v2
        /*  3 */ test_state(3), // v3
        /*  4 */ fold_state,
        // v2 before next
        /*  5 */ test_state(6),
        /*  6 */ test_state(7),
        // v1 before next
        /*  7 */ test_state(4),
        /*  8 */ test_state(5),
        // v3 before next
        /*  9 */ test_state(8),
        /* 10 */ test_state(9),
    ];

    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn merge_folds_with_different_two_values() {
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
        /*  1 */ test_state(1), // v1
        /*  2 */ test_state(2), // v2
        /*  3 */ request_sent_by(""),
        /*  4 */ fold_state,
        // v1 before next
        /*  5 */ test_state(4),
        /*  6 */ test_state(5),
        // v2 before next
        /*  7 */ test_state(6),
        /*  8 */ test_state(7),
        // v2 after next
        /*  9 */ test_state(8),
        /* 10 */ test_state(9),
        // v1 after next
        /* 11 */ test_state(10),
        /* 12 */ test_state(11),
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
        /*  2 */ test_state(2), // v2
        /*  3 */ test_state(3), // v3
        /*  4 */ fold_state,
        // v2 before next
        /*  5 */ test_state(6),
        /*  6 */ test_state(7),
        // v3 before next
        /*  7 */ test_state(12),
        /*  8 */ test_state(13),
        // v3 after next
        /*  9 */ test_state(14),
        /* 10 */ test_state(15),
        // v2 after next
        /* 11 */ test_state(8),
        /* 12 */ test_state(9),
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
        /*  1 */ test_state(1), // v1
        /*  2 */ test_state(2), // v2
        /*  3 */ test_state(3), // v3
        /*  4 */ fold_state,
        // v1 before next
        /*  5 */ test_state(4),
        /*  6 */ test_state(5),
        // v2 before next
        /*  7 */ test_state(6),
        /*  8 */ test_state(7),
        // v2 after next
        /*  9 */ test_state(8),
        /* 10 */ test_state(9),
        // v1 after next
        /* 11 */ test_state(10),
        /* 12 */ test_state(11),
        // v3 before next
        /* 13 */ test_state(12),
        /* 14 */ test_state(13),
        // v3 after next
        /* 15 */ test_state(14),
        /* 16 */ test_state(15),
    ];

    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn merge_folds_with_different_four_values() {
    let lore_v1_before = FoldSubTraceLore::new(1, 8, 2);
    let lore_v1_after = FoldSubTraceLore::new(1, 14, 2);
    let lore_v2_before = FoldSubTraceLore::new(2, 10, 2);
    let lore_v2_after = FoldSubTraceLore::new(2, 12, 2);
    let lore_v3_before = FoldSubTraceLore::new(3, 16, 2);
    let lore_v3_after = FoldSubTraceLore::new(3, 22, 2);
    let lore_v4_before = FoldSubTraceLore::new(4, 18, 2);
    let lore_v4_after = FoldSubTraceLore::new(4, 20, 2);

    let fold_state = fold(vec![
        vec![vec![lore_v1_before, lore_v1_after], vec![lore_v2_before, lore_v2_after]],
        vec![vec![lore_v3_before, lore_v3_after], vec![lore_v4_before, lore_v4_after]],
    ]);

    let prev_trace = vec![
        /*  0 */ par(4, 2),
        /*  1 */ test_state(1), // v1
        /*  2 */ test_state(2), // v2
        /*  3 */ test_state(3), // v3
        /*  4 */ test_state(4), // v4
        /*  5 */ request_sent_by(""),
        /*  6 */ request_sent_by(""),
        /*  7 */ fold_state,
        // v1 before next
        /*  8 */ test_state(4),
        /*  9 */ test_state(5),
        // v2 before next
        /* 10 */ test_state(6),
        /* 11 */ test_state(7),
        // v2 after next
        /* 12 */ test_state(8),
        /* 13 */ test_state(9),
        // v1 after next
        /* 14 */ test_state(10),
        /* 15 */ test_state(11),
        // v3 before next
        /* 16 */ test_state(12),
        /* 17 */ test_state(13),
        // v4 before next
        /* 18 */ test_state(14),
        /* 19 */ test_state(15),
        // v4 after next
        /* 20 */ test_state(16),
        /* 21 */ test_state(17),
        // v3 after next
        /* 22 */ test_state(18),
        /* 23 */ test_state(19),
    ];

    let lore_v2_before = FoldSubTraceLore::new(2, 8, 2);
    let lore_v2_after = FoldSubTraceLore::new(2, 14, 2);
    let lore_v4_before = FoldSubTraceLore::new(4, 10, 2);
    let lore_v4_after = FoldSubTraceLore::new(4, 12, 2);
    let lore_v5_before = FoldSubTraceLore::new(5, 16, 2);
    let lore_v5_after = FoldSubTraceLore::new(5, 18, 2);
    let lore_v6_before = FoldSubTraceLore::new(6, 20, 2);
    let lore_v6_after = FoldSubTraceLore::new(6, 22, 2);

    let fold_state = fold(vec![
        vec![vec![lore_v2_before, lore_v2_after], vec![lore_v4_before, lore_v4_after]],
        vec![vec![lore_v5_before, lore_v5_after]],
        vec![vec![lore_v6_before, lore_v6_after]],
    ]);

    let current_trace = vec![
        /*  0 */ par(4, 2),
        /*  1 */ request_sent_by(""),
        /*  2 */ test_state(2), // v2
        /*  3 */ request_sent_by(""),
        /*  4 */ test_state(4), // v4
        /*  5 */ test_state(5), // v5
        /*  6 */ test_state(6), // v6
        /*  7 */ fold_state,
        // v2 before next
        /*  8 */ test_state(6),
        /*  9 */ test_state(7),
        // v4 before next
        /* 10 */ test_state(14),
        /* 11 */ test_state(15),
        // v4 after next
        /* 12 */ test_state(16),
        /* 13 */ test_state(17),
        // v2 after next
        /* 14 */ test_state(8),
        /* 15 */ test_state(9),
        // v5 before next
        /* 16 */ test_state(18),
        /* 17 */ test_state(19),
        // v5 after next
        /* 18 */ test_state(20),
        /* 19 */ test_state(21),
        // v6 before next
        /* 20 */ test_state(22),
        /* 21 */ test_state(23),
        // v6 after next
        /* 22 */ test_state(24),
        /* 23 */ test_state(25),
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
        /*  1 */ test_state(1), // v1
        /*  2 */ test_state(2), // v2
        /*  3 */ test_state(3), // v3
        /*  4 */ fold_state,
        // v1 before next
        /*  5 */ test_state(4),
        /*  6 */ test_state(5),
        // v2 before next
        /*  7 */ test_state(6),
        /*  8 */ test_state(7),
        // v2 after next
        /*  9 */ test_state(8),
        /* 10 */ test_state(9),
        // v1 after next
        /* 11 */ test_state(10),
        /* 12 */ test_state(11),
        // v3 before next
        /* 13 */ test_state(12),
        /* 14 */ test_state(13),
        // v3 after next
        /* 15 */ test_state(14),
        /* 16 */ test_state(15),
    ];

    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn merge_folds_inside_par() {
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
        /*  2 */ test_state(1), // v1
        /*  3 */ test_state(2), // v2
        /*  4 */ request_sent_by(""),
        /*  5 */ fold_state,
        // v1 before next
        /*  6 */ test_state(4),
        /*  7 */ test_state(5),
        // v2 before next
        /*  8 */ test_state(6),
        /*  9 */ test_state(7),
        // v2 after next
        /* 10 */ test_state(8),
        /* 11 */ test_state(9),
        // v1 after next
        /* 12 */ test_state(10),
        /* 13 */ test_state(11),
        // other states from left par
        /* 14 */ test_state(16),
        /* 15 */ request_sent_by(""),
        /* 16 */ test_state(18),
        /* 17 */ request_sent_by(""),
        // right par subtree
        /* 18 */ test_state(20),
        /* 19 */ test_state(21),
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
        /*  2 */ test_state(2), // v2
        /*  3 */ test_state(3), // v3
        /*  5 */ fold_state,
        // v2 before next
        /*  6 */ test_state(6),
        /*  7 */ test_state(7),
        // v3 before next
        /*  8 */ test_state(12),
        /*  9 */ test_state(13),
        // v3 after next
        /* 10 */ test_state(14),
        /* 11 */ test_state(15),
        // v2 after next
        /* 12 */ test_state(8),
        /* 13 */ test_state(9),
        // other states from left par
        /* 14 */ test_state(16),
        /* 15 */ test_state(17),
        /* 16 */ request_sent_by(""),
        /* 17 */ test_state(19),
        // right par subtree
        /* 18 */ request_sent_by(""),
        /* 19 */ test_state(21),
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
        /*  0 */ par(18, 2), // fold is located inside left Par subtree
        /*  1 */ par(2, 1),
        /*  4 */ test_state(1), // v1
        /*  2 */ test_state(2), // v2
        /*  3 */ test_state(3), // v3
        /*  5 */ fold_state,
        // v1 before next
        /*  6 */ test_state(4),
        /*  7 */ test_state(5),
        // v2 before next
        /*  8 */ test_state(6),
        /*  9 */ test_state(7),
        // v2 after next
        /* 10 */ test_state(8),
        /* 11 */ test_state(9),
        // v1 after next
        /* 12 */ test_state(10),
        /* 13 */ test_state(11),
        // v3 before next
        /* 14 */ test_state(12),
        /* 15 */ test_state(13),
        // v3 after next
        /* 16 */ test_state(14),
        /* 17 */ test_state(15),
        // other states from left Par subtree
        /* 18 */ test_state(16),
        /* 19 */ test_state(17),
        /* 20 */ test_state(18),
        /* 21 */ test_state(19),
        // right Par subtree
        /* 22 */ test_state(20),
        /* 23 */ test_state(21),
    ];

    assert_eq!(actual_trace, expected_trace);
}
