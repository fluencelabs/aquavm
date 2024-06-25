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

use crate::JValue;

use serde::Deserialize;
use serde::Serialize;

use std::cell::RefCell;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(transparent)]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)]
#[archive(check_bytes)]
pub struct RawValue {
    raw: Box<str>,

    #[serde(skip)]
    #[with(::rkyv::with::Skip)]
    parsed: RefCell<Option<JValue>>,
}

impl RawValue {
    pub fn from_value(value: impl Into<JValue>) -> Self {
        let value = value.into();
        let raw = value.to_string().into();
        Self {
            raw,
            parsed: Some(value).into(),
        }
    }

    pub fn get_value(&self) -> JValue {
        let mut parsed_guard = self.parsed.borrow_mut();

        let parsed_value = parsed_guard
            .get_or_insert_with(|| serde_json::from_str(&self.raw).expect("TODO handle error"));
        parsed_value.clone()
    }

    pub(crate) fn as_inner(&self) -> &str {
        &self.raw
    }
}

impl From<JValue> for RawValue {
    fn from(value: JValue) -> Self {
        Self::from_value(value)
    }
}

impl PartialEq for RawValue {
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}

// TODO is it implemented for JValue?
impl Eq for RawValue {}
