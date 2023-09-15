/*
 * Copyright 2021 Fluence Labs Limited
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

use crate::cid_store::CidStore;
use crate::CanonCidAggregate;
use crate::CanonResultCidAggregate;
use crate::ExecutionTrace;
use crate::ServiceResultCidAggregate;

use air_interpreter_signatures::SignatureStore;
use air_utils::measure;
use polyplets::SecurityTetraplet;

use serde::Deserialize;
use serde::Serialize;

/// The AIR interpreter could be considered as a function
/// f(prev_data: InterpreterData, current_data: InterpreterData, ... ) -> (result_data: InterpreterData, ...).
/// This function receives prev and current data and produces a result data. All these data
/// have the following format.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive(check_bytes))]
pub struct InterpreterData {
    /// Versions of data and an interpreter produced this data.
    #[serde(flatten)]
    pub versions: Versions,

    /// Trace of AIR execution, which contains executed call, par, fold, and ap states.
    pub trace: ExecutionTrace,

    /// Last exposed to a peer call request id. All next call request ids will be bigger than this.
    #[serde(default)]
    #[serde(rename = "lcid")]
    pub last_call_request_id: u32,

    /// CID-to-somethings mappings.
    pub cid_info: CidInfo,

    /// Signature store.
    ///
    /// Every peer signs call results and canon values it produced (all together), and stores the signatures
    /// in this store.
    pub signatures: SignatureStore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive(check_bytes))]
pub struct Versions {
    /// Version of this data format.
    #[serde(rename = "version")] // for compatibility with versions <= 0.6.0
    #[cfg_attr(feature = "rkyv", with(WithStringVersion))]
    pub data_version: semver::Version,

    /// Version of an interpreter produced this data.
    #[cfg_attr(feature = "rkyv", with(WithStringVersion))]
    pub interpreter_version: semver::Version,
}

impl InterpreterData {
    pub fn new(interpreter_version: semver::Version) -> Self {
        let versions = Versions::new(interpreter_version);

        Self {
            versions,
            trace: ExecutionTrace::default(),
            last_call_request_id: 0,
            cid_info: <_>::default(),
            signatures: <_>::default(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_execution_result(
        trace: ExecutionTrace,
        cid_info: CidInfo,
        signatures: SignatureStore,
        last_call_request_id: u32,
        interpreter_version: semver::Version,
    ) -> Self {
        let versions = Versions::new(interpreter_version);

        Self {
            versions,
            trace,
            last_call_request_id,
            cid_info,
            signatures,
        }
    }

    /// Tries to de InterpreterData from slice according to the data version.
    pub fn try_from_slice(slice: &[u8]) -> Result<Self, serde_json::Error> {
        measure!(
            serde_json::from_slice(slice),
            tracing::Level::INFO,
            "serde_json::from_slice"
        )
    }

    /// Tries to de only versions part of interpreter data.
    pub fn try_get_versions(slice: &[u8]) -> Result<Versions, serde_json::Error> {
        serde_json::from_slice(slice)
    }
}

impl Versions {
    pub fn new(interpreter_version: semver::Version) -> Self {
        Self {
            data_version: crate::data_version().clone(),
            interpreter_version,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive(check_bytes))]
pub struct CidInfo {
    /// Map CID to value.
    pub value_store: CidStore<RawValueWrapper>,

    /// Map CID to a tetraplet.
    pub tetraplet_store: CidStore<SecurityTetraplet>,

    /// Map CID to a canon element value.
    pub canon_element_store: CidStore<CanonCidAggregate>,

    /// Map CID to a canon result.
    pub canon_result_store: CidStore<CanonResultCidAggregate>,

    /// Map CID to a service result aggregate.
    pub service_result_store: CidStore<ServiceResultCidAggregate>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[repr(transparent)]
#[serde(transparent)]
// So far, use boxes; then switch to unsized.
pub struct RawValueWrapper(#[with(WithRawJson)] Box<serde_json::value::RawValue>);

impl PartialEq for RawValueWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.get() == other.0.get()
    }
}

impl Eq for RawValueWrapper {}

#[cfg(feature = "rkyv")]
impl<C: ?Sized + rkyv::validation::ArchiveContext> rkyv::CheckBytes<C> for ArchivedRawValueWrapper
where
    <C as rkyv::Fallible>::Error: std::error::Error,
{
    type Error = serde_json::Error;

    unsafe fn check_bytes<'a>(
        value: *const Self,
        context: &mut C,
    ) -> Result<&'a Self, Self::Error> {
        let inner =
            <rkyv::Archived<Box<str>> as rkyv::CheckBytes<C>>::check_bytes(value as _, context)
                .expect("SOMETHING GOES WRONG");
        serde_json::from_slice::<&serde_json::value::RawValue>(&inner.as_bytes())
            .map(|_v| std::mem::transmute(inner))
    }
}

#[cfg(feature = "rkyv")]
pub struct WithStringVersion;

#[cfg(feature = "rkyv")]
impl rkyv::with::ArchiveWith<semver::Version> for WithStringVersion {
    type Archived = rkyv::Archived<String>;

    type Resolver = rkyv::string::StringResolver;

    unsafe fn resolve_with(
        field: &semver::Version,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        use rkyv::Archive as _;

        let inner = field.to_string();
        inner.resolve(pos, resolver, out);
    }
}

#[cfg(feature = "rkyv")]
impl<S: rkyv::Fallible + rkyv::ser::Serializer + ?Sized>
    rkyv::with::SerializeWith<semver::Version, S> for WithStringVersion
{
    fn serialize_with(
        field: &semver::Version,
        serializer: &mut S,
    ) -> Result<Self::Resolver, S::Error> {
        let inner = field.to_string();
        rkyv::string::ArchivedString::serialize_from_str(&inner, serializer)
    }
}

#[cfg(feature = "rkyv")]
impl<D: rkyv::Fallible<Error = semver::Error> + ?Sized>
    rkyv::with::DeserializeWith<rkyv::string::ArchivedString, semver::Version, D>
    for WithStringVersion
{
    fn deserialize_with(
        field: &rkyv::string::ArchivedString,
        _deserializer: &mut D,
    ) -> Result<semver::Version, <D as rkyv::Fallible>::Error> {
        semver::Version::parse(&field.as_str())
    }
}

#[cfg(feature = "rkyv")]
pub struct WithRawJson;

#[cfg(feature = "rkyv")]
impl rkyv::with::ArchiveWith<Box<serde_json::value::RawValue>> for WithRawJson {
    type Archived = rkyv::Archived<Box<str>>;

    type Resolver = rkyv::Resolver<Box<str>>;

    unsafe fn resolve_with(
        field: &Box<serde_json::value::RawValue>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        use rkyv::Archive as _;

        // Can be optimized with a cast
        let inner: &Box<str> = std::mem::transmute(field);
        inner.resolve(pos, resolver, out)
    }
}

#[cfg(feature = "rkyv")]
impl<S: rkyv::Fallible + rkyv::ser::Serializer + ?Sized>
    rkyv::with::SerializeWith<Box<serde_json::value::RawValue>, S> for WithRawJson
{
    fn serialize_with(
        field: &Box<serde_json::value::RawValue>,
        serializer: &mut S,
    ) -> Result<Self::Resolver, S::Error> {
        let inner = field.get();
        rkyv::Archived::<Box<str>>::serialize_from_ref(inner, serializer)
    }
}

#[cfg(feature = "rkyv")]
impl<D: rkyv::Fallible<Error = serde_json::Error> + ?Sized>
    rkyv::with::DeserializeWith<rkyv::string::ArchivedString, Box<serde_json::value::RawValue>, D>
    for WithRawJson
{
    fn deserialize_with(
        field: &rkyv::string::ArchivedString,
        _deserializer: &mut D,
    ) -> Result<Box<serde_json::value::RawValue>, <D as rkyv::Fallible>::Error> {
        serde_json::from_str(field.as_str())
    }
}
