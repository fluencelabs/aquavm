/*
 * Copyright 2023 Fluence Labs Limited
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

use super::Stream;
use crate::execution_step::boxed_value::Iterable;
use crate::execution_step::boxed_value::IterableItem;
use crate::execution_step::boxed_value::IterableVecResolvedCall;
use crate::execution_step::ValueAggregate;

pub(crate) type IterableValue = Box<dyn for<'ctx> Iterable<'ctx, Item = IterableItem<'ctx>>>;

pub(crate) struct RecursiveStream;

impl RecursiveStream {
    pub fn fold_started(stream: &mut Stream<ValueAggregate>) -> Vec<IterableValue> {
        let iterable = Self::slice_iter_to_iterable(stream.slice_iter());
        if !iterable.is_empty() {
            // add a new generation to made all consequence "new" (meaning that they are just executed on this peer)
            // write operation to this stream to write to this new generation
            stream.new_values().add_new_generation();
        }

        iterable
    }

    pub fn next_iteration(stream: &mut Stream<ValueAggregate>) -> Vec<IterableValue> {
        let new_values = stream.new_values();
        let new_values_since_last_visit = Self::slice_iter_to_iterable(new_values.slice_iter());
        if new_values_since_last_visit.is_empty() {
            new_values.remove_last_generation();
        }

        new_values_since_last_visit
    }

    fn slice_iter_to_iterable<'value>(iter: impl Iterator<Item = &'value [ValueAggregate]>) -> Vec<IterableValue> {
        iter.map(|iterable| {
            let foldable = IterableVecResolvedCall::init(iterable.to_vec());
            let foldable: IterableValue = Box::new(foldable);
            foldable
        })
        .collect::<Vec<_>>()
    }
}
