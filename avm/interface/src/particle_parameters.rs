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

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

/// Represents parameters obtained from a particle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticleParameters<'ctx> {
    pub init_peer_id: Cow<'ctx, str>,
    pub particle_id: Cow<'ctx, str>,
    pub timestamp: u64,
    pub ttl: u32,
    pub current_peer_id: Cow<'ctx, str>,
}

impl<'ctx> ParticleParameters<'ctx> {
    pub fn new(
        init_peer_id: Cow<'ctx, str>,
        particle_id: Cow<'ctx, str>,
        timestamp: u64,
        ttl: u32,
        current_peer_id: Cow<'ctx, str>,
    ) -> Self {
        Self {
            init_peer_id,
            particle_id,
            timestamp,
            ttl,
            current_peer_id,
        }
    }
}
