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

use super::EvidenceState;
use super::NewEvidenceState;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct CallEvidenceContext {
    pub(crate) current_states: Vec<EvidenceState>,
    pub(crate) left: usize,
    pub(crate) right: usize,
    pub(crate) new_states: Vec<NewEvidenceState>,
}

impl CallEvidenceContext {
    pub fn new(current_states: Vec<EvidenceState>) -> Self {
        let right = current_states.len();
        Self {
            current_states,
            left: 0,
            right,
            new_states: vec![],
        }
    }

    pub fn into_states(self) -> Vec<EvidenceState> {
        let mut result = vec![];
        let mut new_states = self.new_states;
        let mut left_par_size = 0;
        let mut right_par_size = 0;

        for new_state_id in 0..new_states.len() {
            match new_states.remove(new_state_id) {
                NewEvidenceState::LeftPar(left) => {
                    while let NewEvidenceState::RightPar(_) =
                        new_states[new_state_id + left_par_size]
                    {
                        left_par_size += 1;
                    }
                    left_par_size += left;
                }
                NewEvidenceState::RightPar(right) => {
                    for i in new_state_id..new_states.len() {
                        if let NewEvidenceState::LeftPar(_) = new_states[i] {
                            break;
                        }

                        right_par_size += 1;
                    }

                    right_par_size += right;
                }
                NewEvidenceState::EvidenceState(state) => {
                    result.push(state);
                }
            }

            if right_par_size != 0 {
                result.push(EvidenceState::Par(left_par_size, right_par_size));
                left_par_size = 0;
                right_par_size = 0;
            }
        }

        result
    }
}
