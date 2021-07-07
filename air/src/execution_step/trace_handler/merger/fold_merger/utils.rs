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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedFold {
    pub(crate) lore: Vec<ResolvedFoldSubTraceLore>,
    pub(crate) fold_states_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedFoldSubTraceLore {
    pub(crate) value: Rc<JValue>,
    pub(crate) before_subtrace: SubTraceDesc,
    pub(crate) after_subtrace: SubTraceDesc,
}

pub(super) fn resolve_fold_lore(slider: &mut TraceSlider, fold: &FoldResult) -> MergeResult<ResolvedFold> {
    let mut resolved_fold_lore = Vec::with_capacity(fold.0.len());
    let mut fold_states_count = 0;

    for subtrace_lore in fold.0.iter() {
        check_subtrace_lore(subtrace_lore)?;

        let value = call_value_by_pos(slider, subtrace_lore.value_pos)?;
        let fold_value = ResolvedFoldSubTraceLore {
            value,
            before_subtrace: subtrace_lore.subtraces_desc[0],
            after_subtrace: subtrace_lore.subtraces_desc[1],
        };

        fold_states_count += fold_value.len();
        resolved_fold_lore.push(fold_value);
    }

    let resolved_fold_lore = ResolvedFold::new(resolved_fold_lore, fold_states_count);
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

fn call_value_by_pos(slider: &mut TraceSlider, pos: u32) -> MergeResult<Rc<JValue>> {
    let state = slider.state_by_pos(pos)?;
    match state {
        ExecutedState::Call(CallResult::Executed(value, _)) => Ok(value.clone()),
        _ => Err(MergeError::FoldPointsToNonCallResult(state.clone())),
    }
}

impl ResolvedFold {
    pub(crate) fn new(lore: Vec<ResolvedFoldSubTraceLore>, fold_states_count: usize) -> Self {
        Self {
            lore,
            fold_states_count,
        }
    }
}

impl ResolvedFoldSubTraceLore {
    pub(crate) fn len(&self) -> usize {
        self.before_subtrace.subtrace_len as usize + self.after_subtrace.subtrace_len as usize
    }
}
