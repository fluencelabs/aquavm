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
use std::time::Duration;

pub(crate) fn parse_tracing_duration(input: &str) -> Result<Duration, anyhow::Error> {
    for (suffix, scale) in [("ns", 1e-9), ("µs", 1e-6), ("ms", 1e-3), ("s", 1e0)] {
        if let Some(num_str) = input.strip_suffix(suffix) {
            if let Ok(num) = num_str.parse::<f64>() {
                return Ok(Duration::from_secs_f64(num * scale));
            } else {
                break;
            }
        }
    }
    return Err(anyhow::anyhow!("malformed duration {:?}", input));
}

pub(crate) fn unix_timestamp_now() -> u64 {
    use std::time::SystemTime;

    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time befor Unix epoch")
        .as_secs()
}
