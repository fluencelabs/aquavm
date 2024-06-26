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

use marine_call_parameters::SecurityTetraplet;

use serde::Deserialize;
use serde::Serialize;

/// ResolvedTriplet represents peer network location with all
/// variables, literals and etc resolved into final string.
/// This structure contains a subset of values that
/// SecurityTetraplet consists of.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ResolvedTriplet {
    pub peer_pk: String,
    pub service_id: String,
    pub function_name: String,
}

impl From<ResolvedTriplet> for SecurityTetraplet {
    fn from(triplet: ResolvedTriplet) -> Self {
        Self {
            peer_pk: triplet.peer_pk,
            service_id: triplet.service_id,
            function_name: triplet.function_name,
            lens: String::new(),
        }
    }
}
