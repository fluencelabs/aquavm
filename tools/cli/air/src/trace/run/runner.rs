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

use avm_interface::raw_outcome::RawAVMOutcome;
use avm_interface::CallResults;
use avm_server::AVMRuntimeLimits;
use avm_server::AquaVMRuntimeLimits;
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
    ) -> LocalBoxFuture<'this, eyre::Result<RawAVMOutcome>>;
}

pub(crate) trait DataToHumanReadable {
    fn to_human_readable<'this>(
        &'this mut self,
        data: Vec<u8>,
    ) -> LocalBoxFuture<'this, Result<String, Box<dyn StdError>>>;
}

/// This struct is used to set limits for the test runner creating AVMRunner.
#[derive(Debug, Default, Clone)]
pub struct TestInitParameters {
    pub air_size_limit: Option<u64>,
    pub particle_size_limit: Option<u64>,
    pub call_result_size_limit: Option<u64>,
    pub hard_limit_enabled: bool,
}
impl TestInitParameters {
    pub fn new(
        air_size_limit: Option<u64>,
        particle_size_limit: Option<u64>,
        call_result_size_limit: Option<u64>,
        hard_limit_enabled: bool,
    ) -> Self {
        Self {
            air_size_limit,
            particle_size_limit,
            call_result_size_limit,
            hard_limit_enabled,
        }
    }
    pub fn no_limits() -> Self {
        Self {
            air_size_limit: Some(u64::MAX),
            particle_size_limit: Some(u64::MAX),
            call_result_size_limit: Some(u64::MAX),
            hard_limit_enabled: false,
        }
    }
}

impl From<TestInitParameters> for AVMRuntimeLimits {
    fn from(value: TestInitParameters) -> Self {
        AVMRuntimeLimits::new(
            value.air_size_limit,
            value.particle_size_limit,
            value.call_result_size_limit,
            value.hard_limit_enabled,
        )
    }
}

impl From<TestInitParameters> for AquaVMRuntimeLimits {
    fn from(value: TestInitParameters) -> Self {
        use air_interpreter_interface::MAX_AIR_SIZE;
        use air_interpreter_interface::MAX_CALL_RESULT_SIZE;
        use air_interpreter_interface::MAX_PARTICLE_SIZE;

        AquaVMRuntimeLimits::new(
            value.air_size_limit.unwrap_or(MAX_AIR_SIZE),
            value.particle_size_limit.unwrap_or(MAX_PARTICLE_SIZE),
            value.call_result_size_limit.unwrap_or(MAX_CALL_RESULT_SIZE),
            value.hard_limit_enabled,
        )
    }
}
