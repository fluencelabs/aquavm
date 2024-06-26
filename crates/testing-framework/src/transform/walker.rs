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

use super::{Call, Canon, Sexp};
use crate::ephemeral::Network;

use air_test_utils::key_utils::at;
use air_test_utils::test_runner::AirRunner;
use air_test_utils::test_runner::DefaultAirRunner;
use air_test_utils::test_runner::TestInitParameters;

use std::{borrow::Cow, fmt::Write, ops::Deref, rc::Rc, str::FromStr};

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
    pub async fn new(
        annotated_air_script: &str,
        network: Rc<Network<R>>,
        test_init_parameters: TestInitParameters,
    ) -> Result<Self, String> {
        let at_transformed_air_script = at_transform(annotated_air_script);

        // validate the AIR script with the standard parser first
        air_parser::parse(&at_transformed_air_script)?;

        Self::new_unvalidated(&at_transformed_air_script, network, test_init_parameters).await
    }

    pub(crate) async fn new_unvalidated(
        at_transformed_air_script: &str,
        network: Rc<Network<R>>,
        test_init_parameters: TestInitParameters,
    ) -> Result<Self, String> {
        let transformer = Transformer { network: &network };
        let mut sexp = Sexp::from_str(at_transformed_air_script)?;
        transformer.transform(&mut sexp, test_init_parameters).await;

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
    #[async_recursion::async_recursion(?Send)]
    pub(crate) async fn transform(
        &self,
        sexp: &mut Sexp,
        test_init_parameters: TestInitParameters,
    ) {
        match sexp {
            Sexp::Call(call) => self.handle_call(call, test_init_parameters).await,
            Sexp::Canon(canon) => self.handle_canon(canon, test_init_parameters).await,
            Sexp::List(children) => {
                for child in children.iter_mut().skip(1) {
                    self.transform(child, test_init_parameters).await;
                }
            }
            Sexp::Symbol(_) | Sexp::String(_) => {}
        }
    }

    async fn handle_call(&self, call: &mut Call, test_init_parameters: TestInitParameters) {
        // collect peers...
        if let Sexp::String(ref mut peer_name) = &mut call.triplet.0 {
            *peer_name = self
                .network
                .ensure_named_peer(peer_name.as_str(), test_init_parameters)
                .await
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

    async fn handle_canon(&self, canon: &mut Canon, test_init_parameters: TestInitParameters) {
        if let Sexp::String(ref mut peer_name) = &mut canon.peer {
            *peer_name = self
                .network
                .ensure_named_peer(peer_name.as_str(), test_init_parameters)
                .await
                .to_string();
        }
    }
}

/// Replace substrings for the form @"peer_name" by a derived peer ID.
///
/// It works like a pre-processor.
fn at_transform(air_script: &str) -> Cow<'_, str> {
    let transformer = regex::Regex::new(r#"@"([-a-z0-9_]+)""#).unwrap();
    transformer.replace_all(air_script, |c: &regex::Captures| {
        // no escaping needed for peer ID
        format!(r#""{}""#, at(&c[1]))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        asserts::ServiceDefinition, ephemeral::PeerId, services::results::ResultStore,
        AirScriptExecutor,
    };

    use air_test_utils::key_utils::at;
    use air_test_utils::prelude::*;

    use std::{
        collections::{HashMap, HashSet},
        iter::FromIterator,
    };

    impl ResultStore {
        pub fn into_inner(self) -> HashMap<usize, ServiceDefinition> {
            self.results.into_inner()
        }
    }

    #[tokio::test]
    async fn test_translate_null() {
        let network =
            Network::<NativeAirRunner>::new(std::iter::empty::<PeerId>(), vec![], <_>::default())
                .await;
        let transformed = TransformedAirScript::new("(null)", network, <_>::default())
            .await
            .unwrap();
        assert_eq!(&*transformed, "(null)");
    }

    #[tokio::test]
    async fn test_translate_call_no_result() {
        let network =
            Network::<NativeAirRunner>::new(std::iter::empty::<PeerId>(), vec![], <_>::default())
                .await;
        let script = r#"(call peer_id ("service_id" func) [])"#;
        let transformed = TransformedAirScript::new_unvalidated(script, network, <_>::default())
            .await
            .unwrap();
        assert_eq!(&*transformed, script);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_translate_call_no_string() {
        let network =
            Network::<NativeAirRunner>::new(std::iter::empty::<PeerId>(), vec![], <_>::default())
                .await;
        let script = r#"(call "peer_id" (service_id func) [])"#;
        let transformed = TransformedAirScript::new(script, network, <_>::default()).await;
        assert_eq!(transformed.as_deref(), Ok(script));
    }

    #[tokio::test]
    async fn test_translate_call_result() {
        let network =
            Network::<NativeAirRunner>::new(std::iter::empty::<PeerId>(), vec![], <_>::default())
                .await;
        let script = r#"(call "peer_id" ("service_id" func) []) ; ok = 42"#;
        let transformer =
            TransformedAirScript::new_unvalidated(script, network.clone(), <_>::default())
                .await
                .unwrap();

        let peer_id = at("peer_id");

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

    #[tokio::test]
    async fn test_translate_multiple_calls() {
        let script = r#"(seq
   (call peer_id ("service_id" func) [a 11]) ; ok={"test":"me"}
   (seq
      (call peer_id ("service_id" func) [b])
      (call peer_id ("service_id" func) [1]) ; ok=true
))"#;

        let network =
            Network::<NativeAirRunner>::new(std::iter::empty::<PeerId>(), vec![], <_>::default())
                .await;
        let transformed =
            TransformedAirScript::new_unvalidated(script, network.clone(), <_>::default())
                .await
                .unwrap();
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

    #[tokio::test]
    async fn test_peers() {
        // this script is not correct AIR, but our parser handles it
        let script = r#"(seq
   (call "peer_id1" ("service_id" func) [a 11]) ; ok={"test":"me"}
   (seq
      (call "peer_id2" ("service_id" func) [b])
      (call "peer_id1" ("service_id" func) [1]) ; ok=true
      (call peer_id3 ("service_id" func) [b])
      (canon "peer_id4" $stream #canon)
))"#;

        let network =
            Network::<NativeAirRunner>::new(std::iter::empty::<PeerId>(), vec![], <_>::default())
                .await;
        let t = TransformedAirScript::new_unvalidated(script, network.clone(), <_>::default())
            .await
            .unwrap();

        let peer_id1 = at("peer_id1");
        let peer_id2 = at("peer_id2");
        let peer_id4 = at("peer_id4");

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

    #[tokio::test]
    async fn test_at_transform() {
        let script = r#"(call "peer_id1" ("service_id" "func") [1 @"peer_id3"] x) ; ok={"test":@"peer_id2"}"#;

        let network =
            Network::<NativeAirRunner>::new(std::iter::empty::<PeerId>(), vec![], <_>::default())
                .await;
        let t = TransformedAirScript::new(script, network.clone(), <_>::default())
            .await
            .unwrap();

        let peer_id1 = at("peer_id1");
        let peer_id2 = at("peer_id2");
        let peer_id3 = at("peer_id3");

        let expected = format!(
            r#"(call "{peer_id1}" ("service_id..0" "func") [1 "{peer_id3}"] x)"#,
            peer_id1 = peer_id1,
            peer_id3 = peer_id3,
        );
        assert_eq!(*t, expected);

        let peer_name1 = "peer_id1";
        let exec = AirScriptExecutor::from_transformed_air_script(
            TestRunParameters::from_init_peer_id(peer_name1),
            <_>::default(),
            t,
        )
        .await
        .unwrap();
        let res = exec.execute_one(peer_name1).await.unwrap();
        assert_eq!(
            trace_from_result(&res),
            ExecutionTrace::from(vec![scalar!(
                json!({ "test": peer_id2 }),
                peer_name = peer_name1,
                service = "service_id..0",
                function = "func",
                args = vec![json!(1), json!(peer_id3)]
            )])
        );
    }
}
