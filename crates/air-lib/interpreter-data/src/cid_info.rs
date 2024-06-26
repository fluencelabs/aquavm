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

use crate::CidStore;
use crate::CidStoreVerificationError;

use crate::CanonCidAggregate;
use crate::CanonResultCidAggregate;
use crate::RawValue;
use crate::ServiceResultCidAggregate;

use polyplets::SecurityTetraplet;
use serde::Deserialize;
use serde::Serialize;

#[derive(
    Debug,
    Default,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    ::rkyv::Archive,
    ::rkyv::Serialize,
    ::rkyv::Deserialize,
)]
#[archive(check_bytes)]
pub struct CidInfo {
    /// Map CID to value.
    pub value_store: CidStore<RawValue>,

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
