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

use crate::execution_step::Generation;
use crate::execution_step::ValueAggregate;

use air_parser::AirPos;

pub(crate) struct StreamValueDescriptor<'stream_name> {
    pub value: ValueAggregate,
    pub name: &'stream_name str,
    pub generation: Generation,
    pub position: AirPos,
}

impl<'stream_name> StreamValueDescriptor<'stream_name> {
    pub fn new(value: ValueAggregate, name: &'stream_name str, generation: Generation, position: AirPos) -> Self {
        Self {
            value,
            name,
            generation,
            position,
        }
    }
}
