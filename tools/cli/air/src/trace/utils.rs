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
use std::time::Duration;

// unfortunately, external crates don't seem to provide required functionality:
// some do not handle floats, others do not handle suffixes
pub(crate) fn parse_tracing_duration(input: &str) -> Result<Duration, eyre::Error> {
    for (suffix, scale) in [("ns", 1e-9), ("µs", 1e-6), ("ms", 1e-3), ("s", 1e0)] {
        if let Some(num_str) = input.strip_suffix(suffix) {
            if let Ok(num) = num_str.parse::<f64>() {
                return Ok(Duration::from_secs_f64(num * scale));
            } else {
                break;
            }
        }
    }

    Err(eyre::eyre!("malformed duration {:?}", input))
}

pub(crate) fn unix_timestamp_now() -> u64 {
    use std::time::SystemTime;

    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time befor Unix epoch")
        .as_secs()
}
