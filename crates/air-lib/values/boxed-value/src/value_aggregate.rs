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

use super::BoxedValue;
use super::RcBoxedValue;
use super::RcSecurityTetraplet;
use super::RcSecurityTetraplets;
use super::ValueLambdaError;
use crate::algebra::AIRValueAlgebra;
use crate::algebra::ValueWithTetraplet;

use air_lambda_ast::AIRLambdaIter;

#[derive(Clone)]
pub struct ValueAggregate {
    pub value: RcBoxedValue,
    pub tetraplet: RcSecurityTetraplet,
    pub trace_pos: usize,
}

impl ValueAggregate {
    pub fn new(value: RcBoxedValue, tetraplet: RcSecurityTetraplet, trace_pos: usize) -> Self {
        Self {
            value,
            tetraplet,
            trace_pos,
        }
    }
}

impl AIRValueAlgebra for ValueAggregate {
    type Error = ValueLambdaError;

    fn apply_lambda<'value>(
        &'value self,
        lambda: &AIRLambdaIter<'_>,
    ) -> Result<&'value dyn BoxedValue, Self::Error> {
        self.value.apply_lambda(lambda)
    }

    fn apply_lambda_with_tetraplets<'value>(
        &'value self,
        lambda: &AIRLambdaIter<'_>,
    ) -> Result<ValueWithTetraplet<'value, 'value>, Self::Error> {
        let value = self.value.apply_lambda(lambda)?;
        let result = ValueWithTetraplet {
            value,
            tetraplet: &self.tetraplet,
        };

        Ok(result)
    }

    fn as_value(&self) -> &RcBoxedValue {
        &self.value
    }

    fn as_tetraplets(&self) -> RcSecurityTetraplets {
        vec![self.tetraplet.clone()]
    }
}

impl AIRValueAlgebra for &ValueAggregate {
    type Error = ValueLambdaError;

    fn apply_lambda<'value>(
        &'value self,
        lambda: &AIRLambdaIter<'_>,
    ) -> Result<&'value dyn BoxedValue, Self::Error> {
        self.value.apply_lambda(lambda)
    }

    fn apply_lambda_with_tetraplets<'value>(
        &'value self,
        lambda: &AIRLambdaIter<'_>,
    ) -> Result<ValueWithTetraplet<'value, 'value>, Self::Error> {
        let value = self.value.apply_lambda(lambda)?;
        let result = ValueWithTetraplet {
            value,
            tetraplet: &self.tetraplet,
        };

        Ok(result)
    }

    fn as_value(&self) -> &RcBoxedValue {
        &self.value
    }

    fn as_tetraplets(&self) -> RcSecurityTetraplets {
        vec![self.tetraplet.clone()]
    }
}

use std::fmt;
use std::fmt::Formatter;

impl fmt::Debug for ValueAggregate {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
