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

use crate::key_utils::derive_dummy_keypair;
#[cfg(feature = "test_with_native_code")]
pub use crate::native_test_runner::NativeAirRunner as DefaultAirRunner;
#[cfg(not(feature = "test_with_native_code"))]
pub use crate::wasm_test_runner::WasmAirRunner as DefaultAirRunner;

pub use crate::native_test_runner::NativeAirRunner;
pub use crate::wasm_test_runner::ReleaseWasmAirRunner;
pub use crate::wasm_test_runner::WasmAirRunner;

use super::CallServiceClosure;

use avm_server::avm_runner::*;
use avm_server::AVMRuntimeLimits;
use avm_server::AquaVMRuntimeLimits;
use fluence_keypair::KeyPair;
use futures::future::LocalBoxFuture;
use futures::StreamExt;

use std::collections::HashMap;
use std::collections::HashSet;

pub trait AirRunner {
    fn new(
        current_call_id: impl Into<String>,
        test_init_parameters: TestInitParameters,
    ) -> LocalBoxFuture<'static, Self>;

    #[allow(clippy::too_many_arguments)]
    fn call<'this>(
        &'this mut self,
        air: impl Into<String>,
        prev_data: impl Into<Vec<u8>>,
        data: impl Into<Vec<u8>>,
        init_peer_id: impl Into<String>,
        timestamp: u64,
        ttl: u32,
        override_current_peer_id: Option<String>,
        call_results: avm_server::CallResults,
        key_pair: &KeyPair,
        particle_id: String,
    ) -> LocalBoxFuture<'this, Result<RawAVMOutcome, Box<dyn std::error::Error + 'this>>>;

    fn get_current_peer_id(&self) -> &str;
}

pub struct TestRunner<R = DefaultAirRunner> {
    pub runner: R,
    call_service: CallServiceClosure<'static>,
    pub keypair: KeyPair,
}

#[derive(Debug, Default, Clone)]
pub struct TestRunParameters {
    pub init_peer_id: String,
    pub timestamp: u64,
    pub ttl: u32,
    pub override_current_peer_id: Option<String>,
    pub particle_id: String,
}

/// This struct is used to set limits for the test runner creating AVMRunner.
#[derive(Debug, Default, Clone, Copy)]
pub struct TestInitParameters {
    pub air_size_limit: Option<u64>,
    pub particle_size_limit: Option<u64>,
    pub call_result_size_limit: Option<u64>,
    pub hard_limit_enabled: bool,
}

impl<R: AirRunner> TestRunner<R> {
    pub async fn call(
        &mut self,
        air: impl Into<String>,
        prev_data: impl Into<Vec<u8>>,
        data: impl Into<Vec<u8>>,
        test_run_params: TestRunParameters,
    ) -> Result<RawAVMOutcome, String> {
        let air = air.into();
        let mut prev_data = prev_data.into();
        let mut data = data.into();

        let TestRunParameters {
            init_peer_id,
            timestamp,
            ttl,
            override_current_peer_id,
            particle_id,
        } = test_run_params;

        let mut call_results = HashMap::new();
        let mut next_peer_pks = HashSet::new();

        loop {
            let mut outcome: RawAVMOutcome = self
                .runner
                .call(
                    air.clone(),
                    prev_data,
                    data,
                    init_peer_id.clone(),
                    timestamp,
                    ttl,
                    override_current_peer_id.clone(),
                    call_results,
                    &self.keypair,
                    particle_id.clone(),
                )
                .await
                .map_err(|e| e.to_string())?;

            next_peer_pks.extend(outcome.next_peer_pks);

            if outcome.call_requests.is_empty() {
                outcome.next_peer_pks = next_peer_pks.into_iter().collect::<Vec<_>>();
                return Ok(outcome);
            }

            call_results = futures::stream::iter(outcome.call_requests.into_iter())
                .then(|(id, call_parameters)| {
                    let service_result = (self.call_service)(call_parameters);
                    async move { (id, service_result.await) }
                })
                .collect::<HashMap<_, _>>()
                .await;

            prev_data = outcome.data;
            data = vec![];
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn call_single<'this>(
        &'this mut self,
        air: impl Into<String>,
        prev_data: impl Into<Vec<u8>>,
        data: impl Into<Vec<u8>>,
        init_peer_id: impl Into<String>,
        timestamp: u64,
        ttl: u32,
        override_current_peer_id: Option<String>,
        call_results: avm_server::CallResults,
        particle_id: impl Into<String>,
    ) -> Result<RawAVMOutcome, Box<dyn std::error::Error + 'this>> {
        self.runner
            .call(
                air,
                prev_data,
                data,
                init_peer_id,
                timestamp,
                ttl,
                override_current_peer_id,
                call_results,
                &self.keypair,
                particle_id.into(),
            )
            .await
    }
}

pub async fn create_avm(
    call_service: CallServiceClosure<'static>,
    current_peer_id: impl Into<String>,
) -> TestRunner {
    create_custom_avm(call_service, current_peer_id).await
}

pub async fn create_custom_avm<R: AirRunner>(
    call_service: CallServiceClosure<'static>,
    current_peer_id: impl Into<String>,
) -> TestRunner<R> {
    let current_peer_id = current_peer_id.into();
    let (keypair, _) = derive_dummy_keypair(&current_peer_id);
    let runner = R::new(current_peer_id, <_>::default()).await;

    TestRunner {
        runner,
        call_service,
        keypair: keypair.into_inner(),
    }
}

pub async fn create_avm_with_key<R: AirRunner>(
    keypair: impl Into<KeyPair>,
    call_service: CallServiceClosure<'static>,
    test_init_parameters: TestInitParameters,
) -> TestRunner<R> {
    let keypair = keypair.into();
    let current_peer_id = keypair.public().to_peer_id().to_string();
    let runner = R::new(current_peer_id, test_init_parameters).await;

    TestRunner {
        runner,
        call_service,
        keypair,
    }
}

impl TestRunParameters {
    pub fn new(
        init_peer_id: impl Into<String>,
        timestamp: u64,
        ttl: u32,
        particle_id: impl Into<String>,
    ) -> Self {
        Self {
            init_peer_id: init_peer_id.into(),
            timestamp,
            ttl,
            override_current_peer_id: None,
            particle_id: particle_id.into(),
        }
    }

    pub fn from_init_peer_id(init_peer_id: impl Into<String>) -> Self {
        Self {
            init_peer_id: init_peer_id.into(),
            ..<_>::default()
        }
    }

    pub fn from_timestamp(timestamp: u64) -> Self {
        Self {
            timestamp,
            ..<_>::default()
        }
    }

    pub fn from_ttl(ttl: u32) -> Self {
        Self {
            ttl,
            ..<_>::default()
        }
    }

    pub fn with_particle_id(mut self, particle_id: impl Into<String>) -> Self {
        self.particle_id = particle_id.into();
        self
    }
}

impl TestInitParameters {
    pub fn new(
        air_size_limit: u64,
        particle_size_limit: u64,
        call_result_size_limit: u64,
        hard_limit_enabled: bool,
    ) -> Self {
        Self {
            air_size_limit: Some(air_size_limit),
            particle_size_limit: Some(particle_size_limit),
            call_result_size_limit: Some(call_result_size_limit),
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
        let air_size_limit = value.air_size_limit.unwrap_or(MAX_AIR_SIZE);
        let particle_size_limit: u64 = value.particle_size_limit.unwrap_or(MAX_PARTICLE_SIZE);
        let call_result_size_limit = value.call_result_size_limit.unwrap_or(MAX_CALL_RESULT_SIZE);

        AquaVMRuntimeLimits::new(
            air_size_limit,
            particle_size_limit,
            call_result_size_limit,
            value.hard_limit_enabled,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::call_services::{set_variables_call_service, VariableOptionSource};

    use avm_interface::CallRequestParams;
    use serde_json::json;

    #[tokio::test]
    async fn test_override_current_peer_id() {
        let spell_id = "spell_id";
        let host_peer_id = "host_peer_id";
        let script = format!(r#"(call "{spell_id}" ("service" "func") [])"#);

        let variables = maplit::hashmap! {
            "func".to_owned() => json!("success"),
        };

        let key_format = fluence_keypair::KeyFormat::Ed25519;
        let keypair = KeyPair::generate(key_format);
        let keypair2 = KeyPair::generate(key_format);

        let mut client = create_custom_avm::<NativeAirRunner>(
            set_variables_call_service(variables, VariableOptionSource::FunctionName),
            host_peer_id,
        )
        .await;

        let current_result_1 = client
            .runner
            .call(
                &script,
                "",
                "",
                spell_id,
                0,
                0,
                None,
                HashMap::new(),
                &keypair,
                "".to_owned(),
            )
            .await
            .expect("call should be success");

        assert_eq!(
            current_result_1.ret_code, 0,
            "{:?}",
            current_result_1.error_message
        );

        let expected_current_call_requests = HashMap::new();
        let expected_current_next_peers_pks = vec![spell_id.to_owned()];

        assert_eq!(
            current_result_1.call_requests,
            expected_current_call_requests
        );
        assert_eq!(
            current_result_1.next_peer_pks,
            expected_current_next_peers_pks
        );

        let spell_result_1 = client
            .runner
            .call(
                script,
                "",
                "",
                spell_id,
                0,
                0,
                Some(spell_id.to_owned()),
                HashMap::new(),
                &keypair2,
                "".to_owned(),
            )
            .await
            .expect("call should be success");

        let expected_spell_call_requests = maplit::hashmap! {
            1 => CallRequestParams::new("service", "func", vec![], vec![]),
        };
        let expected_spell_next_peers_pks = Vec::<String>::new();

        assert_eq!(spell_result_1.call_requests, expected_spell_call_requests);
        assert_eq!(spell_result_1.next_peer_pks, expected_spell_next_peers_pks);
    }
}
