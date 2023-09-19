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

mod applier;
mod errors;
mod utils;

pub use errors::LambdaError;

pub(crate) type LambdaResult<T> = std::result::Result<T, LambdaError>;

pub(crate) use applier::select_by_lambda_from_canon_map;
pub(crate) use applier::select_by_lambda_from_scalar;
pub(crate) use applier::select_by_lambda_from_stream;

#[macro_export]
macro_rules! lambda_to_execution_error {
    ($lambda_expr: expr) => {
        $lambda_expr.map_err(|lambda_error| {
            $crate::execution_step::ExecutionError::Catchable(std::rc::Rc::new(
                $crate::execution_step::CatchableError::LambdaApplierError(lambda_error),
            ))
        })
    };
}
