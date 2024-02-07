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

use crate::execution_step::RcSecurityTetraplet;
use crate::execution_step::ValueAggregate;
use crate::JValue;
use crate::UncatchableError;

use air_interpreter_cid::CID;
use air_interpreter_data::CanonCidAggregate;
use air_interpreter_data::CanonResultCidAggregate;
use air_interpreter_data::CidInfo;
use air_interpreter_data::CidTracker;
use air_interpreter_data::ServiceResultCidAggregate;
use air_interpreter_data::TracePos;
use polyplets::SecurityTetraplet;

use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct ExecutionCidState {
    pub value_tracker: CidTracker<JValue>,
    pub tetraplet_tracker: CidTracker<SecurityTetraplet>,
    pub canon_element_tracker: CidTracker<CanonCidAggregate>,
    pub canon_result_tracker: CidTracker<CanonResultCidAggregate>,
    pub service_result_agg_tracker: CidTracker<ServiceResultCidAggregate>,
}

impl ExecutionCidState {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn from_cid_info(prev_cid_info: CidInfo, current_cid_info: CidInfo) -> Self {
        let value_tracker = CidTracker::from_cid_stores(prev_cid_info.value_store, current_cid_info.value_store);
        let tetraplet_tracker =
            CidTracker::from_cid_stores(prev_cid_info.tetraplet_store, current_cid_info.tetraplet_store);
        let canon_element_tracker =
            CidTracker::from_cid_stores(prev_cid_info.canon_element_store, current_cid_info.canon_element_store);
        let canon_result_tracker =
            CidTracker::from_cid_stores(prev_cid_info.canon_result_store, current_cid_info.canon_result_store);
        let service_result_agg_tracker = CidTracker::from_cid_stores(
            prev_cid_info.service_result_store,
            current_cid_info.service_result_store,
        );

        Self {
            value_tracker,
            tetraplet_tracker,
            canon_element_tracker,
            canon_result_tracker,
            service_result_agg_tracker,
        }
    }

    pub fn track_service_result(
        &mut self,
        value: JValue,
        tetraplet: RcSecurityTetraplet,
        argument_hash: Rc<str>,
    ) -> Result<CID<ServiceResultCidAggregate>, UncatchableError> {
        let value_cid = self.value_tracker.track_value(value)?;
        let tetraplet_cid = self.tetraplet_tracker.track_value(tetraplet)?;
        let service_result_agg = ServiceResultCidAggregate::new(value_cid, argument_hash, tetraplet_cid);

        self.service_result_agg_tracker
            .track_value(service_result_agg)
            .map_err(UncatchableError::from)
    }

    pub(crate) fn track_canon_value(
        &mut self,
        canon_value: &ValueAggregate,
    ) -> Result<CID<CanonCidAggregate>, UncatchableError> {
        let value_cid = self.value_tracker.track_value(canon_value.get_result().clone())?;
        let tetraplet = self.tetraplet_tracker.track_value(canon_value.get_tetraplet())?;

        let canon_value_aggregate = CanonCidAggregate::new(value_cid, tetraplet, canon_value.get_provenance());
        self.canon_element_tracker
            .track_value(canon_value_aggregate)
            .map_err(UncatchableError::from)
    }

    pub(crate) fn get_value_by_cid(&self, cid: &CID<JValue>) -> Result<JValue, UncatchableError> {
        self.value_tracker
            .get(cid)
            .map(|v| (*v).clone())
            .ok_or_else(|| UncatchableError::ValueForCidNotFound("value", cid.get_inner()))
    }

    pub(crate) fn get_tetraplet_by_cid(
        &self,
        cid: &CID<SecurityTetraplet>,
    ) -> Result<RcSecurityTetraplet, UncatchableError> {
        self.tetraplet_tracker
            .get(cid)
            .ok_or_else(|| UncatchableError::ValueForCidNotFound("tetraplet", cid.get_inner()))
    }

    pub(crate) fn get_canon_value_by_cid(
        &self,
        cid: &CID<CanonCidAggregate>,
    ) -> Result<ValueAggregate, UncatchableError> {
        let canon_aggregate = self
            .canon_element_tracker
            .get(cid)
            .ok_or_else(|| UncatchableError::ValueForCidNotFound("canon aggregate", cid.get_inner()))?;
        let result = self.get_value_by_cid(&canon_aggregate.value)?;
        let tetraplet = self.get_tetraplet_by_cid(&canon_aggregate.tetraplet)?;

        let fake_trace_pos = TracePos::default();
        Ok(ValueAggregate::new(
            result,
            tetraplet,
            fake_trace_pos,
            canon_aggregate.provenance.clone(),
        ))
    }

    pub(crate) fn get_canon_result_by_cid(
        &self,
        cid: &CID<CanonResultCidAggregate>,
    ) -> Result<Rc<CanonResultCidAggregate>, UncatchableError> {
        self.canon_result_tracker
            .get(cid)
            .ok_or_else(|| UncatchableError::ValueForCidNotFound("canon result aggregate", cid.get_inner()))
    }

    pub(crate) fn get_service_result_agg_by_cid(
        &self,
        cid: &CID<ServiceResultCidAggregate>,
    ) -> Result<Rc<ServiceResultCidAggregate>, UncatchableError> {
        self.service_result_agg_tracker
            .get(cid)
            .ok_or_else(|| UncatchableError::ValueForCidNotFound("service result aggregate", cid.get_inner()))
    }

    pub(crate) fn resolve_service_info(
        &self,
        service_result_agg_cid: &CID<ServiceResultCidAggregate>,
    ) -> Result<ResolvedServiceInfo, UncatchableError> {
        let service_result_aggregate = self.get_service_result_agg_by_cid(service_result_agg_cid)?;
        let value = self.get_value_by_cid(&service_result_aggregate.value_cid)?;
        let tetraplet = self.get_tetraplet_by_cid(&service_result_aggregate.tetraplet_cid)?;

        Ok(ResolvedServiceInfo {
            value,
            tetraplet,
            service_result_aggregate,
        })
    }
}

pub(crate) struct ResolvedServiceInfo {
    pub(crate) value: JValue,
    pub(crate) tetraplet: RcSecurityTetraplet,
    pub(crate) service_result_aggregate: Rc<ServiceResultCidAggregate>,
}

impl From<ExecutionCidState> for CidInfo {
    fn from(value: ExecutionCidState) -> Self {
        Self {
            value_store: value.value_tracker.into(),
            tetraplet_store: value.tetraplet_tracker.into(),
            canon_element_store: value.canon_element_tracker.into(),
            canon_result_store: value.canon_result_tracker.into(),
            service_result_store: value.service_result_agg_tracker.into(),
        }
    }
}
