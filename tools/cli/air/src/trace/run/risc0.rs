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
use avm_interface::into_raw_result;
use avm_interface::raw_outcome::RawAVMOutcome;
use fluence_keypair::KeyPair;
use zk_aquavm_interface::AquaVMProvingParameters;

use risc0_zkvm::Executor;
use risc0_zkvm::ExecutorEnv;
use zk_aquavm_methods::ZK_AQUAVM_ELF;
use zk_aquavm_methods::ZK_AQUAVM_ID;

use std::time::Instant;

pub struct Risc0Runner {}

impl Risc0Runner {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl AirRunner for Risc0Runner {
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

        let run_params = RunParameters {
            init_peer_id,
            current_peer_id,
            timestamp,
            ttl,
            key_format,
            secret_key_bytes,
            particle_id,
        };

        let call_results = into_raw_result(call_results);
        let call_results = serde_json::to_vec(&call_results)?;

        let arguments = AquaVMProvingParameters {
            air,
            prev_data,
            current_data,
            run_params,
            call_results,
        };

        execute_on_risc0(arguments)
    }
}

fn execute_on_risc0(arguments: AquaVMProvingParameters) -> anyhow::Result<RawAVMOutcome> {
    use risc0_zkvm::serde::from_slice;
    use risc0_zkvm::serde::to_vec;

    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&arguments)?)
        .build()?;

    let mut executor = Executor::from_elf(env, ZK_AQUAVM_ELF)?;

    eprintln!("executor created, session is started");

    let session_timer = Instant::now();
    let session = executor.run()?;
    let session_duration = session_timer.elapsed();

    eprintln!("session capturing finished:");
    eprintln!("  segments count {}", session.segments.len());
    eprintln!("  elapsed time {:?}", session_duration);

    let proving_timer = Instant::now();
    let receipt = session.prove()?;
    let proving_duration = proving_timer.elapsed();

    eprintln!("proving finished:");
    eprintln!("  elapsed time {:?}", proving_duration);
    eprintln!("  journal size {}", receipt.journal.len());

    let verification_timer = Instant::now();
    receipt.verify(ZK_AQUAVM_ID)?;
    let verification_duration = verification_timer.elapsed();

    eprintln!(
        "verification successfully finished in {:?}",
        verification_duration
    );

    let outcome: InterpreterOutcome = from_slice(&receipt.journal)?;
    Ok(RawAVMOutcome::from_interpreter_outcome(outcome)?)
}
