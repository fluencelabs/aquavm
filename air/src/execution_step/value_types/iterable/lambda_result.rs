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

use super::Iterable;
use super::IterableItem;
use crate::execution_step::RcSecurityTetraplet;
use crate::foldable_next;
use crate::foldable_prev;
use crate::JValue;

use air_interpreter_data::Provenance;

/// Used for iterating over a result of applied to a JValue lambda.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IterableLambdaResult {
    pub(crate) jvalues: Vec<JValue>,
    pub(crate) tetraplet: RcSecurityTetraplet,
    pub(crate) provenance: Provenance,
    pub(crate) cursor: usize,
}

impl IterableLambdaResult {
    pub(crate) fn init(jvalues: Vec<JValue>, tetraplet: RcSecurityTetraplet, provenance: Provenance) -> Self {
        Self {
            jvalues,
            tetraplet,
            provenance,
            cursor: 0,
        }
    }
}

impl<'ctx> Iterable<'ctx> for IterableLambdaResult {
    type Item = IterableItem<'ctx>;

    fn next(&mut self) -> bool {
        foldable_next!(self, self.jvalues.len())
    }

    fn prev(&mut self) -> bool {
        foldable_prev!(self)
    }

    fn peek(&'ctx self) -> Option<Self::Item> {
        if self.jvalues.is_empty() {
            return None;
        }

        let jvalue = &self.jvalues[self.cursor];
        let mut tetraplet = (*self.tetraplet).clone();
        tetraplet.add_lens(&format!(".$.[{}]", self.cursor));
        let result = IterableItem::RefValue((jvalue, tetraplet.into(), 0.into(), self.provenance.clone()));

        Some(result)
    }

    fn len(&self) -> usize {
        self.jvalues.len()
    }
}
