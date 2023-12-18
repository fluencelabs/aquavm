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

use crate::CidStore;
use crate::CidStoreVerificationError;

use crate::CanonCidAggregate;
use crate::CanonResultCidAggregate;
use crate::ServiceResultCidAggregate;
use crate::VmValue;

use polyplets::SecurityTetraplet;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CidInfo {
    /// Map CID to value.
    pub value_store: CidStore<VmValue>,

    /// Map CID to a tetraplet.
    pub tetraplet_store: CidStore<SecurityTetraplet>,

    /// Map CID to a canon element value.
    pub canon_element_store: CidStore<CanonCidAggregate>,

    /// Map CID to a canon result.
    pub canon_result_store: CidStore<CanonResultCidAggregate>,

    /// Map CID to a service result aggregate.
    pub service_result_store: CidStore<ServiceResultCidAggregate>,
}

impl CidInfo {
    #[tracing::instrument(skip_all)]
    pub fn verify(&self) -> Result<(), CidStoreVerificationError> {
        self.verify_value_store()?;
        self.verify_tetraplet_store()?;

        self.verify_canon_result_store()?;
        self.verify_service_result_store()?;

        Ok(())
    }

    fn verify_value_store(&self) -> Result<(), CidStoreVerificationError> {
        self.value_store.verify_raw_value()
    }

    fn verify_tetraplet_store(&self) -> Result<(), CidStoreVerificationError> {
        self.tetraplet_store.verify()
    }

    fn verify_service_result_store(&self) -> Result<(), CidStoreVerificationError> {
        self.service_result_store.verify()?;

        for (serv_cid, serv_result) in self.service_result_store.iter() {
            self.tetraplet_store
                .check_reference(serv_cid, &serv_result.tetraplet_cid)?;
            self.value_store
                .check_reference(serv_cid, &serv_result.value_cid)?;
        }
        Ok(())
    }

    fn verify_canon_result_store(&self) -> Result<(), CidStoreVerificationError> {
        self.canon_element_store.verify()?;
        self.canon_result_store.verify()?;

        for (canon_cid, canon_result) in self.canon_result_store.iter() {
            for val in &canon_result.values {
                self.canon_element_store.check_reference(canon_cid, val)?;
            }
            self.tetraplet_store
                .check_reference(canon_cid, &canon_result.tetraplet)?;
        }

        for (element_cid, canon_element) in self.canon_element_store.iter() {
            self.tetraplet_store
                .check_reference(element_cid, &canon_element.tetraplet)?;
            self.value_store
                .check_reference(element_cid, &canon_element.value)?;

            match &canon_element.provenance {
                crate::Provenance::Literal => {}
                crate::Provenance::ServiceResult { cid } => {
                    self.service_result_store
                        .check_reference(element_cid, cid)?;
                }
                crate::Provenance::Canon { cid } => {
                    self.canon_result_store.check_reference(element_cid, cid)?;
                }
            }
        }

        Ok(())
    }
}
