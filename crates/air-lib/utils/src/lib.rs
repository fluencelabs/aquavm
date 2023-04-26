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

#[macro_export]
macro_rules! measure {
    (target: $target:expr, $expr:expr, $span:literal) => ({
        let span = tracing::span!(target=$target, $span);
        let _enter = span.enter();
        $expr
    });
    ($expr:expr, $level:expr, $span:literal, $($fields:tt)*) => ({
        let span = tracing::span!($level, $span, $($fields)*);
        let _enter = span.enter();
        $expr
    });
    ($expr:expr, $level:expr, $span:literal) => ({
        let span = tracing::span!($level, $span);
        let _enter = span.enter();
        $expr
    });
    ($expr:expr, $span:literal) => ({
        let span = tracing::span!($span);
        let _enter = span.enter();
        $expr
    });
}

#[macro_export]
macro_rules! auto_checked_add {
    [$type:ty] => {
        impl ::num_traits::CheckedAdd for $type {
            fn checked_add(&self, other: &Self) -> Option<Self> {
                self.0.checked_add(other.0).map(Self)
            }
        }
    };
}
