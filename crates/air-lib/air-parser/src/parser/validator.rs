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

use air_lambda_ast::LambdaAST;
use air_lambda_ast::ValueAccessor;
use lalrpop_util::ErrorRecovery;
use lalrpop_util::ParseError;

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

    /// Contains iterators defined in a fold block.
    met_iterator_definitions: MultiMap<&'i str, Span>,

    /// These variables from calls and folds haven't been resolved at the first meet.
    unresolved_variables: MultiMap<&'i str, Span>,

    /// Contains all met iterable in call and next, they will be resolved after the whole parsing
    /// due to the way how lalrpop work.
    unresolved_iterables: MultiMap<&'i str, Span>,

    /// Contains all met iterable in call and next, they will be resolved after the whole parsing
    /// due to the way how lalrpop work.
    multiple_next_candidates: MultiMap<&'i str, Span>,

    /// Contains all names that should be checked that they are not iterators.
    not_iterators_candidates: Vec<(&'i str, Span)>,
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
            CallOutputValue::Scalar(scalar) => self.met_variable_name_definition(scalar.name, span),
            CallOutputValue::Stream(stream) => self.met_variable_name_definition(stream.name, span),
            CallOutputValue::None => {}
        };
    }

    pub(super) fn met_canon(&mut self, canon: &Canon<'i>, span: Span) {
        self.met_variable_name(canon.stream.name, span);
        self.met_variable_name_definition(canon.canon_stream.name, span);
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
        use FoldScalarIterable::*;

        match &fold.iterable {
            Scalar(variable) => {
                self.met_variable_name(variable.name, span);
                self.met_maybe_lambda(&variable.lambda, span);
            }
            CanonStream(canon_stream) => {
                self.met_variable_name(canon_stream.name, span);
            }
            EmptyArray => {}
        };
        self.met_iterator_definition(&fold.iterator, span);
    }

    pub(super) fn meet_fold_stream(&mut self, fold: &FoldStream<'i>, span: Span) {
        self.met_variable_name(fold.iterable.name, span);
        self.met_iterator_definition(&fold.iterator, span);
    }

    pub(super) fn met_new(&mut self, new: &New<'i>, span: Span) {
        self.not_iterators_candidates
            .push((new.argument.name(), span));
        // new defines a new variable
        self.met_variable_name_definition(new.argument.name(), span);
    }

    pub(super) fn met_next(&mut self, next: &Next<'i>, span: Span) {
        let iterable_name = next.iterator.name;
        // due to the right to left convolution in lalrpop, a next instruction will be met earlier
        // than a corresponding fold instruction with the definition of this iterable, so they're
        // just put without a check for being already met
        self.unresolved_iterables.insert(iterable_name, span);
        self.multiple_next_candidates.insert(iterable_name, span);
    }

    pub(super) fn met_ap(&mut self, ap: &Ap<'i>, span: Span) {
        match &ap.argument {
            ApArgument::Number(_)
            | ApArgument::Timestamp
            | ApArgument::TTL
            | ApArgument::InitPeerId
            | ApArgument::Boolean(_)
            | ApArgument::Literal(_)
            | ApArgument::EmptyArray
            | ApArgument::LastError(_) => {}
            ApArgument::Scalar(scalar) => {
                self.met_variable_name(scalar.name, span);
                self.met_maybe_lambda(&scalar.lambda, span);
            }
            ApArgument::CanonStream(canon_stream) => {
                self.met_variable_name(canon_stream.name, span);
                self.met_maybe_lambda(&canon_stream.lambda, span);
            }
        }
        self.met_variable_name_definition(ap.result.name(), span);
    }

    pub(super) fn finalize(self) -> Vec<ErrorRecovery<usize, Token<'i>, ParserError>> {
        ValidatorErrorBuilder::new(self)
            .check_undefined_variables()
            .check_undefined_iterables()
            .check_multiple_next_in_fold()
            .check_new_on_iterators()
            .check_iterator_for_multiple_definitions()
            .build()
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
        self.met_maybe_lambda(variable.lambda(), span);
    }

    fn met_variable_name(&mut self, name: &'i str, span: Span) {
        if !self.contains_variable(name, span) {
            self.unresolved_variables.insert(name, span);
        }
    }

    fn met_maybe_lambda(&mut self, lambda: &Option<LambdaAST<'i>>, span: Span) {
        let lambda = match lambda {
            Some(lambda) => lambda,
            None => return,
        };
        self.met_lambda(lambda, span)
    }

    fn met_lambda(&mut self, lambda: &LambdaAST<'i>, span: Span) {
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
            | Value::Timestamp
            | Value::TTL
            | Value::Number(_)
            | Value::Boolean(_)
            | Value::Literal(_)
            | Value::LastError(_)
            | Value::EmptyArray => {}
            Value::Variable(variable) => self.met_variable_wl(variable, span),
        }
    }

    fn met_iterator_definition(&mut self, iterator: &Scalar<'i>, span: Span) {
        self.met_iterator_definitions.insert(iterator.name, span);
    }
}

struct ValidatorErrorBuilder<'i> {
    errors: Vec<ErrorRecovery<usize, Token<'i>, ParserError>>,
    validator: VariableValidator<'i>,
}

impl<'i> ValidatorErrorBuilder<'i> {
    fn new(validator: VariableValidator<'i>) -> Self {
        let mut builder = Self {
            errors: Vec::new(),
            validator,
        };
        builder.sort_iterator_definitions();

        builder
    }

    fn sort_iterator_definitions(&mut self) {
        for (_, spans) in self.validator.met_iterator_definitions.iter_all_mut() {
            spans.sort()
        }
    }

    /// Check that all variables were defined.
    fn check_undefined_variables(mut self) -> Self {
        for (name, span) in self.validator.unresolved_variables.iter() {
            if !self.validator.contains_variable(name, *span) {
                let error = ParserError::undefined_variable(*span, *name);
                add_to_errors(&mut self.errors, *span, Token::Call, error);
            }
        }

        self
    }

    /// Check that all iterables in fold blocks were defined.
    fn check_undefined_iterables(mut self) -> Self {
        for (name, span) in self.validator.unresolved_iterables.iter() {
            if self.find_closest_fold_span(name, *span).is_none() {
                let error = ParserError::undefined_iterable(*span, *name);
                add_to_errors(&mut self.errors, *span, Token::New, error);
            }
        }

        self
    }

    /// Check that a fold block contains not more than one next with a corresponding iterator.
    fn check_multiple_next_in_fold(mut self) -> Self {
        // Approach used here is based on an assumption that each one iterator belongs only to one
        // fold block at any depth. This is checked by check_iterator_for_multiple_definitions and
        // allows to consider only one fold block where this variable was defined. Then a error
        // is produced if there are more than one suck block.
        for (name, spans) in self.validator.multiple_next_candidates.iter_all() {
            let mut collected_fold_spans = std::collections::HashSet::new();
            for span in spans {
                let current_span = match self.find_closest_fold_span(name, *span) {
                    Some(fold_span) => fold_span,
                    // this would be checked in check_undefined_iterables
                    None => {
                        continue;
                    }
                };

                if !collected_fold_spans.insert(current_span) {
                    let error = ParserError::multiple_next_in_fold(*span, *name);
                    add_to_errors(&mut self.errors, *span, Token::Next, error);
                }
            }
        }

        self
    }

    /// Check that a new operator wasn't applied to iterators.
    fn check_new_on_iterators(mut self) -> Self {
        for (name, span) in self.validator.not_iterators_candidates.iter() {
            if self.find_closest_fold_span(name, *span).is_some() {
                let error = ParserError::invalid_iterator_restriction(*span, *name);
                add_to_errors(&mut self.errors, *span, Token::New, error);
            }
        }

        self
    }

    /// Check that one iterator belongs to only one fold.
    /// F.e. such cases are prohibited
    /// (fold iterable_1 iterator
    ///     ...
    ///     (fold iterable_2 iterator
    ///         ...
    ///     )
    /// )
    fn check_iterator_for_multiple_definitions(mut self) -> Self {
        for (name, spans) in self.validator.met_iterator_definitions.iter_all_mut() {
            spans.sort();
            let mut prev_span: Option<Span> = None;
            for &span in spans.iter() {
                match prev_span {
                    Some(prev_span) if prev_span.contains_span(span) => {
                        let error = ParserError::multiple_iterables(span, *name);
                        add_to_errors(&mut self.errors, span, Token::Fold, error);
                    }
                    Some(_) | None => prev_span = Some(span),
                }
            }
        }

        self
    }

    fn build(self) -> Vec<ErrorRecovery<usize, Token<'i>, ParserError>> {
        self.errors
    }

    /// Checks that met_iterator_definitions contains a span for given key such that provided
    /// span lies inside it. This functions assumes that spans are sorted and that why returns
    /// the closest span in the list.
    fn find_closest_fold_span(&self, key: &str, key_span: Span) -> Option<Span> {
        let found_spans = match self.validator.met_iterator_definitions.get_vec(key) {
            Some(found_spans) => found_spans,
            None => return None,
        };

        found_spans
            .iter()
            .filter(|&s| s.contains_span(key_span))
            .last()
            .cloned()
    }
}

fn add_to_errors<'i>(
    errors: &mut Vec<ErrorRecovery<usize, Token<'i>, ParserError>>,
    span: Span,
    token: Token<'i>,
    error: ParserError,
) {
    let error = ParseError::User { error };

    let dropped_tokens = vec![(span.left, token, span.right)];

    let error = ErrorRecovery {
        error,
        dropped_tokens,
    };

    errors.push(error);
}
