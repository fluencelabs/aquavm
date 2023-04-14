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
use crate::execution_step::WithProvenance;
use crate::JValue;
use crate::UncatchableError;

use air_interpreter_cid::CidCalculationError;
use air_interpreter_cid::CID;
use air_interpreter_data::CanonCidAggregate;
use air_interpreter_data::CanonResultAggregate;
use air_interpreter_data::CidInfo;
use air_interpreter_data::CidTracker;
use air_interpreter_data::ServiceResultAggregate;
use air_interpreter_data::TracePos;
use polyplets::SecurityTetraplet;

use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct ExecutionCidState {
    pub value_tracker: CidTracker<JValue>,
    pub tetraplet_tracker: CidTracker<SecurityTetraplet>,
    pub canon_element_tracker: CidTracker<CanonCidAggregate>,
    pub canon_result_tracker: CidTracker<CanonResultAggregate>,
    pub service_result_agg_tracker: CidTracker<ServiceResultAggregate>,
}

impl ExecutionCidState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_value(
        &mut self,
        value: Rc<JValue>,
        tetraplet: RcSecurityTetraplet,
        argument_hash: Rc<str>,
    ) -> Result<Rc<CID<ServiceResultAggregate>>, CidCalculationError> {
        let value_cid = self.value_tracker.record_value(value)?;
        let tetraplet_cid = self.tetraplet_tracker.record_value(tetraplet)?;

        let service_result_agg = ServiceResultAggregate {
            value_cid,
            argument_hash,
            tetraplet_cid,
        };

        self.service_result_agg_tracker.record_value(service_result_agg)
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

    pub(crate) fn get_value_by_cid(&self, cid: &CID<JValue>) -> Result<Rc<JValue>, UncatchableError> {
        self.value_tracker
            .get(cid)
            .ok_or_else(|| UncatchableError::ValueForCidNotFound("value", cid.clone().into()))
    }

    pub(crate) fn get_tetraplet_by_cid(
        &self,
        cid: &CID<SecurityTetraplet>,
    ) -> Result<RcSecurityTetraplet, UncatchableError> {
        self.tetraplet_tracker
            .get(cid)
            .ok_or_else(|| UncatchableError::ValueForCidNotFound("tetraplet", cid.clone().into()))
    }

    pub(crate) fn get_canon_value_by_cid(
        &self,
        cid: &CID<CanonCidAggregate>,
    ) -> Result<WithProvenance<ValueAggregate>, UncatchableError> {
        let canon_aggregate = self
            .canon_element_tracker
            .get(cid)
            .ok_or_else(|| UncatchableError::ValueForCidNotFound("canon aggregate", cid.clone().into()))?;
        let result = self.get_value_by_cid(&canon_aggregate.value)?;
        let tetraplet = self.get_tetraplet_by_cid(&canon_aggregate.tetraplet)?;

        let fake_trace_pos = TracePos::default();
        Ok(WithProvenance::new(
            ValueAggregate {
                result,
                tetraplet,
                trace_pos: fake_trace_pos,
            },
            canon_aggregate.provenance.clone(),
        ))
    }

    pub(crate) fn get_canon_result_by_cid(
        &self,
        cid: &CID<CanonResultAggregate>,
    ) -> Result<Rc<CanonResultAggregate>, UncatchableError> {
        self.canon_result_tracker
            .get(cid)
            .ok_or_else(|| UncatchableError::ValueForCidNotFound("canon result aggregate", cid.clone().into()))
    }

    pub(crate) fn get_service_result_agg_by_cid(
        &self,
        cid: &CID<ServiceResultAggregate>,
    ) -> Result<Rc<ServiceResultAggregate>, UncatchableError> {
        self.service_result_agg_tracker
            .get(cid)
            .ok_or_else(|| UncatchableError::ValueForCidNotFound("service result aggregate", cid.clone().into()))
    }

    pub(crate) fn resolve_service_value(
        &self,
        service_result_agg_cid: &CID<ServiceResultAggregate>,
    ) -> Result<Rc<JValue>, UncatchableError> {
        let service_result_aggregate = self.get_service_result_agg_by_cid(service_result_agg_cid)?;
        self.get_value_by_cid(&service_result_aggregate.value_cid)
    }
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
