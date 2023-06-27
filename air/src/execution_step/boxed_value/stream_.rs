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

use super::ExecutionResult;
use super::ValueAggregate;
use crate::ExecutionError;
use crate::UncatchableError;

use air_interpreter_data::GenerationIdx;
use air_trace_handler::TraceHandler;



use std::fmt;



#[cfg(test)]
mod test {
    use super::Generation;
    use super::Stream;
    use super::ValueAggregate;
    use super::ValueSource;
    use crate::execution_step::ServiceResultAggregate;

    use air_interpreter_cid::CID;
    use serde_json::json;

    use std::rc::Rc;

    #[test]
    fn test_slice_iter() {
        let value_1 = ValueAggregate::from_service_result(
            ServiceResultAggregate::new(Rc::new(json!("value")), <_>::default(), 1.into()),
            CID::new("some fake cid").into(),
        );
        let value_2 = ValueAggregate::from_service_result(
            ServiceResultAggregate::new(Rc::new(json!("value")), <_>::default(), 1.into()),
            CID::new("some fake cid").into(),
        );
        let mut stream = Stream::from_generations_count(2.into(), 0.into());

        stream
            .add_value(value_1, Generation::previous(0), ValueSource::PreviousData)
            .unwrap();
        stream
            .add_value(value_2, Generation::previous(1), ValueSource::PreviousData)
            .unwrap();

        let slice = stream
            .slice_iter(Generation::previous(0), Generation::previous(1))
            .unwrap();
        assert_eq!(slice.len, 2);

        let slice = stream.slice_iter(Generation::previous(0), Generation::Last).unwrap();
        assert_eq!(slice.len, 2);

        let slice = stream
            .slice_iter(Generation::previous(0), Generation::previous(0))
            .unwrap();
        assert_eq!(slice.len, 1);

        let slice = stream.slice_iter(Generation::Last, Generation::Last).unwrap();
        assert_eq!(slice.len, 1);
    }

    #[test]
    fn test_slice_on_empty_stream() {
        let stream = Stream::from_generations_count(2.into(), 0.into());

        let slice = stream.slice_iter(Generation::previous(0), Generation::previous(1));
        assert!(slice.is_none());

        let slice = stream.slice_iter(Generation::previous(0), Generation::Last);
        assert!(slice.is_none());

        let slice = stream.slice_iter(Generation::previous(0), Generation::previous(0));
        assert!(slice.is_none());

        let slice = stream.slice_iter(Generation::Last, Generation::Last);
        assert!(slice.is_none());
    }

    #[test]
    fn generation_from_current_data() {
        let value_1 = ValueAggregate::from_service_result(
            ServiceResultAggregate::new(Rc::new(json!("value_1")), <_>::default(), 1.into()),
            CID::new("some fake cid").into(),
        );
        let value_2 = ValueAggregate::from_service_result(
            ServiceResultAggregate::new(Rc::new(json!("value_2")), <_>::default(), 2.into()),
            CID::new("some fake cid").into(),
        );
        let mut stream = Stream::from_generations_count(5.into(), 5.into());

        stream
            .add_value(value_1.clone(), Generation::previous(2), ValueSource::CurrentData)
            .unwrap();
        stream
            .add_value(value_2.clone(), Generation::previous(4), ValueSource::PreviousData)
            .unwrap();

        let generations_count = stream.generations_count();
        assert_eq!(generations_count, 10);

        let mut iter = stream.iter(Generation::Last).unwrap();
        let stream_value_1 = iter.next().unwrap();
        let stream_value_2 = iter.next().unwrap();

        assert_eq!(stream_value_1, &value_2);
        assert_eq!(stream_value_2, &value_1);
    }
}
