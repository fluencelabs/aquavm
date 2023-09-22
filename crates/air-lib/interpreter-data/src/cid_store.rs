/*
 * Copyright 2022 Fluence Labs Limited
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

use air_interpreter_cid::value_to_json_cid;
use air_interpreter_cid::CidCalculationError;
use air_interpreter_cid::CID;
use serde::Deserialize;
use serde::Serialize;

use std::collections::HashMap;
use std::rc::Rc;

/// Stores CID to Value corresponance.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
#[cfg_attr(
    feature = "borsh",
    derive(::borsh::BorshSerialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive(check_bytes))]
pub struct CidStore<Val: ::borsh::BorshSerialize + Clone>(
    #[cfg_attr(feature = "rkyv", with(::rkyv::with::AsVec))] HashMap<Rc<CID<Val>>, Rc<Val>>,
);

impl<Val> CidStore<Val>
where
    Val: ::borsh::BorshSerialize + Clone,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, cid: &CID<Val>) -> Option<Rc<Val>> {
        self.0.get(cid).cloned()
    }

    /// Resolve a key if it exists.  Is useful for CID deduplication.
    // TODO feature?
    pub fn get_key(&self, cid: &CID<Val>) -> Option<&Rc<CID<Val>>> {
        self.0.get_key_value(cid).map(|(key, _val)| key)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    // TODO feature?
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, Rc<CID<Val>>, Rc<Val>> {
        self.0.iter()
    }

    // TODO feature?
    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<'_, Rc<CID<Val>>, Rc<Val>> {
        self.0.iter_mut()
    }
}

impl<Val> Default for CidStore<Val>
where
    Val: ::borsh::BorshSerialize + Clone,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

#[derive(Clone, Debug)]
pub struct CidTracker<Val = JValue> {
    cids: HashMap<Rc<CID<Val>>, Rc<Val>>,
}

impl<Val> CidTracker<Val>
where Val: ::borsh::BorshSerialize + Clone
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_cid_stores(prev_cid_map: CidStore<Val>, current_cid_map: CidStore<Val>) -> Self {
        let mut cids = prev_cid_map.0;
        for (cid, val) in current_cid_map.0 {
            // TODO check that values matches?
            cids.insert(cid, val);
        }
        Self { cids }
    }

    pub fn get(&self, cid: &CID<Val>) -> Option<Rc<Val>> {
        self.cids.get(cid).cloned()
    }
}

impl<Val: Serialize> CidTracker<Val> {
    pub fn track_value(
        &mut self,
        value: impl Into<Rc<Val>>,
    ) -> Result<Rc<CID<Val>>, CidCalculationError> {
        let value = value.into();
        let cid = Rc::new(value_to_json_cid(&*value)?);
        self.cids.insert(cid.clone(), value);
        Ok(cid)
    }
}

impl<Val> Default for CidTracker<Val> {
    fn default() -> Self {
        Self {
            cids: Default::default(),
        }
    }
}

impl<Val> From<CidTracker<Val>> for CidStore<Val>
where Val: ::borsh::BorshSerialize + Clone
{
    fn from(value: CidTracker<Val>) -> Self {
        Self(value.cids)
    }
}

impl<Val> IntoIterator for CidStore<Val>
where Val: ::borsh::BorshSerialize + Clone
{
    type Item = (Rc<CID<Val>>, Rc<Val>);

    type IntoIter = std::collections::hash_map::IntoIter<Rc<CID<Val>>, Rc<Val>>;

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
    fn test_iter() {
        let mut tracker = CidTracker::new();
        tracker.track_value(json!("test")).unwrap();
        tracker.track_value(json!(1)).unwrap();
        tracker.track_value(json!([1, 2, 3])).unwrap();
        tracker
            .track_value(json!({
                "key": 42,
            }))
            .unwrap();
        let store = CidStore::from(tracker);
        assert_eq!(
            store.into_iter().collect::<HashMap<_, _>>(),
            HashMap::from_iter(vec![
                (
                    CID::new("bagaaierajwlhumardpzj6dv2ahcerm3vyfrjwl7nahg7zq5o3eprwv6v3vpa")
                        .unwrap()
                        .into(),
                    json!("test").into()
                ),
                (
                    CID::new("bagaaierauyk65lxcdxsrphpaqdpiymcszdnjaejyibv2ohbyyaziix35kt2a")
                        .unwrap()
                        .into(),
                    json!([1, 2, 3]).into(),
                ),
                (
                    CID::new("bagaaieranodle477gt6odhllqbhp6wr7k5d23jhkuixr2soadzjn3n4hlnfq")
                        .unwrap()
                        .into(),
                    json!(1).into(),
                ),
                (
                    CID::new("bagaaierad7lci6475zdrps4h6fmcpmqyknz5z6bw6p6tmpjkfyueavqw4kaq")
                        .unwrap()
                        .into(),
                    json!({
                        "key": 42,
                    })
                    .into(),
                )
            ])
        );
    }

    #[test]
    fn test_store() {
        let mut tracker = CidTracker::new();
        tracker.track_value(json!("test")).unwrap();
        tracker.track_value(json!(1)).unwrap();
        tracker.track_value(json!([1, 2, 3])).unwrap();
        tracker
            .track_value(json!({
                "key": 42,
            }))
            .unwrap();
        let store = CidStore::from(tracker);

        assert_eq!(
            &*store
                .get(
                    &CID::new("bagaaierajwlhumardpzj6dv2ahcerm3vyfrjwl7nahg7zq5o3eprwv6v3vpa")
                        .unwrap()
                )
                .unwrap(),
            &json!("test"),
        );
        assert_eq!(
            &*store
                .get(
                    &CID::new("bagaaierauyk65lxcdxsrphpaqdpiymcszdnjaejyibv2ohbyyaziix35kt2a")
                        .unwrap()
                )
                .unwrap(),
            &json!([1, 2, 3]),
        );
        assert_eq!(
            &*store
                .get(
                    &CID::new("bagaaieranodle477gt6odhllqbhp6wr7k5d23jhkuixr2soadzjn3n4hlnfq")
                        .unwrap()
                )
                .unwrap(),
            &json!(1),
        );
        assert_eq!(
            &*store
                .get(
                    &CID::new("bagaaierad7lci6475zdrps4h6fmcpmqyknz5z6bw6p6tmpjkfyueavqw4kaq")
                        .unwrap()
                )
                .unwrap(),
            &json!({"key": 42}),
        );

        assert_eq!(
            store.get(
                &CID::new("bagaaierad7lci6475zdrps4h6fmcpmqyknz5z6bw6p6tmpjkfyumavqw4kaq").unwrap()
            ),
            None,
        );
    }
}
