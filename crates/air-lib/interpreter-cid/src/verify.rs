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

use crate::{value_json_hash, CidRef, CID, JSON_CODEC};

use fluence_blake3 as blake3;
use serde::Serialize;
use thiserror::Error as ThisError;

use std::convert::TryInto;
use std::rc::Rc;

#[derive(ThisError, Debug)]
pub enum CidVerificationError {
    #[error("Value mismatch in the {type_name:?} store for CID {cid_repr:?}")]
    ValueMismatch {
        // nb: type_name is std::any::type_name() result that may be inconsistent between the Rust compiler versions
        type_name: &'static str,
        cid_repr: Rc<CidRef>,
    },

    #[error("JSON error: {0}")]
    InvalidJson(#[from] serde_json::Error),
    #[error(transparent)]
    MalformedCid(#[from] cid::Error),
    #[error("unsupported CID codec: {0}")]
    UnsupportedCidCodec(u64),
    #[error("unsupported multihash code: {0}")]
    UnsupportedHashCode(u64),
}

pub fn verify_value<Val: Serialize>(
    cid: &CID<Val>,
    value: &Val,
) -> Result<(), CidVerificationError> {
    let real_cid: cid::Cid = cid.try_into()?;

    let codec = real_cid.codec();
    match codec {
        JSON_CODEC => verify_json_value(real_cid.hash(), value, cid),
        _ => Err(CidVerificationError::UnsupportedCidCodec(codec)),
    }
}

pub fn verify_raw_value<Val>(
    cid: &CID<Val>,
    raw_value: impl AsRef<[u8]>,
) -> Result<(), CidVerificationError> {
    use digest::Digest;
    use multihash_codetable::Code;

    let real_cid: cid::Cid = cid.try_into()?;

    let codec = real_cid.codec();
    // we insist ATM that raw values should be JSON-encoded, but
    // we do not validate that it is valid JSON data
    if codec != JSON_CODEC {
        return Err(CidVerificationError::UnsupportedCidCodec(codec));
    }

    let mhash = real_cid.hash();
    let raw_code = mhash.code();

    let code: Code = raw_code
        .try_into()
        .map_err(|_| CidVerificationError::UnsupportedHashCode(raw_code))?;

    let expected_hash = match code {
        Code::Sha2_256 => {
            let mut hasher = sha2::Sha256::new();
            hasher.update(raw_value);
            hasher.finalize().to_vec()
        }
        Code::Blake3_256 => {
            let mut hasher = blake3::Hasher::new();
            hasher.update(raw_value.as_ref());
            hasher.finalize().to_vec()
        }
        _ => return Err(CidVerificationError::UnsupportedHashCode(raw_code)),
    };
    // actually, multihash may contain less bytes than the full hash; to avoid abuse, we reject such multihashes
    if expected_hash == mhash.digest() {
        Ok(())
    } else {
        Err(CidVerificationError::ValueMismatch {
            type_name: std::any::type_name::<Val>(),
            cid_repr: cid.get_inner(),
        })
    }
}

fn verify_json_value<Val: Serialize>(
    mhash: &multihash_codetable::Multihash,
    value: &Val,
    cid: &CID<Val>,
) -> Result<(), CidVerificationError> {
    use multihash_codetable::Code;

    let raw_code = mhash.code();
    let code: Code = raw_code
        .try_into()
        .map_err(|_| CidVerificationError::UnsupportedHashCode(raw_code))?;

    let expected_hash = match code {
        Code::Sha2_256 => value_json_hash::<sha2::Sha256, Val>(value)?,
        Code::Blake3_256 => value_json_hash::<blake3::Hasher, Val>(value)?,
        _ => return Err(CidVerificationError::UnsupportedHashCode(raw_code)),
    };
    // actually, multihash may contain less bytes than the full hash; to avoid abuse, we reject such multihashes
    if expected_hash == mhash.digest() {
        Ok(())
    } else {
        Err(CidVerificationError::ValueMismatch {
            type_name: std::any::type_name::<Val>(),
            cid_repr: cid.get_inner(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use multihash::Multihash;
    use serde_json::json;

    #[test]
    fn test_verify_sha2_256() {
        verify_value(
            &CID::new("bagaaierajwlhumardpzj6dv2ahcerm3vyfrjwl7nahg7zq5o3eprwv6v3vpa"),
            &json!("test"),
        )
        .unwrap();
        verify_value(
            &CID::new("bagaaierauyk65lxcdxsrphpaqdpiymcszdnjaejyibv2ohbyyaziix35kt2a"),
            &json!([1, 2, 3]),
        )
        .unwrap();
        verify_value(
            &CID::new("bagaaieranodle477gt6odhllqbhp6wr7k5d23jhkuixr2soadzjn3n4hlnfq"),
            &json!(1),
        )
        .unwrap();
        verify_value(
            &CID::new("bagaaierad7lci6475zdrps4h6fmcpmqyknz5z6bw6p6tmpjkfyueavqw4kaq"),
            &json!({"key": 42}),
        )
        .unwrap();
    }

    #[test]
    fn test_verify_blake3() {
        verify_value(
            &CID::new("z3v8BBKBcZMDh6ANTaiT7PmfrBWbBmoVQvDxojXt1M4eczFDmhF"),
            &json!("test"),
        )
        .unwrap();
        verify_value(
            &CID::new("z3v8BBK9PYQwY7AGn9wb79BFTzSQiLALGAEmyqSYbCV2D9y8RLw"),
            &json!([1, 2, 3]),
        )
        .unwrap();
        verify_value(
            &CID::new("z3v8BBKGqF5gxukC6oU2EsSnTD7hBRorAabGJ8UDpNKneW7UApe"),
            &json!(1),
        )
        .unwrap();
        verify_value(
            &CID::new("z3v8BBK3kqxb39bomB9bJQ22a734aidv5C7QmjdfKiePgVjdQUQ"),
            &json!({"key": 42}),
        )
        .unwrap();
    }

    #[test]
    fn test_incorrect_value() {
        // CID of json!(1)
        let cid_1 = CID::new("bagaaieranodle477gt6odhllqbhp6wr7k5d23jhkuixr2soadzjn3n4hlnfq");
        let err = verify_value(&cid_1, &json!(2));
        assert!(
            matches!(err, Err(CidVerificationError::ValueMismatch { .. })),
            "{:?}",
            err
        );
    }

    #[test]
    fn test_verify_unknown_codec() {
        use std::str::FromStr;

        //  git raw object
        const GIT_RAW_CODEC: u64 = 0x78;
        // CID of json!(1)
        let cid_1 =
            cid::Cid::from_str("bagaaieranodle477gt6odhllqbhp6wr7k5d23jhkuixr2soadzjn3n4hlnfq")
                .unwrap();

        let unknown_format_cid =
            cid::Cid::new(cid::Version::V1, GIT_RAW_CODEC, cid_1.hash().clone()).unwrap();
        let unknown_format_cid = CID::new(unknown_format_cid.to_string());

        let err = verify_value(&unknown_format_cid, &json!(1));
        match err {
            Err(CidVerificationError::UnsupportedCidCodec(codec)) => {
                assert_eq!(codec, GIT_RAW_CODEC);
            }
            _ => panic!("wrong result: {:?}", err),
        }
    }

    #[test]
    fn test_verify_unknown_hasher() {
        use std::str::FromStr;

        const SHAKE_128_CODE: u64 = 0x18;

        let cid_1 =
            cid::Cid::from_str("bagaaieranodle477gt6odhllqbhp6wr7k5d23jhkuixr2soadzjn3n4hlnfq")
                .unwrap();

        let unknown_hasher_multihash =
            Multihash::wrap(SHAKE_128_CODE, cid_1.hash().digest()).unwrap();

        let unknown_hasher_cid =
            cid::Cid::new(cid::Version::V1, JSON_CODEC, unknown_hasher_multihash).unwrap();
        let unknown_hasher_cid = CID::new(unknown_hasher_cid.to_string());

        let err = verify_value(&unknown_hasher_cid, &json!(1));
        match err {
            Err(CidVerificationError::UnsupportedHashCode(code)) => {
                assert_eq!(code, SHAKE_128_CODE);
            }
            _ => panic!("wrong result: {:?}", err),
        }
    }

    #[test]
    fn test_verify_unsupported_hasher() {
        use multihash_codetable::Code;
        use std::str::FromStr;

        // we have no plan to support it, but it may change, and the test should be corrected
        let ripemd160_code: u64 = Code::Ripemd160.into();

        let cid_1 =
            cid::Cid::from_str("bagaaieranodle477gt6odhllqbhp6wr7k5d23jhkuixr2soadzjn3n4hlnfq")
                .unwrap();

        let unknown_hasher_multihash =
            Multihash::wrap(ripemd160_code, cid_1.hash().digest()).unwrap();

        let unknown_hasher_cid =
            cid::Cid::new(cid::Version::V1, JSON_CODEC, unknown_hasher_multihash).unwrap();
        let unknown_hasher_cid = CID::new(unknown_hasher_cid.to_string());

        let err = verify_value(&unknown_hasher_cid, &json!(1));
        match err {
            Err(CidVerificationError::UnsupportedHashCode(code)) => {
                assert_eq!(code, ripemd160_code);
            }
            _ => panic!("wrong result: {:?}", err),
        }
    }

    #[test]
    fn test_verify_garbage() {
        let garbage_cid = CID::new("garbage");
        let err = verify_value(&garbage_cid, &json!(1));
        assert!(
            matches!(
                err,
                Err(CidVerificationError::MalformedCid(cid::Error::ParsingError))
            ),
            "{:?}",
            err
        );
    }
}
