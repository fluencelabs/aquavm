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
