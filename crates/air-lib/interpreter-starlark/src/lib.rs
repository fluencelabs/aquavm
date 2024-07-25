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

mod tetraplet;

use std::cell::RefCell;
use std::rc::Rc;

use air_interpreter_value::JValue;
use polyplets::SecurityTetraplet;
use starlark::any::ProvidesStaticType;
use starlark::environment::Globals;
use starlark::environment::GlobalsBuilder;
use starlark::environment::LibraryExtension;
use starlark::environment::Module;
use starlark::eval::Evaluator;
use starlark::starlark_module;
use starlark::syntax::AstModule;
use starlark::syntax::Dialect;
use starlark::values::typing::StarlarkNever;
use starlark::values::Value;

use crate::tetraplet::StarlarkSecurityTetraplet;

#[derive(thiserror::Error, Debug)]
pub enum ExecutionError {
    #[error("Starlark fail: {0}, {1}")]
    Fail(i32, String),
    #[error("Starlark value: {0}")]
    Value(String),
    #[error("Starlark function: {0}")]
    Function(String),
    // Should be a uncatchable error as it is broken script
    #[error("Starlark scope: {0}")]
    Scope(String),
    // Should be a uncatchable error as it is broken script
    #[error("Starlark lexer: {0}")]
    Lexer(String),
    // Should be a uncatchable error
    #[error("Starlark internal: {0}")]
    Internal(String),
    #[error("Starlark other: {0}")]
    Other(String),
}

impl ExecutionError {
    fn from_parser_error(err: starlark::Error) -> Self {
        use starlark::ErrorKind::*;
        match err.kind() {
            Fail(_) => unreachable!("Starlark parser is not expected to produce a Fail error"),
            Value(err) => Self::Value(err.to_string()),
            Function(err) => Self::Function(err.to_string()),
            Scope(err) => Self::Scope(err.to_string()),
            Lexer(err) => Self::Lexer(err.to_string()),
            Internal(err) => Self::Internal(err.to_string()),
            Other(err) => Self::Other(err.to_string()),
            _ => Self::Other(err.to_string()),
        }
    }
}

pub fn execute(
    content: &str,
    args: &[(JValue, Rc<SecurityTetraplet>)],
) -> Result<JValue, ExecutionError> {
    // unfortunately,
    // 1. AstModule is not clonable
    // 2. AstModule is consumed on evaluation
    //
    // for that reason, we have to parse the script on each invocation
    let ast: AstModule = AstModule::parse("dummy.star", content.to_owned(), &Dialect::Standard)
        .map_err(ExecutionError::from_parser_error)?;

    // we create a `Globals`, defining the standard library functions available;
    // the `standard` function uses those defined in the Starlark specification
    let globals: Globals = GlobalsBuilder::extended_by(&[
        LibraryExtension::Typing,
        LibraryExtension::Json,
        LibraryExtension::RecordType,
    ])
    // override `fail` and and add `get_value`/`get_tetraplet`
    .with(aquavm_module)
    .build();

    // We create a `Module`, which stores the global variables for our calculation.
    let module: Module = Module::new();

    let ctx = StarlarkCtx::new(args.to_owned());

    let res: Result<Value, _> = {
        // We create an evaluator, which controls how evaluation occurs.
        let mut eval: Evaluator = Evaluator::new(&module);
        eval.extra = Some(&ctx as _);

        // And finally we evaluate the code using the evaluator.
        eval.eval_module(ast, &globals)
    };

    // TODO TryInto may fail if `starlark::Value` serialization to `serde_json::Value` fails
    match res.and_then(TryInto::<JValue>::try_into) {
        Ok(val) => Ok(val),
        Err(err) => {
            use starlark::ErrorKind::*;
            match err.kind() {
                Fail(_e) => {
                    // the error is set by aquavm_module's `fail` function
                    // n.b.: `_e` is an opaque object, for that reason we use `Ctx` to get error's code and message
                    let (code, message) =
                        ctx.error.into_inner().expect("Starlark Ctx error is empty");
                    Err(ExecutionError::Fail(code, message))
                }
                Value(_) => Err(ExecutionError::Value(err.to_string())),
                Function(_) => Err(ExecutionError::Function(err.to_string())),
                Scope(_) => Err(ExecutionError::Scope(err.to_string())),
                Lexer(_) => Err(ExecutionError::Scope(err.to_string())),
                Internal(_) => Err(ExecutionError::Scope(err.to_string())),
                Other(_) => Err(ExecutionError::Scope(err.to_string())),
                _ => Err(ExecutionError::Scope(err.to_string())), // explicitly matches with the previous one
            }
        }
    }
}

// `ProvidesStaticType` is alternative to manually implementing AnyLifetime trait
// (in this case, it is derived automatically)
#[derive(Debug, Default, ProvidesStaticType)]
struct StarlarkCtx {
    error: RefCell<Option<(i32, String)>>,
    args: Vec<(JValue, Rc<SecurityTetraplet>)>,
}

impl StarlarkCtx {
    fn new(args: Vec<(JValue, Rc<SecurityTetraplet>)>) -> Self {
        Self {
            error: <_>::default(),
            args,
        }
    }

    fn set_error(&self, code: i32, message: String) {
        let mut guard = self.error.borrow_mut();
        *guard = Some((code, message));
    }
}

#[starlark_module]
fn aquavm_module(builder: &mut GlobalsBuilder) {
    fn fail(
        code: i32,
        message: String,
        eval: &mut Evaluator<'_, '_>,
    ) -> starlark::Result<StarlarkNever> {
        let ctx = eval
            .extra
            .expect("misconfigured starlark evaluator: `extra` expected");

        let ctx: &StarlarkCtx = ctx
            .downcast_ref()
            .expect("misconfigured starlark evaluator: `extra` of type Ctx expected");

        let error = anyhow::anyhow!("{}: {}", code, message);
        ctx.set_error(code, message);
        // we use `ErrorKind::Fail` to signal about our overridden `fail` call.
        // perhaps, `ErrorKind::Other` should be used instead.
        Err(starlark::Error::new(starlark::ErrorKind::Fail(error)))
    }

    fn get_value<'v>(index: usize, eval: &mut Evaluator<'v, '_>) -> anyhow::Result<Value<'v>> {
        let ctx = eval
            .extra
            .expect("misconfigured starlark evaluator: `extra` expected");

        let ctx: &StarlarkCtx = ctx
            .downcast_ref()
            .expect("misconfigured starlark evaluator: `extra` of type Ctx expected");

        let heap = eval.heap();
        match ctx.args.get(index) {
            Some((ref value, _tetraplet)) => Ok(heap.alloc(value)),
            None => anyhow::bail!("value index {index} not valid"),
        }
    }

    fn get_tetraplet<'v>(index: usize, eval: &mut Evaluator<'v, '_>) -> anyhow::Result<Value<'v>> {
        let ctx = eval
            .extra
            .expect("misconfigured starlark evaluator: `extra` expected");

        let ctx: &StarlarkCtx = ctx
            .downcast_ref()
            .expect("misconfigured starlark evaluator: `extra` of type Ctx expected");

        let heap = eval.heap();
        match ctx.args.get(index) {
            Some((_value, ref tetraplet)) => Ok(heap.alloc(StarlarkSecurityTetraplet::new(
                tetraplet,
                eval.frozen_heap(),
            ))),
            // TODO is it a catchable error?
            None => anyhow::bail!("value index {index} not valid"),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_value() {
        let tetraplet =
            SecurityTetraplet::new("my_peer", "my_service", "my_func", ".$.lens").into();
        let value: JValue = json!({
            "test": 42,
            "property": null,
        })
        .into();
        let script = "get_value(0)";

        let res = execute(script, &[(value.clone(), tetraplet)][..]).unwrap();
        assert_eq!(res, value);
    }

    #[test]
    fn test_value2() {
        let tetraplet =
            SecurityTetraplet::new("my_peer", "my_service", "my_func", ".$.lens").into();
        let value: JValue = json!({
            "test": 42,
            "property": null,
        })
        .into();
        let script = r#"get_value(0)["property"]"#;

        let res = execute(script, &[(value.clone(), tetraplet)]).unwrap();
        assert_eq!(res, JValue::Null);
    }

    #[test]
    fn test_value_eq_self() {
        let tetraplet =
            SecurityTetraplet::new("my_peer", "my_service", "my_func", ".$.lens").into();
        let value: JValue = json!({
            "test": 42,
            "property": null,
        })
        .into();
        let script = "get_value(0) == get_value(0)";

        let res = execute(script, &[(value, tetraplet)]).unwrap();
        assert_eq!(res, true);
    }

    #[test]
    fn test_value_eq_same() {
        let tetraplet: Rc<_> =
            SecurityTetraplet::new("my_peer", "my_service", "my_func", ".$.lens").into();
        let value: JValue = json!({
            "test": 42,
            "property": null,
        })
        .into();
        let script = "get_value(0) == get_value(1)";

        let res = execute(
            script,
            &[(value.clone(), tetraplet.clone()), (value, tetraplet)],
        )
        .unwrap();
        assert_eq!(res, true);
    }

    #[test]
    fn test_value_eq_different() {
        let tetraplet: Rc<_> =
            SecurityTetraplet::new("my_peer", "my_service", "my_func", ".$.lens").into();
        let value1: JValue = json!({
            "test": 42,
            "property": null,
        })
        .into();
        let value2: JValue = json!({
            "test": 48,
            "property": null,
        })
        .into();
        let script = "get_value(0) == get_value(1)";

        let res = execute(script, &[(value1, tetraplet.clone()), (value2, tetraplet)]).unwrap();
        assert_eq!(res, false);
    }

    #[test]
    fn test_escape_cannot_be_used_in_air_parser() {
        let script = r#"'test\#'"#;

        let res = execute(script, &[]).unwrap();
        assert_eq!(res, "test\\#");
    }

}
