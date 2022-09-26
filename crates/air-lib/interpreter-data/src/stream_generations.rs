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

use air_parser::TextPos;

use std::collections::HashMap;

/// Mapping from a stream name to it's generation count.
/// Similar to pi-calculus non-restricted names/channels.
pub type GlobalStreamGens = HashMap<String, u32>;

/// Mapping from a stream name to
///    position of a new instruction in a script that creates a scope for a stream
///        to vector where each position represents a corresponding iteration.
///
/// Vec<u32> is needed because a new instruction could be placed into a fold instruction,
/// so it could be met several times during script execution. This field anchors iteration
/// where it was met.
/// Similar to pi-calculus restricted names/channels.
pub type RestrictedStreamGens = HashMap<String, HashMap<TextPos, Vec<u32>>>;
