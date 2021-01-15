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

mod utils;

use super::ExecutionCtx;
use super::ExecutionError;
use super::ExecutionResult;
use super::ExecutionTraceCtx;
use super::Instruction;
use crate::contexts::execution::AValue;
use crate::contexts::execution::ResolvedCallResult;
use crate::execution::boxed_value::*;
use crate::log_instruction;

use air_parser::ast::Fold;
use air_parser::ast::Next;

use std::collections::HashMap;
use std::rc::Rc;

use utils::IterableValue;

pub(crate) struct FoldState<'i> {
    pub(crate) iterable: IterableValue,
    pub(crate) instr_head: Rc<Instruction<'i>>,
    // map of met variables inside this (not any inner) fold block with their initial values
    pub(crate) met_variables: HashMap<&'i str, ResolvedCallResult>,
}

impl<'i> FoldState<'i> {
    pub fn new(iterable: IterableValue, instr_head: Rc<Instruction<'i>>) -> Self {
        Self {
            iterable,
            instr_head,
            met_variables: HashMap::new(),
        }
    }
}

impl<'i> super::ExecutableInstruction<'i> for Fold<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut ExecutionTraceCtx) -> ExecutionResult<()> {
        use ExecutionError::MultipleFoldStates;

        log_instruction!(fold, exec_ctx, trace_ctx);

        let iterable = match utils::construct_iterable_value(&self.iterable, exec_ctx)? {
            Some(iterable) => iterable,
            None => return Ok(()),
        };

        let fold_state = FoldState::new(iterable, self.instruction.clone());

        let previous_value = exec_ctx
            .data_cache
            .insert(self.iterator.to_string(), AValue::JValueFoldCursor(fold_state));

        if previous_value.is_some() {
            return Err(MultipleFoldStates(self.iterator.to_string()));
        }
        exec_ctx.met_folds.push_back(self.iterator);

        self.instruction.execute(exec_ctx, trace_ctx)?;

        cleanup_variables(exec_ctx, &self.iterator);

        Ok(())
    }
}

impl<'i> super::ExecutableInstruction<'i> for Next<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut ExecutionTraceCtx) -> ExecutionResult<()> {
        use ExecutionError::FoldStateNotFound;
        use ExecutionError::IncompatibleAValueType;

        log_instruction!(next, exec_ctx, trace_ctx);

        let iterator_name = self.0;
        let avalue = exec_ctx
            .data_cache
            .get_mut(iterator_name)
            .ok_or_else(|| FoldStateNotFound(iterator_name.to_string()))?;

        let fold_state = match avalue {
            AValue::JValueFoldCursor(state) => state,
            v => {
                // it's not possible to use unreachable here
                // because at now next syntactically could be used without fold
                return Err(IncompatibleAValueType(
                    format!("{}", v),
                    String::from("JValueFoldCursor"),
                ));
            }
        };

        if !fold_state.iterable.next() {
            // just do nothing to exit
            return Ok(());
        }

        let next_instr = fold_state.instr_head.clone();
        next_instr.execute(exec_ctx, trace_ctx)?;

        // get the same fold state again because of borrow checker
        match exec_ctx.data_cache.get_mut(iterator_name) {
            // move iterator back to provide correct value for possible subtree after next
            // (for example for cases such as right fold)
            Some(AValue::JValueFoldCursor(fold_state)) => fold_state.iterable.prev(),
            _ => unreachable!("iterator value shouldn't changed inside fold"),
        };

        Ok(())
    }
}

fn cleanup_variables(exec_ctx: &mut ExecutionCtx<'_>, iterator: &str) {
    let fold_state = match exec_ctx.data_cache.remove(iterator) {
        Some(AValue::JValueFoldCursor(fold_state)) => fold_state,
        _ => unreachable!("fold cursor is changed only inside fold block"),
    };

    for (variable_name, _) in fold_state.met_variables {
        exec_ctx.data_cache.remove(variable_name);
    }
    exec_ctx.met_folds.pop_back();

    // TODO: fix 3 or more inner folds behaviour
    if let Some(fold_block_name) = exec_ctx.met_folds.back() {
        let fold_state = match exec_ctx.data_cache.get(*fold_block_name) {
            Some(AValue::JValueFoldCursor(fold_state)) => fold_state,
            _ => unreachable!("fold block data must be represented as fold cursor"),
        };

        let mut upper_fold_values = HashMap::new();
        for (variable_name, variable) in fold_state.met_variables.iter() {
            upper_fold_values.insert(variable_name.to_string(), AValue::JValueRef(variable.clone()));
        }

        exec_ctx.data_cache.extend(upper_fold_values);
    }
}

#[cfg(test)]
mod tests {
    use crate::contexts::execution_trace::ExecutionTrace;
    use crate::JValue;

    use aqua_test_utils::call_vm;
    use aqua_test_utils::create_aqua_vm;
    use aqua_test_utils::echo_number_call_service;
    use aqua_test_utils::echo_string_call_service;
    use aqua_test_utils::set_variable_call_service;
    use aqua_test_utils::AquamarineVMError;
    use aqua_test_utils::StepperOutcome;

    use serde_json::json;
    use std::rc::Rc;

    #[test]
    fn lfold() {
        env_logger::try_init().ok();

        use crate::contexts::execution_trace::CallResult::*;
        use crate::contexts::execution_trace::ExecutedState::*;

        let mut vm = create_aqua_vm(echo_number_call_service(), "A");
        let mut set_variable_vm = create_aqua_vm(set_variable_call_service(r#"["1","2","3","4","5"]"#), "set_variable");

        let lfold = String::from(
            r#"
            (seq
                (call "set_variable" ("" "") [] Iterable)
                (fold Iterable i
                    (seq
                        (call "A" ("" "") [i] acc[])
                        (next i)
                    )
                )
            )"#,
        );

        let res = call_vm!(set_variable_vm, "", lfold.clone(), "[]", "[]");
        let res = call_vm!(vm, "", lfold, "[]", res.data);
        let res: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid executed trace");

        assert_eq!(res.len(), 6);
        assert_eq!(res[0], Call(Executed(Rc::new(json!(["1", "2", "3", "4", "5"])))));

        for i in 1..=5 {
            assert_eq!(res[i], Call(Executed(Rc::new(JValue::Number(i.into())))));
        }
    }

    #[test]
    fn rfold() {
        use crate::contexts::execution_trace::CallResult::*;
        use crate::contexts::execution_trace::ExecutedState::*;

        let mut vm = create_aqua_vm(echo_number_call_service(), "A");
        let mut set_variable_vm = create_aqua_vm(set_variable_call_service(r#"["1","2","3","4","5"]"#), "set_variable");

        let rfold = String::from(
            r#"
            (seq
                (call "set_variable" ("" "") [] Iterable)
                (fold Iterable i
                    (seq
                        (next i)
                        (call "A" ("" "") [i] acc[])
                    )
                )
            )"#,
        );

        let res = call_vm!(set_variable_vm, "", rfold.clone(), "[]", "[]");
        let res = call_vm!(vm, "", rfold, "[]", res.data);
        let res: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid executed trace");

        assert_eq!(res.len(), 6);
        assert_eq!(res[0], Call(Executed(Rc::new(json!(["1", "2", "3", "4", "5"])))));

        for i in 1..=5 {
            assert_eq!(res[i], Call(Executed(Rc::new(JValue::Number((6 - i).into())))));
        }
    }

    #[test]
    fn inner_fold() {
        use crate::contexts::execution_trace::CallResult::*;
        use crate::contexts::execution_trace::ExecutedState::*;

        let mut vm = create_aqua_vm(echo_number_call_service(), "A");
        let mut set_variable_vm = create_aqua_vm(set_variable_call_service(r#"["1","2","3","4","5"]"#), "set_variable");

        let script = String::from(
            r#"
            (seq
                (seq
                    (call "set_variable" ("" "") [] Iterable1)
                    (call "set_variable" ("" "") [] Iterable2)
                )
                (fold Iterable1 i
                    (seq
                        (fold Iterable2 j
                            (seq
                                (call "A" ("" "") [i] acc[])
                                (next j)
                            )
                        )
                        (next i)
                    )
                )
            )"#,
        );

        let res = call_vm!(set_variable_vm, "", script.clone(), "[]", "[]");
        let res = call_vm!(vm, "", script, "[]", res.data);
        let res: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid executed trace");

        assert_eq!(res.len(), 27);
        assert_eq!(res[0], Call(Executed(Rc::new(json!(["1", "2", "3", "4", "5"])))));
        assert_eq!(res[1], Call(Executed(Rc::new(json!(["1", "2", "3", "4", "5"])))));

        for i in 1..=5 {
            for j in 1..=5 {
                assert_eq!(
                    res[1 + 5 * (i - 1) + j],
                    Call(Executed(Rc::new(JValue::Number(i.into()))))
                );
            }
        }
    }

    #[test]
    fn inner_fold_with_same_iterator() {
        let mut vm = create_aqua_vm(set_variable_call_service(r#"["1","2","3","4","5"]"#), "set_variable");

        let script = String::from(
            r#"
            (seq
                (seq
                    (call "set_variable" ("" "") [] Iterable1)
                    (call "set_variable" ("" "") [] Iterable2)
                )
                (fold Iterable1 i
                    (seq
                        (fold Iterable2 i
                            (seq
                                (call "A" ("" "") [i] acc[])
                                (next i)
                            )
                        )
                        (next i)
                    )
                )
            )"#,
        );

        let res = call_vm!(vm, "", script, "[]", "[]");

        assert_eq!(res.ret_code, 1012);
    }

    #[test]
    fn empty_fold() {
        use crate::contexts::execution_trace::CallResult::*;
        use crate::contexts::execution_trace::ExecutedState::*;

        let mut vm = create_aqua_vm(echo_number_call_service(), "A");
        let mut set_variable_vm = create_aqua_vm(set_variable_call_service(r#"[]"#), "set_variable");

        let empty_fold = String::from(
            r#"
            (seq
                (call "set_variable" ("" "") [] Iterable)
                (fold Iterable i
                    (seq
                        (call "A" ("" "") [i] acc[])
                        (next i)
                    )
                )
            )"#,
        );

        let res = call_vm!(set_variable_vm, "", empty_fold.clone(), "[]", "[]");
        let res = call_vm!(vm, "", empty_fold, "[]", res.data);
        let res: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid executed trace");

        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Call(Executed(Rc::new(json!([])))));
    }

    #[test]
    fn json_path() {
        use crate::contexts::execution_trace::CallResult::*;
        use crate::contexts::execution_trace::ExecutedState::*;

        let mut vm = create_aqua_vm(echo_number_call_service(), "A");
        let mut set_variable_vm = create_aqua_vm(
            set_variable_call_service(r#"{ "array": ["1","2","3","4","5"] }"#),
            "set_variable",
        );

        let lfold = String::from(
            r#"
            (seq
                (call "set_variable" ("" "") [] Iterable)
                (fold Iterable.$.array! i
                    (seq
                        (call "A" ("" "") [i] acc[])
                        (next i)
                    )
                )
            )"#,
        );

        let res = call_vm!(set_variable_vm, "", lfold.clone(), "[]", "[]");
        let res = call_vm!(vm, "", lfold, "[]", res.data);
        let res: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid executed trace");

        assert_eq!(res.len(), 6);
        assert_eq!(
            res[0],
            Call(Executed(Rc::new(json!({ "array": ["1", "2", "3", "4", "5"] }))))
        );

        for i in 1..=5 {
            assert_eq!(res[i], Call(Executed(Rc::new(JValue::Number(i.into())))));
        }
    }

    #[test]
    fn shadowing() {
        use crate::contexts::execution_trace::CallResult::*;
        use crate::contexts::execution_trace::ExecutedState::*;

        let mut set_variables_vm = create_aqua_vm(set_variable_call_service(r#"["1","2"]"#), "set_variable");
        let mut vm_a = create_aqua_vm(echo_string_call_service(), "A");
        let mut vm_b = create_aqua_vm(echo_string_call_service(), "B");

        let script = String::from(
            r#"
            (seq
                (seq
                    (call "set_variable" ("" "") [] Iterable1)
                    (call "set_variable" ("" "") [] Iterable2)
                )
                (fold Iterable1 i
                    (seq
                        (seq
                            (fold Iterable2 j
                                (seq
                                    (seq
                                        (call "A" ("" "") [i] local_j)
                                        (call "B" ("" "") [local_j])
                                    )
                                    (next j)
                                )
                            )
                            (par
                                (call "A" ("" "") [i] local_i)
                                (call "B" ("" "") [i])
                            )
                        )
                        (next i)
                    )
                )
            )"#,
        );

        let res = call_vm!(set_variables_vm, "", script.clone(), "[]", "[]");
        let res = call_vm!(vm_a, "", script.clone(), "[]", res.data);
        let res = call_vm!(vm_b, "", script.clone(), "[]", res.data);
        let res = call_vm!(vm_a, "", script.clone(), "[]", res.data);
        let res = call_vm!(vm_b, "", script.clone(), "[]", res.data);
        let res = call_vm!(vm_a, "", script.clone(), "[]", res.data);
        let res = call_vm!(vm_b, "", script, "[]", res.data);

        let res: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid executed trace");

        assert_eq!(res.len(), 12);
        for i in 2..11 {
            assert!(matches!(res[i], Call(Executed(_))) || matches!(res[i], Par(..)));
        }
    }

    #[test]
    fn shadowing_scope() {
        use crate::contexts::execution_trace::CallResult::*;
        use crate::contexts::execution_trace::ExecutedState::*;

        fn execute_script(script: String) -> Result<StepperOutcome, AquamarineVMError> {
            let mut set_variables_vm = create_aqua_vm(set_variable_call_service(r#"["1","2"]"#), "set_variable");
            let mut vm_a = create_aqua_vm(echo_string_call_service(), "A");
            let mut vm_b = create_aqua_vm(echo_string_call_service(), "B");

            let res = call_vm!(set_variables_vm, "", script.clone(), "[]", "[]");
            let res = call_vm!(vm_a, "", script.clone(), "[]", res.data);
            let res = call_vm!(vm_b, "", script.clone(), "[]", res.data);
            let res = call_vm!(vm_a, "", script.clone(), "[]", res.data);
            let res = call_vm!(vm_b, "", script.clone(), "[]", res.data);

            vm_a.call_with_prev_data("", script, "[]", res.data)
        }

        let variable_shadowing_script = String::from(
            r#"
            (seq
                (seq
                    (call "set_variable" ("" "") [] Iterable1)
                    (call "set_variable" ("" "") [] Iterable2)
                )
                (fold Iterable1 i
                    (seq
                        (seq
                            (call "A" ("" "") ["value"] local_j)
                            (seq
                                (fold Iterable2 j
                                    (seq
                                        (seq
                                            (call "A" ("" "") [i] local_j)
                                            (call "B" ("" "") [local_j])
                                        )
                                        (next j)
                                    )
                                )
                                (call "A" ("" "") [local_j])
                            )
                        )
                        (next i)
                    )
                )
            )"#,
        );

        let res = execute_script(variable_shadowing_script).unwrap();
        let res: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid executed trace");

        assert_eq!(res.len(), 11);
        for i in 0..10 {
            assert!(matches!(res[i], Call(Executed(_))));
        }
    }
}
