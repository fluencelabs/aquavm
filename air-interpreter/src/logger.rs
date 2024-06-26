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

use air_log_targets::TARGET_MAP;

use log::LevelFilter;

pub fn init_logger(default_level: Option<LevelFilter>) {
    let target_map = TARGET_MAP.iter().cloned().collect();
    let builder = marine_rs_sdk::WasmLoggerBuilder::new().with_target_map(target_map);

    let builder = if let Some(default_level) = default_level {
        builder.with_log_level(default_level)
    } else {
        builder
    };

    builder.build().unwrap();
}

#[allow(dead_code)]
pub fn json_output_mode(trace_mode: u8) -> bool {
    trace_mode == 0
}
