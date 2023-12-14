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

#[macro_export]
macro_rules! derive_serialized_type {
    ($type_name:ident) => {
        #[derive(
            ::core::fmt::Debug,
            ::core::default::Default,
            ::serde::Serialize,
            ::serde::Deserialize,
            ::core::cmp::PartialEq,
            ::core::cmp::Eq,
            ::core::hash::Hash,
            ::core::clone::Clone,
        )]
        #[serde(transparent)]
        #[repr(transparent)]
        #[cfg_attr(
            any(feature = "marine", feature = "marine-abi"),
            ::marine_rs_sdk::marine
        )]
        pub struct $type_name {
            // it cannot be implemented as a tuple as marine doesn't support tuple structs
            #[serde(with = "serde_bytes")]
            value: ::std::vec::Vec<u8>,
        }

        impl ::core::convert::From<::std::vec::Vec<u8>> for $type_name {
            #[inline]
            fn from(value: ::std::vec::Vec<u8>) -> Self {
                Self { value }
            }
        }

        impl ::core::convert::From<$type_name> for ::std::vec::Vec<u8> {
            #[inline]
            fn from(value: $type_name) -> Self {
                value.value
            }
        }

        impl ::core::ops::Deref for $type_name {
            type Target = [u8];

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }
    };

    ($type_name:ident, $decl:meta) => {
        #[derive(
            ::core::fmt::Debug,
            ::core::default::Default,
            ::serde::Serialize,
            ::serde::Deserialize,
            ::core::cmp::PartialEq,
            ::core::cmp::Eq,
            ::core::hash::Hash,
            ::core::clone::Clone,
        )]
        #[serde(transparent)]
        #[repr(transparent)]
        #[$decl]
        pub struct $type_name {
            // it cannot be implemented as a tuple as marine doesn't support tuple structs
            value: ::std::vec::Vec<u8>,
        }

        impl ::core::convert::From<::std::vec::Vec<u8>> for $type_name {
            #[inline]
            fn from(value: ::std::vec::Vec<u8>) -> Self {
                Self { value }
            }
        }

        impl ::core::ops::Deref for $type_name {
            type Target = [u8];

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }
    };
}
