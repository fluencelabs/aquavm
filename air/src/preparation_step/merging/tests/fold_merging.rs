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

#[test]
fn too_many_subtraces() {
    let lore = FoldSubTraceLore::default();
    let fold = fold(vec![vec![lore.clone(), lore.clone(), lore.clone()]]);
    let trace = vec![fold];

    let actual = merge_execution_traces(trace.clone().into(), trace.into());
    let expected: MergeResult<ExecutionTrace> = Err(DataMergingError::FoldTooManySubtraces(
        FoldResult(vec![vec![lore.clone(), lore.clone(), lore]]),
        3,
    ));
    assert_eq!(actual, expected);
}

#[test]
fn fold_subtraces_overflows() {
    let lore = FoldSubTraceLore {
        value_pos: 0,
        begin_pos: 0,
        interval_len: usize::MAX,
    };
    let fold = fold(vec![vec![lore.clone(), lore.clone()]]);
    let trace = vec![scalar_jvalue(json!([])), fold];

    let actual = merge_execution_traces(trace.clone().into(), trace.into());
    let expected: MergeResult<ExecutionTrace> = Err(DataMergingError::FoldLenOverflow(FoldResult(vec![vec![
        lore.clone(),
        lore,
    ]])));
    assert_eq!(actual, expected);
}
