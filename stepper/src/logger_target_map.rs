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

pub const TARGET_MAP: [(&'static str, i64); 7] = [
    ("instruction", 1 << 0),
    ("exec_ctx", 1 << 1),
    ("evidence_ctx", 1 << 2),
    ("subtree_complete", 1 << 3),
    ("evidence_merge", 1 << 4),
    ("evidence_prev_state", 1 << 5),
    ("evidence_new_state", 1 << 6),
];
