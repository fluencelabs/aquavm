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

mod near_aquavm;

use air_interpreter_interface::RunParameters;
use anyhow::Context;
use clap::Parser;
use near_sdk::test_utils::VMContextBuilder;
use near_sdk::{env, testing_env, RuntimeFeesConfig, VMConfig, VMContext};

use std::path::PathBuf;

#[derive(Parser)]
#[clap(about = "Run a built-in NEAR version of AquaVM")]
// TODO similar to run --plain
pub(crate) struct Args {
    #[clap(long = "call-results")]
    call_results_path: Option<PathBuf>,

    #[command(flatten)]
    keys: crate::trace::run::Keys,

    #[clap(flatten)]
    data: crate::trace::run::data::plain::PlainDataArgs,
}

pub(crate) fn near(args: Args) -> anyhow::Result<()> {
    let execution_data =
        crate::trace::run::data::plain::load(&args.data).context("loading input data")?;
    let key = args.keys.get_keypair()?;
    let particle = execution_data.particle;

    let call_results = crate::trace::run::read_call_results(args.call_results_path.as_deref())?;
    let call_results = serde_json::to_string(&call_results).context("serializing call results")?;

    let run_parameters = RunParameters::new(
        particle.init_peer_id.to_string(),
        particle.current_peer_id.to_string(),
        particle.timestamp,
        particle.ttl,
        fluence_keypair::KeyFormat::Ed25519.into(),
        key.secret().expect("cannot happen"),
        particle.particle_id.to_string(),
    );
    let run_parameters =
        serde_json::to_string(&run_parameters).context("failed to serialize run parameters")?;

    let outcome = execute_on_near(
        execution_data.air_script,
        execution_data.prev_data,
        execution_data.current_data,
        run_parameters,
        call_results,
    );

    println!("{}", outcome);
    Ok(())
}

fn execute_on_near(
    air_script: String,
    prev_data: String,
    current_data: String,
    run_parameters: String,
    call_results: String,
) -> String {
    let aquavm = near_aquavm::Aqua::default();
    let context = get_context(false);
    testing_env!(context.clone(), VMConfig::test(), RuntimeFeesConfig::test());

    let outcome = aquavm.execute_script(
        air_script,
        prev_data,
        current_data,
        run_parameters,
        call_results,
    );
    eprintln!("Used gas: {}", env::used_gas().0,);
    outcome
}

// this code is based on the near/sdk-rs-gas-benchmark repository
fn get_context(is_view: bool) -> VMContext {
    VMContextBuilder::new()
        .signer_account_id("alice_fluence".parse().unwrap())
        .is_view(is_view)
        .build()
}
