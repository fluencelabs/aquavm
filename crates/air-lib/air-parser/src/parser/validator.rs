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

use crate::ast::*;

use crate::parser::lexer::Token;
use crate::parser::ParserError;
use crate::parser::Span;

use lalrpop_util::ErrorRecovery;
use lalrpop_util::ParseError;

use air_lambda_ast::ValueAccessor;
use multimap::MultiMap;
use std::collections::HashMap;
use std::ops::Deref;

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
    met_variable_definitions: HashMap<&'i str, Span>,

    /// Contains iterables met in fold iterables.
    met_iterator_definitions: MultiMap<&'i str, Span>,

    /// These variables from calls and folds haven't been resolved at the first meet.
    unresolved_variables: MultiMap<&'i str, Span>,

    /// Contains all met iterable in call and next, they will be resolved after the whole parsing
    /// due to the way how lalrpop work.
    unresolved_iterables: MultiMap<&'i str, Span>,

    /// Contains all names that should be checked that they are not iterators.
    check_for_non_iterators: Vec<(&'i str, Span)>,
}

impl<'i> VariableValidator<'i> {
    pub fn new() -> Self {
        <_>::default()
    }

    pub(super) fn met_call(&mut self, call: &Call<'i>, span: Span) {
        self.met_call_instr_value(&call.triplet.peer_pk, span);
        self.met_call_instr_value(&call.triplet.service_id, span);
        self.met_call_instr_value(&call.triplet.function_name, span);

        self.met_args(call.args.deref(), span);

        match &call.output {
            CallOutputValue::Variable(variable) => self.met_variable_definition(variable, span),
            CallOutputValue::None => {}
        };
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
        self.met_variable_name(fold.iterable.name, span);
        self.met_iterator_definition(&fold.iterator, span);
    }

    pub(super) fn meet_fold_stream(&mut self, fold: &FoldStream<'i>, span: Span) {
        self.met_variable_name(fold.iterable.name, span);
        self.met_iterator_definition(&fold.iterator, span);
    }

    pub(super) fn met_new(&mut self, new: &New<'i>, span: Span) {
        self.check_for_non_iterators
            .push((new.variable.name(), span));
        // new defines a new variable
        self.met_variable_definition(&new.variable, span);
    }

    pub(super) fn met_next(&mut self, next: &Next<'i>, span: Span) {
        let iterable_name = next.iterator.name;
        // due to the right to left convolution in lalrpop, a next instruction will be met earlier
        // than a corresponding fold instruction with the definition of this iterable, so they're
        // just put without a check for being already met
        self.unresolved_iterables.insert(iterable_name, span);
    }

    pub(super) fn met_ap(&mut self, ap: &Ap<'i>, span: Span) {
        match &ap.argument {
            ApArgument::Number(_)
            | ApArgument::InitPeerId
            | ApArgument::Boolean(_)
            | ApArgument::Literal(_)
            | ApArgument::EmptyArray
            | ApArgument::LastError(_) => {}
            ApArgument::Scalar(scalar) => {
                self.met_variable_wl(&VariableWithLambda::Scalar(scalar.clone()), span)
            }
        }
        self.met_variable_definition(&ap.result, span);
    }

    pub(super) fn finalize(self) -> Vec<ErrorRecovery<usize, Token<'i>, ParserError>> {
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

        for (name, span) in self.check_for_non_iterators.iter() {
            if self.contains_iterable(name, *span) {
                add_to_errors(*name, &mut errors, *span, Token::New);
            }
        }

        for (name, mut spans) in self.met_iterator_definitions.into_iter() {
            spans.sort();
            let mut prev_span: Option<Span> = None;
            for span in spans {
                match prev_span {
                    Some(prev_span) if prev_span.contains_span(span) => {
                        add_to_errors(name, &mut errors, span, Token::Fold)
                    }
                    Some(_) | None => prev_span = Some(span),
                }
            }
        }

        errors
    }

    fn met_args(&mut self, args: &[Value<'i>], span: Span) {
        for arg in args {
            self.met_instr_arg_value(arg, span);
        }
    }

    fn met_call_instr_value(&mut self, instr_value: &CallInstrValue<'i>, span: Span) {
        if let CallInstrValue::Variable(variable) = instr_value {
            self.met_variable_wl(variable, span);
        }
    }

    fn met_instr_arg_value(&mut self, instr_arg_value: &Value<'i>, span: Span) {
        if let Value::Variable(variable) = instr_arg_value {
            // skipping streams without lambdas here allows treating non-defined streams as empty arrays
            if let VariableWithLambda::Stream(stream) = variable {
                if stream.lambda.is_none() {
                    return;
                }
            }

            self.met_variable_wl(variable, span);
        }
    }

    fn met_variable_wl(&mut self, variable: &VariableWithLambda<'i>, span: Span) {
        self.met_variable_name(variable.name(), span);
        let lambda = match variable.lambda() {
            Some(lambda) => lambda,
            None => return,
        };

        for accessor in lambda.iter() {
            match accessor {
                &ValueAccessor::FieldAccessByScalar { scalar_name } => {
                    self.met_variable_name(scalar_name, span)
                }
                ValueAccessor::ArrayAccess { .. }
                | ValueAccessor::FieldAccessByName { .. }
                | ValueAccessor::Error => {}
            }
        }
    }

    fn met_variable_name(&mut self, name: &'i str, span: Span) {
        if !self.contains_variable(name, span) {
            self.unresolved_variables.insert(name, span);
        }
    }

    fn contains_variable(&self, key: &str, key_span: Span) -> bool {
        if let Some(found_span) = self.met_variable_definitions.get(key) {
            if found_span < &key_span {
                return true;
            }
        }

        let found_spans = match self.met_iterator_definitions.get_vec(key) {
            Some(found_spans) => found_spans,
            None => return false,
        };

        found_spans.iter().any(|s| s < &key_span)
    }

    fn met_variable_definition(&mut self, variable: &Variable<'i>, span: Span) {
        self.met_variable_name_definition(variable.name(), span);
    }

    fn met_variable_name_definition(&mut self, name: &'i str, span: Span) {
        use std::collections::hash_map::Entry;

        match self.met_variable_definitions.entry(name) {
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

    fn met_matchable(&mut self, matchable: &Value<'i>, span: Span) {
        match matchable {
            Value::InitPeerId
            | Value::Number(_)
            | Value::Boolean(_)
            | Value::Literal(_)
            | Value::LastError(_)
            | Value::EmptyArray => {}
            Value::Variable(variable) => self.met_variable_wl(variable, span),
        }
    }

    /// Checks that multimap contains a span for given key such that provided span lies inside it.
    fn contains_iterable(&self, key: &str, key_span: Span) -> bool {
        let found_spans = match self.met_iterator_definitions.get_vec(key) {
            Some(found_spans) => found_spans,
            None => return false,
        };

        found_spans
            .iter()
            .any(|s| s.left < key_span.left && s.right > key_span.right)
    }

    fn met_iterator_definition(&mut self, iterator: &Scalar<'i>, span: Span) {
        self.met_iterator_definitions.insert(iterator.name, span);
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
        Token::New => {
            ParserError::IteratorRestrictionNotAllowed(span.left, span.right, variable_name)
        }
        Token::Fold => ParserError::MultipleIterableValues(span.left, span.right, variable_name),
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
