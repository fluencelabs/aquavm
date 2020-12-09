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

mod foldable;
mod jvaluable_result;

use foldable::*;
use jvaluable_result::*;
use super::CallEvidenceCtx;
use super::ExecutionCtx;
use super::Instruction;
use crate::log_instruction;
use crate::AValue;
use crate::AquamarineError;
use crate::ExecutedCallResult;
use crate::JValue;
use crate::Result;
use crate::SecurityTetraplet;

use air_parser::ast::Fold;
use air_parser::ast::InstructionValue;
use air_parser::ast::Next;

use std::cell::Ref;
use std::collections::HashMap;
use std::rc::Rc;

/*
 (fold Iterable i
   (par
     (call fn [i] acc[])
     (next i)
   )
 )
*/

// #[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FoldState<'i> {
    pub(crate) iterable: Box<dyn for<'ctx> Foldable<'ctx, Item = FoldableResult<'ctx>>>,
    pub(crate) instr_head: Rc<Instruction<'i>>,
    // map of met variables inside this (not any inner) fold block with their initial values
    pub(crate) met_variables: HashMap<&'i str, ExecutedCallResult>,
}

impl<'i> super::ExecutableInstruction<'i> for Fold<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, call_ctx: &mut CallEvidenceCtx) -> Result<()> {
        use AquamarineError::*;

        log_instruction!(fold, exec_ctx, call_ctx);

        let iterable: Box<dyn for<'ctx> Foldable<'ctx, Item = FoldableResult<'ctx>>> = match &self.iterable {
            InstructionValue::Variable(name) => {
                match exec_ctx.data_cache.get(*name) {
                    Some(AValue::JValueRef(variable)) => {
                        let len = match &variable.result {
                            JValue::Array(array) => {
                                if array.is_empty() {
                                    // skip fold if array is empty
                                    return Ok(());
                                }
                                array.len()
                            }
                            v => return Err(IncompatibleJValueType(v.clone(), "array")),
                        };

                        let foldable = FoldableNamedResult {
                            name: name.to_string(),
                            cursor: 0,
                            len,
                        };

                        Box::new(foldable)
                    }
                    Some(AValue::JValueAccumulatorRef(acc)) => {
                        let foldable = FoldableNamedResult {
                            name: name.to_string(),
                            cursor: 0,
                            len: acc.borrow().len(),
                        };

                        Box::new(foldable)
                    }
                    _ => unreachable!(),
                }
            }
            InstructionValue::JsonPath { variable, path } => match exec_ctx.data_cache.get(*variable) {
                Some(AValue::JValueRef(variable)) => {
                    use jsonpath_lib::select;
                    let jvalues = select(&variable.result, path).unwrap();
                    let len = jvalues.len();
                    let jvalues = jvalues.into_iter().cloned().collect();

                    let foldable = FoldableJsonPathResult {
                        jvalues,
                        tetraplet: variable.triplet.clone(),
                        cursor: 0,
                        len,
                    };

                    Box::new(foldable)
                }
                Some(AValue::JValueAccumulatorRef(acc)) => {
                    use jsonpath_lib::select_with_iter;

                    let acc = acc.borrow();
                    let (jvalues, tetraplet_indices) = select_with_iter(acc.iter().map(|v| &v.result), path).unwrap();
                    let jvalues = jvalues.into_iter().cloned().collect();
                    let tetraplets = tetraplet_indices
                        .iter()
                        .map(|&id| &acc[id].triplet)
                        .cloned()
                        .collect::<Vec<_>>();

                    let foldable = FoldableVecJsonPathResult {
                        jvalues,
                        tetraplets,
                        cursor: 0,
                        len: acc.len(),
                    };

                    Box::new(foldable)
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        let fold_state = FoldState {
            iterable,
            instr_head: self.instruction.clone(),
            met_variables: HashMap::new(),
        };

        let previous_value = exec_ctx
            .data_cache
            .insert(self.iterator.to_string(), AValue::JValueFoldCursor(fold_state));

        if previous_value.is_some() {
            return Err(MultipleFoldStates(self.iterator.to_string()));
        }
        exec_ctx.met_folds.push_back(self.iterator);

        self.instruction.execute(exec_ctx, call_ctx)?;

        let fold_state = match exec_ctx.data_cache.remove(self.iterator) {
            Some(AValue::JValueFoldCursor(fold_state)) => fold_state,
            _ => unreachable!("fold cursor is changed only inside fold block"),
        };

        for (variable_name, _) in fold_state.met_variables {
            exec_ctx.data_cache.remove(variable_name);
        }
        exec_ctx.met_folds.pop_back();

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

        Ok(())
    }
}

impl<'i> super::ExecutableInstruction<'i> for Next<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, call_ctx: &mut CallEvidenceCtx) -> Result<()> {
        use AquamarineError::FoldStateNotFound;
        use AquamarineError::IncompatibleAValueType;

        log_instruction!(next, exec_ctx, call_ctx);

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
        next_instr.execute(exec_ctx, call_ctx)?;

        // get the same fold state again because of borrow checker
        match exec_ctx.data_cache.get_mut(iterator_name) {
            // move iterator back to provide correct value for possible subtree after next
            // (for example for cases such as right fold)
            Some(AValue::JValueFoldCursor(fold_state)) => fold_state.iterable.back(),
            _ => unreachable!("iterator value shouldn't changed inside fold"),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::call_evidence::CallEvidencePath;
    use crate::JValue;

    use aqua_test_utils::create_aqua_vm;
    use aqua_test_utils::echo_number_call_service;
    use aqua_test_utils::set_variable_call_service;
    use aqua_test_utils::{call_vm, echo_string_call_service};
    use aquamarine_vm::AquamarineVMError;
    use aquamarine_vm::StepperError;
    use aquamarine_vm::StepperOutcome;

    use serde_json::json;
    use std::rc::Rc;

    // Check that
    #[test]
    fn lfold() {
        env_logger::try_init().ok();

        use crate::call_evidence::CallResult::*;
        use crate::call_evidence::EvidenceState::*;

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
        let res: CallEvidencePath = serde_json::from_str(&res.data).expect("should be valid call evidence path");

        assert_eq!(res.len(), 6);
        assert_eq!(res[0], Call(Executed(Rc::new(json!(["1", "2", "3", "4", "5"])))));

        for i in 1..=5 {
            assert_eq!(res[i], Call(Executed(Rc::new(JValue::Number(i.into())))));
        }
    }

    #[test]
    fn rfold() {
        use crate::call_evidence::CallResult::*;
        use crate::call_evidence::EvidenceState::*;

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
        let res: CallEvidencePath = serde_json::from_str(&res.data).expect("should be valid call evidence path");

        assert_eq!(res.len(), 6);
        assert_eq!(res[0], Call(Executed(Rc::new(json!(["1", "2", "3", "4", "5"])))));

        for i in 1..=5 {
            assert_eq!(res[i], Call(Executed(Rc::new(JValue::Number((6 - i).into())))));
        }
    }

    #[test]
    fn inner_fold() {
        use crate::call_evidence::CallResult::*;
        use crate::call_evidence::EvidenceState::*;

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
        let res: CallEvidencePath = serde_json::from_str(&res.data).expect("should be valid call evidence path");

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

        let res = vm.call_with_prev_data("", script, "[]", "[]");

        assert!(res.is_err());
        let error = res.err().unwrap();
        let error = match error {
            AquamarineVMError::StepperError(error) => error,
            _ => unreachable!(),
        };

        assert_eq!(
            error,
            StepperError::MultipleFoldStates(String::from("multiple fold states found for iterable i"))
        );
    }

    #[test]
    fn empty_fold() {
        use crate::call_evidence::CallResult::*;
        use crate::call_evidence::EvidenceState::*;

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
        let res: CallEvidencePath = serde_json::from_str(&res.data).expect("should be valid call evidence path");

        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Call(Executed(Rc::new(json!([])))));
    }

    #[test]
    fn json_path() {
        use crate::call_evidence::CallResult::*;
        use crate::call_evidence::EvidenceState::*;

        let mut vm = create_aqua_vm(echo_number_call_service(), "A");
        let mut set_variable_vm = create_aqua_vm(
            set_variable_call_service(r#"{ "array": ["1","2","3","4","5"] }"#),
            "set_variable",
        );

        let lfold = String::from(
            r#"
            (seq
                (call "set_variable" ("" "") [] Iterable)
                (fold Iterable.$["array"] i
                    (seq
                        (call "A" ("" "") [i] acc[])
                        (next i)
                    )
                )
            )"#,
        );

        let res = call_vm!(set_variable_vm, "", lfold.clone(), "[]", "[]");
        let res = call_vm!(vm, "", lfold, "[]", res.data);
        let res: CallEvidencePath = serde_json::from_str(&res.data).expect("should be valid call evidence path");

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
        use crate::call_evidence::CallResult::*;
        use crate::call_evidence::EvidenceState::*;

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

        let res: CallEvidencePath = serde_json::from_str(&res.data).expect("should be valid call evidence path");

        assert_eq!(res.len(), 12);
        for i in 2..11 {
            assert!(matches!(res[i], Call(Executed(_))) || matches!(res[i], Par(..)));
        }
    }

    #[test]
    fn shadowing_scope() {
        use crate::call_evidence::CallResult::*;
        use crate::call_evidence::EvidenceState::*;

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

        let use_non_exist_variable_script = String::from(
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
                            (call "A" ("" "") [local_j])
                        )
                        (next i)
                    )
                )
            )"#,
        );

        let res = execute_script(use_non_exist_variable_script);
        assert!(res.is_err());
        let error = res.err().unwrap();
        let error = match error {
            AquamarineVMError::StepperError(error) => error,
            _ => unreachable!(),
        };

        assert!(matches!(error, StepperError::VariableNotFound(_)));

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
        let res: CallEvidencePath = serde_json::from_str(&res.data).expect("should be valid call evidence path");

        assert_eq!(res.len(), 11);
        for i in 0..10 {
            assert!(matches!(res[i], Call(Executed(_))));
        }
    }
}
