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

/// Intermediate implementation of variable validator.
///
/// It is intended to track variables (i.e., those that were defined as
/// a result of the `call` instruction) and iterables (i.e., those X's defined
/// in a `fold array X` call).
///
/// Validator will catch any undefined variables or iterables and raise an error.
#[derive(Debug, Default, Clone)]
pub struct VariableValidator<'i> {
    /// Contains the most left definition of a variables met in call outputs.
    met_variables: HashMap<&'i str, Span>,

    /// Contains iterables met in fold iterables.
    met_iterators: MultiMap<&'i str, Span>,

    /// These variables from calls and folds haven't been resolved at the first meet.
    unresolved_variables: MultiMap<&'i str, Span>,

    /// Contains all met iterable in call and next, they will be resolved after the whole parsing
    /// due to the way how lalrpop work.
    unresolved_iterables: MultiMap<&'i str, Span>,
}

impl<'i> VariableValidator<'i> {
    pub(super) fn new() -> Self {
        <_>::default()
    }

    pub(super) fn met_call(&mut self, call: &Call<'i>, span: Span) {
        self.met_peer_part(&call.peer_part, span);
        self.met_function_part(&call.function_part, span);
        self.met_args(&call.args, span);
        self.met_call_output_definition(&call.output, span)
    }

    pub(super) fn met_match(&mut self, match_: &Match<'i>, span: Span) {
        self.met_matchable(&match_.left_value, span);
        self.met_matchable(&match_.right_value, span);
    }

    pub(super) fn met_mismatch(&mut self, mismatch: &MisMatch<'i>, span: Span) {
        self.met_matchable(&mismatch.left_value, span);
        self.met_matchable(&mismatch.right_value, span);
    }

    pub(super) fn met_fold_scalar(&mut self, fold: &FoldScalar<'i>, span: Span) {
        self.met_iterable_value(&fold.iterable, span);
        self.met_iterator_definition(fold.iterator, span);
    }

    pub(super) fn met_fold_stream(&mut self, fold: &FoldStream<'i>, span: Span) {
        self.met_variable(&AstVariable::Stream(fold.stream_name), span);
        self.met_iterator_definition(fold.iterator, span);
    }

    pub(super) fn met_next(&mut self, next: &Next<'i>, span: Span) {
        let iterable_name = next.0;
        // due to the right to left convolution in lalrpop, next will be met earlier than
        // a corresponding fold with the definition of this iterable, so they're just put
        // without a check for being already met
        self.unresolved_iterables.insert(iterable_name, span);
    }

    pub(super) fn finalize(&self) -> Vec<ErrorRecovery<usize, Token<'i>, ParserError>> {
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

    fn met_peer_part(&mut self, peer_part: &PeerPart<'i>, span: Span) {
        match peer_part {
            PeerPart::PeerPk(peer_pk) => self.met_instr_value(peer_pk, span),
            PeerPart::PeerPkWithServiceId(peer_pk, service_id) => {
                self.met_instr_value(peer_pk, span);
                self.met_instr_value(service_id, span);
            }
        }
    }

    fn met_function_part(&mut self, function_part: &FunctionPart<'i>, span: Span) {
        match function_part {
            FunctionPart::FuncName(func_name) => self.met_instr_value(func_name, span),
            FunctionPart::ServiceIdWithFuncName(service_id, func_name) => {
                self.met_instr_value(service_id, span);
                self.met_instr_value(func_name, span);
            }
        }
    }

    fn met_args(&mut self, args: &[CallInstrArgValue<'i>], span: Span) {
        for arg in args {
            self.met_instr_arg_value(arg, span);
        }
    }

    fn met_instr_value(&mut self, instr_value: &CallInstrValue<'i>, span: Span) {
        match instr_value {
            CallInstrValue::JsonPath { variable, .. } => self.met_variable(variable, span),
            CallInstrValue::Variable(variable) => self.met_variable(variable, span),
            _ => {}
        }
    }

    fn met_instr_arg_value(&mut self, instr_arg_value: &CallInstrArgValue<'i>, span: Span) {
        match instr_arg_value {
            CallInstrArgValue::JsonPath { variable, .. } => self.met_variable(variable, span),
            CallInstrArgValue::Variable(variable) => {
                // skipping streams here allows treating non-defined streams as empty arrays
                if let AstVariable::Scalar(_) = variable {
                    self.met_variable(variable, span)
                }
            }
            _ => {}
        }
    }

    fn met_variable(&mut self, variable: &AstVariable<'i>, span: Span) {
        let name = match variable {
            AstVariable::Scalar(name) => name,
            AstVariable::Stream(name) => name,
        };

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

        let found_spans = match self.met_iterators.get_vec(key) {
            Some(found_spans) => found_spans,
            None => return false,
        };

        found_spans.iter().any(|s| s < &key_span)
    }

    fn met_call_output_definition(&mut self, call_output: &CallOutputValue<'i>, span: Span) {
        use std::collections::hash_map::Entry;

        let variable_name = match call_output {
            CallOutputValue::Variable(AstVariable::Scalar(name)) => name,
            CallOutputValue::Variable(AstVariable::Stream(name)) => name,
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

    fn met_matchable(&mut self, matchable: &MatchableValue<'i>, span: Span) {
        match matchable {
            MatchableValue::InitPeerId
            | MatchableValue::Number(_)
            | MatchableValue::Boolean(_)
            | MatchableValue::Literal(_) => {}
            MatchableValue::Variable(variable) => self.met_variable(variable, span),
            MatchableValue::JsonPath { variable, .. } => self.met_variable(variable, span),
        }
    }

    /// Checks that multimap contains a span for given key such that provided span lies inside it.
    fn contains_iterable(&self, key: &str, key_span: Span) -> bool {
        let found_spans = match self.met_iterators.get_vec(key) {
            Some(found_spans) => found_spans,
            None => return false,
        };

        found_spans
            .iter()
            .any(|s| s.left < key_span.left && s.right > key_span.right)
    }

    fn met_iterable_value(&mut self, iterable_value: &IterableScalarValue<'i>, span: Span) {
        match iterable_value {
            IterableScalarValue::JsonPath { scalar_name, .. } => {
                self.met_variable(&AstVariable::Scalar(scalar_name), span)
            }
            IterableScalarValue::ScalarVariable(variable) => {
                self.met_variable(&AstVariable::Scalar(variable), span)
            }
        }
    }

    fn met_iterator_definition(&mut self, iterator: &'i str, span: Span) {
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
