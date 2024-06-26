/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use super::AVMDataStore;
use std::path::PathBuf;

/// Describes behaviour of the AVM.
pub struct AVMConfig<E> {
    /// Path to a AIR interpreter Wasm file.
    pub air_wasm_path: PathBuf,

    /// Maximum heap size in bytes available for the interpreter.
    pub max_heap_size: Option<u64>,

    /// Mask used to filter logs, for details see `log_utf8_string` in fluence-faas.
    pub logging_mask: i32,

    pub data_store: AVMDataStore<E>,
}
