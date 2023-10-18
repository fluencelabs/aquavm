/*
 * Copyright 2020 Fluence Labs Limited
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

use crate::key_utils::derive_dummy_keypair;
#[cfg(feature = "test_with_native_code")]
pub use crate::native_test_runner::NativeAirRunner as DefaultAirRunner;
#[cfg(not(feature = "test_with_native_code"))]
pub use crate::wasm_test_runner::WasmAirRunner as DefaultAirRunner;

pub use crate::native_test_runner::NativeAirRunner;
pub use crate::wasm_test_runner::ReleaseWasmAirRunner;
pub use crate::wasm_test_runner::WasmAirRunner;

use super::CallServiceClosure;

use air_interpreter_signatures::KeyPair;
use avm_server::avm_runner::*;

use std::collections::HashMap;
use std::collections::HashSet;

pub trait AirRunner {
    fn new(current_call_id: impl Into<String>) -> Self;

    #[allow(clippy::too_many_arguments)]
    fn call(
        &mut self,
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
    ) -> Result<RawAVMOutcome, Box<dyn std::error::Error>>;

    fn get_current_peer_id(&self) -> &str;
}

pub struct TestRunner<R = DefaultAirRunner> {
    pub runner: R,
    call_service: CallServiceClosure,
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

impl<R: AirRunner> TestRunner<R> {
    pub fn call(
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
                .map_err(|e| e.to_string())?;

            next_peer_pks.extend(outcome.next_peer_pks);

            if outcome.call_requests.is_empty() {
                outcome.next_peer_pks = next_peer_pks.into_iter().collect::<Vec<_>>();
                return Ok(outcome);
            }

            call_results = outcome
                .call_requests
                .into_iter()
                .map(|(id, call_parameters)| {
                    let service_result = (self.call_service)(call_parameters);
                    (id, service_result)
                })
                .collect::<HashMap<_, _>>();

            prev_data = outcome.data;
            data = vec![];
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn call_single(
        &mut self,
        air: impl Into<String>,
        prev_data: impl Into<Vec<u8>>,
        data: impl Into<Vec<u8>>,
        init_peer_id: impl Into<String>,
        timestamp: u64,
        ttl: u32,
        override_current_peer_id: Option<String>,
        call_results: avm_server::CallResults,
        particle_id: impl Into<String>,
    ) -> Result<RawAVMOutcome, Box<dyn std::error::Error>> {
        self.runner.call(
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
    }
}

pub fn create_avm(
    call_service: CallServiceClosure,
    current_peer_id: impl Into<String>,
) -> TestRunner {
    create_custom_avm(call_service, current_peer_id)
}

pub fn create_custom_avm<R: AirRunner>(
    call_service: CallServiceClosure,
    current_peer_id: impl Into<String>,
) -> TestRunner<R> {
    let current_peer_id = current_peer_id.into();

    let (keypair, _) = derive_dummy_keypair(&current_peer_id);

    let runner = R::new(current_peer_id);

    TestRunner {
        runner,
        call_service,
        keypair,
    }
}

pub fn create_avm_with_key<R: AirRunner>(
    keypair: KeyPair,
    call_service: CallServiceClosure,
) -> TestRunner<R> {
    let current_peer_id = keypair.public().to_peer_id().to_string();
    let runner = R::new(current_peer_id);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::call_services::{set_variables_call_service, VariableOptionSource};

    use avm_interface::CallRequestParams;
    use serde_json::json;

    #[test]
    fn test_override_current_peer_id() {
        let spell_id = "spell_id";
        let host_peer_id = "host_peer_id";
        let script = format!(r#"(call "{spell_id}" ("service" "func") [])"#);

        let variables = maplit::hashmap! {
            "func".to_owned() => json!("success"),
        };

        let key_format = fluence_keypair::KeyFormat::Ed25519;
        let keypair = KeyPair::generate(key_format).unwrap();
        let keypair2 = KeyPair::generate(key_format).unwrap();

        let mut client = create_custom_avm::<NativeAirRunner>(
            set_variables_call_service(variables, VariableOptionSource::FunctionName),
            host_peer_id,
        );

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
            .expect("call should be success");

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
            .expect("call should be success");

        let expected_spell_call_requests = maplit::hashmap! {
            1 => CallRequestParams::new("service", "func", vec![], vec![]),
        };
        let expected_spell_next_peers_pks = Vec::<String>::new();

        assert_eq!(spell_result_1.call_requests, expected_spell_call_requests);
        assert_eq!(spell_result_1.next_peer_pks, expected_spell_next_peers_pks);
    }
}
