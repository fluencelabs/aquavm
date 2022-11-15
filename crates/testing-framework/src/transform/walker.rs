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

use std::{fmt::Write, ops::Deref, rc::Rc, str::FromStr};

#[derive(Clone)]
pub struct Transformee {
    network: Rc<Network>,
    tranformed: Rc<str>,
}

impl Transformee {
    pub fn new(annotated_air_script: &str, network: Rc<Network>) -> Result<Self, String> {
        // validate the AIR script with the standard parser first
        air_parser::parse(annotated_air_script)?;

        Self::new_unvalidated(annotated_air_script, network)
    }

    pub(crate) fn new_unvalidated(
        annotated_air_script: &str,
        network: Rc<Network>,
    ) -> Result<Self, String> {
        let transformer = Transformer { network: &network };
        let mut sexp = Sexp::from_str(annotated_air_script)?;
        transformer.transform(&mut sexp);

        Ok(Self {
            network,
            tranformed: Rc::from(sexp.to_string().as_str()),
        })
    }

    pub(crate) fn get_network(&self) -> Rc<Network> {
        self.network.clone()
    }
}

impl Deref for Transformee {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.tranformed
    }
}

struct Transformer<'net> {
    network: &'net Rc<Network>,
}

impl Transformer<'_> {
    pub(crate) fn transform(&self, sexp: &mut Sexp) {
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

    fn handle_call(&self, call: &mut Call) {
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
    };

    impl ResultStore {
        pub fn into_inner(self) -> HashMap<usize, ServiceDefinition> {
            self.results.into_inner()
        }
    }

    #[test]
    fn test_translate_null() {
        let network = Network::empty();
        let transformee = Transformee::new("(null)", network).unwrap();
        assert_eq!(&*transformee, "(null)");
    }

    #[test]
    fn test_translate_call_no_result() {
        let network = Network::empty();
        let script = r#"(call peer_id ("service_id" func) [])"#;
        let transformee = Transformee::new_unvalidated(script, network).unwrap();
        assert_eq!(&*transformee, script);
    }

    #[test]
    #[should_panic]
    fn test_translate_call_no_string() {
        let network = Network::empty();
        let script = r#"(call "peer_id" (service_id func) [])"#;
        let transformee = Transformee::new(script, network);
        assert_eq!(transformee.as_deref(), Ok(script));
    }

    #[test]
    fn test_translate_call_result() {
        let network = Network::empty();
        let script = r#"(call "peer_id" ("service_id" func) []) ; ok = 42"#;
        let transformer = Transformee::new_unvalidated(script, network.clone()).unwrap();
        assert_eq!(
            &*transformer,
            r#"(call "peer_id" ("service_id..0" func) [])"#
        );

        assert_eq!(
            Rc::deref(&network.get_services().get_result_store())
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

        let network = Network::empty();
        let transformee = Transformee::new_unvalidated(script, network.clone()).unwrap();
        assert_eq!(
            &*transformee,
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

        let network = Network::empty();
        let _ = Transformee::new_unvalidated(script, network.clone());

        assert_eq!(
            network.get_peers().collect::<HashSet<_>>(),
            HashSet::from_iter(vec![PeerId::new("peer_id1"), PeerId::new("peer_id2")]),
        )
    }
}
