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

#![allow(improper_ctypes)]
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

mod algebra;
mod boxed_value;
mod value_aggregate;

use std::rc::Rc;

pub use algebra::AIRIterableValueAlgebra;
pub use algebra::AIRValueAlgebra;
pub use algebra::ValueWithTetraplet;
pub use boxed_value::BoxedValue;
pub use boxed_value::ValueLambdaError;
pub use value_aggregate::ValueAggregate;

pub type RcBoxedValue = Rc<dyn BoxedValue>;
pub type RcSecurityTetraplet = Rc<polyplets::SecurityTetraplet>;
pub type RcSecurityTetraplets = Vec<RcSecurityTetraplet>;
