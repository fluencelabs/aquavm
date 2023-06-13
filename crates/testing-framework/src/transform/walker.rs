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

use air_test_utils::test_runner::{AirRunner, DefaultAirRunner};

use super::{Call, Canon, Sexp};
use crate::ephemeral::Network;

use std::{fmt::Write, ops::Deref, rc::Rc, str::FromStr};

/// Transformed script represents transformed script's services' state within the network.
/// Executions that use the same transformed script share same generated services' state.
/// This struct is cheap to clone, and cloned copies share same internal state.
#[derive(Clone)]
pub struct TransformedAirScript<R = DefaultAirRunner> {
    network: Rc<Network<R>>,
    tranformed: Rc<str>,
}

impl<R: AirRunner> TransformedAirScript<R> {
    // TODO peer transformation mode
    pub fn new(annotated_air_script: &str, network: Rc<Network<R>>) -> Result<Self, String> {
        // validate the AIR script with the standard parser first
        air_parser::parse(annotated_air_script)?;

        Self::new_unvalidated(annotated_air_script, network)
    }

    pub(crate) fn new_unvalidated(
        annotated_air_script: &str,
        network: Rc<Network<R>>,
    ) -> Result<Self, String> {
        let transformer = Transformer { network: &network };
        let mut sexp = Sexp::from_str(annotated_air_script)?;
        transformer.transform(&mut sexp);

        Ok(Self {
            network,
            tranformed: Rc::from(sexp.to_string().as_str()),
        })
    }

    pub fn get_network(&self) -> Rc<Network<R>> {
        self.network.clone()
    }
}

impl<R> Deref for TransformedAirScript<R> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.tranformed
    }
}

struct Transformer<'net, R> {
    network: &'net Rc<Network<R>>,
}

impl<R: AirRunner> Transformer<'_, R> {
    pub(crate) fn transform(&self, sexp: &mut Sexp) {
        match sexp {
            Sexp::Call(call) => self.handle_call(call),
            Sexp::Canon(canon) => self.handle_canon(canon),
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
        if let Sexp::String(ref mut peer_name) = &mut call.triplet.0 {
            *peer_name = self
                .network
                .ensure_named_peer(peer_name.as_str())
                .to_string();
        }

        let result_store = self.network.get_services().get_result_store();

        if let Some(service) = &call.service_desc {
            // install a value
            let call_id = result_store.insert(service.clone()).unwrap();

            match &mut call.triplet.1 {
                Sexp::String(ref mut value) => {
                    write!(value, "..{call_id}").unwrap();
                }
                _ => panic!("Incorrect script: non-string service string not supported"),
            }
        }
    }

    fn handle_canon(&self, canon: &mut Canon) {
        if let Sexp::String(ref mut peer_name) = &mut canon.peer {
            *peer_name = self
                .network
                .ensure_named_peer(peer_name.as_str())
                .to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use air_test_utils::key_utils::derive_dummy_keypair;

    use super::*;
    use crate::{asserts::ServiceDefinition, ephemeral::PeerId, services::results::ResultStore};
    use air_test_utils::test_runner::NativeAirRunner;

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
        let network = Network::<NativeAirRunner>::new(std::iter::empty::<PeerId>(), vec![]);
        let transformed = TransformedAirScript::new("(null)", network).unwrap();
        assert_eq!(&*transformed, "(null)");
    }

    #[test]
    fn test_translate_call_no_result() {
        let network = Network::<NativeAirRunner>::new(std::iter::empty::<PeerId>(), vec![]);
        let script = r#"(call peer_id ("service_id" func) [])"#;
        let transformed = TransformedAirScript::new_unvalidated(script, network).unwrap();
        assert_eq!(&*transformed, script);
    }

    #[test]
    #[should_panic]
    fn test_translate_call_no_string() {
        let network = Network::<NativeAirRunner>::new(std::iter::empty::<PeerId>(), vec![]);
        let script = r#"(call "peer_id" (service_id func) [])"#;
        let transformed = TransformedAirScript::new(script, network);
        assert_eq!(transformed.as_deref(), Ok(script));
    }

    #[test]
    fn test_translate_call_result() {
        let network = Network::<NativeAirRunner>::new(std::iter::empty::<PeerId>(), vec![]);
        let script = r#"(call "peer_id" ("service_id" func) []) ; ok = 42"#;
        let transformer = TransformedAirScript::new_unvalidated(script, network.clone()).unwrap();

        let (_peer_pk, peer_id) = derive_dummy_keypair("peer_id");

        assert_eq!(
            &*transformer,
            &format!(r#"(call "{peer_id}" ("service_id..0" func) [])"#)
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
            vec![PeerId::from(peer_id)],
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

        let network = Network::<NativeAirRunner>::new(std::iter::empty::<PeerId>(), vec![]);
        let transformed = TransformedAirScript::new_unvalidated(script, network.clone()).unwrap();
        assert_eq!(
            &*transformed,
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
        use pretty_assertions::assert_eq;

        // this script is not correct AIR, but our parser handles it
        let script = r#"(seq
   (call "peer_id1" ("service_id" func) [a 11]) ; ok={"test":"me"}
   (seq
      (call "peer_id2" ("service_id" func) [b])
      (call "peer_id1" ("service_id" func) [1]) ; ok=true
      (call peer_id3 ("service_id" func) [b])
      (canon "peer_id4" $stream #canon)
))"#;

        let network = Network::<NativeAirRunner>::new(std::iter::empty::<PeerId>(), vec![]);
        let t = TransformedAirScript::new_unvalidated(script, network.clone()).unwrap();

        let (_peer_pk, peer_id1) = derive_dummy_keypair("peer_id1");
        let (_peer_pk, peer_id2) = derive_dummy_keypair("peer_id2");
        let (_peer_pk, peer_id4) = derive_dummy_keypair("peer_id4");

        assert_eq!(
            network.get_peers().collect::<HashSet<_>>(),
            HashSet::from_iter(vec![
                PeerId::from(peer_id1.as_str()),
                PeerId::from(peer_id2.as_str()),
                PeerId::from(peer_id4.as_str()),
            ]),
        );

        let expected = format!(
            concat!(
                "(seq",
                r#" (call "{peer_id1}" ("service_id..0" func) [a 11])"#,
                " (seq",
                r#" (call "{peer_id2}" ("service_id" func) [b])"#,
                r#" (call "{peer_id1}" ("service_id..1" func) [1])"#,
                r#" (call peer_id3 ("service_id" func) [b])"#,
                r#" (canon "{peer_id4}" $stream #canon)))"#
            ),
            peer_id1 = peer_id1,
            peer_id2 = peer_id2,
            peer_id4 = peer_id4
        );
        assert_eq!(*t, expected);
    }
}
