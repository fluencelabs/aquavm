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
use super::RecursiveCursor;
use crate::execution_step::boxed_value::Iterable;
use crate::execution_step::boxed_value::IterableItem;
use crate::execution_step::boxed_value::IterableVecResolvedCall;
use crate::execution_step::ValueAggregate;

pub(crate) type IterableValue = Box<dyn for<'ctx> Iterable<'ctx, Item = IterableItem<'ctx>>>;

pub(crate) struct RecursiveStream {
    cursor: RecursiveCursor,
}

impl RecursiveStream {
    pub fn new() -> Self {
        Self {
            cursor: RecursiveCursor::empty(),
        }
    }

    pub fn fold_started(&mut self, stream: &mut Stream<ValueAggregate>) -> Vec<IterableValue> {
        let slice_iter = stream.slice_iter(self.cursor);
        let iterable = Self::slice_iter_to_iterable(slice_iter);
        self.cursor = stream.cursor();
        if !iterable.is_empty() {
            // add a new generation to made all consequence "new" (meaning that they are just executed on this peer)
            // write operation to this stream to write to this new generation
            //println!("  recursive stream: add new generation");
            stream.new_values().add_new_empty_generation();
        }

        iterable
    }

    pub fn next_iteration(&mut self, stream: &mut Stream<ValueAggregate>) -> Vec<IterableValue> {
        //println!("  recursive stream: next iteration before {:?}", self.cursor);
        let slice_iter = stream.slice_iter(self.cursor);
        let next_iteration_values = Self::slice_iter_to_iterable(slice_iter);
        if stream.new_values().last_generation_is_empty() {
            //println!("  recursive stream: remove the last generation");
            stream.new_values().remove_last_generation();
        }

        self.cursor = stream.cursor();
        //println!("  recursive stream: next iteration after {:?}", self.cursor);

        if !stream.new_values().last_generation_is_empty() {
            stream.new_values().add_new_empty_generation();
        }
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
        let mut recursive_stream = RecursiveStream::new();
        let iterable_values = recursive_stream.fold_started(&mut stream);

        assert!(iterable_values.is_empty())
    }

    #[test]
    fn next_iteration_empty_if_no_values() {
        let mut stream = Stream::new();
        let mut recursive_stream =  RecursiveStream::new();
        let iterable_values = recursive_stream.next_iteration(&mut stream);

        assert!(iterable_values.is_empty())
    }

    #[test]
    fn next_iteration_empty_if_no_values_added() {
        let mut stream = Stream::new();
        let mut recursive_stream = RecursiveStream::new();

        let value = create_value(json!("1"));
        stream.add_value(value, Generation::Current(0.into()));

        let iterable_values = recursive_stream.fold_started(&mut stream);
        assert_eq!(iterable_values.len(), 1);

        let iterable_values = recursive_stream.next_iteration(&mut stream);
        assert!(iterable_values.is_empty());
    }

    #[test]
    fn one_recursive_iteration() {
        let mut stream = Stream::new();
        let mut recursive_stream = RecursiveStream::new();

        let value = create_value(json!("1"));
        stream.add_value(value.clone(), Generation::Current(0.into()));

        let iterable_values = recursive_stream.fold_started(&mut stream);
        assert_eq!(iterable_values.len(), 1);

        stream.add_value(value.clone(), Generation::New);
        stream.add_value(value, Generation::New);

        let iterable_values = recursive_stream.next_iteration(&mut stream);
        assert_eq!(iterable_values.len(), 1);

        let iterable_values = recursive_stream.next_iteration(&mut stream);
        assert!(iterable_values.is_empty());
    }
}
