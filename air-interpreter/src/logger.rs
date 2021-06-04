/*
 * Copyright 2020 Fluence Labs Limited
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

use air::log_targets::TARGET_MAP;

use log::LevelFilter;

pub fn init_logger(default_level: Option<LevelFilter>) {
    let target_map = TARGET_MAP.iter().cloned().collect();
    let builder = fluence::WasmLoggerBuilder::new()
        .with_target_map(target_map)
        .filter("jsonpath_lib", log::LevelFilter::Info);

    let builder = if let Some(default_level) = default_level {
        builder.with_log_level(default_level)
    } else {
        builder
    };

    builder.build().unwrap();
}
