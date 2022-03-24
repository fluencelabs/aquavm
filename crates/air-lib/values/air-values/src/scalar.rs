/*
 * Copyright 2022 Fluence Labs Limited
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

use super::fold_iterable_state::FoldIterableState;
use boxed_value::AIRValueAlgebra;
use boxed_value::ValueAggregate;
use boxed_value::ValueLambdaError;

use std::fmt::Display;
use std::fmt::Formatter;

pub enum ScalarRef<'i> {
    Value(&'i ValueAggregate),
    IterableValue(&'i FoldIterableState<'i>),
}

const ITERABLE_PEEK_EXPECTATION: &str = "peek always return elements inside fold,\
                                         this guaranteed by implementation of next and avoiding empty folds";

impl<'i> ScalarRef<'i> {
    pub fn as_air_value(&self) -> Box<dyn AIRValueAlgebra<Error = ValueLambdaError> + 'i> {
        match self {
            &ScalarRef::Value(value) => Box::new(value),
            ScalarRef::IterableValue(fold_state) => {
                let peeked_value = fold_state.iterable.peek().expect(ITERABLE_PEEK_EXPECTATION);
                Box::new(peeked_value)
            }
        }
    }

    pub fn as_value_aggregate(&self) -> ValueAggregate {
        match self {
            &ScalarRef::Value(value) => value.clone(),
            ScalarRef::IterableValue(fold_state) => {
                let peeked_value = fold_state.iterable.peek().expect(ITERABLE_PEEK_EXPECTATION);
                peeked_value.into_value_aggregate()
            }
        }
    }
}

impl<'i> Display for ScalarRef<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScalarRef::Value(value) => write!(f, "{:?}", value)?,
            ScalarRef::IterableValue(cursor) => {
                let _iterable = &cursor.iterable;
                //write!(f, "cursor, current value: {:?}", iterable.peek())?;
                write!(f, "cursor")?;
            }
        }

        Ok(())
    }
}
