/*
 * Copyright 2023 Fluence Labs Limited
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

pub mod multiformat;

pub(crate) mod format;
pub(crate) mod repr;
#[cfg(feature = "rmp-serde")]
pub(crate) mod rmp_serde;
#[cfg(feature = "serde_json")]
pub(crate) mod serde_json;
pub(crate) mod serialized_type;

pub use crate::format::Format;
pub use crate::repr::{FromSerialiedBorrow, FromSerialized, ToSerialized, ToWriter, TypedFormat};

#[cfg(feature = "rmp-serde")]
pub use crate::rmp_serde::{RmpSerdeFormat, RmpSerdeMultiformat};

#[cfg(feature = "serde_json")]
pub use crate::serde_json::{SerdeJsonFormat, SerdeJsonMultiformat};
