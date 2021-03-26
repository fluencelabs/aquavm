// auto-generated: "lalrpop 0.19.5"
// sha3: 459487c956989dc625789699836b2960f7bc355b78ff9a139d2fa6938832545
use crate::parser::ast::*;
use crate::parser::air_parser::make_flattened_error;
use crate::parser::ParserError;
use crate::parser::VariableValidator;
use crate::parser::Span;
use crate::parser::lexer::Token;
use crate::parser::lexer::Number;
use crate::parser::lexer::Variable;
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
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::parser::ast::*;
    use crate::parser::air_parser::make_flattened_error;
    use crate::parser::ParserError;
    use crate::parser::VariableValidator;
    use crate::parser::Span;
    use crate::parser::lexer::Token;
    use crate::parser::lexer::Number;
    use crate::parser::lexer::Variable;
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
        Variant1(&'input str),
        Variant2(bool),
        Variant3((Variable<'input>, &'input str, bool)),
        Variant4(Number),
        Variant5(__lalrpop_util::ErrorRecovery<usize, Token<'input>, ParserError>),
        Variant6(CallInstrArgValue<'input>),
        Variant7(alloc::vec::Vec<CallInstrArgValue<'input>>),
        Variant8(usize),
        Variant9(Box<Instruction<'input>>),
        Variant10(Vec<CallInstrArgValue<'input>>),
        Variant11(CallInstrValue<'input>),
        Variant12(FunctionPart<'input>),
        Variant13(IterableValue<'input>),
        Variant14(MatchableValue<'input>),
        Variant15(CallOutputValue<'input>),
        Variant16(core::option::Option<CallOutputValue<'input>>),
        Variant17(PeerPart<'input>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29,
        // State 1
        10, 0, 0, 0, 34, 0, 35, 36, 0, 37, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 40, 0, 0, 41, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 43, 44, 0, 45, 0, 46, 47, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 43, 44, 0, 45, 0, 46, 47, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29,
        // State 6
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29,
        // State 7
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29,
        // State 8
        17, 0, 0, 0, 34, 0, 35, 36, 0, 37, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 34, 0, 35, 36, 0, 37, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 43, 44, 0, 45, 0, 46, 47, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 43, 44, 0, 45, 0, 46, 47, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29,
        // State 13
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29,
        // State 14
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29,
        // State 15
        0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 34, 0, 35, 36, 0, 37, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 34, 0, 35, 36, 0, 37, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29,
        // State 19
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29,
        // State 20
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29,
        // State 21
        0, 66, 0, 0, 67, 0, 0, 0, 0, 0, 0, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 71, 72, 73, 74, 75, 76, 77, 78, 79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 34, 0, 35, 36, 0, 37, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 87, 72, 73, 74, 75, 76, 77, 78, 79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 4, 5, 30, 31, 6, 7, 8, 0,
        // State 28
        -38, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38,
        // State 29
        0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        -52, 0, 0, 0, -52, 0, -52, -52, 0, -52, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        -53, 0, 0, 0, -53, 0, -53, -53, 0, -53, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        -21, -21, -21, 0, -21, 0, -21, -21, 0, -21, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        -24, -24, -24, 0, -24, 0, -24, -24, 0, -24, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        -23, -23, -23, 0, -23, 0, -23, -23, 0, -23, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        -20, -20, -20, 0, -20, 0, -20, -20, 0, -20, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        -22, -22, -22, 0, -22, 0, -22, -22, 0, -22, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        -42, 0, 0, 0, -42, -42, 0, -42, 0, -42, -42, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42,
        // State 43
        -45, 0, 0, 0, -45, -45, 0, -45, 0, -45, -45, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45,
        // State 44
        -47, 0, 0, 0, -47, -47, 0, -47, 0, -47, -47, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, -47,
        // State 45
        -44, 0, 0, 0, -44, -44, 0, -44, 0, -44, -44, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44,
        // State 46
        -46, 0, 0, 0, -46, -46, 0, -46, 0, -46, -46, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46,
        // State 47
        -43, 0, 0, 0, -43, -43, 0, -43, 0, -43, -43, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43,
        // State 48
        0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        -32, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32,
        // State 50
        0, -27, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        -34, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34,
        // State 53
        0, 62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, -55, 0, 0, -55, 0, -55, -55, 0, -55, 0, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        -31, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31,
        // State 62
        -30, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30,
        // State 63
        -35, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35,
        // State 64
        0, 85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        -29, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29,
        // State 66
        0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, -4, -4, -4, -4, -4, -4, -4, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, -9, -9, -9, -9, -9, -9, -9, -9, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, -10, 0, 0, -10, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        0, 0, 0, -13, -13, -13, -13, -13, -13, -13, -13, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, -17, -17, -17, -17, -17, -17, -17, -17, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, -18, -18, -18, -18, -18, -18, -18, -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, -15, -15, -15, -15, -15, -15, -15, -15, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 75
        0, 0, 0, -19, -19, -19, -19, -19, -19, -19, -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 0, 0, -12, -12, -12, -12, -12, -12, -12, -12, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 77
        0, 0, 0, -16, -16, -16, -16, -16, -16, -16, -16, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        0, 0, 0, -14, -14, -14, -14, -14, -14, -14, -14, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        0, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        -54, 0, 0, 0, -54, 0, -54, -54, 0, -54, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 81
        -33, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33,
        // State 82
        -36, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36,
        // State 83
        -37, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37,
        // State 84
        -28, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28,
        // State 85
        0, 0, 0, -5, -5, -5, -5, -5, -5, -5, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 86
        0, -11, 0, 0, -11, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 87
        0, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 22 + integer]
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
        -56,
        // State 26
        -8,
        // State 27
        0,
        // State 28
        -38,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
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
        -32,
        // State 50
        0,
        // State 51
        0,
        // State 52
        -34,
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
        -31,
        // State 62
        -30,
        // State 63
        -35,
        // State 64
        0,
        // State 65
        -29,
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
        -33,
        // State 82
        -36,
        // State 83
        -37,
        // State 84
        -28,
        // State 85
        0,
        // State 86
        0,
        // State 87
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 24,
            5 => 25,
            6 => match state {
                24 => 85,
                _ => 68,
            },
            7 => 21,
            8 => 69,
            9 => match state {
                8 | 23 => 50,
                16..=17 => 56,
                _ => 31,
            },
            10 => 15,
            11 => match state {
                23 => 79,
                _ => 51,
            },
            12 => match state {
                6 => 13,
                7 => 14,
                0 => 26,
                12 => 53,
                13 => 54,
                14 => 55,
                18 => 58,
                19 => 59,
                20 => 60,
                _ => 12,
            },
            13 => 38,
            14 => match state {
                4 => 11,
                10 => 19,
                11 => 20,
                _ => 10,
            },
            15 => 64,
            17 => match state {
                1 => 32,
                _ => 17,
            },
            18 => 8,
            19 => match state {
                17 => 57,
                _ => 23,
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
            r###"Alphanumeric"###,
            r###"Boolean"###,
            r###"InitPeerId"###,
            r###"JsonPath"###,
            r###"LastError"###,
            r###"Literal"###,
            r###"Number"###,
            r###"Stream"###,
            r###"call"###,
            r###"fold"###,
            r###"match_"###,
            r###"mismatch"###,
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
            __action(state, 22 - 1)
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
            __Symbol::Variant5(recovery)
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
            Token::Alphanumeric(_) if true => Some(4),
            Token::Boolean(_) if true => Some(5),
            Token::InitPeerId if true => Some(6),
            Token::VariableWithJsonPath(_, _, _) if true => Some(7),
            Token::LastError if true => Some(8),
            Token::StringLiteral(_) if true => Some(9),
            Token::Number(_) if true => Some(10),
            Token::Stream(_) if true => Some(11),
            Token::Call if true => Some(12),
            Token::Fold if true => Some(13),
            Token::Match if true => Some(14),
            Token::MisMatch if true => Some(15),
            Token::Next if true => Some(16),
            Token::Null if true => Some(17),
            Token::Par if true => Some(18),
            Token::Seq if true => Some(19),
            Token::Xor if true => Some(20),
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
            0 | 1 | 2 | 3 | 6 | 8 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 => __Symbol::Variant0(__token),
            4 | 9 | 11 => match __token {
                Token::Alphanumeric(__tok0) | Token::StringLiteral(__tok0) | Token::Stream(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            5 => match __token {
                Token::Boolean(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            7 => match __token {
                Token::VariableWithJsonPath(__tok0, __tok1, __tok2) if true => __Symbol::Variant3((__tok0, __tok1, __tok2)),
                _ => unreachable!(),
            },
            10 => match __token {
                Token::Number(__tok0) if true => __Symbol::Variant4(__tok0),
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
                    states_to_pop: 2,
                    nonterminal_produced: 7,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 7,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
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
                    states_to_pop: 4,
                    nonterminal_produced: 10,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 12,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 12,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 12,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 12,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 12,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 12,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 12,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 12,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 12,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 12,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
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
                    nonterminal_produced: 13,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
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
                    nonterminal_produced: 14,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 16,
                }
            }
            51 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            52 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            53 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 18,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            55 => __state_machine::SimulatedReduce::Accept,
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
                // __AIR = AIR => ActionFn(0);
                let __sym0 = __pop_Variant9(__symbols);
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
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (Variable<'input>, &'input str, bool), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Instruction<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, CallInstrArgValue<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, CallInstrValue<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, CallOutputValue<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, FunctionPart<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IterableValue<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, MatchableValue<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Number, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, PeerPart<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
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
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<CallInstrArgValue<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, ParserError>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<CallInstrArgValue<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, bool, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<CallOutputValue<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
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
        // (<Arg>) = Arg => ActionFn(47);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
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
        // (<Arg>)* =  => ActionFn(45);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action45::<>(input, errors, validator, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
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
        // (<Arg>)* = (<Arg>)+ => ActionFn(46);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
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
        // (<Arg>)+ = Arg => ActionFn(54);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action54::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
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
        // (<Arg>)+ = (<Arg>)+, Arg => ActionFn(55);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action55::<>(input, errors, validator, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
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
        // @L =  => ActionFn(51);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action51::<>(input, errors, validator, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
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
        // @R =  => ActionFn(48);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action48::<>(input, errors, validator, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
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
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
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
        // Arg = CallInstrArgValue => ActionFn(27);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
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
        // Args = "[", "]" => ActionFn(56);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action56::<>(input, errors, validator, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 7)
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
        // Args = "[", (<Arg>)+, "]" => ActionFn(57);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action57::<>(input, errors, validator, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (3, 7)
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
        // CallInstrArgValue = Literal => ActionFn(28);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
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
        // CallInstrArgValue = Alphanumeric => ActionFn(29);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
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
        // CallInstrArgValue = Stream => ActionFn(30);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
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
        // CallInstrArgValue = JsonPath => ActionFn(31);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
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
        // CallInstrArgValue = Number => ActionFn(32);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
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
        // CallInstrArgValue = Boolean => ActionFn(33);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
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
        // CallInstrArgValue = InitPeerId => ActionFn(34);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
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
        // CallInstrArgValue = LastError => ActionFn(35);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
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
        // CallInstrValue = Literal => ActionFn(22);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 9)
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
        // CallInstrValue = Alphanumeric => ActionFn(23);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 9)
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
        // CallInstrValue = Stream => ActionFn(24);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 9)
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
        // CallInstrValue = JsonPath => ActionFn(64);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action64::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 9)
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
        // CallInstrValue = InitPeerId => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 9)
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
        // FPart = Function => ActionFn(13);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
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
        // FPart = "(", ServiceId, Function, ")" => ActionFn(14);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant11(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action14::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 10)
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
        // Function = CallInstrValue => ActionFn(19);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 11)
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
        // Instr = "(", call, PeerPart, FPart, Args, Output, ")" => ActionFn(70);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant15(__symbols);
        let __sym4 = __pop_Variant10(__symbols);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant17(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action70::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (7, 12)
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
        // Instr = "(", call, PeerPart, FPart, Args, ")" => ActionFn(71);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant10(__symbols);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant17(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action71::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (6, 12)
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
        // Instr = "(", seq, Instr, Instr, ")" => ActionFn(3);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant9(__symbols);
        let __sym2 = __pop_Variant9(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action3::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 12)
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
        // Instr = "(", par, Instr, Instr, ")" => ActionFn(4);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant9(__symbols);
        let __sym2 = __pop_Variant9(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action4::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 12)
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
        // Instr = "(", null, ")" => ActionFn(5);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action5::<>(input, errors, validator, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 12)
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
        // Instr = "(", fold, Iterable, Alphanumeric, Instr, ")" => ActionFn(66);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant9(__symbols);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action66::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (6, 12)
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
        // Instr = "(", next, Alphanumeric, ")" => ActionFn(67);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action67::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 12)
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
        // Instr = "(", xor, Instr, Instr, ")" => ActionFn(8);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant9(__symbols);
        let __sym2 = __pop_Variant9(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action8::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 12)
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
        // Instr = "(", match_, Matchable, Matchable, Instr, ")" => ActionFn(68);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant9(__symbols);
        let __sym3 = __pop_Variant14(__symbols);
        let __sym2 = __pop_Variant14(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action68::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (6, 12)
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
        // Instr = "(", mismatch, Matchable, Matchable, Instr, ")" => ActionFn(69);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant9(__symbols);
        let __sym3 = __pop_Variant14(__symbols);
        let __sym2 = __pop_Variant14(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action69::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (6, 12)
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
        // Instr = error => ActionFn(11);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 12)
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
        // Iterable = Alphanumeric => ActionFn(36);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 13)
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
        // Iterable = Stream => ActionFn(37);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 13)
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
        // Iterable = JsonPath => ActionFn(38);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 13)
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
        // Matchable = Alphanumeric => ActionFn(39);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 14)
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
        // Matchable = Stream => ActionFn(40);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 14)
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
        // Matchable = Literal => ActionFn(41);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 14)
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
        // Matchable = Boolean => ActionFn(42);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 14)
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
        // Matchable = Number => ActionFn(43);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 14)
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
        // Matchable = JsonPath => ActionFn(44);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 14)
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
        // Output = Alphanumeric => ActionFn(17);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 15)
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
        // Output = Stream => ActionFn(18);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 15)
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
        // Output? = Output => ActionFn(49);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 16)
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
        // Output? =  => ActionFn(50);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action50::<>(input, errors, validator, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (0, 16)
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
        // PeerId = CallInstrValue => ActionFn(20);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 17)
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
        // PeerPart = PeerId => ActionFn(15);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 18)
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
        // PeerPart = "(", PeerId, ServiceId, ")" => ActionFn(16);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant11(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action16::<>(input, errors, validator, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (4, 18)
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
        // ServiceId = CallInstrValue => ActionFn(21);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(input, errors, validator, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 19)
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
    (_, args, _): (usize, Vec<CallInstrArgValue<'input>>, usize),
    (_, output, _): (usize, core::option::Option<CallOutputValue<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, right, _): (usize, usize, usize),
) -> Box<Instruction<'input>>
{
    {
        let output = output.unwrap_or(CallOutputValue::None);
        let args = Rc::new(args);
        let call = Call { peer_part, function_part, args, output };
        let span = Span { left, right };
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
    (_, _, _): (usize, Token<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, l, _): (usize, Box<Instruction<'input>>, usize),
    (_, r, _): (usize, Box<Instruction<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    Box::new(Instruction::Seq(Seq(l, r)))
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
    Box::new(Instruction::Par(Par(l, r)))
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
    (_, __0, _): (usize, Token<'input>, usize),
    (_, __1, _): (usize, Token<'input>, usize),
    (_, __2, _): (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    Box::new(Instruction::Null(Null))
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
    (_, left, _): (usize, usize, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, iterable, _): (usize, IterableValue<'input>, usize),
    (_, iterator, _): (usize, &'input str, usize),
    (_, i, _): (usize, Box<Instruction<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, right, _): (usize, usize, usize),
) -> Box<Instruction<'input>>
{
    {
        let instruction = Rc::new(*i);
        let fold = Fold { iterable, iterator, instruction };
        let span = Span { left, right };
        validator.met_fold(&fold, span);

        Box::new(Instruction::Fold(fold))
    }
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
    (_, i, _): (usize, &'input str, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, right, _): (usize, usize, usize),
) -> Box<Instruction<'input>>
{
    {
        let next = Next(i);
        let span = Span { left, right };
        validator.met_next(&next, span);

        Box::new(Instruction::Next(next))
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
    (_, l, _): (usize, Box<Instruction<'input>>, usize),
    (_, r, _): (usize, Box<Instruction<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    Box::new(Instruction::Xor(Xor(l, r)))
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
    (_, l, _): (usize, MatchableValue<'input>, usize),
    (_, r, _): (usize, MatchableValue<'input>, usize),
    (_, i, _): (usize, Box<Instruction<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, right, _): (usize, usize, usize),
) -> Box<Instruction<'input>>
{
    {
        let match_ = Match { left_value: l, right_value: r, instruction: i};
        let span = Span { left, right };
        validator.met_match(&match_, span);

        Box::new(Instruction::Match(match_))
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
    (_, l, _): (usize, MatchableValue<'input>, usize),
    (_, r, _): (usize, MatchableValue<'input>, usize),
    (_, i, _): (usize, Box<Instruction<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, right, _): (usize, usize, usize),
) -> Box<Instruction<'input>>
{
    {
        let mismatch = MisMatch { left_value: l, right_value: r, instruction: i};
        let span = Span { left, right };
        validator.met_mismatch(&mismatch, span);

        Box::new(Instruction::MisMatch(mismatch))
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
    (_, __0, _): (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, ParserError>, usize),
) -> Box<Instruction<'input>>
{
    { errors.push(__0); Box::new(Instruction::Error) }
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
    (_, args, _): (usize, alloc::vec::Vec<CallInstrArgValue<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> Vec<CallInstrArgValue<'input>>
{
    args
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
    (_, f, _): (usize, CallInstrValue<'input>, usize),
) -> FunctionPart<'input>
{
    FunctionPart::FuncName(f)
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
    (_, _, _): (usize, Token<'input>, usize),
    (_, sid, _): (usize, CallInstrValue<'input>, usize),
    (_, f, _): (usize, CallInstrValue<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> FunctionPart<'input>
{
    FunctionPart::ServiceIdWithFuncName(sid, f)
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
    (_, pid, _): (usize, CallInstrValue<'input>, usize),
) -> PeerPart<'input>
{
    PeerPart::PeerPk(pid)
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
    (_, pid, _): (usize, CallInstrValue<'input>, usize),
    (_, sid, _): (usize, CallInstrValue<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> PeerPart<'input>
{
    PeerPart::PeerPkWithServiceId(pid, sid)
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
    (_, a, _): (usize, &'input str, usize),
) -> CallOutputValue<'input>
{
    CallOutputValue::Variable(Variable::Scalar(a))
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
    (_, s, _): (usize, &'input str, usize),
) -> CallOutputValue<'input>
{
    CallOutputValue::Variable(Variable::Stream(s))
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
    (_, __0, _): (usize, CallInstrValue<'input>, usize),
) -> CallInstrValue<'input>
{
    __0
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
    (_, __0, _): (usize, CallInstrValue<'input>, usize),
) -> CallInstrValue<'input>
{
    __0
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
    (_, __0, _): (usize, CallInstrValue<'input>, usize),
) -> CallInstrValue<'input>
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
    (_, l, _): (usize, &'input str, usize),
) -> CallInstrValue<'input>
{
    CallInstrValue::Literal(l)
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
    (_, a, _): (usize, &'input str, usize),
) -> CallInstrValue<'input>
{
    CallInstrValue::Variable(Variable::Scalar(a))
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
    (_, s, _): (usize, &'input str, usize),
) -> CallInstrValue<'input>
{
    CallInstrValue::Variable(Variable::Stream(s))
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
    (_, l, _): (usize, usize, usize),
    (_, j, _): (usize, (Variable<'input>, &'input str, bool), usize),
    (_, r, _): (usize, usize, usize),
) -> CallInstrValue<'input>
{
    {
        let variable = j.0;
        let path = j.1;
        let should_flatten = j.2;
        // Due the json path constraints json path should be flattened in a call triplet.
        if !should_flatten {
            let token = Token::VariableWithJsonPath(variable.clone(), path, should_flatten);
            errors.push(make_flattened_error(l, token, r));
        }
        CallInstrValue::JsonPath { variable, path, should_flatten }
    }
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
    (_, __0, _): (usize, Token<'input>, usize),
) -> CallInstrValue<'input>
{
    CallInstrValue::InitPeerId
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
    (_, __0, _): (usize, CallInstrArgValue<'input>, usize),
) -> CallInstrArgValue<'input>
{
    __0
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
    (_, s, _): (usize, &'input str, usize),
) -> CallInstrArgValue<'input>
{
    CallInstrArgValue::Literal(s)
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
    (_, v, _): (usize, &'input str, usize),
) -> CallInstrArgValue<'input>
{
    CallInstrArgValue::Variable(Variable::Scalar(v))
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
    (_, v, _): (usize, &'input str, usize),
) -> CallInstrArgValue<'input>
{
    CallInstrArgValue::Variable(Variable::Stream(v))
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
    (_, j, _): (usize, (Variable<'input>, &'input str, bool), usize),
) -> CallInstrArgValue<'input>
{
    CallInstrArgValue::JsonPath { variable: j.0, path: j.1, should_flatten: j.2 }
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
    (_, n, _): (usize, Number, usize),
) -> CallInstrArgValue<'input>
{
    CallInstrArgValue::Number(n)
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
    (_, b, _): (usize, bool, usize),
) -> CallInstrArgValue<'input>
{
    CallInstrArgValue::Boolean(b)
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
    (_, __0, _): (usize, Token<'input>, usize),
) -> CallInstrArgValue<'input>
{
    CallInstrArgValue::InitPeerId
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
) -> CallInstrArgValue<'input>
{
    CallInstrArgValue::LastError
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
    (_, v, _): (usize, &'input str, usize),
) -> IterableValue<'input>
{
    IterableValue::Variable(Variable::Scalar(v))
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
    (_, v, _): (usize, &'input str, usize),
) -> IterableValue<'input>
{
    IterableValue::Variable(Variable::Stream(v))
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
    (_, j, _): (usize, (Variable<'input>, &'input str, bool), usize),
) -> IterableValue<'input>
{
    IterableValue::JsonPath { variable: j.0, path: j.1, should_flatten: j.2 }
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
    (_, v, _): (usize, &'input str, usize),
) -> MatchableValue<'input>
{
    MatchableValue::Variable(Variable::Scalar(v))
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
    (_, v, _): (usize, &'input str, usize),
) -> MatchableValue<'input>
{
    MatchableValue::Variable(Variable::Stream(v))
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
    (_, s, _): (usize, &'input str, usize),
) -> MatchableValue<'input>
{
    MatchableValue::Literal(s)
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
    (_, b, _): (usize, bool, usize),
) -> MatchableValue<'input>
{
    MatchableValue::Boolean(b)
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
    (_, n, _): (usize, Number, usize),
) -> MatchableValue<'input>
{
    MatchableValue::Number(n)
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
    (_, j, _): (usize, (Variable<'input>, &'input str, bool), usize),
) -> MatchableValue<'input>
{
    MatchableValue::JsonPath { variable: j.0, path: j.1, should_flatten: j.2 }
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
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<CallInstrArgValue<'input>>
{
    alloc::vec![]
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
    (_, v, _): (usize, alloc::vec::Vec<CallInstrArgValue<'input>>, usize),
) -> alloc::vec::Vec<CallInstrArgValue<'input>>
{
    v
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
    (_, __0, _): (usize, CallInstrArgValue<'input>, usize),
) -> CallInstrArgValue<'input>
{
    __0
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
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
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
    (_, __0, _): (usize, CallOutputValue<'input>, usize),
) -> core::option::Option<CallOutputValue<'input>>
{
    Some(__0)
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
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<CallOutputValue<'input>>
{
    None
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
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
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
    (_, __0, _): (usize, CallInstrArgValue<'input>, usize),
) -> alloc::vec::Vec<CallInstrArgValue<'input>>
{
    alloc::vec![__0]
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
    (_, v, _): (usize, alloc::vec::Vec<CallInstrArgValue<'input>>, usize),
    (_, e, _): (usize, CallInstrArgValue<'input>, usize),
) -> alloc::vec::Vec<CallInstrArgValue<'input>>
{
    { let mut v = v; v.push(e); v }
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
    __0: (usize, CallInstrArgValue<'input>, usize),
) -> alloc::vec::Vec<CallInstrArgValue<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action47(
        input,
        errors,
        validator,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action52(
        input,
        errors,
        validator,
        __temp0,
    )
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
    __0: (usize, alloc::vec::Vec<CallInstrArgValue<'input>>, usize),
    __1: (usize, CallInstrArgValue<'input>, usize),
) -> alloc::vec::Vec<CallInstrArgValue<'input>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action47(
        input,
        errors,
        validator,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        input,
        errors,
        validator,
        __0,
        __temp0,
    )
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
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
) -> Vec<CallInstrArgValue<'input>>
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action45(
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
        __0,
        __temp0,
        __1,
    )
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
    __0: (usize, Token<'input>, usize),
    __1: (usize, alloc::vec::Vec<CallInstrArgValue<'input>>, usize),
    __2: (usize, Token<'input>, usize),
) -> Vec<CallInstrArgValue<'input>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action46(
        input,
        errors,
        validator,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        input,
        errors,
        validator,
        __0,
        __temp0,
        __2,
    )
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
    __0: (usize, (Variable<'input>, &'input str, bool), usize),
    __1: (usize, usize, usize),
) -> CallInstrValue<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action51(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action25(
        input,
        errors,
        validator,
        __temp0,
        __0,
        __1,
    )
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
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, PeerPart<'input>, usize),
    __3: (usize, FunctionPart<'input>, usize),
    __4: (usize, Vec<CallInstrArgValue<'input>>, usize),
    __5: (usize, core::option::Option<CallOutputValue<'input>>, usize),
    __6: (usize, Token<'input>, usize),
    __7: (usize, usize, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action51(
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
fn __action60<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, IterableValue<'input>, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, Box<Instruction<'input>>, usize),
    __5: (usize, Token<'input>, usize),
    __6: (usize, usize, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action51(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
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
fn __action61<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, Token<'input>, usize),
    __4: (usize, usize, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action51(
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
    )
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
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, MatchableValue<'input>, usize),
    __3: (usize, MatchableValue<'input>, usize),
    __4: (usize, Box<Instruction<'input>>, usize),
    __5: (usize, Token<'input>, usize),
    __6: (usize, usize, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action51(
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
fn __action63<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, MatchableValue<'input>, usize),
    __3: (usize, MatchableValue<'input>, usize),
    __4: (usize, Box<Instruction<'input>>, usize),
    __5: (usize, Token<'input>, usize),
    __6: (usize, usize, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action51(
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
fn __action64<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, (Variable<'input>, &'input str, bool), usize),
) -> CallInstrValue<'input>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action48(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        input,
        errors,
        validator,
        __0,
        __temp0,
    )
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
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, PeerPart<'input>, usize),
    __3: (usize, FunctionPart<'input>, usize),
    __4: (usize, Vec<CallInstrArgValue<'input>>, usize),
    __5: (usize, core::option::Option<CallOutputValue<'input>>, usize),
    __6: (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __6.2.clone();
    let __end0 = __6.2.clone();
    let __temp0 = __action48(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action59(
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
fn __action66<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, IterableValue<'input>, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, Box<Instruction<'input>>, usize),
    __5: (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __5.2.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action48(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action60(
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
fn __action67<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action48(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
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
fn __action68<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, MatchableValue<'input>, usize),
    __3: (usize, MatchableValue<'input>, usize),
    __4: (usize, Box<Instruction<'input>>, usize),
    __5: (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __5.2.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action48(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
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
fn __action69<
    'err,
    'input,
    'v,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    validator: &'v mut VariableValidator<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, MatchableValue<'input>, usize),
    __3: (usize, MatchableValue<'input>, usize),
    __4: (usize, Box<Instruction<'input>>, usize),
    __5: (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __5.2.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action48(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action63(
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
fn __action70<
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
    __4: (usize, Vec<CallInstrArgValue<'input>>, usize),
    __5: (usize, CallOutputValue<'input>, usize),
    __6: (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __5.0.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action49(
        input,
        errors,
        validator,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
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
fn __action71<
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
    __4: (usize, Vec<CallInstrArgValue<'input>>, usize),
    __5: (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __4.2.clone();
    let __end0 = __5.0.clone();
    let __temp0 = __action50(
        input,
        errors,
        validator,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
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

pub trait __ToTriple<'err, 'input, 'v, > {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, ParserError>>;
}

impl<'err, 'input, 'v, > __ToTriple<'err, 'input, 'v, > for (usize, Token<'input>, usize) {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, ParserError>> {
        Ok(value)
    }
}
impl<'err, 'input, 'v, > __ToTriple<'err, 'input, 'v, > for Result<(usize, Token<'input>, usize), ParserError> {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, ParserError>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
