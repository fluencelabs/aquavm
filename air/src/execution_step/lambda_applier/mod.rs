/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

mod applier;
mod errors;
mod utils;

pub use errors::LambdaError;

pub(crate) type LambdaResult<T> = std::result::Result<T, LambdaError>;

pub(crate) use applier::select_by_lambda_from_canon_map;
pub(crate) use applier::select_by_lambda_from_scalar;
pub(crate) use applier::select_by_lambda_from_stream;
pub(crate) use applier::MapLensResult;

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
