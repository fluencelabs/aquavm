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

use air_interpreter_interface::RunParameters;
use anyhow::Context;
use clap::Parser;
use serde_json::json;

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
        "tools/wasm/air-near-contract/target/wasm32-unknown-unknown/release/aqua_vm.wasm",
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
    path: &str,
    air_script: String,
    prev_data: String,
    current_data: String,
    run_parameters: String,
    call_results: String,
) -> String {
    let outcome = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let worker = workspaces::sandbox().await.unwrap();
            let wasm = std::fs::read(path).unwrap();
            let contract = worker.dev_deploy(&wasm).await.unwrap();
            let result = contract
                .call("execute_script")
                .max_gas()
                .args_json(json!({
                    "air_script": air_script,
                    "prev_data": prev_data,
                    "current_data": current_data,
                    "run_parameters": run_parameters,
                    "call_results": call_results,
                }))
                .transact()
                .await
                .unwrap();
            eprintln!("gas: {}", result.outcome().gas_burnt as f64);

            eprintln!("logs: {:?}", result.logs());

            String::from_utf8(result.raw_bytes().unwrap()).unwrap()
        });
    // let aquavm = near_aquavm::Aqua::default();
    // let context = get_context(false);
    // testing_env!(context.clone(), VMConfig::test(), RuntimeFeesConfig::test());

    // let outcome = aquavm.execute_script(
    //     air_script,
    //     prev_data,
    //     current_data,
    //     run_parameters,
    //     call_results,
    // );
    // eprintln!("Used gas: {}", env::used_gas().0,);
    outcome
}
