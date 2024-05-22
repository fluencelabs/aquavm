/*
 * Copyright 2024 Fluence DAO
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

use std::fmt;

use air_parser::ast;

/// A virtual 'if-else' instruction.
#[derive(Debug)]
pub(crate) struct IfElse<'i> {
    pub(crate) condition: Condition<'i>,
    pub(crate) then_body: &'i ast::Instruction<'i>,
    pub(crate) else_body: Option<&'i ast::Instruction<'i>>,
}

#[derive(Debug)]
pub(crate) enum Condition<'i> {
    Match {
        left_value: &'i ast::ImmutableValue<'i>,
        right_value: &'i ast::ImmutableValue<'i>,
    },
    Mismatch {
        left_value: &'i ast::ImmutableValue<'i>,
        right_value: &'i ast::ImmutableValue<'i>,
    },
}

impl fmt::Display for Condition<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Condition::Match {
                left_value,
                right_value,
            } => write!(f, "{left_value} == {right_value}"),
            Condition::Mismatch {
                left_value,
                right_value,
            } => write!(f, "{left_value} != {right_value}"),
        }
    }
}

/// Try to parse an instruction and its nested elements as a virtual `if else` instruction.
///
/// For example:
/// ```clojure
/// (todo)
/// ```
/// is parsed as a virtual instruction
/// ```python
/// if expr1 == expr2:
///    st1
/// else:
///    st2
/// ```
pub(crate) fn try_if_else<'i>(root_new: &'i ast::New<'i>) -> Option<IfElse<'i>> {
    use ast::Instruction::New;

    let root_scalar = if let ast::NewArgument::Scalar(root_scalar) = &root_new.argument {
        root_scalar
    } else {
        return None;
    };

    let nested1_new = if let New(netsted1_new) = &root_new.instruction {
        netsted1_new
    } else {
        return None;
    };

    let nested1_scalar = if let ast::NewArgument::Scalar(nested1_scalar) = &nested1_new.argument {
        nested1_scalar
    } else {
        return None;
    };

    let nested2_new = if let New(netsted2_new) = &nested1_new.instruction {
        netsted2_new
    } else {
        return None;
    };

    let nested2_scalar = if let ast::NewArgument::Scalar(nested2_scalar) = &nested2_new.argument {
        nested2_scalar
    } else {
        return None;
    };

    if let ast::Instruction::Xor(box ast::Xor(
        expected_match_or_mismatch,
        ast::Instruction::Seq(error_handling),
    )) = &nested2_new.instruction
    {
        let (condition, then_body) = match expected_match_or_mismatch {
            ast::Instruction::Match(match_) => {
                let condition = Condition::Match {
                    left_value: &match_.left_value,
                    right_value: &match_.right_value,
                };
                let then_branch = &match_.instruction;
                (condition, then_branch)
            }
            ast::Instruction::MisMatch(mismatch) => {
                let condition = Condition::Mismatch {
                    left_value: &mismatch.left_value,
                    right_value: &mismatch.right_value,
                };
                let then_branch = &mismatch.instruction;
                (condition, then_branch)
            }
            _ => return None,
        };

        let if_else_error = root_scalar;
        let else_error = nested1_scalar;
        let if_error = nested2_scalar;
        let expected_error_handling_xor = || {
            let air_script = format!(
                // the new instruction are needed to pass a parser validator; they are removed
                r#"(new {if_else_error}
           (new {else_error}
           (new {if_error}
           ; real code
           (seq
             (seq
               (ap :error: {else_error})
               (xor
                 (match :error:.$.error_code 10001
                   (ap {if_error} {if_else_error})
                 )
                 (ap {else_error} {if_else_error})
               )
            )
            (fail {if_else_error})
      ))))"#
            );
            // parse and convert to string to get consistent whitespace
            let expected_tree = air_parser::parse(&air_script).expect("invalid internal AIR");
            let expected_tree = pop_new_from_tree(expected_tree, 3);
            expected_tree.to_string()
        };
        // TODO return a Result here
        if let Some(else_body) =
            validate_error_handling(error_handling, if_error, expected_error_handling_xor)
        {
            // todo!("check free variables");
            Some(IfElse {
                condition,
                then_body,
                else_body: Some(else_body),
            })
        } else {
            None
        }
    } else {
        None
    }
}

pub(crate) fn try_if_then<'i>(root_new: &'i ast::New<'i>) -> Option<IfElse<'i>> {
    let root_scalar = if let ast::NewArgument::Scalar(root_scalar) = &root_new.argument {
        root_scalar
    } else {
        return None;
    };

    if let ast::Instruction::Xor(box ast::Xor(
        expected_match_or_mismatch,
        ast::Instruction::Seq(error_handling),
    )) = &root_new.instruction
    {
        let (condition, then_body) = match expected_match_or_mismatch {
            ast::Instruction::Match(match_) => {
                let condition = Condition::Match {
                    left_value: &match_.left_value,
                    right_value: &match_.right_value,
                };
                let then_branch = &match_.instruction;
                (condition, then_branch)
            }
            ast::Instruction::MisMatch(mismatch) => {
                let condition = Condition::Mismatch {
                    left_value: &mismatch.left_value,
                    right_value: &mismatch.right_value,
                };
                let then_branch = &mismatch.instruction;
                (condition, then_branch)
            }
            _ => return None,
        };

        let if_error = root_scalar;
        let expected_error_handling_xor = || {
            let air_script = format!(
                // the new instruction are needed to pass a parser validator; they are removed
                r#"(new {if_error}
                       (fail {if_error})
                 )"#
            );
            // parse and convert to string to get consistent whitespace
            let expected_tree = air_parser::parse(&air_script).expect("invalid internal AIR");
            let expected_tree = pop_new_from_tree(expected_tree, 1);
            expected_tree.to_string()
        };
        if let Some(ast::Instruction::Null(ast::Null)) =
            validate_error_handling(error_handling, root_scalar, expected_error_handling_xor)
        {
            // todo!("check free variables");
            Some(IfElse {
                condition,
                then_body,
                else_body: None,
            })
        } else {
            None
        }
    } else {
        None
    }
}

fn validate_error_handling<'i>(
    root: &'i ast::Seq<'i>,
    if_error: &ast::Scalar<'_>,
    expected_error_handling_xor: impl FnOnce() -> String,
) -> Option<&'i ast::Instruction<'i>> {
    if validate_error_handling_ap(&root.0, if_error) {
        validate_error_handling_xor(&root.1, expected_error_handling_xor)
    } else {
        None
    }
}

fn validate_error_handling_xor<'i>(
    instruction: &'i ast::Instruction<'i>,
    expected_error_handling_xor: impl FnOnce() -> String,
) -> Option<&'i ast::Instruction<'i>> {
    if let ast::Instruction::Xor(xor) = instruction {
        if !validate_error_handling_xor_second(&xor.1, expected_error_handling_xor) {
            return None;
        }
        if let ast::Instruction::Match(match_) = &xor.0 {
            // check arguments: match :error:.$.error_code 10001
            let lambda =
                air_lambda_parser::parse(".$.error_code").expect("invalid internal lambda");
            let expected_left =
                ast::ImmutableValue::Error(ast::InstructionErrorAST::new(Some(lambda)));
            let expected_right = ast::ImmutableValue::Number(ast::Number::Int(10001));
            if match_.left_value == expected_left && match_.right_value == expected_right {
                return Some(&match_.instruction);
            }
        }
    }
    None
}

fn validate_error_handling_ap(
    instruction: &ast::Instruction<'_>,
    if_error: &ast::Scalar<'_>,
) -> bool {
    match instruction {
        ast::Instruction::Ap(ap) => {
            let expected_argument = ast::ApArgument::Error(ast::InstructionErrorAST { lens: None });
            let result_matches = match &ap.result {
                ast::ApResult::Scalar(scalar) => scalar.name == if_error.name,
                ast::ApResult::Stream(_) => false,
            };
            result_matches && ap.argument == expected_argument
        }
        _ => false,
    }
}

fn validate_error_handling_xor_second(
    else_instruction: &ast::Instruction<'_>,
    expected_error_handling_xor: impl FnOnce() -> String,
) -> bool {
    let expected_air_script = expected_error_handling_xor();

    else_instruction.to_string() == expected_air_script
}

fn pop_new_from_tree(mut expected_tree: ast::Instruction<'_>, arg: usize) -> ast::Instruction<'_> {
    for _ in 0..arg {
        match expected_tree {
            ast::Instruction::New(new) => {
                expected_tree = new.instruction;
            }
            _ => panic!("expected new, got: {expected_tree}"),
        }
    }
    expected_tree
}
