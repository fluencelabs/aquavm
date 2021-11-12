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

use super::Scalars;

use std::fmt;

impl<'i> fmt::Display for Scalars<'i> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "fold_block_id: {}", self.fold_block_id)?;

        for (name, _) in self.values.iter() {
            let value = self.get_value(name);
            if let Ok(last_value) = value {
                writeln!(f, "{} => {}", name, last_value.result)?;
            }
        }

        for (name, _) in self.iterable_values.iter() {
            // it's impossible to print an iterable value for now
            writeln!(f, "{} => iterable", name)?;
        }

        Ok(())
    }
}
