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

/// This enum represents the current state of Par FSM, it is needed to handle
/// errors gracefully.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum ParFSMState {
    Initialized,
    LeftCompleted,
    RightCompleted,
}

impl Default for ParFSMState {
    fn default() -> Self {
        Self::Initialized
    }
}

impl ParFSMState {
    pub(super) fn next(&mut self) {
        let next_state = match self {
            Self::Initialized => Self::LeftCompleted,
            Self::LeftCompleted => Self::RightCompleted,
            Self::RightCompleted => Self::RightCompleted,
        };

        *self = next_state;
    }
}
