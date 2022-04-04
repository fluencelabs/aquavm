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

use air_parser::ast::Instruction;
use boxed_value::AIRIterableValueAlgebra;
use boxed_value::AIRValueAlgebra;
use boxed_value::RcBoxedValue;
use boxed_value::RcSecurityTetraplet;
use boxed_value::RcSecurityTetraplets;
use boxed_value::ValueAggregate;
use boxed_value::ValueLambdaError;
use boxed_value::ValueWithTetraplet;

use air_lambda_ast::AIRLambda;
use boxed_value::BoxedValue;
use std::rc::Rc;

pub struct FoldIterableState<'i> {
    pub iterable: IterableValue,
    pub iterable_type: IterableType,
    pub instr_head: Rc<Instruction<'i>>,
}

pub type IterableValue =
    Box<dyn for<'ctx> AIRIterableValueAlgebra<'ctx, Item = IterableItem<'ctx>>>;

pub struct IterableItem<'ctx> {
    pub value: &'ctx RcBoxedValue,
    pub tetraplet: &'ctx RcSecurityTetraplet,
    pub position: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IterableType {
    Scalar,
    Stream(u32),
}

impl<'i> FoldIterableState<'i> {
    pub fn from_iterable(
        iterable: IterableValue,
        iterable_type: IterableType,
        instr_head: Rc<Instruction<'i>>,
    ) -> Self {
        Self {
            iterable,
            iterable_type,
            instr_head,
        }
    }
}

impl<'ctx> IterableItem<'ctx> {
    pub fn new(
        value: &'ctx RcBoxedValue,
        tetraplet: &'ctx RcSecurityTetraplet,
        position: usize,
    ) -> Self {
        Self {
            value,
            tetraplet,
            position,
        }
    }

    pub fn into_value_aggregate(self) -> ValueAggregate {
        ValueAggregate::new(self.value.clone(), self.tetraplet.clone(), self.position)
    }
}

impl AIRValueAlgebra for IterableItem<'_> {
    type Error = ValueLambdaError;

    fn apply_lambda<'value>(
        &'value self,
        lambda: &AIRLambda<'_>,
    ) -> Result<&'value dyn BoxedValue, Self::Error> {
        self.value.apply_lambda(lambda)
    }

    fn apply_lambda_with_tetraplets<'value>(
        &'value self,
        lambda: &AIRLambda<'_>,
    ) -> Result<ValueWithTetraplet<'value, 'value>, Self::Error> {
        let value = self.value.apply_lambda(lambda)?;
        let result = ValueWithTetraplet {
            value,
            tetraplet: &self.tetraplet,
        };
        Ok(result)
    }

    fn as_value(&self) -> &RcBoxedValue {
        self.value
    }

    fn as_tetraplets(&self) -> RcSecurityTetraplets {
        vec![self.tetraplet.clone()]
    }
}
