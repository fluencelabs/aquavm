/*
 * Copyright 2024 Fluence Labs Limited
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

/*
 * This file is based on serde_json crate by Erick Tryzelaar and David Tolnay
 * licensed under conditions of MIT License and Apache License, Version 2.0.
 */

#[cfg(not(feature = "preserve_order"))]
use std::collections::BTreeMap;
use std::rc::Rc;

// We only use our own error type; no need for From conversions provided by the
// standard library's try! macro. This reduces lines of LLVM IR by 4%.
macro_rules! tri {
    ($e:expr $(,)?) => {
        match $e {
            core::result::Result::Ok(val) => val,
            core::result::Result::Err(err) => return core::result::Result::Err(err),
        }
    };
}

mod number;
mod value;

pub use number::Number;
pub use value::JValue;

#[cfg(not(feature = "preserve_order"))]
pub type Map<K, V> = BTreeMap<K, V>;

#[cfg(feature = "preserve_order")]
pub type Map<K, V> = indexmap::IndexMap<K, V>;

// it is memory- and CPU-wise more effective than a string
pub type JsonString = Rc<str>;
