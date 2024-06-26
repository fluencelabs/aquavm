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

use super::runner::AirRunner;

use air_interpreter_interface::InterpreterOutcome;
use air_interpreter_interface::RunParameters;
use avm_interface::into_raw_result;
use avm_interface::raw_outcome::RawAVMOutcome;
use fluence_keypair::KeyPair;
use zk_aquavm_interface::AquaVMProvingParameters;

use risc0_zkvm::default_prover;
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
    ) -> eyre::Result<RawAVMOutcome> {
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

fn execute_on_risc0(arguments: AquaVMProvingParameters) -> eyre::Result<RawAVMOutcome> {
    use risc0_zkvm::serde::from_slice;

    let env = ExecutorEnv::builder().write(&arguments)?.build()?;

    let prover = default_prover();

    eprintln!("default prover created, proving is started");

    let session_timer = Instant::now();
    let receipt = prover.prove_elf(env, ZK_AQUAVM_ELF)?;
    let session_duration = session_timer.elapsed();

    eprintln!("proving finished:");
    eprintln!("  elapsed time {:?}", session_duration);
    eprintln!("  journal size {}", receipt.journal.bytes.len());

    let verification_timer = Instant::now();
    receipt.verify(ZK_AQUAVM_ID)?;
    let verification_duration = verification_timer.elapsed();

    eprintln!(
        "verification successfully finished in {:?}",
        verification_duration
    );

    let outcome: InterpreterOutcome = from_slice(&receipt.journal.bytes)?;
    Ok(RawAVMOutcome::from_interpreter_outcome(outcome)?)
}
