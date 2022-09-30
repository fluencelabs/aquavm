// auto-generated: "lalrpop 0.19.8"
// sha3: aef9185028be9404fde0a89a1a02d5a5f4506210a3a7b0b5eaeeed8be83bb4cc
use crate::ast::*;
use crate::parser::ParserError;
use crate::parser::VariableValidator;
use crate::parser::Span;
use crate::parser::lexer::{AirPos, Token};
use air_lambda_parser::LambdaAST;
use lalrpop_util::ErrorRecovery;
use std::rc::Rc;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__AIR {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]

    use crate::ast::*;
    use crate::parser::ParserError;
    use crate::parser::VariableValidator;
    use crate::parser::Span;
    use crate::parser::lexer::{AirPos, Token};
    use air_lambda_parser::LambdaAST;
    use lalrpop_util::ErrorRecovery;
    use std::rc::Rc;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(Token<'input>),
        Variant1(bool),
        Variant2((&'input str, AirPos)),
        Variant3((&'input str, LambdaAST<'input>, AirPos)),
        Variant4(f64),
        Variant5(i64),
        Variant6(LambdaAST<'input>),
        Variant7(&'input str),
        Variant8(__lalrpop_util::ErrorRecovery<AirPos, Token<'input>, ParserError>),
        Variant9(Value<'input>),
        Variant10(alloc::vec::Vec<Value<'input>>),
        Variant11(AirPos),
        Variant12(Box<Instruction<'input>>),
        Variant13(ApArgument<'input>),
        Variant14(ApResult<'input>),
        Variant15(Vec<Value<'input>>),
        Variant16(CallInstrValue<'input>),
        Variant17(CallOutputValue<'input>),
        Variant18(core::option::Option<CallOutputValue<'input>>),
        Variant19(CanonStream<'input>),
        Variant20(Fail<'input>),
        Variant21(FoldScalarIterable<'input>),
        Variant22(core::option::Option<Box<Instruction<'input>>>),
        Variant23(NewArgument<'input>),
        Variant24(Number),
        Variant25(Stream<'input>),
        Variant26(Triplet<'input>),
    }
    const __ACTION: &[i16] = &[
        // State 0
        36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37,
        // State 1
        0, 0, 42, 0, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 0, 0, 54, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 58, 59, 0, 0, 60, 0, 0, 61, 62, 63, 64, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 58, 59, 0, 0, 60, 0, 0, 61, 62, 63, 64, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 67, 0, 68, 0, 69, 0, 0, 70, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 73, 0, 0, 74, 0, 0, 0, 0, 0, 0, 0, 75, 76, 77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 79, 0, 80, 81, 82, 46, 47, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 79, 0, 80, 81, 82, 46, 47, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 94, 0, 0, 0, 0, 0, 0, 0, 95, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37,
        // State 10
        36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37,
        // State 11
        36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100, 0, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 79, 0, 80, 81, 82, 46, 47, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 79, 0, 80, 81, 82, 46, 47, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37,
        // State 18
        36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37,
        // State 19
        36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37,
        // State 20
        36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37,
        // State 21
        0, 0, 0, 0, 0, 58, 59, 0, 0, 60, 0, 0, 61, 62, 63, 64, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 117, 0, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 79, 121, 80, 81, 82, 46, 47, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 123, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37,
        // State 26
        36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37,
        // State 27
        36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37,
        // State 28
        36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37,
        // State 29
        0, 0, 0, 0, 0, 58, 59, 0, 0, 60, 0, 0, 61, 62, 63, 64, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 79, 134, 80, 81, 82, 46, 47, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        36, 137, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37,
        // State 32
        36, 139, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 4, 5, 6, 7, 8, 38, 9, 39, 40, 10, 11, 12, 0,
        // State 36
        -68, -68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -68,
        // State 37
        0, 93, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        -75, 0, -75, -75, -75, -75, -75, -75, -75, -75, -75, -75, -75, -75, -75, -75, -75, -75, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -75,
        // State 46
        -74, 0, -74, -74, -74, -74, -74, -74, -74, -74, -74, -74, -74, -74, -74, -74, -74, -74, -74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -74,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        -76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        -33, -33, 0, 0, 0, -33, -33, 0, 0, -33, 0, 0, -33, -33, -33, -33, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        -34, -34, 0, 0, 0, -34, -34, 0, 0, -34, 0, 0, -34, -34, -34, -34, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        -27, -27, 0, 0, 0, -27, -27, 0, 0, -27, 0, 0, -27, -27, -27, -27, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        -28, -28, 0, 0, 0, -28, -28, 0, 0, -28, 0, 0, -28, -28, -28, -28, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        -29, -29, 0, 0, 0, -29, -29, 0, 0, -29, 0, 0, -29, -29, -29, -29, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        -30, -30, 0, 0, 0, -30, -30, 0, 0, -30, 0, 0, -30, -30, -30, -30, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        -31, -31, 0, 0, 0, -31, -31, 0, 0, -31, 0, 0, -31, -31, -31, -31, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        -32, -32, 0, 0, 0, -32, -32, 0, 0, -32, 0, 0, -32, -32, -32, -32, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 106, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 75
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 77
        -86, 0, -86, -86, -86, -86, -86, -86, -86, -86, -86, -86, -86, -86, -86, -86, -86, -86, -86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -86,
        // State 78
        0, 0, 0, 107, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        -87, 0, -87, -87, -87, -87, -87, -87, -87, -87, -87, -87, -87, -87, -87, -87, -87, -87, -87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -87,
        // State 80
        -93, 0, -93, -93, -93, -93, -93, -93, -93, -93, -93, -93, -93, -93, -93, -93, -93, -93, -93, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -93,
        // State 81
        -94, 0, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -94,
        // State 82
        -80, 0, -80, -80, -80, -80, -80, -80, -80, -80, -80, -80, -80, -80, -80, -80, -80, -80, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -80,
        // State 83
        -81, 0, -81, -81, -81, -81, -81, -81, -81, -81, -81, -81, -81, -81, -81, -81, -81, -81, -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -81,
        // State 84
        -82, 0, -82, -82, -82, -82, -82, -82, -82, -82, -82, -82, -82, -82, -82, -82, -82, -82, -82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -82,
        // State 85
        -83, 0, -83, -83, -83, -83, -83, -83, -83, -83, -83, -83, -83, -83, -83, -83, -83, -83, -83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -83,
        // State 86
        -89, 0, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -89,
        // State 87
        -90, 0, -90, -90, -90, -90, -90, -90, -90, -90, -90, -90, -90, -90, -90, -90, -90, -90, -90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -90,
        // State 88
        -91, 0, -91, -91, -91, -91, -91, -91, -91, -91, -91, -91, -91, -91, -91, -91, -91, -91, -91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -91,
        // State 89
        -92, 0, -92, -92, -92, -92, -92, -92, -92, -92, -92, -92, -92, -92, -92, -92, -92, -92, -92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -92,
        // State 90
        -85, 0, -85, -85, -85, -85, -85, -85, -85, -85, -85, -85, -85, -85, -85, -85, -85, -85, -85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -85,
        // State 91
        -84, 0, -84, -84, -84, -84, -84, -84, -84, -84, -84, -84, -84, -84, -84, -84, -84, -84, -84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -84,
        // State 92
        -56, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -56,
        // State 93
        -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73,
        // State 94
        -71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -71,
        // State 95
        -72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -72,
        // State 96
        0, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 97
        -57, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57,
        // State 98
        0, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 99
        0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 100
        0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 101
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 102
        0, 0, 0, 0, 0, -78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 103
        -59, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -59,
        // State 104
        0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 105
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 106
        -88, 0, -88, -88, -88, -88, -88, -88, -88, -88, -88, -88, -88, -88, -88, -88, -88, -88, -88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -88,
        // State 107
        0, 126, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 108
        -64, -64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -64,
        // State 109
        0, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 110
        0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 111
        0, 129, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 112
        -53, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -53,
        // State 113
        0, 0, 0, 0, 0, -77, -77, 0, 0, -77, 0, 0, -77, -77, -77, -77, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 114
        0, 132, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 115
        -51, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51,
        // State 116
        0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 117
        0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 118
        0, 0, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 119
        0, 0, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 120
        0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 121
        0, 135, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 122
        0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 123
        0, 140, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 124
        0, 141, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 125
        -58, -58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -58,
        // State 126
        -55, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -55,
        // State 127
        -54, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -54,
        // State 128
        -65, -65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -65,
        // State 129
        0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 130
        0, 142, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 131
        -50, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -50,
        // State 132
        0, 0, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 133
        0, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 134
        -52, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52,
        // State 135
        0, 143, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 136
        -61, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -61,
        // State 137
        0, 144, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 138
        -63, -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -63,
        // State 139
        -66, -66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -66,
        // State 140
        -67, -67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -67,
        // State 141
        0, 0, -79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 142
        -60, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60,
        // State 143
        -62, -62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -62,
    ];
    fn __action(state: i16, integer: usize) -> i16 {
        __ACTION[(state as usize) * 34 + integer]
    }
    const __EOF_ACTION: &[i16] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        -95,
        // State 34
        -8,
        // State 35
        0,
        // State 36
        -68,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
        // State 74
        0,
        // State 75
        0,
        // State 76
        0,
        // State 77
        0,
        // State 78
        0,
        // State 79
        0,
        // State 80
        0,
        // State 81
        0,
        // State 82
        0,
        // State 83
        0,
        // State 84
        0,
        // State 85
        0,
        // State 86
        0,
        // State 87
        0,
        // State 88
        0,
        // State 89
        0,
        // State 90
        0,
        // State 91
        0,
        // State 92
        -56,
        // State 93
        0,
        // State 94
        0,
        // State 95
        0,
        // State 96
        0,
        // State 97
        -57,
        // State 98
        0,
        // State 99
        0,
        // State 100
        0,
        // State 101
        0,
        // State 102
        0,
        // State 103
        -59,
        // State 104
        0,
        // State 105
        0,
        // State 106
        0,
        // State 107
        0,
        // State 108
        -64,
        // State 109
        0,
        // State 110
        0,
        // State 111
        0,
        // State 112
        -53,
        // State 113
        0,
        // State 114
        0,
        // State 115
        -51,
        // State 116
        0,
        // State 117
        0,
        // State 118
        0,
        // State 119
        0,
        // State 120
        0,
        // State 121
        0,
        // State 122
        0,
        // State 123
        0,
        // State 124
        0,
        // State 125
        -58,
        // State 126
        -55,
        // State 127
        -54,
        // State 128
        -65,
        // State 129
        0,
        // State 130
        0,
        // State 131
        -50,
        // State 132
        0,
        // State 133
        0,
        // State 134
        -52,
        // State 135
        0,
        // State 136
        -61,
        // State 137
        0,
        // State 138
        -63,
        // State 139
        -66,
        // State 140
        -67,
        // State 141
        0,
        // State 142
        -60,
        // State 143
        -62,
    ];
    fn __goto(state: i16, nt: usize) -> i16 {
        match nt {
            2 => 30,
            5 => 33,
            6 => 12,
            7 => 98,
            8 => match state {
                30 => 132,
                _ => 118,
            },
            9 => 22,
            10 => match state {
                2 => 55,
                21 => 113,
                29 => 129,
                _ => 14,
            },
            11 => 114,
            13 => 121,
            14 => 65,
            15 => 71,
            16 => 130,
            17 => match state {
                10 => 19,
                11 => 20,
                25 => 31,
                26 => 32,
                0 => 34,
                17 => 107,
                18 => 109,
                19 => 110,
                20 => 111,
                27 => 123,
                28 => 124,
                31 => 135,
                32 => 137,
                _ => 18,
            },
            19 => 17,
            20 => match state {
                1 => 40,
                _ => 77,
            },
            21 => 56,
            22 => 29,
            23 => 24,
            24 => 13,
            25 => match state {
                6 => 15,
                7 => 16,
                15 => 27,
                16 => 28,
                _ => 119,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i16) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""(""###,
            r###"")""###,
            r###""[""###,
            r###""]""###,
            r###"Boolean"###,
            r###"CanonStream"###,
            r###"CanonStreamWithLambda"###,
            r###"F64"###,
            r###"I64"###,
            r###"InitPeerId"###,
            r###"LastError"###,
            r###"LastErrorWithLambda"###,
            r###"Literal"###,
            r###"Scalar"###,
            r###"ScalarWithLambda"###,
            r###"Stream"###,
            r###"StreamWithLambda"###,
            r###"TTL"###,
            r###"Timestamp"###,
            r###"ap"###,
            r###"call"###,
            r###"canon"###,
            r###"fail"###,
            r###"fold"###,
            r###"match_"###,
            r###"mismatch"###,
            r###"never"###,
            r###"new"###,
            r###"next"###,
            r###"null"###,
            r###"par"###,
            r###"seq"###,
            r###"xor"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'err, 'input, 'v>
    where 'input: 'err, 'input: 'v
    {
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __phantom: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    }
    impl<'err, 'input, 'v> __state_machine::ParserDefinition for __StateMachine<'err, 'input, 'v>
    where 'input: 'err, 'input: 'v
    {
        type Location = AirPos;
        type Error = ParserError;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Box<Instruction<'input>>;
        type StateIndex = i16;
        type Action = i16;
        type ReduceIndex = i16;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&(), &(), &())>)
        }

        #[inline]
        fn action(&self, state: i16, integer: usize) -> i16 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i16) -> i16 {
            __action(state, 34 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i16) -> i16 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i16, nt: usize) -> i16 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&(), &(), &())>)
        }

        fn expected_tokens(&self, state: i16) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            true
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            __Symbol::Variant8(recovery)
        }

        fn reduce(
            &mut self,
            action: i16,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i16>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                self.errors,
                self.validator,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&(), &(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i16) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&(), &(), &())>)
        }
    }
    fn __token_to_integer<
        'err,
        'input,
        'v,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token::OpenRoundBracket if true => Some(0),
            Token::CloseRoundBracket if true => Some(1),
            Token::OpenSquareBracket if true => Some(2),
            Token::CloseSquareBracket if true => Some(3),
            Token::Boolean(_) if true => Some(4),
            Token::CanonStream { name: _, position: _ } if true => Some(5),
            Token::CanonStreamWithLambda { name: _, lambda: _, position: _ } if true => Some(6),
            Token::F64(_) if true => Some(7),
            Token::I64(_) if true => Some(8),
            Token::InitPeerId if true => Some(9),
            Token::LastError if true => Some(10),
            Token::LastErrorWithLambda(_) if true => Some(11),
            Token::StringLiteral(_) if true => Some(12),
            Token::Scalar { name: _, position: _ } if true => Some(13),
            Token::ScalarWithLambda { name: _, lambda: _, position: _ } if true => Some(14),
            Token::Stream { name: _, position: _ } if true => Some(15),
            Token::StreamWithLambda { name: _, lambda: _, position: _ } if true => Some(16),
            Token::TTL if true => Some(17),
            Token::Timestamp if true => Some(18),
            Token::Ap if true => Some(19),
            Token::Call if true => Some(20),
            Token::Canon if true => Some(21),
            Token::Fail if true => Some(22),
            Token::Fold if true => Some(23),
            Token::Match if true => Some(24),
            Token::MisMatch if true => Some(25),
            Token::Never if true => Some(26),
            Token::New if true => Some(27),
            Token::Next if true => Some(28),
            Token::Null if true => Some(29),
            Token::Par if true => Some(30),
            Token::Seq if true => Some(31),
            Token::Xor if true => Some(32),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'err,
        'input,
        'v,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 9 | 10 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 => __Symbol::Variant0(__token),
            4 => match __token {
                Token::Boolean(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            5 | 13 | 15 => match __token {
                Token::CanonStream { name: __tok0, position: __tok1 } | Token::Scalar { name: __tok0, position: __tok1 } | Token::Stream { name: __tok0, position: __tok1 } if true => __Symbol::Variant2((__tok0, __tok1)),
                _ => unreachable!(),
            },
            6 | 14 | 16 => match __token {
                Token::CanonStreamWithLambda { name: __tok0, lambda: __tok1, position: __tok2 } | Token::ScalarWithLambda { name: __tok0, lambda: __tok1, position: __tok2 } | Token::StreamWithLambda { name: __tok0, lambda: __tok1, position: __tok2 } if true => __Symbol::Variant3((__tok0, __tok1, __tok2)),
                _ => unreachable!(),
            },
            7 => match __token {
                Token::F64(__tok0) if true => __Symbol::Variant4(__tok0),
                _ => unreachable!(),
            },
            8 => match __token {
                Token::I64(__tok0) if true => __Symbol::Variant5(__tok0),
                _ => unreachable!(),
            },
            11 => match __token {
                Token::LastErrorWithLambda(__tok0) if true => __Symbol::Variant6(__tok0),
                _ => unreachable!(),
            },
            12 => match __token {
                Token::StringLiteral(__tok0) if true => __Symbol::Variant7(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'err,
        'input,
        'v,
    >(
        __reduce_index: i16,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'err, 'input, 'v>>
    where
        'input: 'err,
        'input: 'v,
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 6,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 9,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 9,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 12,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 14,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 15,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 17,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 17,
                }
            }
            51 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 17,
                }
            }
            52 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 17,
                }
            }
            53 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 17,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 17,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 17,
                }
            }
            56 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 17,
                }
            }
            57 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 17,
                }
            }
            58 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 17,
                }
            }
            59 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 17,
                }
            }
            60 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 17,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 17,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 17,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 17,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 17,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 17,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 17,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 18,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            72 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            73 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            74 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            75 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            76 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            77 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            78 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 24,
                }
            }
            79 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            80 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            81 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            82 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            83 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            84 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            85 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            86 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            87 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 25,
                }
            }
            88 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            89 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            90 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            91 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            92 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            93 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            94 => __state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct AIRParser {
        _priv: (),
    }

    impl AIRParser {
        pub fn new() -> AIRParser {
            AIRParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'err,
            'input,
            'v,
            __TOKEN: __ToTriple<'err, 'input, 'v, >,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            input: &'input str,
            errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
            validator: &'v mut VariableValidator<'input>,
            __tokens0: __TOKENS,
        ) -> Result<Box<Instruction<'input>>, __lalrpop_util::ParseError<AirPos, Token<'input>, ParserError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    errors,
                    validator,
                    __phantom: core::marker::PhantomData::<(&(), &(), &())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __error_state: i16,
        __states: & [i16],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.push(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&(), &(), &())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    pub(crate) fn __reduce<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __action: i16,
        __lookahead_start: Option<&AirPos>,
        __states: &mut alloc::vec::Vec<i16>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> Option<Result<Box<Instruction<'input>>,__lalrpop_util::ParseError<AirPos, Token<'input>, ParserError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            1 => {
                __reduce1(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            2 => {
                __reduce2(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            3 => {
                __reduce3(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            4 => {
                __reduce4(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            5 => {
                __reduce5(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            6 => {
                __reduce6(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            7 => {
                __reduce7(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            8 => {
                __reduce8(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            9 => {
                __reduce9(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            10 => {
                __reduce10(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            11 => {
                __reduce11(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            12 => {
                __reduce12(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            13 => {
                __reduce13(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            14 => {
                __reduce14(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            15 => {
                __reduce15(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            16 => {
                __reduce16(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            17 => {
                __reduce17(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            18 => {
                __reduce18(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            19 => {
                __reduce19(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            20 => {
                __reduce20(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            21 => {
                __reduce21(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            22 => {
                __reduce22(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            23 => {
                __reduce23(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            24 => {
                __reduce24(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            25 => {
                __reduce25(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            26 => {
                __reduce26(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            27 => {
                __reduce27(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            28 => {
                __reduce28(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            29 => {
                __reduce29(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            30 => {
                __reduce30(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            31 => {
                __reduce31(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            32 => {
                __reduce32(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            33 => {
                __reduce33(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            34 => {
                __reduce34(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            35 => {
                __reduce35(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            36 => {
                __reduce36(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            37 => {
                __reduce37(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            38 => {
                __reduce38(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            39 => {
                __reduce39(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            40 => {
                __reduce40(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            41 => {
                __reduce41(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            42 => {
                __reduce42(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            43 => {
                __reduce43(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            44 => {
                __reduce44(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            45 => {
                __reduce45(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            46 => {
                __reduce46(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            47 => {
                __reduce47(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            48 => {
                __reduce48(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            49 => {
                __reduce49(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            50 => {
                __reduce50(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            51 => {
                __reduce51(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            52 => {
                __reduce52(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            53 => {
                __reduce53(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            54 => {
                __reduce54(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            55 => {
                __reduce55(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            56 => {
                __reduce56(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            57 => {
                __reduce57(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            58 => {
                __reduce58(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            59 => {
                __reduce59(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            60 => {
                __reduce60(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            61 => {
                __reduce61(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            62 => {
                __reduce62(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            63 => {
                __reduce63(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            64 => {
                __reduce64(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            65 => {
                __reduce65(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            66 => {
                __reduce66(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            67 => {
                __reduce67(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            68 => {
                __reduce68(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            69 => {
                __reduce69(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            70 => {
                __reduce70(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            71 => {
                __reduce71(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            72 => {
                __reduce72(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            73 => {
                __reduce73(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            74 => {
                __reduce74(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            75 => {
                __reduce75(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            76 => {
                __reduce76(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            77 => {
                __reduce77(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            78 => {
                __reduce78(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            79 => {
                __reduce79(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            80 => {
                __reduce80(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            81 => {
                __reduce81(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            82 => {
                __reduce82(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            83 => {
                __reduce83(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            84 => {
                __reduce84(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            85 => {
                __reduce85(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            86 => {
                __reduce86(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            87 => {
                __reduce87(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            88 => {
                __reduce88(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            89 => {
                __reduce89(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            90 => {
                __reduce90(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            91 => {
                __reduce91(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            92 => {
                __reduce92(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            93 => {
                __reduce93(input, errors, validator, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &(), &())>)
            }
            94 => {
                // __AIR = AIR => ActionFn(0);
                let __sym0 = __pop_Variant12(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, errors, validator, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, (&'input str, AirPos), AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, (&'input str, LambdaAST<'input>, AirPos), AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, AirPos, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, ApArgument<'input>, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, ApResult<'input>, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, Box<Instruction<'input>>, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, CallInstrValue<'input>, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, CallOutputValue<'input>, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, CanonStream<'input>, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant19(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, Fail<'input>, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant20(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, FoldScalarIterable<'input>, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant21(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, LambdaAST<'input>, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant23<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, NewArgument<'input>, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant23(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant24<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, Number, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant24(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant25<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, Stream<'input>, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant25(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, Token<'input>, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant26<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, Triplet<'input>, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant26(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, Value<'input>, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, Vec<Value<'input>>, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, __lalrpop_util::ErrorRecovery<AirPos, Token<'input>, ParserError>, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, alloc::vec::Vec<Value<'input>>, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, bool, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, core::option::Option<Box<Instruction<'input>>>, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant22(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, core::option::Option<CallOutputValue<'input>>, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, f64, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, i64, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>
    ) -> (AirPos, &'input str, AirPos)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // (<Arg>) = Arg => ActionFn(82);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action82::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // (<Arg>)* =  => ActionFn(80);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action80::<>(input, errors, validator, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // (<Arg>)* = (<Arg>)+ => ActionFn(81);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action81::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // (<Arg>)+ = Arg => ActionFn(91);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action91::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce4<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // (<Arg>)+ = (<Arg>)+, Arg => ActionFn(92);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action92::<>(input, errors, validator, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce5<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(88);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action88::<>(input, errors, validator, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 3)
    }
    pub(crate) fn __reduce6<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(85);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action85::<>(input, errors, validator, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // AIR = Instr => ActionFn(1);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce8<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = InitPeerId => ActionFn(65);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action65::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce9<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = LastError => ActionFn(66);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action66::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce10<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = LastErrorWithLambda => ActionFn(67);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce11<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = Timestamp => ActionFn(68);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action68::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce12<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = TTL => ActionFn(69);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action69::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce13<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = Literal => ActionFn(70);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce14<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = Number => ActionFn(71);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action71::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce15<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = Boolean => ActionFn(72);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce16<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = "[", "]" => ActionFn(73);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action73::<>(input, errors, validator, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce17<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = Scalar => ActionFn(74);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action74::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce18<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = ScalarWithLambda => ActionFn(75);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action75::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce19<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = CanonStream => ActionFn(76);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action76::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce20<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = CanonStreamWithLambda => ActionFn(77);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action77::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce21<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApResult = Scalar => ActionFn(20);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce22<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApResult = Stream => ActionFn(21);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce23<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Arg = Value => ActionFn(49);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce24<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Args = "[", "]" => ActionFn(93);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action93::<>(input, errors, validator, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce25<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Args = "[", (<Arg>)+, "]" => ActionFn(94);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action94::<>(input, errors, validator, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 9)
    }
    pub(crate) fn __reduce26<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // CallInstrValue = InitPeerId => ActionFn(36);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce27<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // CallInstrValue = Literal => ActionFn(37);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce28<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // CallInstrValue = Scalar => ActionFn(38);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce29<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // CallInstrValue = ScalarWithLambda => ActionFn(39);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce30<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // CallInstrValue = Stream => ActionFn(40);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce31<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // CallInstrValue = StreamWithLambda => ActionFn(41);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce32<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // CallInstrValue = CanonStream => ActionFn(42);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce33<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // CallInstrValue = CanonStreamWithLambda => ActionFn(43);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce34<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // CallOutput = Scalar => ActionFn(22);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce35<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // CallOutput = Stream => ActionFn(23);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce36<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // CallOutput? = CallOutput => ActionFn(86);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action86::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce37<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // CallOutput? =  => ActionFn(87);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action87::<>(input, errors, validator, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (0, 12)
    }
    pub(crate) fn __reduce38<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // CanonStreamArgument = CanonStream => ActionFn(79);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action79::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce39<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // FailBody = Scalar => ActionFn(24);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce40<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // FailBody = ScalarWithLambda => ActionFn(25);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce41<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // FailBody = I64, Literal => ActionFn(26);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action26::<>(input, errors, validator, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce42<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // FailBody = CanonStreamWithLambda => ActionFn(27);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce43<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // FailBody = LastError => ActionFn(105);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action105::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce44<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // FoldScalarIterable = Scalar => ActionFn(29);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce45<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // FoldScalarIterable = ScalarWithLambda => ActionFn(30);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce46<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // FoldScalarIterable = CanonStream => ActionFn(31);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce47<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // FoldScalarIterable = "[", "]" => ActionFn(32);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action32::<>(input, errors, validator, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (2, 15)
    }
    pub(crate) fn __reduce48<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Function = CallInstrValue => ActionFn(33);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce49<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", call, Triplet, Args, CallOutput, ")" => ActionFn(115);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant17(__symbols);
        let __sym3 = __pop_Variant15(__symbols);
        let __sym2 = __pop_Variant26(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action115::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (6, 17)
    }
    pub(crate) fn __reduce50<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", call, Triplet, Args, ")" => ActionFn(116);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant15(__symbols);
        let __sym2 = __pop_Variant26(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action116::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 17)
    }
    pub(crate) fn __reduce51<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", canon, CallInstrValue, StreamArgument, CanonStreamArgument, ")" => ActionFn(107);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant19(__symbols);
        let __sym3 = __pop_Variant25(__symbols);
        let __sym2 = __pop_Variant16(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action107::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (6, 17)
    }
    pub(crate) fn __reduce52<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", ap, ApArgument, ApResult, ")" => ActionFn(108);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant14(__symbols);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action108::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 17)
    }
    pub(crate) fn __reduce53<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", seq, Instr, Instr, ")" => ActionFn(5);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action5::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 17)
    }
    pub(crate) fn __reduce54<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", par, Instr, Instr, ")" => ActionFn(6);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action6::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 17)
    }
    pub(crate) fn __reduce55<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", never, ")" => ActionFn(7);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action7::<>(input, errors, validator, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce56<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", null, ")" => ActionFn(8);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8::<>(input, errors, validator, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce57<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", new, NewArgument, Instr, ")" => ActionFn(109);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant23(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action109::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 17)
    }
    pub(crate) fn __reduce58<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", fail, FailBody, ")" => ActionFn(10);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant20(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action10::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 17)
    }
    pub(crate) fn __reduce59<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", fold, FoldScalarIterable, Scalar, Instr, Instr, ")" => ActionFn(117);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant12(__symbols);
        let __sym4 = __pop_Variant12(__symbols);
        let __sym3 = __pop_Variant2(__symbols);
        let __sym2 = __pop_Variant21(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action117::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (7, 17)
    }
    pub(crate) fn __reduce60<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", fold, FoldScalarIterable, Scalar, Instr, ")" => ActionFn(118);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant12(__symbols);
        let __sym3 = __pop_Variant2(__symbols);
        let __sym2 = __pop_Variant21(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action118::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (6, 17)
    }
    pub(crate) fn __reduce61<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", fold, Stream, Scalar, Instr, Instr, ")" => ActionFn(119);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant12(__symbols);
        let __sym4 = __pop_Variant12(__symbols);
        let __sym3 = __pop_Variant2(__symbols);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action119::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (7, 17)
    }
    pub(crate) fn __reduce62<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", fold, Stream, Scalar, Instr, ")" => ActionFn(120);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant12(__symbols);
        let __sym3 = __pop_Variant2(__symbols);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action120::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (6, 17)
    }
    pub(crate) fn __reduce63<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", next, Scalar, ")" => ActionFn(112);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action112::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 17)
    }
    pub(crate) fn __reduce64<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", xor, Instr, Instr, ")" => ActionFn(14);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action14::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 17)
    }
    pub(crate) fn __reduce65<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", match_, Value, Value, Instr, ")" => ActionFn(113);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant12(__symbols);
        let __sym3 = __pop_Variant9(__symbols);
        let __sym2 = __pop_Variant9(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action113::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (6, 17)
    }
    pub(crate) fn __reduce66<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", mismatch, Value, Value, Instr, ")" => ActionFn(114);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant12(__symbols);
        let __sym3 = __pop_Variant9(__symbols);
        let __sym2 = __pop_Variant9(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action114::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (6, 17)
    }
    pub(crate) fn __reduce67<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = error => ActionFn(17);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce68<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr? = Instr => ActionFn(83);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action83::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce69<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr? =  => ActionFn(84);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action84::<>(input, errors, validator, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (0, 18)
    }
    pub(crate) fn __reduce70<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // NewArgument = Scalar => ActionFn(44);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce71<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // NewArgument = Stream => ActionFn(45);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce72<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // NewArgument = CanonStream => ActionFn(46);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce73<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Number = I64 => ActionFn(47);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce74<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Number = F64 => ActionFn(48);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action48::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce75<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // PeerId = CallInstrValue => ActionFn(34);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce76<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ServiceId = CallInstrValue => ActionFn(35);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce77<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // StreamArgument = Stream => ActionFn(78);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action78::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce78<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Triplet = PeerId, "(", ServiceId, Function, ")" => ActionFn(19);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant16(__symbols);
        let __sym2 = __pop_Variant16(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action19::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (5, 24)
    }
    pub(crate) fn __reduce79<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = InitPeerId => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce80<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = LastError => ActionFn(51);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce81<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = LastErrorWithLambda => ActionFn(52);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action52::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce82<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = Literal => ActionFn(53);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce83<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = Timestamp => ActionFn(54);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action54::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce84<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = TTL => ActionFn(55);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action55::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce85<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = Number => ActionFn(56);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action56::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce86<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = Boolean => ActionFn(57);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action57::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce87<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = "[", "]" => ActionFn(58);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action58::<>(input, errors, validator, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 25)
    }
    pub(crate) fn __reduce88<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = Scalar => ActionFn(59);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action59::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce89<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = ScalarWithLambda => ActionFn(60);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action60::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce90<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = Stream => ActionFn(61);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action61::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce91<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = StreamWithLambda => ActionFn(62);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action62::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce92<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = CanonStream => ActionFn(63);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action63::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce93<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&AirPos>,
        __symbols: &mut alloc::vec::Vec<(AirPos,__Symbol<'input>,AirPos)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = CanonStreamWithLambda => ActionFn(64);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action64::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 25)
    }
}
pub use self::__parse__AIR::AIRParser;

#[allow(unused_variables)]
fn __action0<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, Box<Instruction<'input>>, AirPos),
) -> Box<Instruction<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action1<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, Box<Instruction<'input>>, AirPos),
) -> Box<Instruction<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action2<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, left, _): (AirPos, AirPos, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, triplet, _): (AirPos, Triplet<'input>, AirPos),
    (_, args, _): (AirPos, Vec<Value<'input>>, AirPos),
    (_, output, _): (AirPos, core::option::Option<CallOutputValue<'input>>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, right, _): (AirPos, AirPos, AirPos),
) -> Box<Instruction<'input>>
{
    {
        let args = Rc::new(args);
        let output = output.unwrap_or(CallOutputValue::None);
        let call = Call::new(triplet, args, output);
        let span = Span::new(left, right);

        validator.met_call(&call, span);

        Box::new(Instruction::Call(call))
    }
}

#[allow(unused_variables)]
fn __action3<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, left, _): (AirPos, AirPos, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, peer_pk, _): (AirPos, CallInstrValue<'input>, AirPos),
    (_, stream, _): (AirPos, Stream<'input>, AirPos),
    (_, canon_stream, _): (AirPos, CanonStream<'input>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, right, _): (AirPos, AirPos, AirPos),
) -> Box<Instruction<'input>>
{
    {
        let canon = Canon::new(peer_pk, stream, canon_stream);

        let span = Span::new(left, right);
        validator.met_canon(&canon, span);

        Box::new(Instruction::Canon(canon))
    }
}

#[allow(unused_variables)]
fn __action4<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, left, _): (AirPos, AirPos, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, arg, _): (AirPos, ApArgument<'input>, AirPos),
    (_, result, _): (AirPos, ApResult<'input>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, right, _): (AirPos, AirPos, AirPos),
) -> Box<Instruction<'input>>
{
    {
        let apply = Ap::new(arg, result);

        let span = Span::new(left, right);
        validator.met_ap(&apply, span);

        Box::new(Instruction::Ap(apply))
    }
}

#[allow(unused_variables)]
fn __action5<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, l, _): (AirPos, Box<Instruction<'input>>, AirPos),
    (_, r, _): (AirPos, Box<Instruction<'input>>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
) -> Box<Instruction<'input>>
{
    Box::new(Instruction::Seq(Seq::new(l, r)))
}

#[allow(unused_variables)]
fn __action6<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, l, _): (AirPos, Box<Instruction<'input>>, AirPos),
    (_, r, _): (AirPos, Box<Instruction<'input>>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
) -> Box<Instruction<'input>>
{
    Box::new(Instruction::Par(Par::new(l, r)))
}

#[allow(unused_variables)]
fn __action7<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, Token<'input>, AirPos),
    (_, __1, _): (AirPos, Token<'input>, AirPos),
    (_, __2, _): (AirPos, Token<'input>, AirPos),
) -> Box<Instruction<'input>>
{
    Box::new(Instruction::Never(Never))
}

#[allow(unused_variables)]
fn __action8<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, Token<'input>, AirPos),
    (_, __1, _): (AirPos, Token<'input>, AirPos),
    (_, __2, _): (AirPos, Token<'input>, AirPos),
) -> Box<Instruction<'input>>
{
    Box::new(Instruction::Null(Null))
}

#[allow(unused_variables)]
fn __action9<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, left, _): (AirPos, AirPos, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, argument, _): (AirPos, NewArgument<'input>, AirPos),
    (_, instruction, _): (AirPos, Box<Instruction<'input>>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, right, _): (AirPos, AirPos, AirPos),
) -> Box<Instruction<'input>>
{
    {
        let span = Span::new(left, right);
        let new = New::new(argument, instruction, span);

        validator.met_new(&new, span);

        Box::new(Instruction::New(new))
    }
}

#[allow(unused_variables)]
fn __action10<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, fail_body, _): (AirPos, Fail<'input>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
) -> Box<Instruction<'input>>
{
    {
        Box::new(Instruction::Fail(fail_body))
    }
}

#[allow(unused_variables)]
fn __action11<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, left, _): (AirPos, AirPos, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, iterable, _): (AirPos, FoldScalarIterable<'input>, AirPos),
    (_, iterator, _): (AirPos, (&'input str, AirPos), AirPos),
    (_, instruction, _): (AirPos, Box<Instruction<'input>>, AirPos),
    (_, last_instruction, _): (AirPos, core::option::Option<Box<Instruction<'input>>>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, right, _): (AirPos, AirPos, AirPos),
) -> Box<Instruction<'input>>
{
    {
        let iterator = Scalar::new(iterator.0, iterator.1);
        let span = Span::new(left, right);
        let fold = FoldScalar::new(iterable, iterator, *instruction, last_instruction.map(|v| *v), span);

        validator.met_fold_scalar(&fold, span);

        Box::new(Instruction::FoldScalar(fold))
    }
}

#[allow(unused_variables)]
fn __action12<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, left, _): (AirPos, AirPos, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, stream, _): (AirPos, (&'input str, AirPos), AirPos),
    (_, iterator, _): (AirPos, (&'input str, AirPos), AirPos),
    (_, instruction, _): (AirPos, Box<Instruction<'input>>, AirPos),
    (_, last_instruction, _): (AirPos, core::option::Option<Box<Instruction<'input>>>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, right, _): (AirPos, AirPos, AirPos),
) -> Box<Instruction<'input>>
{
    {
        let iterable = Stream::new(stream.0, stream.1);
        let iterator = Scalar::new(iterator.0, iterator.1);
        let span = Span::new(left, right);
        let fold = FoldStream::new(iterable, iterator, *instruction, last_instruction.map(|v| *v), span);

        validator.meet_fold_stream(&fold, span);

        Box::new(Instruction::FoldStream(fold))
    }
}

#[allow(unused_variables)]
fn __action13<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, left, _): (AirPos, AirPos, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, iterator, _): (AirPos, (&'input str, AirPos), AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, right, _): (AirPos, AirPos, AirPos),
) -> Box<Instruction<'input>>
{
    {
        let iterator = Scalar::new(iterator.0, iterator.1);
        let next = Next::new(iterator);
        let span = Span::new(left, right);
        validator.met_next(&next, span);

        Box::new(Instruction::Next(next))
    }
}

#[allow(unused_variables)]
fn __action14<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, l, _): (AirPos, Box<Instruction<'input>>, AirPos),
    (_, r, _): (AirPos, Box<Instruction<'input>>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
) -> Box<Instruction<'input>>
{
    Box::new(Instruction::Xor(Xor(l, r)))
}

#[allow(unused_variables)]
fn __action15<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, left, _): (AirPos, AirPos, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, l, _): (AirPos, Value<'input>, AirPos),
    (_, r, _): (AirPos, Value<'input>, AirPos),
    (_, i, _): (AirPos, Box<Instruction<'input>>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, right, _): (AirPos, AirPos, AirPos),
) -> Box<Instruction<'input>>
{
    {
        let match_ = Match::new(l, r, i);
        let span = Span::new(left, right);
        validator.met_match(&match_, span);

        Box::new(Instruction::Match(match_))
    }
}

#[allow(unused_variables)]
fn __action16<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, left, _): (AirPos, AirPos, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, l, _): (AirPos, Value<'input>, AirPos),
    (_, r, _): (AirPos, Value<'input>, AirPos),
    (_, i, _): (AirPos, Box<Instruction<'input>>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, right, _): (AirPos, AirPos, AirPos),
) -> Box<Instruction<'input>>
{
    {
        let mismatch = MisMatch::new(l, r, i);
        let span = Span::new(left, right);
        validator.met_mismatch(&mismatch, span);

        Box::new(Instruction::MisMatch(mismatch))
     }
}

#[allow(unused_variables)]
fn __action17<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, __lalrpop_util::ErrorRecovery<AirPos, Token<'input>, ParserError>, AirPos),
) -> Box<Instruction<'input>>
{
    { errors.push(__0); Box::new(Instruction::Error) }
}

#[allow(unused_variables)]
fn __action18<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, args, _): (AirPos, alloc::vec::Vec<Value<'input>>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
) -> Vec<Value<'input>>
{
    args
}

#[allow(unused_variables)]
fn __action19<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, peer_pk, _): (AirPos, CallInstrValue<'input>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
    (_, service_id, _): (AirPos, CallInstrValue<'input>, AirPos),
    (_, function_name, _): (AirPos, CallInstrValue<'input>, AirPos),
    (_, _, _): (AirPos, Token<'input>, AirPos),
) -> Triplet<'input>
{
    Triplet {
        peer_pk,
        service_id,
        function_name
    }
}

#[allow(unused_variables)]
fn __action20<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (AirPos, (&'input str, AirPos), AirPos),
) -> ApResult<'input>
{
    ApResult::scalar(scalar.0, scalar.1)
}

#[allow(unused_variables)]
fn __action21<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, stream, _): (AirPos, (&'input str, AirPos), AirPos),
) -> ApResult<'input>
{
    ApResult::stream(stream.0, stream.1)
}

#[allow(unused_variables)]
fn __action22<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (AirPos, (&'input str, AirPos), AirPos),
) -> CallOutputValue<'input>
{
    CallOutputValue::scalar(scalar.0, scalar.1)
}

#[allow(unused_variables)]
fn __action23<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, stream, _): (AirPos, (&'input str, AirPos), AirPos),
) -> CallOutputValue<'input>
{
    CallOutputValue::stream(stream.0, stream.1)
}

#[allow(unused_variables)]
fn __action24<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (AirPos, (&'input str, AirPos), AirPos),
) -> Fail<'input>
{
    Fail::Scalar(ScalarWithLambda::new(scalar.0, None, scalar.1))
}

#[allow(unused_variables)]
fn __action25<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (AirPos, (&'input str, LambdaAST<'input>, AirPos), AirPos),
) -> Fail<'input>
{
    Fail::Scalar(ScalarWithLambda::new(scalar.0, Some(scalar.1), scalar.2))
}

#[allow(unused_variables)]
fn __action26<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, ret_code, _): (AirPos, i64, AirPos),
    (_, error_message, _): (AirPos, &'input str, AirPos),
) -> Fail<'input>
{
    Fail::Literal {
        ret_code,
        error_message,
    }
}

#[allow(unused_variables)]
fn __action27<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, canon_stream, _): (AirPos, (&'input str, LambdaAST<'input>, AirPos), AirPos),
) -> Fail<'input>
{
    Fail::CanonStream {
        name: canon_stream.0,
        lambda: canon_stream.1,
    }
}

#[allow(unused_variables)]
fn __action28<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, left, _): (AirPos, AirPos, AirPos),
    (_, l, _): (AirPos, Token<'input>, AirPos),
    (_, right, _): (AirPos, AirPos, AirPos),
) -> Fail<'input>
{
    {
        Fail::LastError
    }
}

#[allow(unused_variables)]
fn __action29<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (AirPos, (&'input str, AirPos), AirPos),
) -> FoldScalarIterable<'input>
{
    FoldScalarIterable::Scalar(ScalarWithLambda::new(scalar.0, None, scalar.1))
}

#[allow(unused_variables)]
fn __action30<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (AirPos, (&'input str, LambdaAST<'input>, AirPos), AirPos),
) -> FoldScalarIterable<'input>
{
    FoldScalarIterable::Scalar(ScalarWithLambda::new(scalar.0, Some(scalar.1), scalar.2))
}

#[allow(unused_variables)]
fn __action31<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, canon_stream, _): (AirPos, (&'input str, AirPos), AirPos),
) -> FoldScalarIterable<'input>
{
    FoldScalarIterable::CanonStream(CanonStream::new(canon_stream.0, canon_stream.1))
}

#[allow(unused_variables)]
fn __action32<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, Token<'input>, AirPos),
    (_, __1, _): (AirPos, Token<'input>, AirPos),
) -> FoldScalarIterable<'input>
{
    FoldScalarIterable::EmptyArray
}

#[allow(unused_variables)]
fn __action33<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, CallInstrValue<'input>, AirPos),
) -> CallInstrValue<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action34<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, CallInstrValue<'input>, AirPos),
) -> CallInstrValue<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action35<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, CallInstrValue<'input>, AirPos),
) -> CallInstrValue<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action36<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, Token<'input>, AirPos),
) -> CallInstrValue<'input>
{
    CallInstrValue::InitPeerId
}

#[allow(unused_variables)]
fn __action37<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, l, _): (AirPos, &'input str, AirPos),
) -> CallInstrValue<'input>
{
    CallInstrValue::Literal(l)
}

#[allow(unused_variables)]
fn __action38<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (AirPos, (&'input str, AirPos), AirPos),
) -> CallInstrValue<'input>
{
    CallInstrValue::Variable(VariableWithLambda::scalar(scalar.0, scalar.1))
}

#[allow(unused_variables)]
fn __action39<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (AirPos, (&'input str, LambdaAST<'input>, AirPos), AirPos),
) -> CallInstrValue<'input>
{
    CallInstrValue::Variable(VariableWithLambda::scalar_wl(scalar.0, scalar.1, scalar.2))
}

#[allow(unused_variables)]
fn __action40<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, stream, _): (AirPos, (&'input str, AirPos), AirPos),
) -> CallInstrValue<'input>
{
    CallInstrValue::Variable(VariableWithLambda::stream(stream.0, stream.1))
}

#[allow(unused_variables)]
fn __action41<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, stream, _): (AirPos, (&'input str, LambdaAST<'input>, AirPos), AirPos),
) -> CallInstrValue<'input>
{
    CallInstrValue::Variable(VariableWithLambda::stream_wl(stream.0, stream.1, stream.2))
}

#[allow(unused_variables)]
fn __action42<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, canon_stream, _): (AirPos, (&'input str, AirPos), AirPos),
) -> CallInstrValue<'input>
{
    CallInstrValue::Variable(VariableWithLambda::canon_stream(canon_stream.0, canon_stream.1))
}

#[allow(unused_variables)]
fn __action43<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, canon_stream, _): (AirPos, (&'input str, LambdaAST<'input>, AirPos), AirPos),
) -> CallInstrValue<'input>
{
    CallInstrValue::Variable(VariableWithLambda::canon_stream_wl(canon_stream.0, canon_stream.1, canon_stream.2))
}

#[allow(unused_variables)]
fn __action44<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (AirPos, (&'input str, AirPos), AirPos),
) -> NewArgument<'input>
{
    NewArgument::Scalar(Scalar::new(scalar.0, scalar.1))
}

#[allow(unused_variables)]
fn __action45<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, stream, _): (AirPos, (&'input str, AirPos), AirPos),
) -> NewArgument<'input>
{
    NewArgument::Stream(Stream::new(stream.0, stream.1))
}

#[allow(unused_variables)]
fn __action46<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, canon_stream, _): (AirPos, (&'input str, AirPos), AirPos),
) -> NewArgument<'input>
{
    NewArgument::CanonStream(CanonStream::new(canon_stream.0, canon_stream.1))
}

#[allow(unused_variables)]
fn __action47<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, integer, _): (AirPos, i64, AirPos),
) -> Number
{
    Number::Int(integer)
}

#[allow(unused_variables)]
fn __action48<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, float, _): (AirPos, f64, AirPos),
) -> Number
{
    Number::Float(float)
}

#[allow(unused_variables)]
fn __action49<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, Value<'input>, AirPos),
) -> Value<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action50<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, Token<'input>, AirPos),
) -> Value<'input>
{
    Value::InitPeerId
}

#[allow(unused_variables)]
fn __action51<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, Token<'input>, AirPos),
) -> Value<'input>
{
    Value::LastError(None)
}

#[allow(unused_variables)]
fn __action52<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, le, _): (AirPos, LambdaAST<'input>, AirPos),
) -> Value<'input>
{
    Value::LastError(Some(le))
}

#[allow(unused_variables)]
fn __action53<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, l, _): (AirPos, &'input str, AirPos),
) -> Value<'input>
{
    Value::Literal(l)
}

#[allow(unused_variables)]
fn __action54<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, Token<'input>, AirPos),
) -> Value<'input>
{
    Value::Timestamp
}

#[allow(unused_variables)]
fn __action55<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, Token<'input>, AirPos),
) -> Value<'input>
{
    Value::TTL
}

#[allow(unused_variables)]
fn __action56<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, n, _): (AirPos, Number, AirPos),
) -> Value<'input>
{
    Value::Number(n)
}

#[allow(unused_variables)]
fn __action57<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, b, _): (AirPos, bool, AirPos),
) -> Value<'input>
{
    Value::Boolean(b)
}

#[allow(unused_variables)]
fn __action58<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, Token<'input>, AirPos),
    (_, __1, _): (AirPos, Token<'input>, AirPos),
) -> Value<'input>
{
    Value::EmptyArray
}

#[allow(unused_variables)]
fn __action59<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (AirPos, (&'input str, AirPos), AirPos),
) -> Value<'input>
{
    Value::Variable(VariableWithLambda::scalar(scalar.0, scalar.1))
}

#[allow(unused_variables)]
fn __action60<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (AirPos, (&'input str, LambdaAST<'input>, AirPos), AirPos),
) -> Value<'input>
{
    Value::Variable(VariableWithLambda::scalar_wl(scalar.0, scalar.1, scalar.2))
}

#[allow(unused_variables)]
fn __action61<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, stream, _): (AirPos, (&'input str, AirPos), AirPos),
) -> Value<'input>
{
    Value::Variable(VariableWithLambda::stream(stream.0, stream.1))
}

#[allow(unused_variables)]
fn __action62<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, stream, _): (AirPos, (&'input str, LambdaAST<'input>, AirPos), AirPos),
) -> Value<'input>
{
    Value::Variable(VariableWithLambda::stream_wl(stream.0, stream.1, stream.2))
}

#[allow(unused_variables)]
fn __action63<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, canon_stream, _): (AirPos, (&'input str, AirPos), AirPos),
) -> Value<'input>
{
    Value::Variable(VariableWithLambda::canon_stream(canon_stream.0, canon_stream.1))
}

#[allow(unused_variables)]
fn __action64<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, canon_stream, _): (AirPos, (&'input str, LambdaAST<'input>, AirPos), AirPos),
) -> Value<'input>
{
    Value::Variable(VariableWithLambda::canon_stream_wl(canon_stream.0, canon_stream.1, canon_stream.2))
}

#[allow(unused_variables)]
fn __action65<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, Token<'input>, AirPos),
) -> ApArgument<'input>
{
    ApArgument::InitPeerId
}

#[allow(unused_variables)]
fn __action66<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, Token<'input>, AirPos),
) -> ApArgument<'input>
{
    ApArgument::LastError(None)
}

#[allow(unused_variables)]
fn __action67<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, le, _): (AirPos, LambdaAST<'input>, AirPos),
) -> ApArgument<'input>
{
    ApArgument::LastError(Some(le))
}

#[allow(unused_variables)]
fn __action68<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, Token<'input>, AirPos),
) -> ApArgument<'input>
{
    ApArgument::Timestamp
}

#[allow(unused_variables)]
fn __action69<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, Token<'input>, AirPos),
) -> ApArgument<'input>
{
    ApArgument::TTL
}

#[allow(unused_variables)]
fn __action70<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, l, _): (AirPos, &'input str, AirPos),
) -> ApArgument<'input>
{
    ApArgument::Literal(l)
}

#[allow(unused_variables)]
fn __action71<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, n, _): (AirPos, Number, AirPos),
) -> ApArgument<'input>
{
    ApArgument::Number(n)
}

#[allow(unused_variables)]
fn __action72<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, b, _): (AirPos, bool, AirPos),
) -> ApArgument<'input>
{
    ApArgument::Boolean(b)
}

#[allow(unused_variables)]
fn __action73<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, Token<'input>, AirPos),
    (_, __1, _): (AirPos, Token<'input>, AirPos),
) -> ApArgument<'input>
{
    ApArgument::EmptyArray
}

#[allow(unused_variables)]
fn __action74<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (AirPos, (&'input str, AirPos), AirPos),
) -> ApArgument<'input>
{
    ApArgument::Scalar(ScalarWithLambda::new(scalar.0, None, scalar.1))
}

#[allow(unused_variables)]
fn __action75<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (AirPos, (&'input str, LambdaAST<'input>, AirPos), AirPos),
) -> ApArgument<'input>
{
    ApArgument::Scalar(ScalarWithLambda::new(scalar.0, Some(scalar.1), scalar.2))
}

#[allow(unused_variables)]
fn __action76<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, canon_stream, _): (AirPos, (&'input str, AirPos), AirPos),
) -> ApArgument<'input>
{
    ApArgument::CanonStream(CanonStreamWithLambda::new(canon_stream.0, None, canon_stream.1))
}

#[allow(unused_variables)]
fn __action77<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, canon_stream, _): (AirPos, (&'input str, LambdaAST<'input>, AirPos), AirPos),
) -> ApArgument<'input>
{
    ApArgument::CanonStream(CanonStreamWithLambda::new(canon_stream.0, Some(canon_stream.1), canon_stream.2))
}

#[allow(unused_variables)]
fn __action78<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, stream, _): (AirPos, (&'input str, AirPos), AirPos),
) -> Stream<'input>
{
    Stream::new(stream.0, stream.1)
}

#[allow(unused_variables)]
fn __action79<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, canon_stream, _): (AirPos, (&'input str, AirPos), AirPos),
) -> CanonStream<'input>
{
    CanonStream::new(canon_stream.0, canon_stream.1)
}

#[allow(unused_variables)]
fn __action80<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __lookbehind: &AirPos,
    __lookahead: &AirPos,
) -> alloc::vec::Vec<Value<'input>>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action81<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, v, _): (AirPos, alloc::vec::Vec<Value<'input>>, AirPos),
) -> alloc::vec::Vec<Value<'input>>
{
    v
}

#[allow(unused_variables)]
fn __action82<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, Value<'input>, AirPos),
) -> Value<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action83<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, Box<Instruction<'input>>, AirPos),
) -> core::option::Option<Box<Instruction<'input>>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action84<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __lookbehind: &AirPos,
    __lookahead: &AirPos,
) -> core::option::Option<Box<Instruction<'input>>>
{
    None
}

#[allow(unused_variables)]
fn __action85<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __lookbehind: &AirPos,
    __lookahead: &AirPos,
) -> AirPos
{
    __lookbehind.clone()
}

#[allow(unused_variables)]
fn __action86<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, CallOutputValue<'input>, AirPos),
) -> core::option::Option<CallOutputValue<'input>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action87<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __lookbehind: &AirPos,
    __lookahead: &AirPos,
) -> core::option::Option<CallOutputValue<'input>>
{
    None
}

#[allow(unused_variables)]
fn __action88<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __lookbehind: &AirPos,
    __lookahead: &AirPos,
) -> AirPos
{
    __lookahead.clone()
}

#[allow(unused_variables)]
fn __action89<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (AirPos, Value<'input>, AirPos),
) -> alloc::vec::Vec<Value<'input>>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action90<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, v, _): (AirPos, alloc::vec::Vec<Value<'input>>, AirPos),
    (_, e, _): (AirPos, Value<'input>, AirPos),
) -> alloc::vec::Vec<Value<'input>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action91<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Value<'input>, AirPos),
) -> alloc::vec::Vec<Value<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action82(
        input,
        errors,
        validator,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action89(
        input,
        errors,
        validator,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action92<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, alloc::vec::Vec<Value<'input>>, AirPos),
    __1: (AirPos, Value<'input>, AirPos),
) -> alloc::vec::Vec<Value<'input>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action82(
        input,
        errors,
        validator,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action90(
        input,
        errors,
        validator,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action93<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
) -> Vec<Value<'input>>
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action80(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action18(
        input,
        errors,
        validator,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action94<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, alloc::vec::Vec<Value<'input>>, AirPos),
    __2: (AirPos, Token<'input>, AirPos),
) -> Vec<Value<'input>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action81(
        input,
        errors,
        validator,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action18(
        input,
        errors,
        validator,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action95<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, AirPos, AirPos),
) -> Fail<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action88(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        input,
        errors,
        validator,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action96<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, Triplet<'input>, AirPos),
    __3: (AirPos, Vec<Value<'input>>, AirPos),
    __4: (AirPos, core::option::Option<CallOutputValue<'input>>, AirPos),
    __5: (AirPos, Token<'input>, AirPos),
    __6: (AirPos, AirPos, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action88(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        errors,
        validator,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
fn __action97<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, CallInstrValue<'input>, AirPos),
    __3: (AirPos, Stream<'input>, AirPos),
    __4: (AirPos, CanonStream<'input>, AirPos),
    __5: (AirPos, Token<'input>, AirPos),
    __6: (AirPos, AirPos, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action88(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        input,
        errors,
        validator,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
fn __action98<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, ApArgument<'input>, AirPos),
    __3: (AirPos, ApResult<'input>, AirPos),
    __4: (AirPos, Token<'input>, AirPos),
    __5: (AirPos, AirPos, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action88(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        input,
        errors,
        validator,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
fn __action99<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, NewArgument<'input>, AirPos),
    __3: (AirPos, Box<Instruction<'input>>, AirPos),
    __4: (AirPos, Token<'input>, AirPos),
    __5: (AirPos, AirPos, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action88(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
        input,
        errors,
        validator,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
fn __action100<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, FoldScalarIterable<'input>, AirPos),
    __3: (AirPos, (&'input str, AirPos), AirPos),
    __4: (AirPos, Box<Instruction<'input>>, AirPos),
    __5: (AirPos, core::option::Option<Box<Instruction<'input>>>, AirPos),
    __6: (AirPos, Token<'input>, AirPos),
    __7: (AirPos, AirPos, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action88(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        input,
        errors,
        validator,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
fn __action101<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, (&'input str, AirPos), AirPos),
    __3: (AirPos, (&'input str, AirPos), AirPos),
    __4: (AirPos, Box<Instruction<'input>>, AirPos),
    __5: (AirPos, core::option::Option<Box<Instruction<'input>>>, AirPos),
    __6: (AirPos, Token<'input>, AirPos),
    __7: (AirPos, AirPos, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action88(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        input,
        errors,
        validator,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
fn __action102<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, (&'input str, AirPos), AirPos),
    __3: (AirPos, Token<'input>, AirPos),
    __4: (AirPos, AirPos, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action88(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        input,
        errors,
        validator,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
fn __action103<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, Value<'input>, AirPos),
    __3: (AirPos, Value<'input>, AirPos),
    __4: (AirPos, Box<Instruction<'input>>, AirPos),
    __5: (AirPos, Token<'input>, AirPos),
    __6: (AirPos, AirPos, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action88(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        input,
        errors,
        validator,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
fn __action104<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, Value<'input>, AirPos),
    __3: (AirPos, Value<'input>, AirPos),
    __4: (AirPos, Box<Instruction<'input>>, AirPos),
    __5: (AirPos, Token<'input>, AirPos),
    __6: (AirPos, AirPos, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action88(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        input,
        errors,
        validator,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
fn __action105<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
) -> Fail<'input>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action85(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action95(
        input,
        errors,
        validator,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action106<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, Triplet<'input>, AirPos),
    __3: (AirPos, Vec<Value<'input>>, AirPos),
    __4: (AirPos, core::option::Option<CallOutputValue<'input>>, AirPos),
    __5: (AirPos, Token<'input>, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __5.2.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action85(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action96(
        input,
        errors,
        validator,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action107<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, CallInstrValue<'input>, AirPos),
    __3: (AirPos, Stream<'input>, AirPos),
    __4: (AirPos, CanonStream<'input>, AirPos),
    __5: (AirPos, Token<'input>, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __5.2.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action85(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action97(
        input,
        errors,
        validator,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action108<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, ApArgument<'input>, AirPos),
    __3: (AirPos, ApResult<'input>, AirPos),
    __4: (AirPos, Token<'input>, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __4.2.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action85(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action98(
        input,
        errors,
        validator,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action109<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, NewArgument<'input>, AirPos),
    __3: (AirPos, Box<Instruction<'input>>, AirPos),
    __4: (AirPos, Token<'input>, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __4.2.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action85(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action99(
        input,
        errors,
        validator,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action110<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, FoldScalarIterable<'input>, AirPos),
    __3: (AirPos, (&'input str, AirPos), AirPos),
    __4: (AirPos, Box<Instruction<'input>>, AirPos),
    __5: (AirPos, core::option::Option<Box<Instruction<'input>>>, AirPos),
    __6: (AirPos, Token<'input>, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __6.2.clone();
    let __end0 = __6.2.clone();
    let __temp0 = __action85(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action100(
        input,
        errors,
        validator,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action111<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, (&'input str, AirPos), AirPos),
    __3: (AirPos, (&'input str, AirPos), AirPos),
    __4: (AirPos, Box<Instruction<'input>>, AirPos),
    __5: (AirPos, core::option::Option<Box<Instruction<'input>>>, AirPos),
    __6: (AirPos, Token<'input>, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __6.2.clone();
    let __end0 = __6.2.clone();
    let __temp0 = __action85(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action101(
        input,
        errors,
        validator,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action112<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, (&'input str, AirPos), AirPos),
    __3: (AirPos, Token<'input>, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action85(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action102(
        input,
        errors,
        validator,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action113<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, Value<'input>, AirPos),
    __3: (AirPos, Value<'input>, AirPos),
    __4: (AirPos, Box<Instruction<'input>>, AirPos),
    __5: (AirPos, Token<'input>, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __5.2.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action85(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action103(
        input,
        errors,
        validator,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action114<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, Value<'input>, AirPos),
    __3: (AirPos, Value<'input>, AirPos),
    __4: (AirPos, Box<Instruction<'input>>, AirPos),
    __5: (AirPos, Token<'input>, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __5.2.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action85(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action104(
        input,
        errors,
        validator,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action115<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, Triplet<'input>, AirPos),
    __3: (AirPos, Vec<Value<'input>>, AirPos),
    __4: (AirPos, CallOutputValue<'input>, AirPos),
    __5: (AirPos, Token<'input>, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action86(
        input,
        errors,
        validator,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action106(
        input,
        errors,
        validator,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __5,
    )
}

#[allow(unused_variables)]
fn __action116<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, Triplet<'input>, AirPos),
    __3: (AirPos, Vec<Value<'input>>, AirPos),
    __4: (AirPos, Token<'input>, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __temp0 = __action87(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action106(
        input,
        errors,
        validator,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
    )
}

#[allow(unused_variables)]
fn __action117<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, FoldScalarIterable<'input>, AirPos),
    __3: (AirPos, (&'input str, AirPos), AirPos),
    __4: (AirPos, Box<Instruction<'input>>, AirPos),
    __5: (AirPos, Box<Instruction<'input>>, AirPos),
    __6: (AirPos, Token<'input>, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __5.0.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action83(
        input,
        errors,
        validator,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action110(
        input,
        errors,
        validator,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __6,
    )
}

#[allow(unused_variables)]
fn __action118<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, FoldScalarIterable<'input>, AirPos),
    __3: (AirPos, (&'input str, AirPos), AirPos),
    __4: (AirPos, Box<Instruction<'input>>, AirPos),
    __5: (AirPos, Token<'input>, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __4.2.clone();
    let __end0 = __5.0.clone();
    let __temp0 = __action84(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action110(
        input,
        errors,
        validator,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __5,
    )
}

#[allow(unused_variables)]
fn __action119<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, (&'input str, AirPos), AirPos),
    __3: (AirPos, (&'input str, AirPos), AirPos),
    __4: (AirPos, Box<Instruction<'input>>, AirPos),
    __5: (AirPos, Box<Instruction<'input>>, AirPos),
    __6: (AirPos, Token<'input>, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __5.0.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action83(
        input,
        errors,
        validator,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action111(
        input,
        errors,
        validator,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __6,
    )
}

#[allow(unused_variables)]
fn __action120<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<AirPos, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (AirPos, Token<'input>, AirPos),
    __1: (AirPos, Token<'input>, AirPos),
    __2: (AirPos, (&'input str, AirPos), AirPos),
    __3: (AirPos, (&'input str, AirPos), AirPos),
    __4: (AirPos, Box<Instruction<'input>>, AirPos),
    __5: (AirPos, Token<'input>, AirPos),
) -> Box<Instruction<'input>>
{
    let __start0 = __4.2.clone();
    let __end0 = __5.0.clone();
    let __temp0 = __action84(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action111(
        input,
        errors,
        validator,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __5,
    )
}

pub trait __ToTriple<'err, 'input, 'v, >
{
    fn to_triple(value: Self) -> Result<(AirPos,Token<'input>,AirPos), __lalrpop_util::ParseError<AirPos, Token<'input>, ParserError>>;
}

impl<'err, 'input, 'v, > __ToTriple<'err, 'input, 'v, > for (AirPos, Token<'input>, AirPos)
{
    fn to_triple(value: Self) -> Result<(AirPos,Token<'input>,AirPos), __lalrpop_util::ParseError<AirPos, Token<'input>, ParserError>> {
        Ok(value)
    }
}
impl<'err, 'input, 'v, > __ToTriple<'err, 'input, 'v, > for Result<(AirPos, Token<'input>, AirPos), ParserError>
{
    fn to_triple(value: Self) -> Result<(AirPos,Token<'input>,AirPos), __lalrpop_util::ParseError<AirPos, Token<'input>, ParserError>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
