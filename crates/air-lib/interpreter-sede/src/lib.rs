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
