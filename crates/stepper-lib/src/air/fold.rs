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

use super::CallEvidenceCtx;
use super::ExecutionCtx;
use super::Instruction;
use crate::AValue;
use crate::AquamarineError;
use crate::JValue;
use crate::Result;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::rc::Rc;

/*
 (fold Iterable i
   (par
     (call fn [i] acc[])
     (next i)
   )
 )
*/

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Fold(String, String, Rc<Instruction>);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Next(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FoldState {
    // TODO: maybe change to bidirectional iterator
    pub(crate) cursor: usize,
    pub(crate) iterable: Rc<JValue>,
    pub(crate) instr_head: Rc<Instruction>,
}

impl super::ExecutableInstruction for Fold {
    fn execute(&self, exec_ctx: &mut ExecutionCtx, call_ctx: &mut CallEvidenceCtx) -> Result<()> {
        use AquamarineError::*;

        log::info!(
            "fold {} {} is called with contexts {:?} {:?}",
            self.0,
            self.1,
            exec_ctx,
            call_ctx
        );

        let iterable_name = &self.0;
        let iterator_name = &self.1;
        let instr_head = self.2.clone();

        // check that value exists and has array type
        let iterable = match exec_ctx.data_cache.get(iterable_name) {
            Some(AValue::JValueRef(jvalue_rc)) => {
                match jvalue_rc.as_ref() {
                    JValue::Array(array) => {
                        if array.is_empty() {
                            // skip fold if array is empty
                            return Ok(());
                        }

                        jvalue_rc
                    }
                    v => return Err(IncompatibleJValueType(v.clone(), String::from("Array"))),
                }
            }
            Some(v) => return Err(IncompatibleAValueType(format!("{:?}", v), String::from("JValueRef"))),
            None => return Err(VariableNotFound(String::from(iterable_name))),
        };

        let fold_state = FoldState {
            cursor: 0,
            iterable: iterable.clone(),
            instr_head: instr_head.clone(),
        };

        let previous_value = exec_ctx
            .data_cache
            .insert(iterator_name.clone(), AValue::JValueFoldCursor(fold_state));

        if previous_value.is_some() {
            return Err(MultipleFoldStates(iterable_name.clone()));
        }

        instr_head.execute(exec_ctx, call_ctx)?;
        exec_ctx.data_cache.remove(iterator_name);

        Ok(())
    }
}

impl super::ExecutableInstruction for Next {
    fn execute(&self, exec_ctx: &mut ExecutionCtx, call_ctx: &mut CallEvidenceCtx) -> Result<()> {
        use AquamarineError::IncompatibleAValueType;

        log::info!("next {:?} is called with contexts {:?} {:?}", self, exec_ctx, call_ctx);

        let iterator_name = &self.0;
        let avalue = exec_ctx
            .data_cache
            .get_mut(iterator_name)
            .ok_or_else(|| AquamarineError::FoldStateNotFound(iterator_name.clone()))?;
        let fold_state = match avalue {
            AValue::JValueFoldCursor(state) => state,
            v => {
                return Err(IncompatibleAValueType(
                    format!("{:?}", v),
                    String::from("JValueFoldCursor"),
                ))
            }
        };
        let value_len = match fold_state.iterable.as_ref() {
            JValue::Array(array) => array.len(),
            _ => unreachable!("iterable value shouldn't changed inside fold"),
        };

        fold_state.cursor += 1;
        if value_len == 0 || value_len <= fold_state.cursor {
            fold_state.cursor -= 1;
            // just do nothing to exit
            return Ok(());
        }

        let next_instr = fold_state.instr_head.clone();
        next_instr.execute(exec_ctx, call_ctx)?;

        // get the same fold state again because of borrow checker
        match exec_ctx.data_cache.get_mut(iterator_name) {
            Some(AValue::JValueFoldCursor(fold_state)) => fold_state.cursor -= 1,
            _ => unreachable!("iterator value shouldn't changed inside fold"),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::call_evidence::CallEvidencePath;
    use crate::JValue;

    use aqua_test_utils::call_vm;
    use aqua_test_utils::create_aqua_vm;
    use aqua_test_utils::echo_number_call_service;
    use aqua_test_utils::set_variable_call_service;
    use aquamarine_vm::AquamarineVMError;
    use aquamarine_vm::StepperError;

    use serde_json::json;
    use std::rc::Rc;

    #[test]
    fn lfold() {
        use crate::call_evidence::CallResult::*;
        use crate::call_evidence::EvidenceState::*;

        let mut vm = create_aqua_vm(echo_number_call_service(), "A");
        let mut set_variable_vm = create_aqua_vm(set_variable_call_service(r#"["1","2","3","4","5"]"#), "set_variable");

        let lfold = String::from(
            r#"
            (seq (
                (call ("set_variable" ("" "") () Iterable))
                (fold (Iterable i
                    (seq (
                        (call ("A" ("" "") (i) acc[]))
                        (next i)
                    ))
                ))
            ))"#,
        );

        let res = call_vm!(set_variable_vm, "", lfold, "[]", "[]");
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
            (seq (
                (call ("set_variable" ("" "") () Iterable))
                (fold (Iterable i
                    (seq (
                        (next i)
                        (call ("A" ("" "") (i) acc[]))
                    ))
                ))
            ))"#,
        );

        let res = call_vm!(set_variable_vm, "", rfold, "[]", "[]");
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
            (seq (
                (seq (
                    (call ("set_variable" ("" "") () Iterable1))
                    (call ("set_variable" ("" "") () Iterable2))
                ))
                (fold (Iterable1 i
                    (seq (
                        (fold (Iterable2 j
                            (seq (
                                (call ("A" ("" "") (i) acc[]))
                                (next j)
                            ))
                        ))
                        (next i)
                    ))
                ))
            ))"#,
        );

        let res = call_vm!(set_variable_vm, "", script, "[]", "[]");
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
            (seq (
                (seq (
                    (call ("set_variable" ("" "") () Iterable1))
                    (call ("set_variable" ("" "") () Iterable2))
                ))
                (fold (Iterable1 i
                    (seq (
                        (fold (Iterable2 i
                            (seq (
                                (call ("A" ("" "") (i) acc[]))
                                (next i)
                            ))
                        ))
                        (next i)
                    ))
                ))
            ))"#,
        );

        let res = vm.call(json!(["", script, "[]", "[]"]));

        assert!(res.is_err());
        let error = res.err().unwrap();
        let error = match error {
            AquamarineVMError::StepperError(error) => error,
            _ => unreachable!(),
        };

        assert_eq!(
            error,
            StepperError::FoldStateNotFound(String::from("multiple fold states found for iterable Iterable2"))
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
            (seq (
                (call ("set_variable" ("" "") () Iterable))
                (fold (Iterable i
                    (seq (
                        (call ("A" ("" "") (i) acc[]))
                        (next i)
                    ))
                ))
            ))"#,
        );

        let res = call_vm!(set_variable_vm, "", empty_fold, "[]", "[]");
        let res = call_vm!(vm, "", empty_fold, "[]", res.data);
        let res: CallEvidencePath = serde_json::from_str(&res.data).expect("should be valid call evidence path");

        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Call(Executed(Rc::new(json!([])))));
    }
}
