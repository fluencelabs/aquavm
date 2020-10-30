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
pub const DATA_CACHE: &str = "data_cache";
pub const NEXT_PEER_PKS: &str = "next_peer_pks";
pub const SUBTREE_COMPLETE: &str = "subtree_complete";
pub const CALL_EVIDENCE_PATH: &str = "call_evidence_path";
pub const SUBTREE_ELEMENTS: &str = "subtree_elements_count";
pub const NEW_CALL_EVIDENCE_PATH: &str = "new_call_evidence_path";
pub const EVIDENCE_MERGE: &str = "evidence_merge";

/// This map should be used by rust-sdk logger that allows print only necessary targets by id.
pub const TARGET_MAP: [(&str, i64); 8] = [
    (INSTRUCTION, 1 << 1),
    (DATA_CACHE, 1 << 2),
    (NEXT_PEER_PKS, 1 << 3),
    (SUBTREE_COMPLETE, 1 << 4),
    (CALL_EVIDENCE_PATH, 1 << 5),
    (SUBTREE_ELEMENTS, 1 << 6),
    (NEW_CALL_EVIDENCE_PATH, 1 << 7),
    (EVIDENCE_MERGE, 1 << 8),
];
