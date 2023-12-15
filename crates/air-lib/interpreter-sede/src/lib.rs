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
pub(crate) mod representation;
pub(crate) mod serialized_type;

pub use crate::format::Format;
pub use crate::representation::FromSerialiedBorrow;
pub use crate::representation::FromSerialized;
pub use crate::representation::Representation;
pub use crate::representation::ToSerialized;
pub use crate::representation::ToWriter;

#[cfg(feature = "rmp-serde")]
pub(crate) mod rmp_serde;
#[cfg(feature = "rmp-serde")]
pub use crate::rmp_serde::RmpSerdeFormat;
#[cfg(feature = "rmp-serde")]
pub use crate::rmp_serde::RmpSerdeMultiformat;

#[cfg(feature = "msgpack")]
pub use crate::rmp_serde::RmpSerdeFormat as MsgPackFormat;
#[cfg(feature = "msgpack")]
pub use crate::rmp_serde::RmpSerdeMultiformat as MsgPackMultiformat;

#[cfg(feature = "serde_json")]
pub(crate) mod serde_json;
#[cfg(feature = "serde_json")]
pub use crate::serde_json::SerdeJsonFormat;
#[cfg(feature = "serde_json")]
pub use crate::serde_json::SerdeJsonMultiformat;

#[cfg(feature = "json")]
pub use crate::serde_json::SerdeJsonFormat as JsonFormat;
#[cfg(feature = "json")]
pub use crate::serde_json::SerdeJsonMultiformat as JsonMultiformat;
