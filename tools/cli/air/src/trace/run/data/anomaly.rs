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

use crate::trace::run::runner::TestInitParameters;

use super::super::load_data;
use super::ExecutionData;
use avm_data_store::AnomalyData;
use avm_interface::ParticleParameters;

use clap::Parser;
use eyre::Context;

use std::path::PathBuf;

#[derive(Parser, Debug)]
pub(crate) struct AnomalyDataArgs {
    anomaly_data_path: PathBuf,
}

pub(crate) fn load(args: &AnomalyDataArgs) -> eyre::Result<super::ExecutionData<'_>> {
    let anomaly_json = load_data(&args.anomaly_data_path).context("Failed to read anomaly data")?;
    let anomaly_data: AnomalyData<'_> =
        serde_json::from_slice(&anomaly_json).context("Failed to parse anomaly data")?;

    let air_script = anomaly_data.air_script.to_string();
    let prev_data = anomaly_data.prev_data.to_vec();
    let current_data = anomaly_data.current_data.to_vec();
    let particle: ParticleParameters<'static> = serde_json::from_slice(&anomaly_data.particle)
        .context("Anomaly particle is not a valid JSON")?;
    let test_init_parameters = TestInitParameters::no_limits();

    Ok(ExecutionData {
        air_script,
        prev_data,
        current_data,
        particle,
        test_init_parameters,
    })
}
