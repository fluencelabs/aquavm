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
pub struct ParticleParameters<'init_peer_id, 'particle_id> {
    pub init_peer_id: Cow<'init_peer_id, str>,
    pub particle_id: Cow<'particle_id, str>,
    pub timestamp: u64,
    pub ttl: u32,
    pub override_current_peer_id: Option<String>,
}

impl<'init_peer_id, 'particle_id> ParticleParameters<'init_peer_id, 'particle_id> {
    pub fn new(
        init_peer_id: Cow<'init_peer_id, str>,
        particle_id: Cow<'particle_id, str>,
        timestamp: u64,
        ttl: u32,
    ) -> Self {
        Self {
            init_peer_id,
            particle_id,
            timestamp,
            ttl,
            override_current_peer_id: None,
        }
    }
}
