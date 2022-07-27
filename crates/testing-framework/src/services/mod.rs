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

pub mod builtins;

use std::time::Duration;

pub type JValue = serde_json::Value;

// TODO: consider using Result instead.
/// Somewhat modified type from fluence.  The Duration defines when the caller receives it, imitating
/// real execution time.
#[derive(Debug)]
pub enum FunctionOutcome {
    Ok(JValue, Duration),
    Err(JValue, Duration),
    NotDefined,
    Empty,
}

/// A mocked Marine service.  It is stateful, and each peer has own instances.
pub trait Service {
    fn call(&self, service_id: &str, function_name: &str, args: &[JValue]) -> FunctionOutcome;
}
