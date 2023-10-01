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
    feature = "borsh",
    derive(::borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive(check_bytes))]
pub struct InterpreterData<Val = RawValueWrapper>
where
    Val: ::borsh::BorshSerialize + borsh::BorshDeserialize + Clone,
{
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
    pub cid_info: CidInfo<Val>,

    /// Signature store.
    ///
    /// Every peer signs call results and canon values it produced (all together), and stores the signatures
    /// in this store.
    pub signatures: SignatureStore,
}

impl From<InterpreterData<RawValueWrapper>> for InterpreterData<String> {
    fn from(val: InterpreterData<RawValueWrapper>) -> Self {
        InterpreterData::<String> {
            versions: val.versions,
            trace: val.trace,
            last_call_request_id: val.last_call_request_id,
            cid_info: val.cid_info.into(),
            signatures: val.signatures,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "borsh",
    derive(::borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive(check_bytes))]
pub struct Versions {
    /// Version of this data format.
    #[serde(rename = "version")] // for compatibility with versions <= 0.6.0
    #[cfg_attr(feature = "rkyv", with(WithStringVersion))]
    #[cfg_attr(feature = "borsh", borsh_skip)]
    pub data_version: Option<semver::Version>,

    /// Version of an interpreter produced this data.
    #[cfg_attr(feature = "rkyv", with(WithStringVersion))]
    #[cfg_attr(feature = "borsh", borsh_skip)]
    pub interpreter_version: Option<semver::Version>,
}

impl<Val> InterpreterData<Val>
where
    Val: ::borsh::BorshSerialize + borsh::BorshDeserialize + Clone,
{
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
        cid_info: CidInfo<Val>,
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
}

impl<Val> InterpreterData<Val>
where
    Val: ::borsh::BorshSerialize + borsh::BorshDeserialize + Clone + serde::de::DeserializeOwned,
{
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
            data_version: Some(crate::data_version().clone()),
            interpreter_version: Some(interpreter_version),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(
    feature = "borsh",
    derive(::borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive(check_bytes))]
pub struct CidInfo<Val>
where
    Val: ::borsh::BorshSerialize + borsh::BorshDeserialize + Clone,
{
    /// Map CID to value.
    pub value_store: CidStore<Val>,

    /// Map CID to a tetraplet.
    pub tetraplet_store: CidStore<SecurityTetraplet>,

    /// Map CID to a canon element value.
    pub canon_element_store: CidStore<CanonCidAggregate>,

    /// Map CID to a canon result.
    pub canon_result_store: CidStore<CanonResultCidAggregate>,

    /// Map CID to a service result aggregate.
    pub service_result_store: CidStore<ServiceResultCidAggregate>,
}

impl<Val> Default for CidInfo<Val>
where
    Val: ::borsh::BorshSerialize + borsh::BorshDeserialize + Clone,
{
    fn default() -> Self {
        Self {
            value_store: Default::default(),
            tetraplet_store: Default::default(),
            canon_element_store: Default::default(),
            canon_result_store: Default::default(),
            service_result_store: Default::default(),
        }
    }
}

impl From<CidInfo<RawValueWrapper>> for CidInfo<String> {
    fn from(source: CidInfo<RawValueWrapper>) -> Self {
        Self {
            value_store: source.value_store.into(),
            tetraplet_store: source.tetraplet_store,
            canon_element_store: source.canon_element_store,
            canon_result_store: source.canon_result_store,
            service_result_store: source.service_result_store,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[repr(transparent)]
#[serde(transparent)]
// So far, use boxes; then switch to unsized.
pub struct RawValueWrapper(
    #[with(WithRawJson)]
    #[serde(serialize_with = "raw_value_as_str")]
    Box<serde_json::value::RawValue>,
);

fn raw_value_as_str<S: serde::ser::Serializer>(
    val: &Box<serde_json::value::RawValue>,
    s: S,
) -> Result<S::Ok, S::Error> {
    s.serialize_str(val.get())
}

#[cfg(feature = "borsh")]
impl ::borsh::BorshSerialize for RawValueWrapper {
    #[inline]
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        ::borsh::BorshSerialize::serialize(self.0.get().as_bytes(), writer)
    }
}

#[cfg(feature = "borsh")]
impl ::borsh::BorshDeserialize for RawValueWrapper {
    #[inline]
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let data: Vec<u8> = borsh::BorshDeserialize::deserialize_reader(reader)?;
        Ok(Self(serde_json::from_slice(&data).unwrap()))
    }
}

impl RawValueWrapper {
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.get()
    }
}

impl PartialEq for RawValueWrapper {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.get() == other.0.get()
    }
}

impl Eq for RawValueWrapper {}

impl From<RawValueWrapper> for String {
    #[inline]
    fn from(val: RawValueWrapper) -> Self {
        let v: Box<str> = val.0.into();
        v.into()
    }
}

#[cfg(feature = "rkyv")]
impl<C: ?Sized + rkyv::validation::ArchiveContext> rkyv::CheckBytes<C> for ArchivedRawValueWrapper
where
    <C as rkyv::Fallible>::Error: std::error::Error,
{
    type Error = serde_json::Error;

    #[inline]
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
impl rkyv::with::ArchiveWith<Option<semver::Version>> for WithStringVersion {
    type Archived = rkyv::Archived<String>;

    type Resolver = rkyv::string::StringResolver;

    #[inline]
    unsafe fn resolve_with(
        field: &Option<semver::Version>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        use rkyv::Archive as _;

        let inner = field.as_ref().unwrap().to_string();
        inner.resolve(pos, resolver, out);
    }
}

#[cfg(feature = "rkyv")]
impl<S: rkyv::Fallible + rkyv::ser::Serializer + ?Sized>
    rkyv::with::SerializeWith<Option<semver::Version>, S> for WithStringVersion
{
    #[inline]
    fn serialize_with(
        field: &Option<semver::Version>,
        serializer: &mut S,
    ) -> Result<Self::Resolver, S::Error> {
        let inner = field.as_ref().unwrap().to_string();
        rkyv::Archived::<String>::serialize_from_str(&inner, serializer)
    }
}

#[cfg(feature = "rkyv")]
impl<D: rkyv::Fallible<Error = InterpreterDataDeserializerError> + ?Sized>
    rkyv::with::DeserializeWith<rkyv::Archived<String>, Option<semver::Version>, D>
    for WithStringVersion
{
    #[inline]
    fn deserialize_with(
        field: &rkyv::string::ArchivedString,
        _deserializer: &mut D,
    ) -> Result<Option<semver::Version>, <D as rkyv::Fallible>::Error> {
        Ok(Some(semver::Version::parse(&field.as_str())?))
    }
}

#[cfg(feature = "rkyv")]
pub struct WithRawJson;

#[cfg(feature = "rkyv")]
impl rkyv::with::ArchiveWith<Box<serde_json::value::RawValue>> for WithRawJson {
    type Archived = rkyv::Archived<Box<str>>;

    type Resolver = rkyv::Resolver<Box<str>>;

    #[inline]
    unsafe fn resolve_with(
        field: &Box<serde_json::value::RawValue>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        use rkyv::Archive as _;

        let inner: &Box<str> = std::mem::transmute(field);
        inner.resolve(pos, resolver, out)
    }
}

#[cfg(feature = "rkyv")]
impl<S: rkyv::Fallible + rkyv::ser::Serializer + ?Sized>
    rkyv::with::SerializeWith<Box<serde_json::value::RawValue>, S> for WithRawJson
{
    #[inline]
    fn serialize_with(
        field: &Box<serde_json::value::RawValue>,
        serializer: &mut S,
    ) -> Result<Self::Resolver, S::Error> {
        let inner = field.get();
        rkyv::Archived::<Box<str>>::serialize_from_ref(inner, serializer)
    }
}

#[cfg(feature = "rkyv")]
impl<D: rkyv::Fallible<Error = InterpreterDataDeserializerError> + ?Sized>
    rkyv::with::DeserializeWith<rkyv::boxed::ArchivedBox<str>, Box<serde_json::value::RawValue>, D>
    for WithRawJson
{
    #[inline]
    fn deserialize_with(
        field: &rkyv::boxed::ArchivedBox<str>,
        _deserializer: &mut D,
    ) -> Result<Box<serde_json::value::RawValue>, <D as rkyv::Fallible>::Error> {
        // Ok(serde_json::from_str(field.as_ref())?)
        let value: Box<str> = field.as_ref().into();
        // safe because JSON was validated on archive validation; otherwise, WithRawJson is not a safe API,
        // but for an example, is OK
        Ok(unsafe { std::mem::transmute(value) })
    }
}

#[cfg(feature = "rkyv")]
#[derive(Debug, thiserror::Error)]
pub enum InterpreterDataDeserializerError {
    #[error(transparent)]
    SharedMap(#[from] ::rkyv::de::deserializers::SharedDeserializeMapError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Version(#[from] semver::Error),
}

#[cfg(feature = "rkyv")]
#[derive(Debug, Default)]
/// Deserializer that has common error type for different types.
pub struct InterpreterDataDeserializer {
    shared: ::rkyv::de::deserializers::SharedDeserializeMap,
}

impl InterpreterDataDeserializer {
    #[inline]
    pub fn new() -> Self {
        Self {
            shared: ::rkyv::de::deserializers::SharedDeserializeMap::with_capacity(1024),
        }
    }

    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            shared: ::rkyv::de::deserializers::SharedDeserializeMap::with_capacity(cap),
        }
    }
}

#[cfg(feature = "rkyv")]
impl ::rkyv::Fallible for InterpreterDataDeserializer {
    type Error = InterpreterDataDeserializerError;
}

#[cfg(feature = "rkyv")]
impl ::rkyv::de::SharedDeserializeRegistry for InterpreterDataDeserializer {
    #[inline]
    fn get_shared_ptr(&mut self, ptr: *const u8) -> Option<&dyn rkyv::de::SharedPointer> {
        self.shared.get_shared_ptr(ptr)
    }

    #[inline]
    fn add_shared_ptr(
        &mut self,
        ptr: *const u8,
        shared: Box<dyn rkyv::de::SharedPointer>,
    ) -> Result<(), Self::Error> {
        Ok(self.shared.add_shared_ptr(ptr, shared)?)
    }
}
