/*
 * Copyright 2023 Fluence Labs Limited
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

use super::runner::AirRunner;

use air_interpreter_interface::InterpreterOutcome;
use air_interpreter_interface::RunParameters;
use anyhow::Context;
use avm_interface::raw_outcome::RawAVMOutcome;
use fluence_keypair::KeyPair;

use std::path::{Path, PathBuf};

pub(crate) struct NearRunner {
    air_contract_wasm_path: PathBuf,
}

impl AirRunner for NearRunner {
    fn call_tracing(
        &mut self,
        air: String,
        prev_data: Vec<u8>,
        current_data: Vec<u8>,
        init_peer_id: String,
        timestamp: u64,
        ttl: u32,
        current_peer_id: String,
        call_results: avm_interface::CallResults,
        _tracing_params: String,
        _tracing_output_mode: u8,
        keypair: &KeyPair,
        particle_id: String,
    ) -> anyhow::Result<RawAVMOutcome> {
        let key_format = keypair.key_format().into();
        let secret_key_bytes = keypair.secret().expect("Failed to get secret key");

        let run_parameters = RunParameters {
            init_peer_id,
            current_peer_id,
            timestamp,
            ttl,
            key_format,
            secret_key_bytes,
            particle_id,
        };

        execute_on_near(
            &self.air_contract_wasm_path,
            air,
            prev_data,
            current_data,
            run_parameters,
            call_results,
        )
    }
}

pub(crate) fn create_near_runner(
    air_contract_wasm_path: &Path,
) -> anyhow::Result<Box<dyn AirRunner>> {
    let air_contract_wasm_path = air_contract_wasm_path.to_owned();

    Ok(Box::new(NearRunner {
        air_contract_wasm_path,
    }))
}

fn execute_on_near(
    path: &Path,
    air_script: String,
    prev_data: Vec<u8>,
    current_data: Vec<u8>,
    run_parameters: RunParameters,
    call_results: avm_interface::CallResults,
) -> anyhow::Result<avm_interface::raw_outcome::RawAVMOutcome> {
    use avm_interface::into_raw_result;

    let run_parameters = serde_json::to_string(&run_parameters)?;

    // some inner parts transformations
    let raw_call_results = into_raw_result(call_results);
    let raw_call_results = serde_json::to_vec(&raw_call_results)?;

    let wasm = std::fs::read(path)?;

    let result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            let worker = workspaces::sandbox().await?;
            let contract = worker.dev_deploy(&wasm).await?;
            contract
                .call("execute_script")
                .max_gas()
                .args_borsh((
                    air_script,
                    prev_data,
                    current_data,
                    run_parameters,
                    raw_call_results,
                ))
                .transact()
                .await
        })
        .context("failed to execute contract")?;

    eprintln!("total gas: {:e}", result.total_gas_burnt);
    eprintln!("transaction gas: {:e}", result.outcome().gas_burnt);

    let data: String = result
        .borsh()
        .context("failed to deserialize contract result")?;
    let outcome: InterpreterOutcome =
        serde_json::from_str(&data).context("failed to parse JSON data")?;
    Ok(RawAVMOutcome::from_interpreter_outcome(outcome)?)
}
