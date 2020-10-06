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
use crate::AValue;
use crate::AquamarineError;
use crate::Result;
use crate::SerdeValue;

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
    instr_head: Rc<Instruction>,
}

impl super::ExecutableInstruction for Fold {
    fn execute(&self, ctx: &mut ExecutionContext) -> Result<()> {
        log::info!("fold {:?} is called with context {:?}", self, ctx);

        let iterable_name = &self.0;
        let iterator_name = &self.1;
        let instr_head = self.2.clone();

        let iterable = ctx
            .data
            .get(iterable_name)
            .ok_or_else(|| AquamarineError::VariableNotFound(String::from(iterable_name)))?;
        let iterable = (crate::to_svalue!(iterable) as Result<&_>)?;

        let iterable = match iterable {
            SerdeValue::Array(json_array) => json_array.clone(),
            v => {
                return Err(AquamarineError::VariableIsNotArray(
                    v.clone(),
                    iterable_name.clone(),
                ))
            }
        };

        let fold_state = FoldState {
            instr_head: instr_head.clone(),
        };

        // TODO: check for result
        ctx.data
            .insert(iterator_name.clone(), AValue::Iterator(iterable, 0));
        ctx.folds.insert(iterator_name.clone(), fold_state);

        instr_head.execute(ctx)?;

        ctx.data.remove(iterator_name);
        ctx.folds.remove(iterator_name);

        Ok(())
    }
}

impl super::ExecutableInstruction for Next {
    fn execute(&self, ctx: &mut ExecutionContext) -> Result<()> {
        log::info!("next {:?} is called with context {:?}", self, ctx);

        let iterator_name = &self.0;
        let iterator = ctx
            .data
            .get_mut(iterator_name)
            .ok_or_else(|| AquamarineError::VariableNotFound(iterator_name.clone()))?;
        let iterator =
            (crate::to_iterator!(iterator) as Result<(&mut Vec<SerdeValue>, &mut usize)>)?;

        if iterator.0.is_empty() || (*iterator.1 >= iterator.0.len() - 1) {
            // just do nothing to exit
            return Ok(());
        }

        *iterator.1 += 1;

        let next_instr = ctx
            .folds
            .get(iterator_name)
            .expect("folds should correspond to data")
            .instr_head
            .clone();

        next_instr.execute(ctx)?;

        // get the same fold state again because of borrow checker
        match ctx.data.get_mut(iterator_name) {
            Some(AValue::Iterator(_, cursor)) => *cursor -= 1,
            _ => unreachable!("iterator value shouldn't changed inside fold"),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::SerdeValue;

    use aqua_test_utils::create_aqua_vm;
    use aquamarine_vm::vec1::Vec1;
    use aquamarine_vm::HostExportedFunc;
    use aquamarine_vm::IValue;

    use serde_json::json;

    #[test]
    fn fold() {
        env_logger::init();

        let call_service: HostExportedFunc = Box::new(|_, args| -> Option<IValue> {
            println!("call_service called with {:?}\n", args);

            let arg = match &args[2] {
                IValue::String(str) => str,
                _ => unreachable!(),
            };

            let arg: Vec<String> = serde_json::from_str(arg).unwrap();

            Some(IValue::Record(
                Vec1::new(vec![IValue::S32(0), IValue::String(format!("{}", arg[0]))]).unwrap(),
            ))
        });
        let mut vm = create_aqua_vm(call_service);

        let script = String::from(
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
                script,
                String::from("{\"Iterable\": {\"serde-value\": [\"1\",\"2\",\"3\",\"4\",\"5\"]}}"),
            ]))
            .expect("call should be successful");

        let res: SerdeValue = serde_json::from_str(&res.data).unwrap();

        assert_eq!(
            res.get("acc").unwrap(),
            &json!({"accumulator": [1,2,3,4,5]})
        );
    }
}
