// auto-generated: "lalrpop 0.19.6"
// sha3: db7b23bede3d6914e4bbadd2b1df6f44d50b164a9c2ed557274f415fc6b1d5
use crate::ValueAccessor;
use crate::parser::lexer::LexerError;
use crate::parser::lexer::Token;
use lalrpop_util::ErrorRecovery;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Lambda {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::ValueAccessor;
    use crate::parser::lexer::LexerError;
    use crate::parser::lexer::Token;
    use lalrpop_util::ErrorRecovery;
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
        Variant1(u32),
        Variant2(&'input str),
        Variant3(__lalrpop_util::ErrorRecovery<usize, Token<'input>, LexerError>),
        Variant4(core::option::Option<Token<'input>>),
        Variant5(Vec<ValueAccessor<'input>>),
        Variant6(ValueAccessor<'input>),
        Variant7(alloc::vec::Vec<ValueAccessor<'input>>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 5, 6, 0, 0, 0, 7,
        // State 1
        0, 5, 6, 0, 0, 0, 7,
        // State 2
        0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, -16, -16, 0, 0, 0, -16,
        // State 4
        0, 0, 9, 0, 0, 10, 0,
        // State 5
        0, 0, 0, 0, 11, 0, 0,
        // State 6
        0, -13, -13, 0, 0, 0, -13,
        // State 7
        0, -17, -17, 0, 0, 0, -17,
        // State 8
        0, 0, 0, 0, 12, 0, 0,
        // State 9
        13, -12, -12, 0, 0, 0, -12,
        // State 10
        0, 0, 0, 14, 0, 0, 0,
        // State 11
        0, 0, 0, 15, 0, 0, 0,
        // State 12
        0, -11, -11, 0, 0, 0, -11,
        // State 13
        16, -10, -10, 0, 0, 0, -10,
        // State 14
        17, -9, -9, 0, 0, 0, -9,
        // State 15
        0, -8, -8, 0, 0, 0, -8,
        // State 16
        0, -7, -7, 0, 0, 0, -7,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 7 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        -5,
        // State 1
        -6,
        // State 2
        -18,
        // State 3
        -16,
        // State 4
        0,
        // State 5
        0,
        // State 6
        -13,
        // State 7
        -17,
        // State 8
        0,
        // State 9
        -12,
        // State 10
        0,
        // State 11
        0,
        // State 12
        -11,
        // State 13
        -10,
        // State 14
        -9,
        // State 15
        -8,
        // State 16
        -7,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 2,
            3 => match state {
                1 => 7,
                _ => 3,
            },
            5 => 1,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###"".""###,
            r###""[""###,
            r###""]""###,
            r###"array_idx"###,
            r###"field_name"###,
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
    pub(crate) struct __StateMachine<'err, 'input>
    where 'input: 'err
    {
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __phantom: core::marker::PhantomData<(&'err (), &'input ())>,
    }
    impl<'err, 'input> __state_machine::ParserDefinition for __StateMachine<'err, 'input>
    where 'input: 'err
    {
        type Location = usize;
        type Error = LexerError;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Vec<ValueAccessor<'input>>;
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
            __token_to_integer(token, core::marker::PhantomData::<(&(), &())>)
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
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&(), &())>)
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
            __Symbol::Variant3(recovery)
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
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&(), &())>)
        }
    }
    fn __token_to_integer<
        'err,
        'input,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'err (), &'input ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token::FlatteningSign if true => Some(0),
            Token::Selector if true => Some(1),
            Token::OpenSquareBracket if true => Some(2),
            Token::CloseSquareBracket if true => Some(3),
            Token::ArrayIdx(_) if true => Some(4),
            Token::FieldName(_) if true => Some(5),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'err,
        'input,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'err (), &'input ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 => __Symbol::Variant0(__token),
            4 => match __token {
                Token::ArrayIdx(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            5 => match __token {
                Token::FieldName(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'err,
        'input,
    >(
        __reduce_index: i8,
        _: core::marker::PhantomData<(&'err (), &'input ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'err, 'input>>
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
                    nonterminal_produced: 0,
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
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 2,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 3,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 3,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 3,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 3,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 3,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 4,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            17 => __state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct LambdaParser {
        _priv: (),
    }

    impl LambdaParser {
        pub fn new() -> LambdaParser {
            LambdaParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'err,
            'input,
            __TOKEN: __ToTriple<'err, 'input, >,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            input: &'input str,
            errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
            __tokens0: __TOKENS,
        ) -> Result<Vec<ValueAccessor<'input>>, __lalrpop_util::ParseError<usize, Token<'input>, LexerError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    errors,
                    __phantom: core::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __error_state: i8,
        __states: & [i8],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'err (), &'input ())>,
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
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&(), &())>) {
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
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input ())>,
    ) -> Option<Result<Vec<ValueAccessor<'input>>,__lalrpop_util::ParseError<usize, Token<'input>, LexerError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                __reduce1(input, errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            2 => {
                __reduce2(input, errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(input, errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(input, errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(input, errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(input, errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(input, errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(input, errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                __reduce9(input, errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            10 => {
                __reduce10(input, errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            11 => {
                __reduce11(input, errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(input, errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(input, errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(input, errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(input, errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(input, errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                // __Lambda = Lambda => ActionFn(0);
                let __sym0 = __pop_Variant5(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, errors, __sym0);
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
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ValueAccessor<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ValueAccessor<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, LexerError>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<ValueAccessor<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<Token<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u32, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // "!"? = "!" => ActionFn(5);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // "!"? =  => ActionFn(6);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action6::<>(input, errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce2<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // "."? = "." => ActionFn(7);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // "."? =  => ActionFn(8);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action8::<>(input, errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce4<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Lambda =  => ActionFn(21);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action21::<>(input, errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 2)
    }
    pub(crate) fn __reduce5<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Lambda = ValueAccessor+ => ActionFn(22);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce6<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // ValueAccessor = ".", "[", array_idx, "]", "!" => ActionFn(17);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action17::<>(input, errors, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (5, 3)
    }
    pub(crate) fn __reduce7<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // ValueAccessor = "[", array_idx, "]", "!" => ActionFn(18);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action18::<>(input, errors, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (4, 3)
    }
    pub(crate) fn __reduce8<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // ValueAccessor = ".", "[", array_idx, "]" => ActionFn(19);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action19::<>(input, errors, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (4, 3)
    }
    pub(crate) fn __reduce9<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // ValueAccessor = "[", array_idx, "]" => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action20::<>(input, errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce10<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // ValueAccessor = ".", field_name, "!" => ActionFn(15);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action15::<>(input, errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce11<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // ValueAccessor = ".", field_name => ActionFn(16);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action16::<>(input, errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce12<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // ValueAccessor = error => ActionFn(4);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 3)
    }
    pub(crate) fn __reduce13<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // ValueAccessor* =  => ActionFn(9);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action9::<>(input, errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce14<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // ValueAccessor* = ValueAccessor+ => ActionFn(10);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce15<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // ValueAccessor+ = ValueAccessor => ActionFn(11);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce16<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // ValueAccessor+ = ValueAccessor+, ValueAccessor => ActionFn(12);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action12::<>(input, errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 5)
    }
}
pub use self::__parse__Lambda::LambdaParser;

#[allow(unused_variables)]
fn __action0<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, __0, _): (usize, Vec<ValueAccessor<'input>>, usize),
) -> Vec<ValueAccessor<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action1<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, __0, _): (usize, alloc::vec::Vec<ValueAccessor<'input>>, usize),
) -> Vec<ValueAccessor<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action2<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, maybe_dot_selector, _): (usize, core::option::Option<Token<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, idx, _): (usize, u32, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, maybe_flatten_sign, _): (usize, core::option::Option<Token<'input>>, usize),
) -> ValueAccessor<'input>
{
    {
        ValueAccessor::ArrayAccess { idx }
    }
}

#[allow(unused_variables)]
fn __action3<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, field_name, _): (usize, &'input str, usize),
    (_, maybe_flatten_sign, _): (usize, core::option::Option<Token<'input>>, usize),
) -> ValueAccessor<'input>
{
    {
        ValueAccessor::FieldAccess { field_name }
    }
}

#[allow(unused_variables)]
fn __action4<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, __0, _): (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, LexerError>, usize),
) -> ValueAccessor<'input>
{
    { errors.push(__0); ValueAccessor::Error }
}

#[allow(unused_variables)]
fn __action5<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> core::option::Option<Token<'input>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action6<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Token<'input>>
{
    None
}

#[allow(unused_variables)]
fn __action7<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> core::option::Option<Token<'input>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action8<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Token<'input>>
{
    None
}

#[allow(unused_variables)]
fn __action9<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<ValueAccessor<'input>>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action10<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, v, _): (usize, alloc::vec::Vec<ValueAccessor<'input>>, usize),
) -> alloc::vec::Vec<ValueAccessor<'input>>
{
    v
}

#[allow(unused_variables)]
fn __action11<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, __0, _): (usize, ValueAccessor<'input>, usize),
) -> alloc::vec::Vec<ValueAccessor<'input>>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action12<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, v, _): (usize, alloc::vec::Vec<ValueAccessor<'input>>, usize),
    (_, e, _): (usize, ValueAccessor<'input>, usize),
) -> alloc::vec::Vec<ValueAccessor<'input>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action13<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    __0: (usize, core::option::Option<Token<'input>>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, u32, usize),
    __3: (usize, Token<'input>, usize),
    __4: (usize, Token<'input>, usize),
) -> ValueAccessor<'input>
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action5(
        input,
        errors,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        errors,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action14<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    __0: (usize, core::option::Option<Token<'input>>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, u32, usize),
    __3: (usize, Token<'input>, usize),
) -> ValueAccessor<'input>
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action6(
        input,
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        errors,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action15<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Token<'input>, usize),
) -> ValueAccessor<'input>
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action5(
        input,
        errors,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        input,
        errors,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action16<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, &'input str, usize),
) -> ValueAccessor<'input>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action6(
        input,
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        input,
        errors,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action17<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, u32, usize),
    __3: (usize, Token<'input>, usize),
    __4: (usize, Token<'input>, usize),
) -> ValueAccessor<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action7(
        input,
        errors,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        input,
        errors,
        __temp0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
fn __action18<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, u32, usize),
    __2: (usize, Token<'input>, usize),
    __3: (usize, Token<'input>, usize),
) -> ValueAccessor<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action8(
        input,
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        input,
        errors,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action19<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, u32, usize),
    __3: (usize, Token<'input>, usize),
) -> ValueAccessor<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action7(
        input,
        errors,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action14(
        input,
        errors,
        __temp0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action20<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, u32, usize),
    __2: (usize, Token<'input>, usize),
) -> ValueAccessor<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action8(
        input,
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action14(
        input,
        errors,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action21<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<ValueAccessor<'input>>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action9(
        input,
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        errors,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action22<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    __0: (usize, alloc::vec::Vec<ValueAccessor<'input>>, usize),
) -> Vec<ValueAccessor<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action10(
        input,
        errors,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        errors,
        __temp0,
    )
}

pub trait __ToTriple<'err, 'input, > {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, LexerError>>;
}

impl<'err, 'input, > __ToTriple<'err, 'input, > for (usize, Token<'input>, usize) {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, LexerError>> {
        Ok(value)
    }
}
impl<'err, 'input, > __ToTriple<'err, 'input, > for Result<(usize, Token<'input>, usize), LexerError> {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, LexerError>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
