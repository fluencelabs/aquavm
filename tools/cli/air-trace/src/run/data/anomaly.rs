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

use super::ExecutionData;
use crate::run::load_data;
use avm_data_store::AnomalyData;
use avm_interface::ParticleParameters;

use anyhow::Context;
use clap::Parser;

use std::path::PathBuf;

#[derive(Parser, Debug)]
pub(crate) struct AnomalyDataArgs {
    anomaly_data_path: PathBuf,
}

pub(crate) fn load(args: &AnomalyDataArgs) -> anyhow::Result<super::ExecutionData> {
    let anomaly_json = load_data(&args.anomaly_data_path).context("Failed to read anomaly data")?;
    let anomaly_data: AnomalyData =
        serde_json::from_str(&anomaly_json).context("Failed to parse anomaly data")?;

    let air_script = anomaly_data.air_script.to_string();
    let prev_data = String::from_utf8(anomaly_data.prev_data.to_vec())
        .context("Anomaly current_data is not a valid string")?;
    let current_data = String::from_utf8(anomaly_data.current_data.to_vec())
        .context("Anomaly current_data is not a valid string")?;
    let particle: ParticleParameters<'static, 'static> =
        serde_json::from_reader(&*anomaly_data.particle.to_vec())
            .context("Anomaly particle is not a valid JSON")?;

    Ok(ExecutionData {
        air_script,
        prev_data,
        current_data,
        particle,
    })
}
