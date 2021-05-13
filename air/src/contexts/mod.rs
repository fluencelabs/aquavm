/*
 * Copyright 2020 Fluence Labs Limited
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

mod execution_context;
mod execution_trace_context;

pub mod execution_trace {
    pub use super::execution_trace_context::CallResult;
    pub use super::execution_trace_context::ParResult;
    pub use super::execution_trace_context::ExecutedState;
    pub use super::execution_trace_context::ExecutionTrace;
    pub(crate) use super::execution_trace_context::ExecutionTraceCtx;
}

pub(crate) mod execution {
    pub(crate) use super::execution_context::AValue;
    pub(crate) use super::execution_context::ExecutionCtx;
    pub(crate) use super::execution_context::LastErrorDescriptor;
    pub(crate) use super::execution_context::ResolvedCallResult;
}
