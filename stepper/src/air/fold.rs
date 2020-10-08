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

use super::ExecutionContext;
use super::Instruction;
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
    pub(crate) cursor: usize,
    pub(crate) iterable_name: String,
    pub(crate) instr_head: Rc<Instruction>,
}

impl super::ExecutableInstruction for Fold {
    fn execute(&self, ctx: &mut ExecutionContext) -> Result<()> {
        log::info!("fold {:?} is called with context {:?}", self, ctx);

        let iterable_name = &self.0;
        let iterator_name = &self.1;
        let instr_head = self.2.clone();

        // check that value exists and has array type
        match ctx.data.get(iterable_name) {
            Some(JValue::Array(_)) => {}
            Some(v) => {
                return Err(AquamarineError::IncompatibleJValueType(
                    v.clone(),
                    String::from("Array"),
                ))
            }
            None => {
                return Err(AquamarineError::VariableNotFound(String::from(
                    iterable_name,
                )))
            }
        };

        let fold_state = FoldState {
            cursor: 0,
            iterable_name: iterable_name.clone(),
            instr_head: instr_head.clone(),
        };

        if ctx
            .folds
            .insert(iterator_name.clone(), fold_state)
            .is_some()
        {
            return Err(AquamarineError::MultipleFoldStates(iterable_name.clone()));
        }

        instr_head.execute(ctx)?;
        ctx.folds.remove(iterator_name);

        Ok(())
    }
}

impl super::ExecutableInstruction for Next {
    fn execute(&self, ctx: &mut ExecutionContext) -> Result<()> {
        log::info!("next {:?} is called with context {:?}", self, ctx);

        let iterator_name = &self.0;
        let fold_state = ctx
            .folds
            .get_mut(iterator_name)
            .ok_or_else(|| AquamarineError::FoldStateNotFound(iterator_name.clone()))?;
        let value = ctx
            .data
            .get(&fold_state.iterable_name)
            .expect("this has been checked on the fold instruction");
        let value_len = match value {
            JValue::Array(array) => array.len(),
            _ => unreachable!(),
        };

        fold_state.cursor += 1;
        if value_len == 0 || value_len <= fold_state.cursor {
            fold_state.cursor -= 1;
            // just do nothing to exit
            return Ok(());
        }

        let next_instr = fold_state.instr_head.clone();
        next_instr.execute(ctx)?;

        // get the same fold state again because of borrow checker
        match ctx.folds.get_mut(iterator_name) {
            Some(fold_state) => fold_state.cursor -= 1,
            _ => unreachable!("iterator value shouldn't changed inside fold"),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::JValue;

    use aqua_test_utils::create_aqua_vm;
    use aquamarine_vm::vec1::Vec1;
    use aquamarine_vm::HostExportedFunc;
    use aquamarine_vm::IValue;

    use serde_json::json;

    fn get_echo_call_service() -> HostExportedFunc {
        Box::new(|_, args| -> Option<IValue> {
            let arg = match &args[2] {
                IValue::String(str) => str,
                _ => unreachable!(),
            };

            let arg: Vec<String> = serde_json::from_str(arg).unwrap();

            Some(IValue::Record(
                Vec1::new(vec![IValue::S32(0), IValue::String(arg[0].clone())]).unwrap(),
            ))
        })
    }

    #[test]
    fn fold() {
        let mut vm = create_aqua_vm(get_echo_call_service());

        let lfold = String::from(
            r#"
            (fold (Iterable i
                (seq (
                    (call (%current_peer_id% (local_service_id local_fn_name) (i) acc[]))
                    (next i)
                )
            )))"#,
        );

        let res = vm
            .call(json!([
                String::from("asd"),
                lfold,
                String::from("{\"Iterable\": [\"1\",\"2\",\"3\",\"4\",\"5\"]}"),
            ]))
            .expect("call should be successful");

        let res: JValue = serde_json::from_str(&res.data).unwrap();

        assert_eq!(res.get("acc").unwrap(), &json!([1, 2, 3, 4, 5]));

        let rfold = String::from(
            r#"
            (fold (Iterable i
                (seq (
                    (next i)
                    (call (%current_peer_id% (local_service_id local_fn_name) (i) acc[]))
                )
            )))"#,
        );

        let res = vm
            .call(json!([
                String::from("asd"),
                rfold,
                String::from("{\"Iterable\": [\"1\",\"2\",\"3\",\"4\",\"5\"]}"),
            ]))
            .expect("call should be successful");

        let res: JValue = serde_json::from_str(&res.data).unwrap();

        assert_eq!(res.get("acc").unwrap(), &json!([5, 4, 3, 2, 1]));
    }

    #[test]
    fn inner_fold() {
        env_logger::init();

        let mut vm = create_aqua_vm(get_echo_call_service());

        let script = String::from(
            r#"
            (fold (Iterable1 i
                (seq (
                    (fold (Iterable2 j
                        (seq (
                            (call (%current_peer_id% (local_service_id local_fn_name) (i) acc[]))
                            (next j)
                        ))
                    ))
                    (next i)
                ))
            ))"#,
        );

        let res = vm
            .call(json!([
                String::from("asd"),
                script,
                String::from("{\"Iterable1\": [\"1\",\"2\",\"3\",\"4\",\"5\"], \"Iterable2\": [\"1\",\"2\",\"3\",\"4\",\"5\"]}"),
            ]))
            .expect("call should be successful");

        let res: JValue = serde_json::from_str(&res.data).unwrap();

        assert_eq!(
            res.get("acc").unwrap(),
            &json!([1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5])
        );
    }
}
