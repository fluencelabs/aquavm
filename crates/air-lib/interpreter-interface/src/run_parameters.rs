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

#[cfg(feature = "marine")]
use fluence_it_types::ne_vec::NEVec;
#[cfg(feature = "marine")]
use fluence_it_types::IValue;
#[cfg(feature = "marine")]
use marine_rs_sdk::marine;
use serde::Deserialize;
use serde::Serialize;

/// Parameters that a host side should pass to an interpreter and that necessary for execution.
#[cfg_attr(feature = "marine", marine)]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RunParameters {
    /// Peer id of a peer that start this particle.
    pub init_peer_id: String,

    /// Peer id of a current peer.
    pub current_peer_id: String,

    /// Unix timestamp from a particle in milliseconds.
    /// It represents time when this particle was sent from the init peer id.
    pub timestamp: u64,

    /// TTL set by init peer id in milliseconds.
    pub ttl: u32,

    /// A key format.
    ///
    /// This value is the result of `fluence_keypair::KeyType::into`.
    pub key_format: u8,

    /// A secret key material.
    ///
    /// The value is the result `fluence_keypair::KeyPair::secret`, for compatibility
    /// with JS client who can only serialize to secret key, not to keypair.
    pub secret_key_bytes: Vec<u8>,

    /// Unique particle ID.
    pub particle_id: String,

    /// The AIR script size limit.
    pub air_size_limit: u64,

    /// The particle data size limit.
    pub particle_size_limit: u64,

    /// This is the limit for the size of service call result.
    pub call_result_size_limit: u64,

    /// This knob controls hard RAM limits behavior for AVMRunner.
    pub hard_limit_enabled: bool,
}

impl RunParameters {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        init_peer_id: String,
        current_peer_id: String,
        timestamp: u64,
        ttl: u32,
        key_format: u8,
        secret_key_bytes: Vec<u8>,
        particle_id: String,
        air_size_limit: u64,
        particle_size_limit: u64,
        call_result_size_limit: u64,
        hard_limit_enabled: bool,
    ) -> Self {
        Self {
            init_peer_id,
            current_peer_id,
            timestamp,
            ttl,
            key_format,
            secret_key_bytes,
            particle_id,
            air_size_limit,
            particle_size_limit,
            call_result_size_limit,
            hard_limit_enabled,
        }
    }

    #[cfg(feature = "marine")]
    pub fn into_ivalue(self) -> IValue {
        let run_parameters = vec![
            IValue::String(self.init_peer_id),
            IValue::String(self.current_peer_id),
            IValue::U64(self.timestamp),
            IValue::U32(self.ttl),
            IValue::U8(self.key_format),
            IValue::ByteArray(self.secret_key_bytes),
            IValue::String(self.particle_id),
            IValue::U64(self.air_size_limit),
            IValue::U64(self.particle_size_limit),
            IValue::U64(self.call_result_size_limit),
            IValue::Boolean(self.hard_limit_enabled),
        ];
        // unwrap is safe here because run_parameters is non-empty array
        let run_parameters = NEVec::new(run_parameters).unwrap();
        IValue::Record(run_parameters)
    }
}
