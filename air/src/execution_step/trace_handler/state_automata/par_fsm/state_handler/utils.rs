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

pub(super) fn prepare_total_lens(
    prev_size: usize,
    current_size: usize,
    data_keeper: &mut DataKeeper,
) -> FSMResult<(usize, usize)> {
    sizes_suits(prev_size, current_size, data_keeper)?;

    // these lens should be set after end of a par
    let prev_total_len = data_keeper.prev_ctx.total_subtrace_len() - prev_size;
    let current_total_len = data_keeper.current_ctx.total_subtrace_len() - current_size;

    data_keeper.prev_ctx.set_total_subtrace_len(prev_size);
    data_keeper.current_ctx.set_total_subtrace_len(current_size);

    Ok((prev_total_len, current_total_len))
}

pub(super) fn compute_par_total_lens(prev_par: ParResult, current_par: ParResult) -> FSMResult<(usize, usize)> {
    let prev_par_len = prev_par.size().ok_or(StateFSMError::ParLenOverflow(prev_par))?;
    let current_par_len = current_par.size().ok_or(StateFSMError::ParLenOverflow(prev_par))?;

    Ok((prev_par_len, current_par_len))
}

fn sizes_suits(prev_par_len: usize, current_par_len: usize, data_keeper: &DataKeeper) -> FSMResult<()> {
    let prev_total_len = data_keeper.prev_ctx.total_subtrace_len();
    if prev_par_len > prev_total_len {
        return Err(StateFSMError::TotalSubtraceLenIsLess(
            prev_par_len,
            prev_total_len,
            MergeCtxType::Previous,
        ));
    }

    let current_total_len = data_keeper.current_ctx.total_subtrace_len();
    if current_par_len > current_total_len {
        return Err(StateFSMError::TotalSubtraceLenIsLess(
            current_par_len,
            current_total_len,
            MergeCtxType::Current,
        ));
    }

    Ok(())
}
