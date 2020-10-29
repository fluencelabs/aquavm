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

pub const INSTRUCTION: &str = "instruction";
pub const EXEC_CTX: &str = "exec_ctx";
pub const CALL_EVIDENCE_CTX: &str = "evidence_ctx";
pub const SUBTREE: &str = "subtree_complete";
pub const EVIDENCE_MERGE: &str = "evidence_merge";
pub const EVIDENCE_PREV_STATE: &str = "evidence_prev_state";
pub const EVIDENCE_NEW_STATE: &str = "evidence_new_state";

/// This map should be used by rust-sdk logger that allows print only necessary targets by id.
pub const TARGET_MAP: [(&'static str, i64); 7] = [
    (INSTRUCTION, 1 << 1),
    (EXEC_CTX, 1 << 2),
    (CALL_EVIDENCE_CTX, 1 << 3),
    (SUBTREE, 1 << 4),
    (EVIDENCE_MERGE, 1 << 5),
    (EVIDENCE_PREV_STATE, 1 << 6),
    (EVIDENCE_NEW_STATE, 1 << 7),
];
