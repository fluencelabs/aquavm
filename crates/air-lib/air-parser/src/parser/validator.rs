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

use super::lexer::AirPos;
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

    // This contains info about unssuported map key arguments used with ap instruction,
    // namely (key map ApArgument)
    unsupported_map_keys: Vec<(String, &'i str, Span)>,
}

impl<'i> VariableValidator<'i> {
    pub fn new() -> Self {
        <_>::default()
    }

    pub(super) fn met_call(&mut self, call: &Call<'i>, span: Span) {
        self.met_peer_id_resolvable_value(&call.triplet.peer_id, span);
        self.met_string_resolvable_value(&call.triplet.service_id, span);
        self.met_string_resolvable_value(&call.triplet.function_name, span);

        self.met_args(call.args.deref(), span);

        match &call.output {
            CallOutputValue::Scalar(scalar) => self.met_variable_name_definition(scalar.name, span),
            CallOutputValue::Stream(stream) => self.met_variable_name_definition(stream.name, span),
            CallOutputValue::None => {}
        };
    }

    // canon doesn't check stream to be defined, because empty streams are considered to be empty
    // and it is useful for code generation
    pub(super) fn met_canon(&mut self, canon: &Canon<'i>, span: Span) {
        self.met_variable_name_definition(canon.canon_stream.name, span);
    }

    pub(super) fn met_canon_map(&mut self, canon_map: &CanonMap<'i>, span: Span) {
        self.met_variable_name_definition(canon_map.canon_stream_map.name, span);
    }

    pub(super) fn met_canon_map_scalar(
        &mut self,
        canon_stream_map_scalar: &CanonStreamMapScalar<'i>,
        span: Span,
    ) {
        self.met_variable_name_definition(canon_stream_map_scalar.scalar.name, span);
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
            Scalar(scalar) => self.met_scalar(scalar, span),
            ScalarWithLambda(scalar) => self.met_scalar_wl(scalar, span),
            CanonStream(canon_stream) => self.met_canon_stream(canon_stream, span),
            EmptyArray => {}
        };
        self.met_iterator_definition(&fold.iterator, span);
    }

    pub(super) fn meet_fold_stream(&mut self, fold: &FoldStream<'i>, span: Span) {
        self.met_variable_name(fold.iterable.name, span);
        self.met_iterator_definition(&fold.iterator, span);
    }

    pub(super) fn meet_fold_stream_map(&mut self, fold: &FoldStreamMap<'i>, span: Span) {
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
            ApArgument::Scalar(scalar) => self.met_scalar(scalar, span),
            ApArgument::ScalarWithLambda(scalar) => self.met_scalar_wl(scalar, span),
            ApArgument::CanonStream(canon_stream) => self.met_canon_stream(canon_stream, span),
            ApArgument::CanonStreamWithLambda(canon_stream) => {
                self.met_canon_stream_wl(canon_stream, span)
            }
            ApArgument::CanonStreamMap(canon_stream_map) => {
                self.met_canon_stream_map(canon_stream_map.name, span)
            }
            ApArgument::CanonStreamMapWithLambda(canon_stream_map) => {
                self.met_canon_stream_map_wl(canon_stream_map, span)
            }
            ApArgument::CanonStreamMapIndex(canon_stream_map_index) => {
                self.met_canon_stream_map_index(canon_stream_map_index, span);
            }
        }
        self.met_variable_name_definition(ap.result.name(), span);
    }

    fn met_canon_stream_map_index(
        &mut self,
        canon_stream_map_index: &CanonStreamMapIndex<'i>,
        span: Span,
    ) {
        let map_name = canon_stream_map_index.canon_stream_map.name;
        let key = &canon_stream_map_index.index;
        self.met_canon_stream_map(map_name, span);
        self.met_map_key(key, span)
    }

    pub(super) fn met_ap_map(&mut self, ap_map: &ApMap<'i>, span: Span) {
        let key = &ap_map.key;
        self.met_map_key(key, span);
        self.met_variable_name_definition(ap_map.map.name, span);
    }

    fn met_map_key(&mut self, key: &StreamMapKeyClause<'i>, span: Span) {
        match key {
            StreamMapKeyClause::Literal(_) | StreamMapKeyClause::Int(_) => {}
            StreamMapKeyClause::Scalar(scalar) => self.met_scalar(scalar, span),
            StreamMapKeyClause::ScalarWithLambda(scalar) => self.met_scalar_wl(scalar, span),
            StreamMapKeyClause::CanonStreamWithLambda(stream) => {
                self.met_canon_stream_wl(stream, span)
            }
        }
    }

    pub(super) fn finalize(self) -> Vec<ErrorRecovery<AirPos, Token<'i>, ParserError>> {
        ValidatorErrorBuilder::new(self)
            .check_undefined_variables()
            .check_undefined_iterables()
            .check_multiple_next_in_fold()
            .check_new_on_iterators()
            .check_iterator_for_multiple_definitions()
            .check_for_unsupported_map_keys()
            .build()
    }

    fn met_args(&mut self, args: &[ImmutableValue<'i>], span: Span) {
        for arg in args {
            self.met_instr_arg_value(arg, span);
        }
    }

    fn met_peer_id_resolvable_value(
        &mut self,
        variable: &ResolvableToPeerIdVariable<'i>,
        span: Span,
    ) {
        use ResolvableToPeerIdVariable::*;

        match variable {
            InitPeerId | Literal(_) => {}
            Scalar(scalar) => self.met_scalar(scalar, span),
            ScalarWithLambda(scalar) => self.met_scalar_wl(scalar, span),
            CanonStreamWithLambda(stream) => self.met_canon_stream_wl(stream, span),
            CanonStreamMapIndex(canon_stream_map_index) => {
                self.met_canon_stream_map_index(canon_stream_map_index, span)
            }
        }
    }

    fn met_string_resolvable_value(
        &mut self,
        variable: &ResolvableToStringVariable<'i>,
        span: Span,
    ) {
        use ResolvableToStringVariable::*;

        match variable {
            Literal(_) => {}
            Scalar(scalar) => self.met_scalar(scalar, span),
            ScalarWithLambda(scalar) => self.met_scalar_wl(scalar, span),
            CanonStreamWithLambda(stream) => self.met_canon_stream_wl(stream, span),
            CanonStreamMapIndex(canon_stream_map_index) => {
                self.met_canon_stream_map_index(canon_stream_map_index, span)
            }
        }
    }

    fn met_instr_arg_value(&mut self, instr_arg_value: &ImmutableValue<'i>, span: Span) {
        use ImmutableValue::*;

        match instr_arg_value {
            InitPeerId | LastError(_) | Timestamp | TTL | Literal(_) | Number(_) | Boolean(_)
            | EmptyArray => {}
            Variable(variable) => self.met_variable(variable, span),
            VariableWithLambda(variable) => self.met_variable_wl(variable, span),
        }
    }

    fn met_variable(&mut self, variable: &ImmutableVariable<'i>, span: Span) {
        self.met_variable_name(variable.name(), span);
    }

    fn met_variable_wl(&mut self, variable: &ImmutableVariableWithLambda<'i>, span: Span) {
        self.met_variable_name(variable.name(), span);
        self.met_lambda(variable.lambda(), span);
    }

    fn met_scalar(&mut self, scalar: &Scalar<'i>, span: Span) {
        self.met_variable_name(scalar.name, span);
    }

    fn met_scalar_wl(&mut self, scalar: &ScalarWithLambda<'i>, span: Span) {
        self.met_variable_name(scalar.name, span);
        self.met_lambda(&scalar.lambda, span);
    }

    fn met_canon_stream(&mut self, stream: &CanonStream<'i>, span: Span) {
        self.met_variable_name(stream.name, span);
    }

    fn met_canon_stream_wl(&mut self, stream: &CanonStreamWithLambda<'i>, span: Span) {
        self.met_variable_name(stream.name, span);
        self.met_lambda(&stream.lambda, span);
    }

    fn met_canon_stream_map(&mut self, stream_map_name: &'i str, span: Span) {
        self.met_variable_name(stream_map_name, span);
    }

    fn met_canon_stream_map_wl(&mut self, stream_map: &CanonStreamMapWithLambda<'i>, span: Span) {
        self.met_variable_name(stream_map.name, span);
        self.met_lambda(&stream_map.lambda, span);
    }

    fn met_variable_name(&mut self, name: &'i str, span: Span) {
        if !self.contains_variable(name, span) {
            self.unresolved_variables.insert(name, span);
        }
    }

    fn met_lambda(&mut self, lambda: &LambdaAST<'i>, span: Span) {
        let accessors = match lambda {
            LambdaAST::ValuePath(accessors) => accessors,
            LambdaAST::Functor(_) => return,
        };

        for accessor in accessors.iter() {
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

    fn met_matchable(&mut self, matchable: &ImmutableValue<'i>, span: Span) {
        match matchable {
            ImmutableValue::InitPeerId
            | ImmutableValue::Timestamp
            | ImmutableValue::TTL
            | ImmutableValue::Number(_)
            | ImmutableValue::Boolean(_)
            | ImmutableValue::Literal(_)
            | ImmutableValue::LastError(_)
            | ImmutableValue::EmptyArray => {}
            ImmutableValue::Variable(variable) => self.met_variable(variable, span),
            ImmutableValue::VariableWithLambda(variable) => self.met_variable_wl(variable, span),
        }
    }

    fn met_iterator_definition(&mut self, iterator: &Scalar<'i>, span: Span) {
        self.met_iterator_definitions.insert(iterator.name, span);
    }
}

struct ValidatorErrorBuilder<'i> {
    errors: Vec<ErrorRecovery<AirPos, Token<'i>, ParserError>>,
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

    // Unsupported StreamMap keys check, e.g. Stream can not be a map key (ap ($stream "value") %map)
    fn check_for_unsupported_map_keys(mut self) -> Self {
        for (arg_key_type, ap_result_name, span) in self.validator.unsupported_map_keys.iter_mut() {
            let error = ParserError::unsupported_map_key_type(
                *span,
                arg_key_type.to_string(),
                *ap_result_name,
            );
            add_to_errors(&mut self.errors, *span, Token::New, error);
        }
        self
    }

    fn build(self) -> Vec<ErrorRecovery<AirPos, Token<'i>, ParserError>> {
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
    errors: &mut Vec<ErrorRecovery<AirPos, Token<'i>, ParserError>>,
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
