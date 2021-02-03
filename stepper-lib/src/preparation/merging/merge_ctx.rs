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

use super::TraceSlider;
use crate::preparation::CallResult;
use crate::preparation::ExecutionTrace;
use crate::preparation::ValueType;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub(super) type JValue = serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MergeCtx {
    pub(super) slider: TraceSlider,
    pub(super) streams: HashMap<String, Rc<RefCell<Vec<Rc<JValue>>>>>,
}

impl MergeCtx {
    pub(super) fn new(trace: ExecutionTrace) -> Self {
        let slider = TraceSlider::new(trace);

        Self {
            slider,
            streams: HashMap::new(),
        }
    }

    pub(super) fn maybe_update_stream(&mut self, call_result: &CallResult) {
        if let CallResult::Executed(value, ValueType::Stream(stream_name)) = call_result {
            match self.streams.get_mut(stream_name) {
                None => {
                    self.streams
                        .insert(stream_name.clone(), Rc::new(RefCell::new(vec![value.clone()])));
                }
                Some(values) => values.borrow_mut().push(value.clone()),
            }
        }
    }
}
