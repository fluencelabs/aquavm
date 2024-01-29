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
