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

#![forbid(unsafe_code)]
#![warn(rust_2018_idioms)]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

mod verify;

pub use crate::verify::{verify_raw_value, verify_value, CidVerificationError};

use serde::Deserialize;
use serde::Serialize;
use thiserror::Error as ThisError;

use std::fmt;
use std::io::BufWriter;
use std::marker::PhantomData;
use std::rc::Rc;

/// Should-be-opaque type for the inner representation of CID.
/// It has to be serializable and Borsh-serializable, as well as implement `Debug`, `Eq`, `Ord`, `Hash` and similar
/// basic traits.  It is also can be unsized.
// you should be able to replace it with [u8], and most of the code will just work
pub type CidRef = str;

// there is no Rust multicodec crate with appropriate constants
const JSON_CODEC: u64 = 0x0200;

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
#[cfg_attr(
    feature = "rkyv",
    derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive(check_bytes))]
#[cfg_attr(feature = "rkyv", omit_bounds)] // TODO look close, may be a misuse
pub struct CID<T: ?Sized>(
    Rc<CidRef>,
    #[serde(skip)]
    #[cfg_attr(feature = "rkyv", with(::rkyv::with::Skip))]
    PhantomData<*const T>,
);

impl<T: ?Sized> CID<T> {
    pub fn new(cid: impl Into<Rc<CidRef>>) -> Self {
        Self(cid.into(), PhantomData)
    }

    pub fn get_inner(&self) -> Rc<CidRef> {
        self.0.clone()
    }
}

impl<T: ?Sized> std::convert::AsRef<CidRef> for CID<T> {
    fn as_ref(&self) -> &CidRef {
        &self.0
    }
}

impl<T: ?Sized> std::borrow::Borrow<CidRef> for CID<T> {
    fn borrow(&self) -> &CidRef {
        &self.0
    }
}

impl<T: ?Sized> Clone for CID<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1)
    }
}

impl<T: ?Sized> fmt::Debug for CID<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("CID").field(&self.0).finish()
    }
}

impl<Val> PartialEq for CID<Val> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<Val> Eq for CID<Val> {}

impl<Val> std::hash::Hash for CID<Val> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

impl<T: ?Sized> std::convert::TryFrom<&'_ CID<T>> for cid::Cid {
    type Error = cid::Error;

    fn try_from(value: &CID<T>) -> Result<Self, Self::Error> {
        use std::str::FromStr;

        cid::Cid::from_str(&value.0)
    }
}

#[derive(Debug, ThisError)]
pub enum CidCalculationError {
    #[error(transparent)]
    InvalidJson(#[from] serde_json::Error),
}

/// Calculate a CID of JSON-serialized value.
// TODO we might refactor this to `SerializationCodec` trait
// that both transform data to binary/text form (be it JSON, CBOR or something else)
// and produces CID too
pub fn value_to_json_cid<Val: Serialize + ?Sized>(
    value: &Val,
) -> Result<CID<Val>, CidCalculationError> {
    use cid::Cid;
    use multihash_codetable::{Code, MultihashDigest};

    let hash = value_json_hash::<blake3::Hasher, Val>(value)?;

    let digest = Code::Blake3_256
        .wrap(&hash)
        .expect("can't happen: incorrect hash length");

    let cid = Cid::new_v1(JSON_CODEC, digest);
    Ok(CID::new(cid.to_string()))
}

pub(crate) fn value_json_hash<D: digest::Digest + std::io::Write, Val: Serialize + ?Sized>(
    value: &Val,
) -> Result<Vec<u8>, serde_json::Error> {
    const HASH_BUFFER_SIZE: usize = 8 * 1024;

    let mut hasher = D::new();
    serde_json::to_writer(
        BufWriter::with_capacity(HASH_BUFFER_SIZE, &mut hasher),
        value,
    )?;
    let hash = hasher.finalize();

    Ok(hash.to_vec())
}

pub fn raw_value_to_json_cid<Val>(raw_value: impl AsRef<[u8]>) -> CID<Val> {
    use cid::Cid;
    use multihash_codetable::{Code, MultihashDigest};

    let hash = raw_value_hash::<blake3::Hasher>(raw_value);
    let digest = Code::Blake3_256
        .wrap(&hash)
        .expect("can't happen: incorrect hash length");

    let cid = Cid::new_v1(JSON_CODEC, digest);
    CID::new(cid.to_string())
}

pub(crate) fn raw_value_hash<D: digest::Digest>(raw_value: impl AsRef<[u8]>) -> Vec<u8> {
    let mut hasher = D::new();
    hasher.update(raw_value);
    let hash = hasher.finalize();

    hash.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_cid_default() {
        assert_eq!(
            value_to_json_cid(&json!("test")).unwrap(),
            CID::new("bagaaihrarcyykpv4oj7zwdbepczyfthxya4og7s2rwvrzolm5kg2eu5dz3xa")
        );
        assert_eq!(
            value_to_json_cid(&json!([1, 2, 3])).unwrap(),
            CID::new("bagaaihram6sitn77tquub77n2jzjgttrlwkverv44pv3gns6qghm6hx6d36a"),
        );
        assert_eq!(
            value_to_json_cid(&json!(1)).unwrap(),
            CID::new("bagaaihra2y55tkbgv6i4d7vdoglfuzhbd3ra6e7ennpvfrmzaejwmbntusdq"),
        );
        assert_eq!(
            value_to_json_cid(&json!({"key": 42})).unwrap(),
            CID::new("bagaaihracpzxhsrpviexa7k6glwdhyh3a4kvy6j7qlcqokzqbs3q424cmxyq"),
        );
    }
}
