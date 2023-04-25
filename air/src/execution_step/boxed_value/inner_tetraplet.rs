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

use air_interpreter_cid::CID;
use air_interpreter_data::ServiceResultCidAggregate;
use polyplets::{ResolvedTriplet, SecurityTetraplet};
use serde::{Deserialize, Serialize};

use std::rc::Rc;

#[derive(Debug, Serialize, Deserialize)]
pub struct LiteralTetraplet {
    // literal tetraplet has no lambda because literal + lambda is just a literal
    pub init_peer_id: Rc<str>,
}

impl LiteralTetraplet {
    pub fn new(init_peer_id: impl Into<Rc<str>>) -> Self {
        Self {
            init_peer_id: init_peer_id.into(),
        }
    }

    pub fn add_lambda(&self, _lambda: &str) -> Self {
        Self {
            init_peer_id: self.init_peer_id.clone(),
        }
    }
}

impl From<&LiteralTetraplet> for SecurityTetraplet {
    fn from(source: &LiteralTetraplet) -> Self {
        SecurityTetraplet::literal_tetraplet(Rc::as_ref(&source.init_peer_id))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ServiceResultTetraplet {
    pub tetraplet: Box<SecurityTetraplet>,
    #[serde(skip)] // to get consistent CID calculation
    pub cid_cache: Option<Rc<CID<ServiceResultCidAggregate>>>,
}

impl ServiceResultTetraplet {
    pub fn add_lambda(&self, lambda: &str) -> Self {
        let mut new_tetraplet = self.tetraplet.clone();
        new_tetraplet.add_lambda(lambda);

        Self {
            tetraplet: new_tetraplet,
            cid_cache: None, // invalidated
        }
    }
}

impl From<&ServiceResultTetraplet> for SecurityTetraplet {
    fn from(source: &ServiceResultTetraplet) -> Self {
        source.tetraplet.as_ref().clone()
    }
}

impl From<ResolvedTriplet> for ServiceResultTetraplet {
    fn from(value: ResolvedTriplet) -> Self {
        Self {
            tetraplet: Box::new(SecurityTetraplet::from_triplet(value)),
            cid_cache: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CanonTetraplet {
    pub peer_id: Rc<str>,
    pub lambda_path: Box<str>,
    #[serde(skip)] // to get consistent CID calculation
    pub cid_cache: Option<Rc<CID<ServiceResultCidAggregate>>>,
}

impl CanonTetraplet {
    pub fn new(peer_id: impl Into<Rc<str>>) -> Self {
        Self {
            peer_id: peer_id.into(),
            lambda_path: Default::default(),
            cid_cache: None,
        }
    }
    pub fn add_lambda(&self, lambda: &str) -> Self {
        // TODO some common lambda API is to be invented
        let lambda_path = format!("{}{}", self.lambda_path, lambda);

        Self {
            peer_id: self.peer_id.clone(),
            lambda_path: lambda_path.into(),
            cid_cache: None, // invalidated
        }
    }
}

impl From<&CanonTetraplet> for SecurityTetraplet {
    fn from(source: &CanonTetraplet) -> Self {
        SecurityTetraplet {
            peer_pk: Rc::as_ref(&source.peer_id).into(),
            json_path: source.lambda_path.as_ref().into(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum InnerTetraplet {
    Literal(LiteralTetraplet),
    ServiceResult(ServiceResultTetraplet),
    Canon(Box<CanonTetraplet>),
}

impl InnerTetraplet {
    pub fn literal_tetraplet(init_peer_id: impl Into<Rc<str>>) -> Self {
        Self::Literal(LiteralTetraplet::new(init_peer_id))
    }

    pub fn service_result_tetraplet(triplet: ResolvedTriplet) -> Self {
        Self::ServiceResult(triplet.into())
    }

    pub fn canon_tetraplet(peer_id: impl Into<Rc<str>>) -> Self {
        Self::Canon(Box::new(CanonTetraplet::new(peer_id)))
    }

    pub fn add_lambda(&self, lambda: &str) -> Self {
        use InnerTetraplet::*;

        match self {
            Literal(nested) => Literal(nested.add_lambda(lambda)),
            ServiceResult(nested) => ServiceResult(nested.add_lambda(lambda)),
            Canon(nested) => Canon(nested.add_lambda(lambda).into()),
        }
    }
}

impl From<&InnerTetraplet> for SecurityTetraplet {
    fn from(source: &InnerTetraplet) -> Self {
        use InnerTetraplet::*;

        match source {
            Literal(nested) => nested.into(),
            ServiceResult(nested) => nested.into(),
            Canon(nested) => nested.as_ref().into(),
        }
    }
}
