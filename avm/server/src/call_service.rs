/*
 * Copyright 2021 Fluence Labs Limited
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

use crate::{AVMError, IValue, ParticleParameters};

use std::path::PathBuf;

pub type Effect<T> = Box<dyn Fn() -> Result<T, AVMError> + 'static>;
pub struct CallServiceArgs {
    pub particle_parameters: ParticleParameters,
    pub function_args: Vec<IValue>,
    pub create_vault: Effect<PathBuf>,
}

pub type CallServiceClosure = Box<dyn Fn(CallServiceArgs) -> Option<IValue> + 'static>;
