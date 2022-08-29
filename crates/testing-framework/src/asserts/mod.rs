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

pub(crate) mod parser;

use crate::services::JValue;

use air_test_utils::CallServiceResult;

use std::collections::HashMap;

/// Assert language structure: Assert.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssertionChain {
    pub(crate) assertions: Vec<AssertionBranch>,
}

impl AssertionChain {
    pub fn new(assertions: Vec<AssertionBranch>) -> Self {
        Self { assertions }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssertionBranch {
    pub(crate) conditions: Vec<Condition>,
    pub(crate) assertions: Vec<Assertion>,
    pub(crate) metas: Vec<Meta>,
    pub(crate) service_desc: Option<ServiceDesc>,
}

impl AssertionBranch {
    pub fn new(
        conditions: Vec<Condition>,
        assertions: Vec<Assertion>,
        metas: Vec<Meta>,
        service_desc: Option<ServiceDesc>,
    ) -> Self {
        Self {
            conditions,
            assertions,
            metas,
            service_desc,
        }
    }

    pub fn from_conditions(conditions: Vec<Condition>) -> Self {
        Self {
            conditions,
            assertions: vec![],
            metas: vec![],
            service_desc: None,
        }
    }

    pub fn from_assertions(assertions: Vec<Assertion>) -> Self {
        Self {
            conditions: vec![],
            assertions,
            metas: vec![],
            service_desc: None,
        }
    }

    pub fn from_metas(metas: Vec<Meta>) -> Self {
        Self {
            conditions: vec![],
            assertions: vec![],
            metas,
            service_desc: None,
        }
    }

    pub fn from_service_desc(service_desc: ServiceDesc) -> Self {
        Self {
            conditions: vec![],
            assertions: vec![],
            metas: vec![],
            service_desc: Some(service_desc),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Condition {
    Iter(u32),
    On(Equation),
    Filter(FuncName),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Assertion {
    Before(CallPlaceId),
    After(CallPlaceId),
    IsCalled(bool),
    Callback(FuncName),
    /// The call is in n-th AVM iteration.
    Seq(u32),
    /// assert that next_peer_pks contains this PK; a branch can contain several assertion
    /// of this type, all must hold.
    NextPeerPk(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Equation {
    Equal(String, String), // TODO
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Meta {
    Id(CallPlaceId),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ServiceDesc {
    Result(JValue),
    CallResult(CallServiceResult),
    SeqResult(HashMap<String, JValue>),
    // For example: echo, service, function, argument.N, fail
    Service(String),
}

pub type CallPlaceId = String;
pub type FuncName = String;
