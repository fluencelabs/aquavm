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

use crate::execution_step::execution_context::StreamMapValueDescriptor;
use crate::execution_step::execution_context::StreamValueDescriptor;
use crate::execution_step::Generation;
use crate::execution_step::ValueAggregate;

use air_parser::ast;
use air_trace_handler::merger::MergerApResult;

pub(super) fn generate_value_descriptor<'stream>(
    value: ValueAggregate,
    stream: &'stream ast::Stream<'_>,
    ap_result: &MergerApResult,
) -> StreamValueDescriptor<'stream> {
    match ap_result {
        MergerApResult::NotMet => StreamValueDescriptor::new(value, stream.name, Generation::New, stream.position),
        MergerApResult::Met(met_result) => StreamValueDescriptor::new(
            value,
            stream.name,
            Generation::from_met_result(met_result),
            stream.position,
        ),
    }
}

pub(crate) fn generate_map_value_descriptor<'stream>(
    value: ValueAggregate,
    stream: &'stream ast::StreamMap<'_>,
    ap_result: &MergerApResult,
) -> StreamMapValueDescriptor<'stream> {
    match ap_result {
        MergerApResult::NotMet => StreamMapValueDescriptor::new(value, stream.name, Generation::New, stream.position),
        MergerApResult::Met(met_result) => StreamMapValueDescriptor::new(
            value,
            stream.name,
            Generation::from_met_result(met_result),
            stream.position,
        ),
    }
}
