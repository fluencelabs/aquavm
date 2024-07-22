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

const ERROR_STARLARK_OTHER: i32 = 42;

pub fn execute(
    content: &str,
    args: &[(JValue, Rc<SecurityTetraplet>)],
) -> starlark::Result<JValue> {
    // unfortunately,
    // 1. AstModule is not clonable
    // 2. AstModule is consumed on evaluation
    //
    // for that reason, we have to parse the script on each invocation
    let ast: AstModule = AstModule::parse("dummy.star", content.to_owned(), &Dialect::Standard)?;

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

    let mut ctx = StarlarkCtx::default();
    ctx.args = args.to_owned();

    // We create an evaluator, which controls how evaluation occurs.
    let mut eval: Evaluator = Evaluator::new(&module);
    eval.extra = Some(&ctx as _);

    // And finally we evaluate the code using the evaluator.
    let res: Result<Value, _> = eval.eval_module(ast, &globals);
    match res {
        Ok(val) => {
            println!("val: {val}");
            ctx.clear_error();
            val.try_into()
        }
        Err(err) => {
            use starlark::ErrorKind::*;
            match err.kind() {
                Fail(e) => {
                    // the error is set by aquavm_module's `fail` function
                    println!("fail: {ctx:?} / {e:#?}");
                }
                Other(e) => {
                    eprintln!("Other: {e}");
                    ctx.set_error(ERROR_STARLARK_OTHER, e.to_string());
                }
                e => todo!("AquaVM uncatchable error: {:?}", e),
            }
            Err(err)
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
    fn set_error(&self, code: i32, message: String) {
        let mut guard = self.error.borrow_mut();
        *guard = Some((code, message));
    }

    fn clear_error(&self) {
        *self.error.borrow_mut() = None;
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
