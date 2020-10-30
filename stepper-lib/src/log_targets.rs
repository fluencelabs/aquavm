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

/// Print out each instruction name at the beginning of its execution.
pub const INSTRUCTION: &str = "instruction";

/// Print out data cache at the beginning of each instruction execution.
pub const DATA_CACHE: &str = "data_cache";

/// Print out next_peer_pks at the beginning of each instruction execution.
pub const NEXT_PEER_PKS: &str = "next_peer_pks";

/// Print out subtree_complete value at the beginning of each instruction execution.
pub const SUBTREE_COMPLETE: &str = "subtree_complete";

/// Print out current call_evidence path at the beginning of each instruction execution.
pub const CALL_EVIDENCE_PATH: &str = "call_evidence_path";

/// Print out count of element in the current subtree at the beginning of each instruction execution.
pub const SUBTREE_ELEMENTS: &str = "subtree_elements_count";

/// Print out state of data cache at the beginning of each instruction execution.
pub const NEW_CALL_EVIDENCE_PATH: &str = "new_call_evidence_path";

/// Print out logs at the evidence merging stage.
pub const EVIDENCE_PATH_MERGE: &str = "evidence_merge";

/// Print out running arguments and params of a script.
pub const INITIAL_PARAMS: &str = "initial_params";

/// Print out state of data cache at the beginning of each instruction execution.
pub const EVIDENCE_CHANGING: &str = "evidence_changing";

/// This map should be used by rust-sdk logger that allows print only necessary targets by id.
pub const TARGET_MAP: [(&str, i64); 10] = [
    (INSTRUCTION, 1 << 1),
    (DATA_CACHE, 1 << 2),
    (NEXT_PEER_PKS, 1 << 3),
    (SUBTREE_COMPLETE, 1 << 4),
    (CALL_EVIDENCE_PATH, 1 << 5),
    (SUBTREE_ELEMENTS, 1 << 6),
    (NEW_CALL_EVIDENCE_PATH, 1 << 7),
    (EVIDENCE_PATH_MERGE, 1 << 8),
    (INITIAL_PARAMS, 1 << 9),
    (EVIDENCE_CHANGING, 1 << 9),
];
