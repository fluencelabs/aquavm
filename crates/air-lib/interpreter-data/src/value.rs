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

use serde::{Deserialize, Serialize};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct VmValue {
    raw: Box<serde_json::value::RawValue>,

    #[serde(skip)]
    parsed: RefCell<Option<Rc<JValue>>>,
}

impl VmValue {
    pub fn from_value(value: impl Into<Rc<JValue>>) -> Self {
        let value = value.into();
        // it seems that serde_json API is too limited for no reason...
        let raw =
            serde_json::from_value((*value).clone()).expect("RawValue should be create from Value");
        Self {
            raw,
            parsed: Some(value).into(),
        }
    }

    pub fn get_value(&self) -> Rc<JValue> {
        let mut parsed_guard = self.parsed.borrow_mut();

        let parsed_value = parsed_guard.get_or_insert_with(|| {
            serde_json::to_value(&self.raw)
                .expect("RawValue should be always valid")
                .into()
        });
        parsed_value.clone()
    }
}

impl From<JValue> for VmValue {
    fn from(value: JValue) -> Self {
        Self::from_value(value)
    }
}

impl PartialEq for VmValue {
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}

// TODO is it implemented for JValue?
impl Eq for VmValue {}
