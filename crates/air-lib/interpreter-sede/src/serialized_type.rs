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
