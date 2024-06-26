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

#[macro_export]
macro_rules! farewell_if_fail {
    ($cmd:expr, $raw_prev_data:expr, $soft_limits_triggering:expr) => {
        match $cmd {
            Ok(result) => result,
            // return the prev data in case of errors
            Err(error) => {
                return Err(farewell::from_uncatchable_error(
                    $raw_prev_data,
                    error,
                    $soft_limits_triggering,
                ))
            }
        };
    };
}
