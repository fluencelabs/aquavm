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
            stream.new_values().add_new_empty_generation();
        }

        iterable
    }

    pub fn next_iteration(stream: &mut Stream<ValueAggregate>) -> Vec<IterableValue> {
        let new_values = stream.new_values();
        if new_values.last_generation().is_empty() {
            new_values.remove_last_generation();
            return vec![];
        }

        let last_generation = stream.new_values().last_generation();
        let next_iteration_values = Self::slice_iter_to_iterable(std::iter::once(last_generation));
        stream.new_values().add_new_empty_generation();

        next_iteration_values
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

#[cfg(test)]
mod test {
    use super::RecursiveStream;
    use super::Stream;
    use super::ValueAggregate;
    use crate::execution_step::Generation;
    use crate::execution_step::ServiceResultAggregate;
    use crate::JValue;

    use air_interpreter_cid::CID;
    use serde_json::json;

    use std::rc::Rc;

    fn create_value(value: JValue) -> ValueAggregate {
        ValueAggregate::from_service_result(
            ServiceResultAggregate::new(Rc::new(value), <_>::default(), 0.into()),
            CID::new("some fake cid").into(),
        )
    }

    #[test]
    fn fold_started_empty_if_no_values() {
        let mut stream = Stream::new();
        let iterable_values = RecursiveStream::fold_started(&mut stream);

        assert!(iterable_values.is_empty())
    }

    #[test]
    fn next_iteration_empty_if_no_values() {
        let mut stream = Stream::new();
        let iterable_values = RecursiveStream::next_iteration(&mut stream);

        assert!(iterable_values.is_empty())
    }

    #[test]
    fn next_iteration_empty_if_no_values_added() {
        let mut stream = Stream::new();

        let value = create_value(json!("1"));
        stream.add_value(value, Generation::Current(0.into()));

        let iterable_values = RecursiveStream::fold_started(&mut stream);
        assert_eq!(iterable_values.len(), 1);

        let iterable_values = RecursiveStream::next_iteration(&mut stream);
        assert!(iterable_values.is_empty());
    }

    #[test]
    fn one_recursive_iteration() {
        let mut stream = Stream::new();

        let value = create_value(json!("1"));
        stream.add_value(value.clone(), Generation::Current(0.into()));

        let iterable_values = RecursiveStream::fold_started(&mut stream);
        assert_eq!(iterable_values.len(), 1);

        stream.add_value(value.clone(), Generation::New);
        stream.add_value(value, Generation::New);

        let iterable_values = RecursiveStream::next_iteration(&mut stream);
        assert_eq!(iterable_values.len(), 1);

        let iterable_values = RecursiveStream::next_iteration(&mut stream);
        assert!(iterable_values.is_empty());
    }
}
