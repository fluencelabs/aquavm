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

use super::ast::*;

use crate::parser::lexer::Token;
use crate::parser::ParserError;
use crate::parser::Span;

use lalrpop_util::ErrorRecovery;
use lalrpop_util::ParseError;

use multimap::MultiMap;
use std::collections::HashMap;

/// This is an intermediate realization of variable and iterable validator.
/// Now it checks them in a non strict way, by just tracking all met variables
/// set in call and iterables set in fold without any context. Then checking
/// all other values used inside call, fold, next instructions are checked to
/// be set in one of preceding calls or fold.
#[derive(Debug, Default, Clone)]
pub struct VariableValidator<'i> {
    /// Contains the most left definition of a variables met in call outputs.
    met_variables: HashMap<&'i str, Span>,

    /// Contains iterables met in fold iterables.
    met_iterators: MultiMap<&'i str, Span>,

    /// These variables from calls and folds haven't been resolved at the first meet
    unresolved_variables: MultiMap<&'i str, Span>,

    /// Contains all met iterable in call and next, they will be resolved after the whole parsing
    /// due to the way how lalrpop work.
    unresolved_iterables: MultiMap<&'i str, Span>,
}

impl<'i> VariableValidator<'i> {
    pub(super) fn new() -> Self {
        <_>::default()
    }

    pub(super) fn meet_call(&mut self, call: &Call<'i>, span: Span) {
        self.meet_peer_part(&call.peer_part, span);
        self.meet_function_part(&call.function_part, span);
        self.meet_args(&call.args, span);
        self.meet_call_output_definition(&call.output, span)
    }

    pub(super) fn meet_fold(&mut self, fold: &Fold<'i>, span: Span) {
        self.meet_iterable_value(&fold.iterable, span);
        self.meet_iterator_definition(&fold.iterator, span);
    }

    pub(super) fn meet_next(&mut self, next: &Next<'i>, span: Span) {
        let iterable_name = next.0;
        // due to the right to left convolution in lalrpop, next will be met earlier than
        // a corresponding fold with the definition of this iterable, so they're just put
        // without a check for being already met
        self.unresolved_iterables.insert(iterable_name, span);
    }

    pub(super) fn finalize<'err>(&self) -> Vec<ErrorRecovery<usize, Token<'i>, ParserError>> {
        let mut errors = Vec::new();
        for (name, span) in self.unresolved_variables.iter() {
            if !self.contains_variable(name, *span) {
                add_to_errors(*name, &mut errors, *span, Token::Call);
            }
        }

        for (name, span) in self.unresolved_iterables.iter() {
            if !self.contains_iterable(name, *span) {
                add_to_errors(*name, &mut errors, *span, Token::Next);
            }
        }

        errors
    }

    fn meet_peer_part(&mut self, peer_part: &PeerPart<'i>, span: Span) {
        match peer_part {
            PeerPart::PeerPk(peer_pk) => self.meet_instr_value(peer_pk, span),
            PeerPart::PeerPkWithServiceId(peer_pk, service_id) => {
                self.meet_instr_value(peer_pk, span);
                self.meet_instr_value(service_id, span);
            }
        }
    }

    fn meet_function_part(&mut self, function_part: &FunctionPart<'i>, span: Span) {
        match function_part {
            FunctionPart::FuncName(func_name) => self.meet_instr_value(func_name, span),
            FunctionPart::ServiceIdWithFuncName(service_id, func_name) => {
                self.meet_instr_value(service_id, span);
                self.meet_instr_value(func_name, span);
            }
        }
    }

    fn meet_args(&mut self, args: &[CallInstrArgValue<'i>], span: Span) {
        for arg in args {
            self.meet_instr_arg_value(arg, span);
        }
    }

    fn meet_instr_value(&mut self, instr_value: &CallInstrValue<'i>, span: Span) {
        match instr_value {
            CallInstrValue::JsonPath { variable, .. } => self.meet_variable(variable, span),
            CallInstrValue::Variable(variable) => self.meet_variable(variable, span),
            _ => {}
        }
    }

    fn meet_instr_arg_value(&mut self, instr_arg_value: &CallInstrArgValue<'i>, span: Span) {
        match instr_arg_value {
            CallInstrArgValue::JsonPath { variable, .. } => self.meet_variable(variable, span),
            CallInstrArgValue::Variable(variable) => self.meet_variable(variable, span),
            _ => {}
        }
    }

    fn meet_variable(&mut self, name: &'i str, span: Span) {
        if !self.contains_variable(name, span) {
            self.unresolved_variables.insert(name, span);
        }
    }

    fn contains_variable(&self, key: &str, key_span: Span) -> bool {
        if let Some(found_span) = self.met_variables.get(key) {
            if found_span < &key_span {
                return true;
            }
        }

        let found_spans = match self.unresolved_iterables.get_vec(key) {
            Some(found_spans) => found_spans,
            None => return false,
        };

        found_spans.iter().any(|s| s < &key_span)
    }

    fn meet_call_output_definition(&mut self, call_output: &CallOutputValue<'i>, span: Span) {
        use std::collections::hash_map::Entry;

        let variable_name = match call_output {
            CallOutputValue::Scalar(variable) => variable,
            CallOutputValue::Accumulator(accumulator) => accumulator,
            CallOutputValue::None => return,
        };

        match self.met_variables.entry(variable_name) {
            Entry::Occupied(occupied) => {
                if occupied.get() > &span {
                    *occupied.into_mut() = span;
                }
            }
            Entry::Vacant(vacant) => {
                vacant.insert(span);
            }
        }
    }

    /// Checks that multimap contains a span for given key such that provided span lies inside it.
    fn contains_iterable(&self, key: &str, key_span: Span) -> bool {
        let found_spans = match self.unresolved_iterables.get_vec(key) {
            Some(found_spans) => found_spans,
            None => return false,
        };

        found_spans
            .iter()
            .any(|s| s.left < key_span.left && s.right > key_span.right)
    }

    fn meet_iterable_value(&mut self, iterable_value: &IterableValue<'i>, span: Span) {
        match iterable_value {
            IterableValue::JsonPath { variable, .. } => self.meet_variable(variable, span),
            IterableValue::Variable(variable) => self.meet_variable(variable, span),
        }
    }

    fn meet_iterator_definition(&mut self, iterator: &'i str, span: Span) {
        self.met_iterators.insert(iterator, span);
    }
}

use std::cmp::Ordering;
impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_min = std::cmp::min(self.left, self.right);
        let other_min = std::cmp::min(other.left, other.right);

        if self_min < other_min {
            Some(Ordering::Less)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}

fn add_to_errors<'err, 'i>(
    variable_name: impl Into<String>,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'i>, ParserError>>,
    span: Span,
    token: Token<'i>,
) {
    let variable_name = variable_name.into();
    let error = match token {
        Token::Next => ParserError::UndefinedIterable(span.left, span.right, variable_name),
        _ => ParserError::UndefinedVariable(span.left, span.right, variable_name),
    };
    let error = ParseError::User { error };

    let dropped_tokens = vec![(span.left, token, span.right)];

    let error = ErrorRecovery {
        error,
        dropped_tokens,
    };

    errors.push(error);
}
