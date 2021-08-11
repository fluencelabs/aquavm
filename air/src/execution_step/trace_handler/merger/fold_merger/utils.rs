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

pub(super) fn resolve_fold_lore(fold: &FoldResult) -> MergeResult<ResolvedFold> {
    let mut lore = HashMap::with_capacity(fold.0.len());
    let mut fold_states_count = 0usize;

    for subtrace_lore in fold.0.iter() {
        check_subtrace_lore(subtrace_lore)?;

        let resolved_descs = ResolvedSubTraceDescs {
            before_subtrace: subtrace_lore.subtraces_desc[0],
            after_subtrace: subtrace_lore.subtraces_desc[1],
        };

        fold_states_count += resolved_descs.len();

        if lore.insert(subtrace_lore.value_pos as usize, resolved_descs).is_some() {
            return Err(MergeError::ManyRecordsWithSamePos(
                fold.clone(),
                subtrace_lore.value_pos as usize,
            ));
        }
    }

    let resolved_fold_lore = ResolvedFold::new(lore, fold_states_count);
    Ok(resolved_fold_lore)
}

fn check_subtrace_lore(subtrace_lore: &FoldSubTraceLore) -> MergeResult<()> {
    // this limitation is due to current constraint on count of next inside one fold,
    // for more info please see comments in the interpreter-data crate
    const SUBTRACE_DESC_COUNT: usize = 2;

    if subtrace_lore.subtraces_desc.len() != SUBTRACE_DESC_COUNT {
        return Err(MergeError::FoldIncorrectSubtracesCount(
            subtrace_lore.subtraces_desc.len(),
        ));
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
    pub(crate) fn len(&self) -> usize {
        self.before_subtrace.subtrace_len as usize + self.after_subtrace.subtrace_len as usize
    }
}
