// auto-generated: "lalrpop 0.19.7"
// sha3: 3cf8b9ff34d7504a1819dc458ed256ef01be2d012e6dfd9ed4bd867fcb398ae
use crate::ast::*;
use crate::parser::ParserError;
use crate::parser::VariableValidator;
use crate::parser::Span;
use crate::parser::lexer::Token;
use crate::parser::air_utils::*;
use crate::make_user_error;
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
    use crate::parser::lexer::Token;
    use crate::parser::air_utils::*;
    use crate::make_user_error;
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
        Variant2(f64),
        Variant3(i64),
        Variant4(LambdaAST<'input>),
        Variant5(&'input str),
        Variant6((&'input str, usize)),
        Variant7((&'input str, LambdaAST<'input>, usize)),
        Variant8(__lalrpop_util::ErrorRecovery<usize, Token<'input>, ParserError>),
        Variant9(Value<'input>),
        Variant10(alloc::vec::Vec<Value<'input>>),
        Variant11(usize),
        Variant12(Box<Instruction<'input>>),
        Variant13(ApArgument<'input>),
        Variant14(Variable<'input>),
        Variant15(Vec<Value<'input>>),
        Variant16(CallInstrValue<'input>),
        Variant17(core::option::Option<Variable<'input>>),
        Variant18(FunctionPart<'input>),
        Variant19(Fail<'input>),
        Variant20(FoldScalarIterable<'input>),
        Variant21(Number),
        Variant22(PeerPart<'input>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35,
        // State 1
        0, 0, 39, 0, 40, 41, 42, 43, 44, 45, 46, 47, 48, 0, 0, 49, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        14, 0, 0, 0, 0, 0, 0, 53, 0, 0, 54, 55, 56, 57, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 60, 0, 61, 0, 0, 62, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 65, 0, 0, 0, 0, 0, 0, 0, 0, 66, 67, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 70, 0, 71, 41, 42, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 70, 0, 71, 41, 42, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 82, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35,
        // State 9
        34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35,
        // State 10
        34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 82, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        22, 0, 0, 0, 0, 0, 0, 53, 0, 0, 54, 55, 56, 57, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 53, 0, 0, 54, 55, 56, 57, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 70, 0, 71, 41, 42, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 70, 0, 71, 41, 42, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35,
        // State 17
        34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35,
        // State 18
        34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35,
        // State 19
        34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35,
        // State 20
        0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 53, 0, 0, 54, 55, 56, 57, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 53, 0, 0, 54, 55, 56, 57, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35,
        // State 24
        34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35,
        // State 25
        34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35,
        // State 26
        34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35,
        // State 27
        0, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 82, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 70, 116, 71, 41, 42, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 53, 0, 0, 54, 55, 56, 57, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 70, 125, 71, 41, 42, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 4, 5, 6, 7, 8, 36, 37, 9, 10, 11, 0,
        // State 34
        -57, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        -59, 0, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -59,
        // State 41
        -58, 0, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -58,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        -60, 0, 0, 0, 0, 0, 0, -60, 0, 0, -60, -60, -60, -60, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        -61, 0, 0, 0, 0, 0, 0, -61, 0, 0, -61, -61, -61, -61, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        -24, -24, -24, 0, 0, 0, 0, -24, 0, 0, -24, -24, -24, -24, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        -25, -25, -25, 0, 0, 0, 0, -25, 0, 0, -25, -25, -25, -25, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        -26, -26, -26, 0, 0, 0, 0, -26, 0, 0, -26, -26, -26, -26, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        -27, -27, -27, 0, 0, 0, 0, -27, 0, 0, -27, -27, -27, -27, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        -28, -28, -28, 0, 0, 0, 0, -28, 0, 0, -28, -28, -28, -28, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        -29, -29, -29, 0, 0, 0, 0, -29, 0, 0, -29, -29, -29, -29, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 93, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        -72, 0, -72, -72, -72, -72, -72, -72, -72, -72, -72, -72, -72, -72, -72, -72, -72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -72,
        // State 69
        0, 0, 0, 94, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        -73, 0, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73,
        // State 71
        -66, 0, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -66,
        // State 72
        -67, 0, -67, -67, -67, -67, -67, -67, -67, -67, -67, -67, -67, -67, -67, -67, -67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -67,
        // State 73
        -68, 0, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -68,
        // State 74
        -69, 0, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -69,
        // State 75
        -75, 0, -75, -75, -75, -75, -75, -75, -75, -75, -75, -75, -75, -75, -75, -75, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -75,
        // State 76
        -76, 0, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -76,
        // State 77
        -77, 0, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -77,
        // State 78
        -78, 0, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -78,
        // State 79
        -71, 0, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -71,
        // State 80
        -70, 0, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -70,
        // State 81
        -63, -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -63,
        // State 82
        -64, -64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -64,
        // State 83
        0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 84
        -48, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -48,
        // State 85
        0, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 86
        0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 87
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 88
        0, -42, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 89
        0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 90
        -50, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -50,
        // State 91
        0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 92
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 93
        -74, 0, -74, -74, -74, -74, -74, -74, -74, -74, -74, -74, -74, -74, -74, -74, -74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -74,
        // State 94
        0, 107, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 95
        -53, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -53,
        // State 96
        0, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 97
        0, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 98
        0, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 99
        -45, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45,
        // State 100
        0, -65, 0, 0, 0, 0, 0, -65, 0, 0, -65, -65, -65, -65, -65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 101
        0, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 102
        0, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 103
        0, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 104
        0, 121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 105
        0, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 106
        -49, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -49,
        // State 107
        -47, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -47,
        // State 108
        -46, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46,
        // State 109
        -54, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -54,
        // State 110
        0, 123, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 111
        0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 112
        -44, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44,
        // State 113
        0, 0, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 114
        0, 0, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 115
        0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 116
        0, 126, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 117
        -62, 0, 0, 0, 0, 0, 0, -62, 0, 0, -62, -62, -62, -62, -62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 118
        -51, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51,
        // State 119
        -52, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52,
        // State 120
        -55, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -55,
        // State 121
        -56, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -56,
        // State 122
        -43, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43,
        // State 123
        0, 0, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 124
        0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 125
        0, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 30 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
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
        -79,
        // State 32
        -8,
        // State 33
        0,
        // State 34
        -57,
        // State 35
        0,
        // State 36
        0,
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
        -48,
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
        -50,
        // State 91
        0,
        // State 92
        0,
        // State 93
        0,
        // State 94
        0,
        // State 95
        -53,
        // State 96
        0,
        // State 97
        0,
        // State 98
        0,
        // State 99
        -45,
        // State 100
        0,
        // State 101
        0,
        // State 102
        0,
        // State 103
        0,
        // State 104
        0,
        // State 105
        0,
        // State 106
        -49,
        // State 107
        -47,
        // State 108
        -46,
        // State 109
        -54,
        // State 110
        0,
        // State 111
        0,
        // State 112
        -44,
        // State 113
        0,
        // State 114
        0,
        // State 115
        0,
        // State 116
        0,
        // State 117
        0,
        // State 118
        -51,
        // State 119
        -52,
        // State 120
        -55,
        // State 121
        -56,
        // State 122
        -43,
        // State 123
        0,
        // State 124
        0,
        // State 125
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 30,
            5 => 31,
            6 => 11,
            7 => 85,
            8 => match state {
                30 => 123,
                _ => 113,
            },
            9 => 27,
            10 => match state {
                12 | 29 => 88,
                21..=22 => 100,
                _ => 50,
            },
            11 => 110,
            13 => 20,
            14 => 58,
            15 => 63,
            16 => match state {
                29 => 116,
                _ => 89,
            },
            17 => match state {
                9 => 18,
                10 => 19,
                0 => 32,
                16 => 94,
                17 => 96,
                18 => 97,
                19 => 98,
                23 => 102,
                24 => 103,
                25 => 104,
                26 => 105,
                _ => 17,
            },
            18 => match state {
                1 => 37,
                _ => 68,
            },
            19 => match state {
                2 => 51,
                _ => 22,
            },
            20 => 12,
            21 => match state {
                11 => 86,
                27 => 111,
                _ => 16,
            },
            22 => match state {
                22 => 101,
                _ => 29,
            },
            23 => match state {
                5 => 14,
                6 => 15,
                14 => 25,
                15 => 26,
                _ => 114,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""(""###,
            r###"")""###,
            r###""[""###,
            r###""]""###,
            r###"Boolean"###,
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
            r###"fail"###,
            r###"fold"###,
            r###"match_"###,
            r###"mismatch"###,
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
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __phantom: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    }
    impl<'err, 'input, 'v> __state_machine::ParserDefinition for __StateMachine<'err, 'input, 'v>
    where 'input: 'err, 'input: 'v
    {
        type Location = usize;
        type Error = ParserError;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Box<Instruction<'input>>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
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
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 30 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&(), &(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
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
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
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

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
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
            Token::F64(_) if true => Some(5),
            Token::I64(_) if true => Some(6),
            Token::InitPeerId if true => Some(7),
            Token::LastError if true => Some(8),
            Token::LastErrorWithLambda(_) if true => Some(9),
            Token::StringLiteral(_) if true => Some(10),
            Token::Scalar { name: _, position: _ } if true => Some(11),
            Token::ScalarWithLambda { name: _, lambda: _, position: _ } if true => Some(12),
            Token::Stream { name: _, position: _ } if true => Some(13),
            Token::StreamWithLambda { name: _, lambda: _, position: _ } if true => Some(14),
            Token::TTL if true => Some(15),
            Token::Timestamp if true => Some(16),
            Token::Ap if true => Some(17),
            Token::Call if true => Some(18),
            Token::Fail if true => Some(19),
            Token::Fold if true => Some(20),
            Token::Match if true => Some(21),
            Token::MisMatch if true => Some(22),
            Token::New if true => Some(23),
            Token::Next if true => Some(24),
            Token::Null if true => Some(25),
            Token::Par if true => Some(26),
            Token::Seq if true => Some(27),
            Token::Xor if true => Some(28),
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
            0 | 1 | 2 | 3 | 7 | 8 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 => __Symbol::Variant0(__token),
            4 => match __token {
                Token::Boolean(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            5 => match __token {
                Token::F64(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            6 => match __token {
                Token::I64(__tok0) if true => __Symbol::Variant3(__tok0),
                _ => unreachable!(),
            },
            9 => match __token {
                Token::LastErrorWithLambda(__tok0) if true => __Symbol::Variant4(__tok0),
                _ => unreachable!(),
            },
            10 => match __token {
                Token::StringLiteral(__tok0) if true => __Symbol::Variant5(__tok0),
                _ => unreachable!(),
            },
            11 | 13 => match __token {
                Token::Scalar { name: __tok0, position: __tok1 } | Token::Stream { name: __tok0, position: __tok1 } if true => __Symbol::Variant6((__tok0, __tok1)),
                _ => unreachable!(),
            },
            12 | 14 => match __token {
                Token::ScalarWithLambda { name: __tok0, lambda: __tok1, position: __tok2 } | Token::StreamWithLambda { name: __tok0, lambda: __tok1, position: __tok2 } if true => __Symbol::Variant7((__tok0, __tok1, __tok2)),
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
        __reduce_index: i8,
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
                    nonterminal_produced: 7,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 9,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 9,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
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
                    nonterminal_produced: 11,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 12,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 13,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 14,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 15,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 17,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 17,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 17,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 17,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 17,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 17,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 17,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 17,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
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
                    states_to_pop: 4,
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
                    states_to_pop: 6,
                    nonterminal_produced: 17,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 17,
                }
            }
            56 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            57 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            58 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            59 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            60 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 20,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            72 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            73 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 23,
                }
            }
            74 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            75 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            76 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            77 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            78 => __state_machine::SimulatedReduce::Accept,
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
            errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
            validator: &'v mut VariableValidator<'input>,
            __tokens0: __TOKENS,
        ) -> Result<Box<Instruction<'input>>, __lalrpop_util::ParseError<usize, Token<'input>, ParserError>>
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
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __error_state: i8,
        __states: & [i8],
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
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> Option<Result<Box<Instruction<'input>>,__lalrpop_util::ParseError<usize, Token<'input>, ParserError>>>
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
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (&'input str, LambdaAST<'input>, usize), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (&'input str, usize), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ApArgument<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Instruction<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, CallInstrValue<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Fail<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant19(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, FoldScalarIterable<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant20(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, FunctionPart<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, LambdaAST<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Number, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant21(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, PeerPart<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant22(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Value<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Variable<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Value<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, ParserError>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Value<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, bool, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<Variable<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // (<Arg>) = Arg => ActionFn(70);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // (<Arg>)* =  => ActionFn(68);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action68::<>(input, errors, validator, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // (<Arg>)* = (<Arg>)+ => ActionFn(69);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action69::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // (<Arg>)+ = Arg => ActionFn(77);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action77::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce4<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // (<Arg>)+ = (<Arg>)+, Arg => ActionFn(78);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action78::<>(input, errors, validator, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce5<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(74);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action74::<>(input, errors, validator, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 3)
    }
    pub(crate) fn __reduce6<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(71);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action71::<>(input, errors, validator, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
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
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = InitPeerId => ActionFn(57);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action57::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce9<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = LastError => ActionFn(58);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce10<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = LastErrorWithLambda => ActionFn(59);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action59::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce11<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = Timestamp => ActionFn(60);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action60::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce12<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = TTL => ActionFn(61);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action61::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce13<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = Literal => ActionFn(62);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action62::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce14<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = Number => ActionFn(63);
        let __sym0 = __pop_Variant21(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action63::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce15<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = Boolean => ActionFn(64);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action64::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce16<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = "[", "]" => ActionFn(65);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action65::<>(input, errors, validator, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce17<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = Scalar => ActionFn(66);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action66::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce18<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApArgument = ScalarWithLambda => ActionFn(67);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce19<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ApResult = ScriptVariable => ActionFn(21);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce20<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Arg = Value => ActionFn(43);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce21<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Args = "[", "]" => ActionFn(79);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action79::<>(input, errors, validator, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce22<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Args = "[", (<Arg>)+, "]" => ActionFn(80);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action80::<>(input, errors, validator, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 9)
    }
    pub(crate) fn __reduce23<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // CallInstrValue = InitPeerId => ActionFn(35);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce24<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // CallInstrValue = Literal => ActionFn(36);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce25<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // CallInstrValue = Scalar => ActionFn(37);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce26<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // CallInstrValue = ScalarWithLambda => ActionFn(38);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce27<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // CallInstrValue = Stream => ActionFn(39);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce28<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // CallInstrValue = StreamWithLambda => ActionFn(40);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce29<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // CallOutput = ScriptVariable => ActionFn(22);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce30<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // CallOutput? = CallOutput => ActionFn(72);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce31<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // CallOutput? =  => ActionFn(73);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action73::<>(input, errors, validator, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (0, 12)
    }
    pub(crate) fn __reduce32<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // FPart = Function => ActionFn(17);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce33<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // FPart = "(", ServiceId, Function, ")" => ActionFn(18);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant16(__symbols);
        let __sym1 = __pop_Variant16(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action18::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (4, 13)
    }
    pub(crate) fn __reduce34<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // FailBody = Scalar => ActionFn(25);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce35<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // FailBody = ScalarWithLambda => ActionFn(26);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce36<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // FailBody = I64, Literal => ActionFn(27);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action27::<>(input, errors, validator, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce37<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // FailBody = LastError => ActionFn(90);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action90::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce38<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // FoldScalarIterable = Scalar => ActionFn(29);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce39<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // FoldScalarIterable = ScalarWithLambda => ActionFn(30);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce40<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // FoldScalarIterable = "[", "]" => ActionFn(31);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action31::<>(input, errors, validator, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (2, 15)
    }
    pub(crate) fn __reduce41<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Function = CallInstrValue => ActionFn(32);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce42<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", call, PeerPart, FPart, Args, CallOutput, ")" => ActionFn(99);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant14(__symbols);
        let __sym4 = __pop_Variant15(__symbols);
        let __sym3 = __pop_Variant18(__symbols);
        let __sym2 = __pop_Variant22(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action99::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (7, 17)
    }
    pub(crate) fn __reduce43<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", call, PeerPart, FPart, Args, ")" => ActionFn(100);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant15(__symbols);
        let __sym3 = __pop_Variant18(__symbols);
        let __sym2 = __pop_Variant22(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action100::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (6, 17)
    }
    pub(crate) fn __reduce44<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", ap, ApArgument, ApResult, ")" => ActionFn(92);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant14(__symbols);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action92::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 17)
    }
    pub(crate) fn __reduce45<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", seq, Instr, Instr, ")" => ActionFn(4);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action4::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 17)
    }
    pub(crate) fn __reduce46<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", par, Instr, Instr, ")" => ActionFn(5);
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
    pub(crate) fn __reduce47<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", null, ")" => ActionFn(6);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action6::<>(input, errors, validator, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce48<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", new, ScriptVariable, Instr, ")" => ActionFn(93);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant14(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action93::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 17)
    }
    pub(crate) fn __reduce49<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", fail, FailBody, ")" => ActionFn(8);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant19(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action8::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 17)
    }
    pub(crate) fn __reduce50<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", fold, FoldScalarIterable, Scalar, Instr, ")" => ActionFn(94);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant12(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant20(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action94::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (6, 17)
    }
    pub(crate) fn __reduce51<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", fold, Stream, Scalar, Instr, ")" => ActionFn(95);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant12(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action95::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (6, 17)
    }
    pub(crate) fn __reduce52<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", next, Scalar, ")" => ActionFn(96);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action96::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 17)
    }
    pub(crate) fn __reduce53<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", xor, Instr, Instr, ")" => ActionFn(12);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action12::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 17)
    }
    pub(crate) fn __reduce54<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", match_, Value, Value, Instr, ")" => ActionFn(97);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant12(__symbols);
        let __sym3 = __pop_Variant9(__symbols);
        let __sym2 = __pop_Variant9(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action97::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (6, 17)
    }
    pub(crate) fn __reduce55<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", mismatch, Value, Value, Instr, ")" => ActionFn(98);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant12(__symbols);
        let __sym3 = __pop_Variant9(__symbols);
        let __sym2 = __pop_Variant9(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action98::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (6, 17)
    }
    pub(crate) fn __reduce56<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Instr = error => ActionFn(15);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce57<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Number = I64 => ActionFn(41);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce58<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Number = F64 => ActionFn(42);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce59<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // PeerId = CallInstrValue => ActionFn(33);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce60<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // PeerPart = PeerId => ActionFn(19);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce61<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // PeerPart = "(", PeerId, ServiceId, ")" => ActionFn(20);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant16(__symbols);
        let __sym1 = __pop_Variant16(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action20::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (4, 20)
    }
    pub(crate) fn __reduce62<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ScriptVariable = Scalar => ActionFn(23);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce63<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ScriptVariable = Stream => ActionFn(24);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce64<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // ServiceId = CallInstrValue => ActionFn(34);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce65<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = InitPeerId => ActionFn(44);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce66<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = LastError => ActionFn(45);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce67<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = LastErrorWithLambda => ActionFn(46);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce68<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = Literal => ActionFn(47);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce69<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = Timestamp => ActionFn(48);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action48::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce70<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = TTL => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce71<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = Number => ActionFn(50);
        let __sym0 = __pop_Variant21(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce72<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = Boolean => ActionFn(51);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce73<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = "[", "]" => ActionFn(52);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action52::<>(input, errors, validator, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 23)
    }
    pub(crate) fn __reduce74<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = Scalar => ActionFn(53);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce75<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = ScalarWithLambda => ActionFn(54);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action54::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce76<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = Stream => ActionFn(55);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action55::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce77<
        'err,
        'input,
        'v,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        validator: &'v mut VariableValidator<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input (), &'v ())>,
    ) -> (usize, usize)
    {
        // Value = StreamWithLambda => ActionFn(56);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action56::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 23)
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
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, Box<Instruction<'input>>, usize),
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
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, Box<Instruction<'input>>, usize),
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
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, left, _): (usize, usize, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, peer_part, _): (usize, PeerPart<'input>, usize),
    (_, function_part, _): (usize, FunctionPart<'input>, usize),
    (_, args, _): (usize, Vec<Value<'input>>, usize),
    (_, output, _): (usize, core::option::Option<Variable<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, right, _): (usize, usize, usize),
) -> Box<Instruction<'input>>
{
    {
        let triplet = match try_to_raw_triplet(peer_part, function_part) {
            Some(raw_triplet) => raw_triplet,
            None => {
                // none means error
                errors.push(make_user_error!(InvalidCallTriplet, left, Token::Call, right));
                return Box::new(Instruction::Error);
            }
        };

        let args = Rc::new(args);
        let output = output.map(CallOutputValue::Variable).unwrap_or(CallOutputValue::None);
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
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, left, _): (usize, usize, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, arg, _): (usize, ApArgument<'input>, usize),
    (_, result, _): (usize, Variable<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, right, _): (usize, usize, usize),
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
fn __action4<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, l, _): (usize, Box<Instruction<'input>>, usize),
    (_, r, _): (usize, Box<Instruction<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    Box::new(Instruction::Seq(Seq::new(l, r)))
}

#[allow(unused_variables)]
fn __action5<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, l, _): (usize, Box<Instruction<'input>>, usize),
    (_, r, _): (usize, Box<Instruction<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    Box::new(Instruction::Par(Par::new(l, r)))
}

#[allow(unused_variables)]
fn __action6<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, Token<'input>, usize),
    (_, __1, _): (usize, Token<'input>, usize),
    (_, __2, _): (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    Box::new(Instruction::Null(Null))
}

#[allow(unused_variables)]
fn __action7<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, left, _): (usize, usize, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, variable, _): (usize, Variable<'input>, usize),
    (_, instruction, _): (usize, Box<Instruction<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, right, _): (usize, usize, usize),
) -> Box<Instruction<'input>>
{
    {
        let span = Span::new(left, right);
        let new = New::new(variable, instruction, span);

        validator.met_new(&new, span);

        Box::new(Instruction::New(new))
    }
}

#[allow(unused_variables)]
fn __action8<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, fail_body, _): (usize, Fail<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    {
        Box::new(Instruction::Fail(fail_body))
    }
}

#[allow(unused_variables)]
fn __action9<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, left, _): (usize, usize, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, iterable, _): (usize, FoldScalarIterable<'input>, usize),
    (_, iterator, _): (usize, (&'input str, usize), usize),
    (_, i, _): (usize, Box<Instruction<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, right, _): (usize, usize, usize),
) -> Box<Instruction<'input>>
{
    {
        let iterator = Scalar::new(iterator.0, iterator.1);
        let span = Span::new(left, right);
        let fold = FoldScalar::new(iterable, iterator, *i, span);

        validator.met_fold_scalar(&fold, span);

        Box::new(Instruction::FoldScalar(fold))
    }
}

#[allow(unused_variables)]
fn __action10<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, left, _): (usize, usize, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, stream, _): (usize, (&'input str, usize), usize),
    (_, iterator, _): (usize, (&'input str, usize), usize),
    (_, i, _): (usize, Box<Instruction<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, right, _): (usize, usize, usize),
) -> Box<Instruction<'input>>
{
    {
        let iterable = Stream::new(stream.0, stream.1);
        let iterator = Scalar::new(iterator.0, iterator.1);
        let span = Span::new(left, right);
        let fold = FoldStream::new(iterable, iterator, *i, span);

        validator.meet_fold_stream(&fold, span);

        Box::new(Instruction::FoldStream(fold))
    }
}

#[allow(unused_variables)]
fn __action11<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, left, _): (usize, usize, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, iterator, _): (usize, (&'input str, usize), usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, right, _): (usize, usize, usize),
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
fn __action12<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, l, _): (usize, Box<Instruction<'input>>, usize),
    (_, r, _): (usize, Box<Instruction<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    Box::new(Instruction::Xor(Xor(l, r)))
}

#[allow(unused_variables)]
fn __action13<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, left, _): (usize, usize, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, l, _): (usize, Value<'input>, usize),
    (_, r, _): (usize, Value<'input>, usize),
    (_, i, _): (usize, Box<Instruction<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, right, _): (usize, usize, usize),
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
fn __action14<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, left, _): (usize, usize, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, l, _): (usize, Value<'input>, usize),
    (_, r, _): (usize, Value<'input>, usize),
    (_, i, _): (usize, Box<Instruction<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, right, _): (usize, usize, usize),
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
fn __action15<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, ParserError>, usize),
) -> Box<Instruction<'input>>
{
    { errors.push(__0); Box::new(Instruction::Error) }
}

#[allow(unused_variables)]
fn __action16<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, args, _): (usize, alloc::vec::Vec<Value<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> Vec<Value<'input>>
{
    args
}

#[allow(unused_variables)]
fn __action17<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, f, _): (usize, CallInstrValue<'input>, usize),
) -> FunctionPart<'input>
{
    FunctionPart::FuncName(f)
}

#[allow(unused_variables)]
fn __action18<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, sid, _): (usize, CallInstrValue<'input>, usize),
    (_, f, _): (usize, CallInstrValue<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> FunctionPart<'input>
{
    FunctionPart::ServiceIdWithFuncName(sid, f)
}

#[allow(unused_variables)]
fn __action19<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, pid, _): (usize, CallInstrValue<'input>, usize),
) -> PeerPart<'input>
{
    PeerPart::PeerPk(pid)
}

#[allow(unused_variables)]
fn __action20<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, pid, _): (usize, CallInstrValue<'input>, usize),
    (_, sid, _): (usize, CallInstrValue<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> PeerPart<'input>
{
    PeerPart::PeerPkWithServiceId(pid, sid)
}

#[allow(unused_variables)]
fn __action21<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, Variable<'input>, usize),
) -> Variable<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action22<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, Variable<'input>, usize),
) -> Variable<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action23<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (usize, (&'input str, usize), usize),
) -> Variable<'input>
{
    Variable::scalar(scalar.0, scalar.1)
}

#[allow(unused_variables)]
fn __action24<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, stream, _): (usize, (&'input str, usize), usize),
) -> Variable<'input>
{
    Variable::stream(stream.0, stream.1)
}

#[allow(unused_variables)]
fn __action25<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (usize, (&'input str, usize), usize),
) -> Fail<'input>
{
    Fail::Scalar(ScalarWithLambda::new(scalar.0, None, scalar.1))
}

#[allow(unused_variables)]
fn __action26<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (usize, (&'input str, LambdaAST<'input>, usize), usize),
) -> Fail<'input>
{
    Fail::Scalar(ScalarWithLambda::new(scalar.0, Some(scalar.1), scalar.2))
}

#[allow(unused_variables)]
fn __action27<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, ret_code, _): (usize, i64, usize),
    (_, error_message, _): (usize, &'input str, usize),
) -> Fail<'input>
{
    Fail::Literal {
        ret_code,
        error_message,
    }
}

#[allow(unused_variables)]
fn __action28<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, left, _): (usize, usize, usize),
    (_, l, _): (usize, Token<'input>, usize),
    (_, right, _): (usize, usize, usize),
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
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (usize, (&'input str, usize), usize),
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
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (usize, (&'input str, LambdaAST<'input>, usize), usize),
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
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, Token<'input>, usize),
    (_, __1, _): (usize, Token<'input>, usize),
) -> FoldScalarIterable<'input>
{
    FoldScalarIterable::EmptyArray
}

#[allow(unused_variables)]
fn __action32<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, CallInstrValue<'input>, usize),
) -> CallInstrValue<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action33<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, CallInstrValue<'input>, usize),
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
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, CallInstrValue<'input>, usize),
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
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> CallInstrValue<'input>
{
    CallInstrValue::InitPeerId
}

#[allow(unused_variables)]
fn __action36<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, l, _): (usize, &'input str, usize),
) -> CallInstrValue<'input>
{
    CallInstrValue::Literal(l)
}

#[allow(unused_variables)]
fn __action37<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (usize, (&'input str, usize), usize),
) -> CallInstrValue<'input>
{
    CallInstrValue::Variable(VariableWithLambda::scalar(scalar.0, scalar.1))
}

#[allow(unused_variables)]
fn __action38<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (usize, (&'input str, LambdaAST<'input>, usize), usize),
) -> CallInstrValue<'input>
{
    CallInstrValue::Variable(VariableWithLambda::scalar_wl(scalar.0, scalar.1, scalar.2))
}

#[allow(unused_variables)]
fn __action39<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, stream, _): (usize, (&'input str, usize), usize),
) -> CallInstrValue<'input>
{
    CallInstrValue::Variable(VariableWithLambda::stream(stream.0, stream.1))
}

#[allow(unused_variables)]
fn __action40<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, stream, _): (usize, (&'input str, LambdaAST<'input>, usize), usize),
) -> CallInstrValue<'input>
{
    CallInstrValue::Variable(VariableWithLambda::stream_wl(stream.0, stream.1, stream.2))
}

#[allow(unused_variables)]
fn __action41<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, integer, _): (usize, i64, usize),
) -> Number
{
    Number::Int(integer)
}

#[allow(unused_variables)]
fn __action42<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, float, _): (usize, f64, usize),
) -> Number
{
    Number::Float(float)
}

#[allow(unused_variables)]
fn __action43<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, Value<'input>, usize),
) -> Value<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action44<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> Value<'input>
{
    Value::InitPeerId
}

#[allow(unused_variables)]
fn __action45<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> Value<'input>
{
    Value::LastError(None)
}

#[allow(unused_variables)]
fn __action46<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, le, _): (usize, LambdaAST<'input>, usize),
) -> Value<'input>
{
    Value::LastError(Some(le))
}

#[allow(unused_variables)]
fn __action47<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, l, _): (usize, &'input str, usize),
) -> Value<'input>
{
    Value::Literal(l)
}

#[allow(unused_variables)]
fn __action48<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> Value<'input>
{
    Value::Timestamp
}

#[allow(unused_variables)]
fn __action49<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> Value<'input>
{
    Value::TTL
}

#[allow(unused_variables)]
fn __action50<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, n, _): (usize, Number, usize),
) -> Value<'input>
{
    Value::Number(n)
}

#[allow(unused_variables)]
fn __action51<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, b, _): (usize, bool, usize),
) -> Value<'input>
{
    Value::Boolean(b)
}

#[allow(unused_variables)]
fn __action52<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, Token<'input>, usize),
    (_, __1, _): (usize, Token<'input>, usize),
) -> Value<'input>
{
    Value::EmptyArray
}

#[allow(unused_variables)]
fn __action53<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (usize, (&'input str, usize), usize),
) -> Value<'input>
{
    Value::Variable(VariableWithLambda::scalar(scalar.0, scalar.1))
}

#[allow(unused_variables)]
fn __action54<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (usize, (&'input str, LambdaAST<'input>, usize), usize),
) -> Value<'input>
{
    Value::Variable(VariableWithLambda::scalar_wl(scalar.0, scalar.1, scalar.2))
}

#[allow(unused_variables)]
fn __action55<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, stream, _): (usize, (&'input str, usize), usize),
) -> Value<'input>
{
    Value::Variable(VariableWithLambda::stream(stream.0, stream.1))
}

#[allow(unused_variables)]
fn __action56<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, stream, _): (usize, (&'input str, LambdaAST<'input>, usize), usize),
) -> Value<'input>
{
    Value::Variable(VariableWithLambda::stream_wl(stream.0, stream.1, stream.2))
}

#[allow(unused_variables)]
fn __action57<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> ApArgument<'input>
{
    ApArgument::InitPeerId
}

#[allow(unused_variables)]
fn __action58<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> ApArgument<'input>
{
    ApArgument::LastError(None)
}

#[allow(unused_variables)]
fn __action59<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, le, _): (usize, LambdaAST<'input>, usize),
) -> ApArgument<'input>
{
    ApArgument::LastError(Some(le))
}

#[allow(unused_variables)]
fn __action60<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> ApArgument<'input>
{
    ApArgument::Timestamp
}

#[allow(unused_variables)]
fn __action61<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> ApArgument<'input>
{
    ApArgument::TTL
}

#[allow(unused_variables)]
fn __action62<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, l, _): (usize, &'input str, usize),
) -> ApArgument<'input>
{
    ApArgument::Literal(l)
}

#[allow(unused_variables)]
fn __action63<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, n, _): (usize, Number, usize),
) -> ApArgument<'input>
{
    ApArgument::Number(n)
}

#[allow(unused_variables)]
fn __action64<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, b, _): (usize, bool, usize),
) -> ApArgument<'input>
{
    ApArgument::Boolean(b)
}

#[allow(unused_variables)]
fn __action65<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, Token<'input>, usize),
    (_, __1, _): (usize, Token<'input>, usize),
) -> ApArgument<'input>
{
    ApArgument::EmptyArray
}

#[allow(unused_variables)]
fn __action66<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (usize, (&'input str, usize), usize),
) -> ApArgument<'input>
{
    ApArgument::Scalar(ScalarWithLambda::new(scalar.0, None, scalar.1))
}

#[allow(unused_variables)]
fn __action67<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, scalar, _): (usize, (&'input str, LambdaAST<'input>, usize), usize),
) -> ApArgument<'input>
{
    ApArgument::Scalar(ScalarWithLambda::new(scalar.0, Some(scalar.1), scalar.2))
}

#[allow(unused_variables)]
fn __action68<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Value<'input>>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action69<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, v, _): (usize, alloc::vec::Vec<Value<'input>>, usize),
) -> alloc::vec::Vec<Value<'input>>
{
    v
}

#[allow(unused_variables)]
fn __action70<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, Value<'input>, usize),
) -> Value<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action71<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

#[allow(unused_variables)]
fn __action72<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, Variable<'input>, usize),
) -> core::option::Option<Variable<'input>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action73<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Variable<'input>>
{
    None
}

#[allow(unused_variables)]
fn __action74<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

#[allow(unused_variables)]
fn __action75<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, __0, _): (usize, Value<'input>, usize),
) -> alloc::vec::Vec<Value<'input>>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action76<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    (_, v, _): (usize, alloc::vec::Vec<Value<'input>>, usize),
    (_, e, _): (usize, Value<'input>, usize),
) -> alloc::vec::Vec<Value<'input>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action77<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Value<'input>, usize),
) -> alloc::vec::Vec<Value<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action70(
        input,
        errors,
        validator,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action75(
        input,
        errors,
        validator,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action78<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, alloc::vec::Vec<Value<'input>>, usize),
    __1: (usize, Value<'input>, usize),
) -> alloc::vec::Vec<Value<'input>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action70(
        input,
        errors,
        validator,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action76(
        input,
        errors,
        validator,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action79<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
) -> Vec<Value<'input>>
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action68(
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
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action80<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, alloc::vec::Vec<Value<'input>>, usize),
    __2: (usize, Token<'input>, usize),
) -> Vec<Value<'input>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action69(
        input,
        errors,
        validator,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        input,
        errors,
        validator,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action81<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, usize, usize),
) -> Fail<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action74(
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
fn __action82<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, PeerPart<'input>, usize),
    __3: (usize, FunctionPart<'input>, usize),
    __4: (usize, Vec<Value<'input>>, usize),
    __5: (usize, core::option::Option<Variable<'input>>, usize),
    __6: (usize, Token<'input>, usize),
    __7: (usize, usize, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action74(
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
        __7,
    )
}

#[allow(unused_variables)]
fn __action83<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, ApArgument<'input>, usize),
    __3: (usize, Variable<'input>, usize),
    __4: (usize, Token<'input>, usize),
    __5: (usize, usize, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action74(
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
    )
}

#[allow(unused_variables)]
fn __action84<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, Variable<'input>, usize),
    __3: (usize, Box<Instruction<'input>>, usize),
    __4: (usize, Token<'input>, usize),
    __5: (usize, usize, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action74(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
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
fn __action85<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, FoldScalarIterable<'input>, usize),
    __3: (usize, (&'input str, usize), usize),
    __4: (usize, Box<Instruction<'input>>, usize),
    __5: (usize, Token<'input>, usize),
    __6: (usize, usize, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action74(
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
        __6,
    )
}

#[allow(unused_variables)]
fn __action86<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, (&'input str, usize), usize),
    __3: (usize, (&'input str, usize), usize),
    __4: (usize, Box<Instruction<'input>>, usize),
    __5: (usize, Token<'input>, usize),
    __6: (usize, usize, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action74(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
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
fn __action87<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, (&'input str, usize), usize),
    __3: (usize, Token<'input>, usize),
    __4: (usize, usize, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action74(
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
    )
}

#[allow(unused_variables)]
fn __action88<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, Value<'input>, usize),
    __3: (usize, Value<'input>, usize),
    __4: (usize, Box<Instruction<'input>>, usize),
    __5: (usize, Token<'input>, usize),
    __6: (usize, usize, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action74(
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
        __5,
        __6,
    )
}

#[allow(unused_variables)]
fn __action89<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, Value<'input>, usize),
    __3: (usize, Value<'input>, usize),
    __4: (usize, Box<Instruction<'input>>, usize),
    __5: (usize, Token<'input>, usize),
    __6: (usize, usize, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action74(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action14(
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
fn __action90<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
) -> Fail<'input>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action71(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action81(
        input,
        errors,
        validator,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action91<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, PeerPart<'input>, usize),
    __3: (usize, FunctionPart<'input>, usize),
    __4: (usize, Vec<Value<'input>>, usize),
    __5: (usize, core::option::Option<Variable<'input>>, usize),
    __6: (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __6.2.clone();
    let __end0 = __6.2.clone();
    let __temp0 = __action71(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action82(
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
fn __action92<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, ApArgument<'input>, usize),
    __3: (usize, Variable<'input>, usize),
    __4: (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __4.2.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action71(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action83(
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
fn __action93<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, Variable<'input>, usize),
    __3: (usize, Box<Instruction<'input>>, usize),
    __4: (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __4.2.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action71(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action84(
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
fn __action94<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, FoldScalarIterable<'input>, usize),
    __3: (usize, (&'input str, usize), usize),
    __4: (usize, Box<Instruction<'input>>, usize),
    __5: (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __5.2.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action71(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action85(
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
fn __action95<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, (&'input str, usize), usize),
    __3: (usize, (&'input str, usize), usize),
    __4: (usize, Box<Instruction<'input>>, usize),
    __5: (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __5.2.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action71(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action86(
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
fn __action96<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, (&'input str, usize), usize),
    __3: (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action71(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action87(
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
fn __action97<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, Value<'input>, usize),
    __3: (usize, Value<'input>, usize),
    __4: (usize, Box<Instruction<'input>>, usize),
    __5: (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __5.2.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action71(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action88(
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
fn __action98<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, Value<'input>, usize),
    __3: (usize, Value<'input>, usize),
    __4: (usize, Box<Instruction<'input>>, usize),
    __5: (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __5.2.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action71(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action89(
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
fn __action99<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, PeerPart<'input>, usize),
    __3: (usize, FunctionPart<'input>, usize),
    __4: (usize, Vec<Value<'input>>, usize),
    __5: (usize, Variable<'input>, usize),
    __6: (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __5.0.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action72(
        input,
        errors,
        validator,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action91(
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
fn __action100<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, PeerPart<'input>, usize),
    __3: (usize, FunctionPart<'input>, usize),
    __4: (usize, Vec<Value<'input>>, usize),
    __5: (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __4.2.clone();
    let __end0 = __5.0.clone();
    let __temp0 = __action73(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action91(
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
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, ParserError>>;
}

impl<'err, 'input, 'v, > __ToTriple<'err, 'input, 'v, > for (usize, Token<'input>, usize)
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, ParserError>> {
        Ok(value)
    }
}
impl<'err, 'input, 'v, > __ToTriple<'err, 'input, 'v, > for Result<(usize, Token<'input>, usize), ParserError>
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, ParserError>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
