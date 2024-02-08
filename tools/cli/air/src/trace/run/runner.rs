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

use avm_interface::raw_outcome::RawAVMOutcome;
use avm_interface::CallResults;
use fluence_keypair::KeyPair;
use futures::future::LocalBoxFuture;

use std::error::Error as StdError;

pub(crate) trait AirRunner {
    #[allow(clippy::too_many_arguments)]
    fn call_tracing<'this>(
        &'this mut self,
        air: String,
        prev_data: Vec<u8>,
        data: Vec<u8>,
        init_peer_id: String,
        timestamp: u64,
        ttl: u32,
        current_peer_id: String,
        call_results: CallResults,
        tracing_params: String,
        tracing_output_mode: u8,
        key_pair: &KeyPair,
        particle_id: String,
    ) -> LocalBoxFuture<'this, anyhow::Result<RawAVMOutcome>>;
}

pub(crate) trait DataToHumanReadable {
    fn to_human_readable<'this>(&'this mut self, data: Vec<u8>) -> LocalBoxFuture<'this, Result<String, Box<dyn StdError>>>;
}
