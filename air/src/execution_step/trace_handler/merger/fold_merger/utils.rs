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
use crate::execution_step::trace_handler::data_keeper::MergeCtx;
use air_interpreter_data::FoldSubTraceLore;
use air_interpreter_data::SubTraceDesc;

use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedFold {
    pub(crate) lore: HashMap<usize, ResolvedSubTraceDescs>,
    pub(crate) fold_states_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedSubTraceDescs {
    pub(crate) before_subtrace: SubTraceDesc,
    pub(crate) after_subtrace: SubTraceDesc,
}

pub(super) fn resolve_fold_lore(fold: &FoldResult, merge_ctx: &MergeCtx) -> MergeResult<ResolvedFold> {
    let (fold_states_count, lens) = compute_lens_convolution(fold, merge_ctx)?;

    let lore = fold.lore.iter().zip(lens).try_fold::<_, _, MergeResult<_>>(
        HashMap::with_capacity(fold.lore.len()),
        |mut resolved_lore, (lore, lens)| {
            let before_subtrace = SubTraceDesc::new(lore.subtraces_desc[0].begin_pos as _, lens.before_len as _);
            let after_subtrace = SubTraceDesc::new(lore.subtraces_desc[1].begin_pos as _, lens.after_len as _);
            let resolved_descs = ResolvedSubTraceDescs::new(before_subtrace, after_subtrace);

            match resolved_lore.insert(lore.value_pos as usize, resolved_descs) {
                Some(_) => Err(FoldResultError::SeveralRecordsWithSamePos(
                    fold.clone(),
                    lore.value_pos as usize,
                ))
                .map_err(Into::into),
                None => Ok(resolved_lore),
            }
        },
    )?;

    let resolved_fold_lore = ResolvedFold::new(lore, fold_states_count);
    Ok(resolved_fold_lore)
}

/// This function does conversion subtrace_lens of a fold result, it's better to explain it on
/// examples.
///
/// Imagine a fold on stream with 3 elements that have the same generation, in this case the
/// conversion will look like this:
/// [1, 1] [2, 2] [3, 3] => [6, 1] [5, 3] [3, 6]
///   g0     g0     g0
/// here a number before comma represents count of elements before next, and after the comma - after
///
/// For fold with 5 elements of two generations:
/// [1, 1] [2, 2] [3, 3] [4, 4] [5, 5] [1, 1] => [6, 1] [5, 3] [3, 6] [9, 4] [5, 9] [1, 1]
///   g0     g0     g0     g1     g1     g2
///
/// It could be seen that this function does a convolution of lens with respect to generations.
/// This is needed to handle (fold (par (next ... cases, because of subtrace_len of a Fold state
/// describes only states inside this iteration without states that next brings, however a Par
/// lens describe the whole subtree, where "next" states are included.

// TODO: in future it's possible to change a format of a Fold state to one behaves like Par,
// because this function adds some overhead
fn compute_lens_convolution(fold: &FoldResult, merge_ctx: &MergeCtx) -> MergeResult<(usize, Vec<LoresLen>)> {
    let subtraces_count = fold.lore.len();
    let mut lens = Vec::with_capacity(subtraces_count);
    let mut fold_states_count: usize = 0;
    let mut last_seen_generation = 0;
    let mut last_seen_generation_pos = 0;
    let mut cum_after_len = 0;

    for subtrace_id in 0..subtraces_count {
        let subtrace_lore = &fold.lore[subtrace_id];
        check_subtrace_lore(subtrace_lore)?;

        let current_generation = merge_ctx.try_get_generation(subtrace_lore.value_pos)?;
        // TODO: check sequence for monotone
        if last_seen_generation != current_generation {
            if subtrace_id > 0 {
                // do a back traversal for
                compute_before_lens(&mut lens, last_seen_generation_pos, subtrace_id - 1);
            }
            last_seen_generation = current_generation;
            last_seen_generation_pos = subtrace_id;
            cum_after_len = 0;
        }

        let before_len = subtrace_lore.subtraces_desc[0].subtrace_len;
        let after_len = subtrace_lore.subtraces_desc[1].subtrace_len;
        // this checks for overflow both cum_before_len and cum_after_len
        fold_states_count = fold_states_count
            .checked_add(before_len as usize)
            .and_then(|v| v.checked_add(after_len as usize))
            .ok_or_else(|| FoldResultError::SubtraceLenOverflow {
                fold_result: fold.clone(),
                count: subtrace_id,
            })?;

        cum_after_len += after_len;

        // temporary set not cumulative before len
        let new_lens = LoresLen::new(before_len, cum_after_len);
        lens.push(new_lens);
    }

    if subtraces_count > 0 {
        compute_before_lens(&mut lens, last_seen_generation_pos, subtraces_count - 1);
    }

    Ok((fold_states_count, lens))
}

fn compute_before_lens(lore_lens: &mut [LoresLen], begin_pos: usize, end_pos: usize) {
    let mut cum_before_len = 0;

    for subtrace_id in begin_pos..=end_pos {
        let subtrace_id = end_pos - (subtrace_id - begin_pos);
        let lens = &mut lore_lens[subtrace_id];

        let current_before_len = lens.before_len;
        cum_before_len += current_before_len;
        lens.before_len = cum_before_len;
    }
}

fn check_subtrace_lore(subtrace_lore: &FoldSubTraceLore) -> MergeResult<()> {
    // this limitation is due to current constraint on count of next inside one fold,
    // for more info please see comments in the interpreter-data crate
    const SUBTRACE_DESC_COUNT: usize = 2;

    if subtrace_lore.subtraces_desc.len() != SUBTRACE_DESC_COUNT {
        return Err(FoldResultError::FoldIncorrectSubtracesCount(
            subtrace_lore.subtraces_desc.len(),
        ))
        .map_err(Into::into);
    }

    Ok(())
}

impl ResolvedFold {
    pub(crate) fn new(lore: HashMap<usize, ResolvedSubTraceDescs>, fold_states_count: usize) -> Self {
        Self {
            lore,
            fold_states_count,
        }
    }
}

impl ResolvedSubTraceDescs {
    pub(self) fn new(before_subtrace: SubTraceDesc, after_subtrace: SubTraceDesc) -> Self {
        Self {
            before_subtrace,
            after_subtrace,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct LoresLen {
    pub(self) before_len: u32,
    pub(self) after_len: u32,
}

impl LoresLen {
    pub(self) fn new(before_len: u32, after_len: u32) -> Self {
        Self { before_len, after_len }
    }
}
