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

use std::collections::HashSet;

/// This is an intermediate realization of variable and iterable validator.
/// Now it checks them in a non strict way, by just tracking all met variables
/// set in call and iterables set in fold without any context. Then checking
/// all other values used inside call, fold, next instructions are checked to
/// be set in one of preceding calls or fold.
#[derive(Debug, Default, Clone)]
pub struct VariableValidator<'i> {
    met_variables: HashSet<&'i str>,
    met_iterable: HashSet<&'i str>,
}

impl<'i> VariableValidator<'i> {
    pub(super) fn new() -> Self {
        <_>::default()
    }

    pub(super) fn check_call<'err>(
        &self,
        call: &Call,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'i>, ParserError>>,
        span: Span,
    ) {
        self.check_peer_part(&call.peer_part, errors, span);
        self.check_function_part(&call.function_part, errors, span);
        self.check_args(&call.args, errors, span);
    }

    pub(super) fn check_fold<'err>(
        &self,
        fold: &Fold,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'i>, ParserError>>,
        span: Span,
    ) {
        self.check_iterable_value(&fold.iterable, errors, span);
    }

    pub(super) fn check_next<'err>(
        &self,
        next: &Next,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'i>, ParserError>>,
        span: Span,
    ) {
        if !self.met_iterable.contains(next.0) {
            add_to_errors(next.0.to_string(), errors, span, Token::Next);
        }
    }

    pub(super) fn met_variable(&mut self, variable: &'i str) {
        self.met_variables.insert(variable);
    }

    pub(super) fn met_iterable(&mut self, iterable: &'i str) {
        self.met_iterable.insert(iterable);
    }

    fn check_peer_part<'err>(
        &self,
        peer_part: &PeerPart<'_>,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'i>, ParserError>>,
        span: Span,
    ) {
        match peer_part {
            PeerPart::PeerPk(peer_pk) => self.check_instr_value(peer_pk, errors, span),
            PeerPart::PeerPkWithServiceId(peer_pk, service_id) => {
                self.check_instr_value(peer_pk, errors, span);
                self.check_instr_value(service_id, errors, span);
            }
        }
    }

    fn check_function_part<'err>(
        &self,
        function_part: &FunctionPart<'_>,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'i>, ParserError>>,
        span: Span,
    ) {
        match function_part {
            FunctionPart::FuncName(func_name) => self.check_instr_value(func_name, errors, span),
            FunctionPart::ServiceIdWithFuncName(service_id, func_name) => {
                self.check_instr_value(service_id, errors, span);
                self.check_instr_value(func_name, errors, span);
            }
        }
    }

    fn check_args<'err>(
        &self,
        args: &[CallInstrArgValue<'_>],
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'i>, ParserError>>,
        span: Span,
    ) {
        for arg in args {
            self.check_instr_arg_value(arg, errors, span);
        }
    }

    fn check_instr_value<'err>(
        &self,
        instr_value: &CallInstrValue,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'i>, ParserError>>,
        span: Span,
    ) {
        match instr_value {
            CallInstrValue::JsonPath { variable, .. } => {
                self.check_call_variable(variable, errors, span)
            }
            CallInstrValue::Variable(variable) => self.check_call_variable(variable, errors, span),
            _ => {}
        }
    }

    fn check_instr_arg_value<'err>(
        &self,
        instr_arg_value: &CallInstrArgValue,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'i>, ParserError>>,
        span: Span,
    ) {
        match instr_arg_value {
            CallInstrArgValue::JsonPath { variable, .. } => {
                self.check_call_variable(variable, errors, span)
            }
            CallInstrArgValue::Variable(variable) => {
                self.check_call_variable(variable, errors, span)
            }
            _ => {}
        }
    }

    fn check_iterable_value<'err>(
        &self,
        iterable_value: &IterableValue,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'i>, ParserError>>,
        span: Span,
    ) {
        match iterable_value {
            IterableValue::JsonPath { variable, .. } => {
                self.check_iterable_variable(variable, errors, span)
            }
            IterableValue::Variable(variable) => {
                self.check_iterable_variable(variable, errors, span)
            }
        }
    }

    fn check_call_variable<'err>(
        &self,
        variable_name: &str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'i>, ParserError>>,
        span: Span,
    ) {
        if !self.met_variables.contains(variable_name) && !self.met_iterable.contains(variable_name)
        {
            add_to_errors(variable_name.to_string(), errors, span, Token::Call);
        }
    }

    fn check_iterable_variable<'err>(
        &self,
        variable_name: &str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'i>, ParserError>>,
        span: Span,
    ) {
        if !self.met_iterable.contains(variable_name) {
            add_to_errors(variable_name.to_string(), errors, span, Token::Fold);
        }
    }
}

fn add_to_errors<'err, 'i>(
    variable_name: String,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'i>, ParserError>>,
    span: Span,
    token: Token<'i>,
) {
    let error = ParserError::UndefinedVariable(span.left, span.right, variable_name);
    let error = ParseError::User { error };

    let dropped_tokens = vec![(span.left, token, span.right)];

    let error = ErrorRecovery {
        error,
        dropped_tokens,
    };

    errors.push(error);
}
