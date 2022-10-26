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

use super::{Call, Sexp};
use crate::ephemeral::Network;

use std::{fmt::Write, rc::Rc};

pub(crate) struct Transformer {
    pub(crate) network: Rc<Network>,
}

impl Transformer {
    pub(crate) fn new(network: Rc<Network>) -> Self {
        Self { network }
    }

    pub(crate) fn transform(&mut self, sexp: &mut Sexp) {
        match sexp {
            Sexp::Call(call) => self.handle_call(call),
            Sexp::List(children) => {
                for child in children.iter_mut().skip(1) {
                    self.transform(child);
                }
            }
            Sexp::Symbol(_) | Sexp::String(_) => {}
        }
    }

    fn handle_call(&mut self, call: &mut Call) {
        // collect peers...
        if let Sexp::String(peer_id) = &call.triplet.0 {
            self.network.ensure_peer(peer_id.clone());
        }

        let result_store = self.network.get_services().get_result_store();

        if let Some(service) = &call.service_desc {
            // install a value
            let call_id = result_store.insert(service.clone()).unwrap();

            match &mut call.triplet.1 {
                Sexp::String(ref mut value) => {
                    write!(value, "..{}", call_id).unwrap();
                }
                _ => panic!("Incorrect script: non-string service string not supported"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{asserts::ServiceDefinition, ephemeral::PeerId, services::results::ResultStore};

    use std::{
        collections::{HashMap, HashSet},
        iter::FromIterator,
        str::FromStr,
    };

    impl ResultStore {
        pub fn into_inner(self) -> HashMap<usize, ServiceDefinition> {
            self.results.into_inner()
        }
    }

    #[test]
    fn test_translate_null() {
        let network = Rc::new(Network::empty());
        let mut tree = Sexp::from_str("(null)").unwrap();
        let mut transformer = Transformer::new(network);
        transformer.transform(&mut tree);
        assert_eq!(tree.to_string(), "(null)");
    }

    #[test]
    fn test_translate_call_no_result() {
        let network = Rc::new(Network::empty());
        let script = r#"(call peer_id ("service_id" func) [])"#;
        let mut tree = Sexp::from_str(script).unwrap();
        let mut transformer = Transformer::new(network);
        transformer.transform(&mut tree);
        assert_eq!(tree.to_string(), script);
    }

    #[test]
    #[should_panic]
    fn test_translate_call_no_string() {
        let network = Rc::new(Network::empty());
        // TODO rewrite to Result instead of panic?
        let script = r#"(call "peer_id" (service_id func) [])"#;
        let mut tree = Sexp::from_str(script).unwrap();
        let mut transformer = Transformer::new(network);
        transformer.transform(&mut tree);
        assert_eq!(tree.to_string(), script);
    }

    #[test]
    fn test_translate_call_result() {
        let network = Rc::new(Network::empty());
        let script = r#"(call "peer_id" ("service_id" func) []) ; ok = 42"#;
        let mut tree = Sexp::from_str(script).unwrap();
        let mut transformer = Transformer::new(network.clone());
        transformer.transform(&mut tree);
        assert_eq!(
            tree.to_string(),
            r#"(call "peer_id" ("service_id..0" func) [])"#
        );

        assert_eq!(
            (*network.get_services().get_result_store())
                .clone()
                .into_inner(),
            maplit::hashmap! {
                0usize => ServiceDefinition::Ok(serde_json::json!(42)),
            }
        );

        assert_eq!(
            network.get_peers().collect::<Vec<_>>(),
            vec![PeerId::new("peer_id")],
        );
    }

    #[test]
    fn test_translate_multiple_calls() {
        let script = r#"(seq
   (call peer_id ("service_id" func) [a 11]) ; ok={"test":"me"}
   (seq
      (call peer_id ("service_id" func) [b])
      (call peer_id ("service_id" func) [1]) ; ok=true
))"#;

        let network = Rc::new(Network::empty());
        let mut tree = Sexp::from_str(script).unwrap();
        let mut transformer = Transformer::new(network.clone());
        transformer.transform(&mut tree);
        assert_eq!(
            tree.to_string(),
            concat!(
                "(seq ",
                r#"(call peer_id ("service_id..0" func) [a 11])"#,
                " (seq ",
                r#"(call peer_id ("service_id" func) [b])"#,
                " ",
                r#"(call peer_id ("service_id..1" func) [1])"#,
                "))",
            )
        );

        assert_eq!(
            (*network.get_services().get_result_store())
                .clone()
                .into_inner(),
            maplit::hashmap! {
                0usize => ServiceDefinition::Ok(serde_json::json!({"test":"me"})),
                1 => ServiceDefinition::Ok(serde_json::json!(true)),
            }
        );

        assert!(network.get_peers().collect::<Vec<_>>().is_empty());
    }

    #[test]
    fn test_peers() {
        // this script is not correct AIR, but our parser handles it
        let script = r#"(seq
   (call "peer_id1" ("service_id" func) [a 11]) ; ok={"test":"me"}
   (seq
      (call "peer_id2" ("service_id" func) [b])
      (call "peer_id1" ("service_id" func) [1]) ; ok=true
      (call peer_id3 ("service_id" func) [b])
))"#;

        let network = Rc::new(Network::empty());
        let mut tree = Sexp::from_str(script).unwrap();
        let mut transformer = Transformer::new(network.clone());
        transformer.transform(&mut tree);

        assert_eq!(
            network.get_peers().collect::<HashSet<_>>(),
            HashSet::from_iter(vec![PeerId::new("peer_id1"), PeerId::new("peer_id2")]),
        )
    }
}
