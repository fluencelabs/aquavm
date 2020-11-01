// auto-generated: "lalrpop 0.19.1"
// sha256: 6158fd9537bdff745f51fc8c893d4def0dc25363827b4f2ce3aa4ad8304ce1
use crate::ast::*;
use crate::lalrpop::InstructionError;
use lalrpop_util::ErrorRecovery;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Instr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::ast::*;
    use crate::lalrpop::InstructionError;
    use lalrpop_util::ErrorRecovery;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(__lalrpop_util::ErrorRecovery<usize, Token<'input>, InstructionError>),
        Variant2(::std::vec::Vec<&'input str>),
        Variant3(Vec<&'input str>),
        Variant4(FunctionPart<'input>),
        Variant5(Box<Instruction<'input>>),
        Variant6(CallOutput<'input>),
        Variant7(PeerPart<'input>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        2, 0, 0, 0, 0, 0, 16,
        // State 1
        0, 0, 17, 18, 0, 0, 0,
        // State 2
        6, 0, 0, 0, 0, 21, 0,
        // State 3
        2, 0, 0, 0, 0, 0, 16,
        // State 4
        9, 0, 0, 0, 0, 21, 0,
        // State 5
        0, 0, 0, 0, 0, 21, 0,
        // State 6
        2, 0, 0, 0, 0, 0, 16,
        // State 7
        12, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 21, 0,
        // State 9
        0, 0, 0, 0, 0, 21, 0,
        // State 10
        0, 0, 0, 0, 30, 21, 0,
        // State 11
        0, 33, 0, 0, 0, 21, 0,
        // State 12
        0, 0, 0, 0, 0, 21, 0,
        // State 13
        0, 38, 0, 0, 0, 21, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0,
        // State 15
        -16, -16, 0, 0, 0, 0, -16,
        // State 16
        -10, 0, 0, 0, 0, -10, 0,
        // State 17
        -24, 0, 0, 0, 0, 0, -24,
        // State 18
        -20, 0, 0, 0, 0, -20, 0,
        // State 19
        -21, 0, 0, 0, 0, -21, 0,
        // State 20
        -6, -6, 0, 0, 0, -6, 0,
        // State 21
        -13, -13, 0, 0, 0, 0, 0,
        // State 22
        -11, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 27, 0, 0, 0, 0, 0,
        // State 24
        0, -25, 0, 0, 0, -25, 0,
        // State 25
        0, 35, 0, 0, 0, 0, 0,
        // State 26
        -14, -14, 0, 0, 0, 0, -14,
        // State 27
        0, -18, 0, 0, 0, 0, 0,
        // State 28
        0, 36, 0, 0, 0, 0, 0,
        // State 29
        0, -19, 0, 0, 0, 0, 0,
        // State 30
        0, -7, 0, 0, 0, -7, 0,
        // State 31
        0, -4, 0, 0, 0, -4, 0,
        // State 32
        0, 0, 0, 0, -8, -8, 0,
        // State 33
        0, 39, 0, 0, 0, 0, 0,
        // State 34
        -22, 0, 0, 0, 0, -22, 0,
        // State 35
        -15, -15, 0, 0, 0, 0, -15,
        // State 36
        0, -5, 0, 0, 0, -5, 0,
        // State 37
        0, 0, 0, 0, -9, -9, 0,
        // State 38
        -12, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 7 + integer]
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
        -26,
        // State 15
        -16,
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
        -14,
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
        0,
        // State 34
        0,
        // State 35
        -15,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 13,
            3 => match state {
                4 | 12 => 21,
                8..=9 => 24,
                10 => 27,
                11 | 13 => 30,
                _ => 18,
            },
            4 => match state {
                13 => 36,
                _ => 31,
            },
            5 => 10,
            6 => 2,
            7 => 7,
            8 => match state {
                12 => 33,
                _ => 22,
            },
            9 => match state {
                0 => 14,
                6 => 23,
                _ => 6,
            },
            11 => 28,
            12 => match state {
                2 => 19,
                _ => 9,
            },
            13 => 4,
            15 => 3,
            16 => match state {
                9 => 25,
                _ => 12,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""(""###,
            r###"")""###,
            r###""call""###,
            r###""seq""###,
            r###"ACCUMULATOR"###,
            r###"ALPHANUMERIC"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<'input, 'err>
    where 'input: 'err
    {
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __phantom: ::std::marker::PhantomData<(&'input (), &'err ())>,
    }
    impl<'input, 'err> __state_machine::ParserDefinition for __StateMachine<'input, 'err>
    where 'input: 'err
    {
        type Location = usize;
        type Error = InstructionError;
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
            __token_to_integer(token, ::std::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 7 - 1)
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
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
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
            __Symbol::Variant1(recovery)
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.errors,
                self.input,
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, ::std::marker::PhantomData::<(&(), &())>)
        }
    }
    fn __token_to_integer<
        'input,
        'err,
    >(
        __token: &Token<'input>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(0, _) if true => Some(0),
            Token(1, _) if true => Some(1),
            Token(4, _) if true => Some(2),
            Token(5, _) if true => Some(3),
            Token(2, _) if true => Some(4),
            Token(3, _) if true => Some(5),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        'err,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 => match __token {
                Token(0, __tok0) | Token(1, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(2, __tok0) | Token(3, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
        'err,
    >(
        __reduce_index: i8,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input, 'err>>
    where
        'input: 'err,
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
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
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
                    nonterminal_produced: 7,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 7,
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
                    states_to_pop: 5,
                    nonterminal_produced: 9,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 9,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 13,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            25 => __state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct InstrParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl InstrParser {
        pub fn new() -> InstrParser {
            let __builder = super::__intern_token::new_builder();
            InstrParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            'err,
        >(
            &self,
            errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
            input: &'input str,
        ) -> Result<Box<Instruction<'input>>, __lalrpop_util::ParseError<usize, Token<'input>, InstructionError>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    errors,
                    input,
                    __phantom: ::std::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __error_state: i8,
        __states: & [i8],
        __opt_integer: Option<usize>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
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
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), ::std::marker::PhantomData::<(&(), &())>) {
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
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> Option<Result<Box<Instruction<'input>>,__lalrpop_util::ParseError<usize, Token<'input>, InstructionError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                __reduce1(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            2 => {
                __reduce2(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                __reduce9(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            10 => {
                __reduce10(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            11 => {
                __reduce11(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                // __Instr = Instr => ActionFn(0);
                let __sym0 = __pop_Variant5(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
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
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Instruction<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, CallOutput<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, FunctionPart<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, PeerPart<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, InstructionError>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Arg>) = Arg => ActionFn(22);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Arg>)* =  => ActionFn(20);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action20::<>(errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Arg>)* = (<Arg>)+ => ActionFn(21);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Arg>)+ = Arg => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce4<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Arg>)+ = (<Arg>)+, Arg => ActionFn(26);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action26::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce5<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Alphanumeric = ALPHANUMERIC => ActionFn(19);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 3)
    }
    pub(crate) fn __reduce6<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Arg = Alphanumeric => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce7<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Args = "(", ")" => ActionFn(27);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action27::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce8<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Args = "(", (<Arg>)+, ")" => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce9<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Call = "call" => ActionFn(5);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce10<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // FPart = Function => ActionFn(7);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce11<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // FPart = "(", ServiceId, Function, ")" => ActionFn(8);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action8::<>(errors, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 7)
    }
    pub(crate) fn __reduce12<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Function = Alphanumeric => ActionFn(13);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce13<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", Seq, Instr, Instr, ")" => ActionFn(1);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action1::<>(errors, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (5, 9)
    }
    pub(crate) fn __reduce14<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", Call, PeerPart, FPart, Args, Output, ")" => ActionFn(2);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant6(__symbols);
        let __sym4 = __pop_Variant3(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action2::<>(errors, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (7, 9)
    }
    pub(crate) fn __reduce15<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Instr = error => ActionFn(3);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce16<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // LBR = "(" => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce17<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Output = Alphanumeric => ActionFn(11);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce18<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Output = ACCUMULATOR => ActionFn(12);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce19<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // PeerId = Alphanumeric => ActionFn(14);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce20<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // PeerPart = PeerId => ActionFn(9);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce21<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // PeerPart = "(", PeerId, ServiceId, ")" => ActionFn(10);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action10::<>(errors, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 13)
    }
    pub(crate) fn __reduce22<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // RBR = ")" => ActionFn(18);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce23<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Seq = "seq" => ActionFn(4);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce24<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ServiceId = Alphanumeric => ActionFn(15);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 16)
    }
}
pub use self::__parse__Instr::InstrParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use crate::ast::*;
    use crate::lalrpop::InstructionError;
    use lalrpop_util::ErrorRecovery;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("^(\\()", false),
            ("^(\\))", false),
            ("^([0-9A-Za-z]+\\[\\])", false),
            ("^([0-9A-Za-z]+)", false),
            ("^(call)", false),
            ("^(seq)", false),
            (r"^(\s*)", true),
        ];
        __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
    }
}
pub use self::__lalrpop_util::lexer::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Instruction<'input>>, usize),
) -> Box<Instruction<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action1<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, s, _): (usize, &'input str, usize),
    (_, l, _): (usize, Box<Instruction<'input>>, usize),
    (_, r, _): (usize, Box<Instruction<'input>>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Instruction<'input>>
{
    Box::new(Instruction::Seq(Seq(l, r)))
}

#[allow(unused_variables)]
fn __action2<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, c, _): (usize, &'input str, usize),
    (_, peer, _): (usize, PeerPart<'input>, usize),
    (_, f, _): (usize, FunctionPart<'input>, usize),
    (_, args, _): (usize, Vec<&'input str>, usize),
    (_, output, _): (usize, CallOutput<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Instruction<'input>>
{
    Box::new(Instruction::Call(Call{peer, f, args, output}))
}

#[allow(unused_variables)]
fn __action3<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, __0, _): (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, InstructionError>, usize),
) -> Box<Instruction<'input>>
{
    { errors.push(__0); Box::new(Instruction::Error) }
}

#[allow(unused_variables)]
fn __action4<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    __0
}

#[allow(unused_variables)]
fn __action5<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    __0
}

#[allow(unused_variables)]
fn __action6<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, args, _): (usize, ::std::vec::Vec<&'input str>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Vec<&'input str>
{
    args
}

#[allow(unused_variables)]
fn __action7<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, f, _): (usize, &'input str, usize),
) -> FunctionPart<'input>
{
    FunctionPart::FuncName(f)
}

#[allow(unused_variables)]
fn __action8<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, sid, _): (usize, &'input str, usize),
    (_, f, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> FunctionPart<'input>
{
    FunctionPart::ServiceIdWithFuncName(sid, f)
}

#[allow(unused_variables)]
fn __action9<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, pid, _): (usize, &'input str, usize),
) -> PeerPart<'input>
{
    PeerPart::PeerPk(pid)
}

#[allow(unused_variables)]
fn __action10<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, pid, _): (usize, &'input str, usize),
    (_, sid, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> PeerPart<'input>
{
    PeerPart::PeerPkWithServiceId(pid, sid)
}

#[allow(unused_variables)]
fn __action11<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, o, _): (usize, &'input str, usize),
) -> CallOutput<'input>
{
    CallOutput::Scalar(o)
}

#[allow(unused_variables)]
fn __action12<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, o, _): (usize, &'input str, usize),
) -> CallOutput<'input>
{
    CallOutput::Accumulator(o)
}

#[allow(unused_variables)]
fn __action13<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    __0
}

#[allow(unused_variables)]
fn __action14<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    __0
}

#[allow(unused_variables)]
fn __action15<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    __0
}

#[allow(unused_variables)]
fn __action16<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    __0
}

#[allow(unused_variables)]
fn __action17<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    __0
}

#[allow(unused_variables)]
fn __action18<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    __0
}

#[allow(unused_variables)]
fn __action19<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    __0
}

#[allow(unused_variables)]
fn __action20<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<&'input str>
{
    vec![]
}

#[allow(unused_variables)]
fn __action21<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<&'input str>, usize),
) -> ::std::vec::Vec<&'input str>
{
    v
}

#[allow(unused_variables)]
fn __action22<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    __0
}

#[allow(unused_variables)]
fn __action23<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ::std::vec::Vec<&'input str>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action24<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<&'input str>, usize),
    (_, e, _): (usize, &'input str, usize),
) -> ::std::vec::Vec<&'input str>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action25<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> ::std::vec::Vec<&'input str>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action22(
        errors,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action26<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<&'input str>, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<&'input str>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action22(
        errors,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action27<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
) -> Vec<&'input str>
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action20(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        errors,
        input,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action28<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, InstructionError>>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::vec::Vec<&'input str>, usize),
    __2: (usize, &'input str, usize),
) -> Vec<&'input str>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action21(
        errors,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        errors,
        input,
        __0,
        __temp0,
        __2,
    )
}

pub trait __ToTriple<'input, 'err, > {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, InstructionError>>;
}

impl<'input, 'err, > __ToTriple<'input, 'err, > for (usize, Token<'input>, usize) {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, InstructionError>> {
        Ok(value)
    }
}
impl<'input, 'err, > __ToTriple<'input, 'err, > for Result<(usize, Token<'input>, usize), InstructionError> {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, InstructionError>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
