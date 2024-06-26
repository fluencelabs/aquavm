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

#[derive(Clone, Copy, Debug)]
pub enum CheckInstructionKind<'names> {
    PivotalNext(&'names str),
    Merging,
    PopStack1,
    PopStack2,
    Replacing,
    ReplacingWithCheck(&'names str),
    Xoring,
    PopStack1ReplacingWithCheck(&'names str),
    Simple,
}

/// This machine is used to check that there are no instructions after next
/// in a fold block over stream and map, e.g.
/// (fold $iterable iterator
///  (seq
///    (next iterator)
///    (call  ...)  <- instruction after next
///  )
/// )
/// Note that the fold over scalar doesn't have this restriction.
#[derive(Clone, Debug)]
struct AfterNextCheckMachine<'name> {
    /// stack for the machine.
    stack: Vec<(CheckInstructionKind<'name>, Span)>,

    /// lalrpop parses AIR so that `next`
    /// is met before `fold``. At the moment when `next` is met
    /// it is impossible to tell whether it belongs to a stream/map fold or not.
    /// This field stores iterator name to span mapping to be used when `fold` is met.
    potentially_malformed_spans: HashMap<&'name str, Span>,

    /// This vector contains all spans where an instruction after next was met.
    malformed_spans: Vec<Span>,

    /// This flag disables the machine if invariants are broken, e.g. par/seq must see
    /// at least 2 instruction kinds in the stack.
    is_enabled: bool,
}

impl<'name> Default for AfterNextCheckMachine<'name> {
    fn default() -> Self {
        Self {
            stack: Default::default(),
            potentially_malformed_spans: Default::default(),
            malformed_spans: Default::default(),
            is_enabled: true,
        }
    }
}

impl<'name> AfterNextCheckMachine<'name> {
    fn disable(&mut self) {
        self.stack.clear();
        self.is_enabled = false;
    }

    fn malformed_spans_iter(&self) -> std::slice::Iter<Span> {
        self.malformed_spans.iter()
    }

    fn met_instruction_kind(&mut self, instr_kind: CheckInstructionKind<'name>, span: Span) {
        use CheckInstructionKind::*;

        if !self.is_enabled {
            return;
        }
        match instr_kind {
            CheckInstructionKind::Replacing => {
                self.process_replacing(span);
            }
            CheckInstructionKind::Xoring => {
                self.process_xoring(span);
            }
            CheckInstructionKind::Merging => {
                self.process_merging(span);
            }
            CheckInstructionKind::ReplacingWithCheck(iterator_name) => {
                self.replacing_with_check_common(iterator_name, span, &instr_kind);
            }
            PopStack1ReplacingWithCheck(iterator_name) => {
                self.stack.pop();
                self.replacing_with_check_common(iterator_name, span, &instr_kind);
            }
            PivotalNext(_) | Simple => self.stack.push((instr_kind, span)),
            PopStack1 => {
                self.stack.pop();
                self.stack.push((instr_kind, span))
            }
            PopStack2 => {
                self.stack.pop();
                self.stack.pop();
                self.stack.push((instr_kind, span))
            }
        }
    }

    fn process_replacing(&mut self, span: Span) {
        use CheckInstructionKind::*;

        let child = self.stack.pop();
        match child {
            Some((pattern_kind @ PivotalNext(_), ..)) => {
                self.stack.push((pattern_kind, span));
            }
            Some(_) => {
                self.stack.push((Replacing, span));
            }
            None => self.disable(),
        }
    }

    fn process_xoring(&mut self, span: Span) {
        use CheckInstructionKind::*;

        let right_branch = self.stack.pop();
        let left_branch = self.stack.pop();
        let left_right = left_branch.zip(right_branch);

        match left_right {
            Some(((PivotalNext(_), ..), (PivotalNext(_), ..))) => {
                // `xor` has `next` in both branches. It is impossible
                // to tell which branch should poped up.
                self.disable();
            }
            Some(((pattern_kind @ PivotalNext(_), ..), ..))
            | Some((_, (pattern_kind @ PivotalNext(_), _))) => {
                // potential failure but need to check when fold pops up.
                self.stack.push((pattern_kind, span));
            }
            Some(_) => {
                self.stack.push((Xoring, span));
            }
            _ => {
                // disable machine if Xoring invariant, namely there must be 2 kinds on a stack, is broken.
                self.disable();
            }
        }
    }

    fn process_merging(&mut self, span: Span) {
        use CheckInstructionKind::*;

        let right_branch = self.stack.pop();
        let left_branch = self.stack.pop();
        let left_right = left_branch.zip(right_branch);
        match left_right {
            Some((
                (pattern_kind @ PivotalNext(left_iterable), ..),
                (PivotalNext(right_iterable), ..),
            )) if left_iterable == right_iterable => {
                self.stack.push((pattern_kind, span));
            }
            Some(((PivotalNext(iterator), _), ..)) => {
                // potential failure but need to check when fold pops up.
                self.stack.push((Merging, span));
                self.potentially_malformed_spans
                    .entry(iterator)
                    .or_insert(span);
            }
            Some((_, (pattern_kind @ PivotalNext(_), ..))) => {
                self.stack.push((pattern_kind, span));
            }
            Some(_) => {
                self.stack.push((Merging, span));
            }
            _ => {
                // disable machine if Merging invariant, namely there must be 2 kinds on a stack, is broken.
                self.disable();
            }
        }
    }

    fn after_next_check(&mut self, iterable: &'name str) {
        let malformed_span = self.potentially_malformed_spans.get(iterable);
        if let Some(span) = malformed_span {
            self.malformed_spans.push(*span);
        }
    }

    fn replacing_with_check_common(
        &mut self,
        iterator_name: &'name str,
        span: Span,
        instr_kind: &CheckInstructionKind<'name>,
    ) {
        use CheckInstructionKind::*;

        let child = self.stack.pop();
        match child {
            Some((PivotalNext(pivotal_next_iterator_name), ..))
                if pivotal_next_iterator_name == iterator_name =>
            {
                self.after_next_check(iterator_name);
                self.potentially_malformed_spans.remove(iterator_name);
                self.stack.push((Simple, span));
            }
            Some((pattern_kind @ PivotalNext(_), ..)) => {
                self.after_next_check(iterator_name);
                self.stack.push((pattern_kind, span));
            }
            Some(_) => {
                self.after_next_check(iterator_name);
                self.stack.push((*instr_kind, span));
            }
            None => self.is_enabled = false,
        }
    }
}

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

    /// This contains info about unssuported map key arguments used with ap instruction,
    /// namely (key map ApArgument).
    unsupported_map_keys: Vec<(String, &'i str, Span)>,

    /// This vector contains all literal error codes used with fail.
    unsupported_literal_errcodes: Vec<(i64, Span)>,

    /// This machine is for after next instruction check.
    after_next_machine: AfterNextCheckMachine<'i>,
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

        self.met_simple_instr(span);

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
        self.met_simple_instr(span);
    }

    pub(super) fn met_canon_map(&mut self, canon_map: &CanonMap<'i>, span: Span) {
        self.met_variable_name_definition(canon_map.canon_stream_map.name, span);
        self.met_simple_instr(span);
    }

    pub(super) fn met_canon_map_scalar(
        &mut self,
        canon_stream_map_scalar: &CanonStreamMapScalar<'i>,
        span: Span,
    ) {
        self.met_variable_name_definition(canon_stream_map_scalar.scalar.name, span);

        self.met_simple_instr(span);
    }

    pub(super) fn met_match(&mut self, match_: &Match<'i>, span: Span) {
        self.met_matchable(&match_.left_value, span);
        self.met_matchable(&match_.right_value, span);
        self.met_replacing_instr(span);
    }

    pub(super) fn met_mismatch(&mut self, mismatch: &MisMatch<'i>, span: Span) {
        self.met_matchable(&mismatch.left_value, span);
        self.met_matchable(&mismatch.right_value, span);
        self.met_replacing_instr(span);
    }

    pub(super) fn met_fold_scalar(&mut self, fold: &FoldScalar<'i>, span: Span) {
        use FoldScalarIterable::*;

        match &fold.iterable {
            Scalar(scalar) => self.met_scalar(scalar, span),
            ScalarWithLambda(scalar) => self.met_scalar_wl(scalar, span),
            CanonStream(canon_stream) => self.met_canon_stream(canon_stream, span),
            CanonStreamMap(canon_stream_map) => self.met_canon_stream_map(canon_stream_map, span),
            CanonStreamMapWithLambda(canon_stream) => {
                self.met_canon_stream_map_wl(canon_stream, span)
            }
            EmptyArray => {}
        };
        self.met_iterator_definition(&fold.iterator, span);
        self.met_popstack_instr(fold, span);
    }

    pub(super) fn meet_fold_stream(&mut self, fold: &FoldStream<'i>, span: Span) {
        self.met_variable_name(fold.iterable.name, span);
        self.met_iterator_definition(&fold.iterator, span);

        match fold.last_instruction {
            Some(_) => self.met_popstack_replacing_with_check_instr(fold.iterator.name, span),
            None => self.met_replacing_with_check_instr(fold.iterator.name, span),
        }
    }

    pub(super) fn meet_fold_stream_map(&mut self, fold: &FoldStreamMap<'i>, span: Span) {
        self.met_variable_name(fold.iterable.name, span);
        self.met_iterator_definition(&fold.iterator, span);

        match fold.last_instruction {
            Some(_) => self.met_popstack_replacing_with_check_instr(fold.iterator.name, span),
            None => self.met_replacing_with_check_instr(fold.iterator.name, span),
        }
    }

    pub(super) fn met_new(&mut self, new: &New<'i>, span: Span) {
        self.not_iterators_candidates
            .push((new.argument.name(), span));
        // new defines a new variable
        self.met_variable_name_definition(new.argument.name(), span);
        self.met_replacing_instr(span);
    }

    pub(super) fn met_next(&mut self, next: &Next<'i>, span: Span) {
        let iterable_name = next.iterator.name;
        // due to the right to left convolution in lalrpop, a next instruction will be met earlier
        // than a corresponding fold instruction with the definition of this iterable, so they're
        // just put without a check for being already met
        self.unresolved_iterables.insert(iterable_name, span);
        self.multiple_next_candidates.insert(iterable_name, span);
        self.met_pivotalnext_instr(iterable_name, span);
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
            ApArgument::Error(_) => {}
            ApArgument::Scalar(scalar) => self.met_scalar(scalar, span),
            ApArgument::ScalarWithLambda(scalar) => self.met_scalar_wl(scalar, span),
            ApArgument::CanonStream(canon_stream) => self.met_canon_stream(canon_stream, span),
            ApArgument::CanonStreamWithLambda(canon_stream) => {
                self.met_canon_stream_wl(canon_stream, span)
            }
            ApArgument::CanonStreamMap(canon_stream_map) => {
                self.met_canon_stream_map(canon_stream_map, span)
            }
            ApArgument::CanonStreamMapWithLambda(canon_stream_map) => {
                self.met_canon_stream_map_wl(canon_stream_map, span)
            }
        }
        self.met_variable_name_definition(ap.result.name(), span);
        self.met_simple_instr(span);
    }

    pub(super) fn met_ap_map(&mut self, ap_map: &ApMap<'i>, span: Span) {
        let key = &ap_map.key;
        self.met_map_key(key, span);
        self.met_variable_name_definition(ap_map.map.name, span);
        self.met_simple_instr(span);
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

    pub(super) fn met_fail_literal(&mut self, fail: &Fail<'i>, span: Span) {
        match fail {
            Fail::Literal { ret_code, .. } if *ret_code == 0 => {
                self.unsupported_literal_errcodes.push((*ret_code, span))
            }
            _ => {}
        }
        self.met_simple_instr(span);
    }

    pub(super) fn met_merging_instr(&mut self, span: Span) {
        self.after_next_machine
            .met_instruction_kind(CheckInstructionKind::Merging, span);
    }

    pub(super) fn met_pivotalnext_instr(&mut self, iterable_name: &'i str, span: Span) {
        self.after_next_machine
            .met_instruction_kind(CheckInstructionKind::PivotalNext(iterable_name), span);
    }

    fn met_popstack_instr(&mut self, fold: &FoldScalar<'i>, span: Span) {
        let instruction_kind = match fold.last_instruction {
            Some(_) => CheckInstructionKind::PopStack2,
            None => CheckInstructionKind::PopStack1,
        };
        self.after_next_machine
            .met_instruction_kind(instruction_kind, span);
    }

    fn met_popstack_replacing_with_check_instr(&mut self, iterator_name: &'i str, span: Span) {
        self.after_next_machine.met_instruction_kind(
            CheckInstructionKind::PopStack1ReplacingWithCheck(iterator_name),
            span,
        );
    }

    pub(super) fn met_replacing_instr(&mut self, span: Span) {
        self.after_next_machine
            .met_instruction_kind(CheckInstructionKind::Replacing, span);
    }

    pub(super) fn met_replacing_with_check_instr(&mut self, iterator_name: &'i str, span: Span) {
        self.after_next_machine.met_instruction_kind(
            CheckInstructionKind::ReplacingWithCheck(iterator_name),
            span,
        );
    }

    pub(super) fn met_xoring_instr(&mut self, span: Span) {
        self.after_next_machine
            .met_instruction_kind(CheckInstructionKind::Xoring, span);
    }

    pub(super) fn met_simple_instr(&mut self, span: Span) {
        self.after_next_machine
            .met_instruction_kind(CheckInstructionKind::Simple, span);
    }

    pub(super) fn finalize(self) -> Vec<ErrorRecovery<AirPos, Token<'i>, ParserError>> {
        ValidatorErrorBuilder::new(self)
            .check_undefined_variables()
            .check_undefined_iterables()
            .check_multiple_next_in_fold()
            .check_new_on_iterators()
            .check_iterator_for_multiple_definitions()
            .check_for_unsupported_map_keys()
            .check_for_unsupported_literal_errcodes()
            .check_after_next_instr()
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
            CanonStreamMapWithLambda(canon_stream_map) => {
                self.met_canon_stream_map_wl(canon_stream_map, span)
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
            CanonStreamMapWithLambda(canon_stream_map) => {
                self.met_canon_stream_map_wl(canon_stream_map, span)
            }
        }
    }

    fn met_instr_arg_value(&mut self, instr_arg_value: &ImmutableValue<'i>, span: Span) {
        use ImmutableValue::*;

        match instr_arg_value {
            InitPeerId | Error(_) | LastError(_) | Timestamp | TTL | Literal(_) | Number(_)
            | Boolean(_) | EmptyArray => {}
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

    fn met_canon_stream_map(&mut self, canon_stream_map: &CanonStreamMap<'i>, span: Span) {
        self.met_variable_name(canon_stream_map.name, span);
    }

    fn met_canon_stream_wl(&mut self, stream: &CanonStreamWithLambda<'i>, span: Span) {
        self.met_variable_name(stream.name, span);
        self.met_lambda(&stream.lambda, span);
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
            | ImmutableValue::Error(_)
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

    fn check_for_unsupported_literal_errcodes(mut self) -> Self {
        for (_, span) in self.validator.unsupported_literal_errcodes.iter_mut() {
            let error = ParserError::unsupported_literal_errcodes(*span);
            add_to_errors(&mut self.errors, *span, Token::New, error);
        }
        self
    }

    fn check_after_next_instr(mut self) -> Self {
        for span in self.validator.after_next_machine.malformed_spans_iter() {
            let error = ParserError::fold_has_instruction_after_next(*span);
            add_to_errors(&mut self.errors, *span, Token::Next, error);
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
