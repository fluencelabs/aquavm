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

use crate::AquaData;
use crate::Result;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Call {
    pub peer_part: (String, Option<String>),
    pub fn_part: (Option<String>, String),
    pub args: String,
    pub result_name: String,
}

impl super::ExecutableInstruction for Call {
    fn execute(self, _data: &mut AquaData) -> Result<()> {
        let service_id = match (self.peer_part.1, self.fn_part.0) {
            (Some(service_id), None) => service_id,
            (None, Some(service_id)) => service_id,
            _ => unimplemented!(),
        };

        let _result = unsafe { crate::call_service(service_id, self.fn_part.1, self.args) };

        Ok(())
    }
}
