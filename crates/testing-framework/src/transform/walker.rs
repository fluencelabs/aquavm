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
use crate::{asserts::ServiceDefinition, ephemeral::PeerId};

use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub(crate) struct Transformer {
    cnt: u32,
    pub(crate) results: HashMap<u32, ServiceDefinition>,
    pub(crate) peers: HashSet<PeerId>,
}

impl Transformer {
    pub(crate) fn new() -> Self {
        Default::default()
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
            self.peers.insert(peer_id.clone().into());
        }

        if let Some(service) = &call.service_desc {
            // install a value
            let call_id = self.cnt;
            self.cnt += 1;

            self.results.insert(call_id, service.clone());

            match &mut call.triplet.1 {
                Sexp::String(ref mut value) => value.push_str(&format!("..{}", call_id)),
                _ => panic!("Incorrect script: non-string service string not supported"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{iter::FromIterator, str::FromStr};

    #[test]
    fn test_translate_null() {
        let mut tree = Sexp::from_str("(null)").unwrap();
        let mut transformer = Transformer::new();
        transformer.transform(&mut tree);
        assert_eq!(tree.to_string(), "(null)");
    }

    #[test]
    fn test_translate_call_no_result() {
        let script = r#"(call peer_id ("service_id" func) [])"#;
        let mut tree = Sexp::from_str(script).unwrap();
        let mut transformer = Transformer::new();
        transformer.transform(&mut tree);
        assert_eq!(tree.to_string(), script);
    }

    #[test]
    #[should_panic]
    fn test_translate_call_no_string() {
        // TODO rewrite to Result instead of panic?
        let script = r#"(call "peer_id" (service_id func) [])"#;
        let mut tree = Sexp::from_str(script).unwrap();
        let mut transformer = Transformer::new();
        transformer.transform(&mut tree);
        assert_eq!(tree.to_string(), script);
    }

    #[test]
    fn test_translate_call_result() {
        let script = r#"(call "peer_id" ("service_id" func) []) ; result = 42"#;
        let mut tree = Sexp::from_str(script).unwrap();
        let mut transformer = Transformer::new();
        transformer.transform(&mut tree);
        assert_eq!(
            tree.to_string(),
            r#"(call "peer_id" ("service_id..0" func) [])"#
        );

        assert_eq!(
            transformer.results,
            maplit::hashmap! {
                0u32 => ServiceDefinition::Result(serde_json::json!(42)),
            }
        );

        assert_eq!(
            transformer.peers.into_iter().collect::<Vec<_>>(),
            vec![PeerId::new("peer_id")],
        );
    }

    #[test]
    fn test_translate_multiple_calls() {
        let script = r#"(seq
   (call peer_id ("service_id" func) [a 11]) ; result={"test":"me"}
   (seq
      (call peer_id ("service_id" func) [b])
      (call peer_id ("service_id" func) [1]) ; result=true
))"#;

        let mut tree = Sexp::from_str(script).unwrap();
        let mut transformer = Transformer::new();
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
            transformer.results,
            maplit::hashmap! {
                0u32 => ServiceDefinition::Result(serde_json::json!({"test":"me"})),
                1 => ServiceDefinition::Result(serde_json::json!(true)),
            }
        );

        assert!(transformer.peers.is_empty());
    }

    #[test]
    fn test_peers() {
        // this script is not correct AIR, but our parser handles it
        let script = r#"(seq
   (call "peer_id1" ("service_id" func) [a 11]) ; result={"test":"me"}
   (seq
      (call "peer_id2" ("service_id" func) [b])
      (call "peer_id1" ("service_id" func) [1]) ; result=true
      (call peer_id3 ("service_id" func) [b])
))"#;

        let mut tree = Sexp::from_str(script).unwrap();
        let mut transformer = Transformer::new();
        transformer.transform(&mut tree);

        assert_eq!(
            transformer.peers,
            HashSet::from_iter(vec![PeerId::new("peer_id1"), PeerId::new("peer_id2")]),
        )
    }
}
