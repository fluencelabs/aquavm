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
