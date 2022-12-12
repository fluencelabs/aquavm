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

use crate::JValue;

use air_interpreter_interface::{value_to_cid, CID};
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, rc::Rc};

/// Stores CID to Value corresponance.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CidStore<Val>(HashMap<Rc<CID>, Rc<Val>>);

impl<Val> CidStore<Val> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, cid: &CID) -> Option<Rc<Val>> {
        self.0.get(cid).cloned()
    }
}

impl<Val> Default for CidStore<Val> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[derive(Clone, Debug)]
pub struct CidTracker<Val = JValue> {
    cids: HashMap<Rc<CID>, Rc<Val>>,
}

impl<Val> CidTracker<Val> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_cid_stores(prev_cid_map: &CidStore<Val>, current_cid_map: &CidStore<Val>) -> Self {
        let mut cids = prev_cid_map.0.clone();
        for (cid, val) in &current_cid_map.0 {
            // TODO check that values matches?
            cids.insert(cid.clone(), val.clone());
        }
        Self { cids }
    }

    pub fn get(&self, cid: &CID) -> Option<Rc<Val>> {
        self.cids.get(cid).cloned()
    }
}

impl<Val: Serialize> CidTracker<Val> {
    pub fn record_value(&mut self, value: impl Into<Rc<Val>>) -> Rc<CID> {
        // TODO do something with error: propagate, or unwrap earlier.
        let value = value.into();
        let cid = Rc::new(value_to_cid(&value).unwrap());
        self.cids.insert(cid.clone(), value);
        cid
    }
}

impl<Val> Default for CidTracker<Val> {
    fn default() -> Self {
        Self {
            cids: Default::default(),
        }
    }
}

impl<Val> From<CidTracker<Val>> for CidStore<Val> {
    fn from(value: CidTracker<Val>) -> Self {
        Self(value.cids)
    }
}

impl<Val> IntoIterator for CidStore<Val> {
    type Item = (Rc<CID>, Rc<Val>);

    type IntoIter = std::collections::hash_map::IntoIter<Rc<CID>, Rc<Val>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use super::*;
    use serde_json::json;

    #[test]
    fn test_record() {
        let mut tracker = CidTracker::new();
        tracker.record_value(json!("test"));
        tracker.record_value(json!(1));
        tracker.record_value(json!([1, 2, 3]));
        tracker.record_value(json!({
            "key": 42,
        }));
        let store = CidStore::from(tracker);
        assert_eq!(
            store.into_iter().collect::<HashMap<_, _>>(),
            HashMap::from_iter(vec![
                (
                    CID::new("bagaaierajwlhumardpzj6dv2ahcerm3vyfrjwl7nahg7zq5o3eprwv6v3vpa")
                        .into(),
                    json!("test").into()
                ),
                (
                    CID::new("bagaaierauyk65lxcdxsrphpaqdpiymcszdnjaejyibv2ohbyyaziix35kt2a")
                        .into(),
                    json!([1, 2, 3]).into(),
                ),
                (
                    CID::new("bagaaieranodle477gt6odhllqbhp6wr7k5d23jhkuixr2soadzjn3n4hlnfq")
                        .into(),
                    json!(1).into(),
                ),
                (
                    CID::new("bagaaierad7lci6475zdrps4h6fmcpmqyknz5z6bw6p6tmpjkfyueavqw4kaq")
                        .into(),
                    json!({
                        "key": 42,
                    })
                    .into(),
                )
            ])
        );
    }
}
