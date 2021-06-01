#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use lexgen::lexer;
use criterion::{criterion_group, criterion_main, Criterion};
mod logos {
    use lexgen::lexer;
    use criterion::{black_box, Criterion};
    pub enum Token {
        InvalidToken,
        Identifier,
        String,
        Private,
        Primitive,
        Protected,
        In,
        Instanceof,
        Accessor,
        Ellipsis,
        ParenOpen,
        ParenClose,
        BraceOpen,
        BraceClose,
        OpAddition,
        OpIncrement,
        OpAssign,
        OpEquality,
        OpStrictEquality,
        FatArrow,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Token {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Token::InvalidToken,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "InvalidToken");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::Identifier,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Identifier");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::String,) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "String");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::Private,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Private");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::Primitive,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Primitive");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::Protected,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Protected");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::In,) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "In");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::Instanceof,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Instanceof");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::Accessor,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Accessor");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::Ellipsis,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Ellipsis");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::ParenOpen,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "ParenOpen");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::ParenClose,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "ParenClose");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::BraceOpen,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "BraceOpen");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::BraceClose,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "BraceClose");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::OpAddition,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "OpAddition");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::OpIncrement,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "OpIncrement");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::OpAssign,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "OpAssign");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::OpEquality,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "OpEquality");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::OpStrictEquality,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "OpStrictEquality");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::FatArrow,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "FatArrow");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Token {
        #[inline]
        fn clone(&self) -> Token {
            {
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Token {}
    impl ::core::marker::StructuralPartialEq for Token {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Token {
        #[inline]
        fn eq(&self, other: &Token) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    static SOURCE: &str = "
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    foobar(protected primitive private instanceof in) { + ++ = == === => }
    ";
    static STRINGS: &str = r#""tree" "to" "a" "graph" "that can" "more adequately represent" "loops and arbitrary state jumps" "with\"\"\"out" "the\n\n\n\n\n" "expl\"\"\"osive" "nature\"""of trying to build up all possible permutations in a tree." "tree" "to" "a" "graph" "that can" "more adequately represent" "loops and arbitrary state jumps" "with\"\"\"out" "the\n\n\n\n\n" "expl\"\"\"osive" "nature\"""of trying to build up all possible permutations in a tree." "tree" "to" "a" "graph" "that can" "more adequately represent" "loops and arbitrary state jumps" "with\"\"\"out" "the\n\n\n\n\n" "expl\"\"\"osive" "nature\"""of trying to build up all possible permutations in a tree." "tree" "to" "a" "graph" "that can" "more adequately represent" "loops and arbitrary state jumps" "with\"\"\"out" "the\n\n\n\n\n" "expl\"\"\"osive" "nature\"""of trying to build up all possible permutations in a tree.""#;
    static IDENTIFIERS: &str = "It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton";
    enum LexerAction<T> {
        Continue,
        Return(T),
        Switch(LexerRule),
        SwitchAndReturn(T, LexerRule),
    }
    enum LexerRule {
        String,
        StringEscape,
        Init,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for LexerRule {
        #[inline]
        fn clone(&self) -> LexerRule {
            {
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for LexerRule {}
    struct LexerHandle<'lexer, 'input> {
        iter: &'lexer mut std::iter::Peekable<std::str::CharIndices<'input>>,
        match_: &'input str,
        user_state: &'lexer mut (),
    }
    struct Lexer<'input> {
        state: usize,
        initial_state: usize,
        user_state: (),
        input: &'input str,
        iter: std::iter::Peekable<std::str::CharIndices<'input>>,
        current_match_start: usize,
        current_match_end: usize,
    }
    struct LexerError {
        char_idx: usize,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for LexerError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                LexerError {
                    char_idx: ref __self_0_0,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "LexerError");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "char_idx",
                        &&(*__self_0_0),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for LexerError {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for LexerError {
        #[inline]
        fn eq(&self, other: &LexerError) -> bool {
            match *other {
                LexerError {
                    char_idx: ref __self_1_0,
                } => match *self {
                    LexerError {
                        char_idx: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &LexerError) -> bool {
            match *other {
                LexerError {
                    char_idx: ref __self_1_0,
                } => match *self {
                    LexerError {
                        char_idx: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl ::core::marker::StructuralEq for LexerError {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for LexerError {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<usize>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for LexerError {
        #[inline]
        fn clone(&self) -> LexerError {
            {
                let _: ::core::clone::AssertParamIsClone<usize>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for LexerError {}
    impl<'lexer, 'input> LexerHandle<'lexer, 'input> {
        fn switch_and_return<T>(self, rule: LexerRule, token: T) -> LexerAction<T> {
            LexerAction::SwitchAndReturn(token, rule)
        }
        fn return_<T>(self, token: T) -> LexerAction<T> {
            LexerAction::Return(token)
        }
        fn switch<T>(self, rule: LexerRule) -> LexerAction<T> {
            LexerAction::Switch(rule)
        }
        fn continue_<T>(self) -> LexerAction<T> {
            LexerAction::Continue
        }
        fn state(&mut self) -> &mut () {
            self.user_state
        }
        fn match_(&self) -> &'input str {
            self.match_
        }
        fn peek(&mut self) -> Option<char> {
            self.iter.peek().map(|(_, char)| *char)
        }
    }
    impl<'input> Lexer<'input> {
        fn new(input: &'input str) -> Self {
            Self::new_with_state(input, Default::default())
        }
        fn new_with_state(input: &'input str, user_state: ()) -> Self {
            Lexer {
                state: 0,
                initial_state: 0,
                user_state: Default::default(),
                input,
                iter: input.char_indices().peekable(),
                current_match_start: 0,
                current_match_end: 0,
            }
        }
        fn switch(&mut self, rule: LexerRule) {
            match rule {
                LexerRule::String => self.state = 56usize,
                LexerRule::StringEscape => self.state = 60usize,
                LexerRule::Init => self.state = 0usize,
            }
            self.initial_state = self.state;
        }
    }
    impl<'input> Iterator for Lexer<'input> {
        type Item = Result<(usize, Token, usize), LexerError>;
        fn next(&mut self) -> Option<Self::Item> {
            loop {
                match self.state {
                    0usize => match self.iter.peek().copied() {
                        None => return None,
                        Some((char_idx, char)) => {
                            self.current_match_start = char_idx;
                            self.current_match_end = char_idx;
                            match char {
                                '\u{29}' => {
                                    self.current_match_end += char.len_utf8();
                                    let _ = self.iter.next();
                                    self.state = 13usize;
                                }
                                '\u{61}' => {
                                    self.current_match_end += char.len_utf8();
                                    let _ = self.iter.next();
                                    self.state = 10usize;
                                }
                                '\u{22}' => {
                                    self.current_match_end += char.len_utf8();
                                    let _ = self.iter.next();
                                    self.state = 7usize;
                                }
                                '\u{2e}' => {
                                    self.current_match_end += char.len_utf8();
                                    let _ = self.iter.next();
                                    self.state = 4usize;
                                }
                                '\u{20}' | '\u{a}' | '\u{c}' | '\u{9}' => {
                                    self.current_match_end += char.len_utf8();
                                    let _ = self.iter.next();
                                    self.state = 1usize;
                                }
                                '\u{7b}' => {
                                    self.current_match_end += char.len_utf8();
                                    let _ = self.iter.next();
                                    self.state = 11usize;
                                }
                                '\u{5f}' | '\u{24}' => {
                                    self.current_match_end += char.len_utf8();
                                    let _ = self.iter.next();
                                    self.state = 8usize;
                                }
                                '\u{2b}' => {
                                    self.current_match_end += char.len_utf8();
                                    let _ = self.iter.next();
                                    self.state = 5usize;
                                }
                                '\u{7d}' => {
                                    self.current_match_end += char.len_utf8();
                                    let _ = self.iter.next();
                                    self.state = 2usize;
                                }
                                '\u{69}' => {
                                    self.current_match_end += char.len_utf8();
                                    let _ = self.iter.next();
                                    self.state = 12usize;
                                }
                                '\u{70}' => {
                                    self.current_match_end += char.len_utf8();
                                    let _ = self.iter.next();
                                    self.state = 9usize;
                                }
                                '\u{28}' => {
                                    self.current_match_end += char.len_utf8();
                                    let _ = self.iter.next();
                                    self.state = 6usize;
                                }
                                '\u{3d}' => {
                                    self.current_match_end += char.len_utf8();
                                    let _ = self.iter.next();
                                    self.state = 3usize;
                                }
                                x if (x >= '\u{41}' && x <= '\u{5a}')
                                    || (x >= '\u{61}' && x <= '\u{7a}') =>
                                {
                                    self.current_match_end += x.len_utf8();
                                    let _ = self.iter.next();
                                    self.state = 8usize;
                                }
                                _ => {
                                    return Some(Err(LexerError {
                                        char_idx: self.current_match_start,
                                    }));
                                }
                            }
                        }
                    },
                    1usize => {
                        self.state = self.initial_state;
                        continue;
                    }
                    2usize => {
                        let rhs: Token = Token::BraceClose;
                        self.state = self.initial_state;
                        return Some(Ok((self.current_match_start, rhs, self.current_match_end)));
                    }
                    3usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::OpAssign;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{3d}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 53usize;
                            }
                            '\u{3e}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 54usize;
                            }
                            _ => {
                                let rhs: Token = Token::OpAssign;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    4usize => match self.iter.peek().copied() {
                        None => {
                            return Some(Err(LexerError {
                                char_idx: self.current_match_start,
                            }));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{2e}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 51usize;
                            }
                            _ => {
                                return Some(Err(LexerError {
                                    char_idx: self.current_match_start,
                                }));
                            }
                        },
                    },
                    5usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::OpAddition;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{2b}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 50usize;
                            }
                            _ => {
                                let rhs: Token = Token::OpAddition;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    6usize => {
                        let rhs: Token = Token::ParenOpen;
                        self.state = self.initial_state;
                        return Some(Ok((self.current_match_start, rhs, self.current_match_end)));
                    }
                    7usize => {
                        let rhs: fn(LexerHandle<'_, 'input>) -> LexerAction<Token> =
                            |lexer| lexer.switch(LexerRule::String);
                        let str = &self.input[self.current_match_start..self.current_match_end];
                        let handle = LexerHandle {
                            iter: &mut self.iter,
                            match_: str,
                            user_state: &mut self.user_state,
                        };
                        match rhs(handle) {
                            LexerAction::Continue => {
                                self.state = self.initial_state;
                                continue;
                            }
                            LexerAction::Return(tok) => {
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    tok,
                                    self.current_match_end,
                                )));
                            }
                            LexerAction::Switch(rule_set) => {
                                self.switch(rule_set);
                                continue;
                            }
                            LexerAction::SwitchAndReturn(tok, rule_set) => {
                                self.switch(rule_set);
                                return Some(Ok((
                                    self.current_match_start,
                                    tok,
                                    self.current_match_end,
                                )));
                            }
                        }
                    }
                    8usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    9usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            '\u{72}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 31usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    10usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{63}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 24usize;
                            }
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    11usize => {
                        let rhs: Token = Token::BraceOpen;
                        self.state = self.initial_state;
                        return Some(Ok((self.current_match_start, rhs, self.current_match_end)));
                    }
                    12usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            '\u{6e}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 15usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    13usize => {
                        let rhs: Token = Token::ParenClose;
                        self.state = self.initial_state;
                        return Some(Ok((self.current_match_start, rhs, self.current_match_end)));
                    }
                    14usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    15usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{73}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 16usize;
                            }
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    16usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{74}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 17usize;
                            }
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    17usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            '\u{61}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 18usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    18usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            '\u{6e}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 19usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    19usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{63}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 20usize;
                            }
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    20usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{65}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 21usize;
                            }
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    21usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            '\u{6f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 22usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    22usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            '\u{66}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 23usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    23usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    24usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{63}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 25usize;
                            }
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    25usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            '\u{65}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 26usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    26usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            '\u{73}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 27usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    27usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{73}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 28usize;
                            }
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    28usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{6f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 29usize;
                            }
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    29usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            '\u{72}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 30usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    30usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    31usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{69}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 32usize;
                            }
                            '\u{6f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 33usize;
                            }
                            '\u{5f}' | '\u{24}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    32usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{6d}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 40usize;
                            }
                            '\u{76}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 41usize;
                            }
                            '\u{5f}' | '\u{24}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    33usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            '\u{74}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 34usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    34usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            '\u{65}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 35usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    35usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{63}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 36usize;
                            }
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    36usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{74}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 37usize;
                            }
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    37usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            '\u{65}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 38usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    38usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            '\u{64}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 39usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    39usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    40usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{69}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 45usize;
                            }
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    41usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            '\u{61}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 42usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    42usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            '\u{74}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 43usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    43usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{65}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 44usize;
                            }
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    44usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    45usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            '\u{74}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 46usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    46usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            '\u{69}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 47usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    47usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{76}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 48usize;
                            }
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    48usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{65}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 49usize;
                            }
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    49usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::Identifier;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{24}' | '\u{5f}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            x if (x >= '\u{30}' && x <= '\u{39}')
                                || (x >= '\u{41}' && x <= '\u{5a}')
                                || (x >= '\u{61}' && x <= '\u{7a}') =>
                            {
                                self.current_match_end += x.len_utf8();
                                let _ = self.iter.next();
                                self.state = 14usize;
                            }
                            _ => {
                                let rhs: Token = Token::Identifier;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    50usize => {
                        let rhs: Token = Token::OpIncrement;
                        self.state = self.initial_state;
                        return Some(Ok((self.current_match_start, rhs, self.current_match_end)));
                    }
                    51usize => match self.iter.peek().copied() {
                        None => {
                            return Some(Err(LexerError {
                                char_idx: self.current_match_start,
                            }));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{2e}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 52usize;
                            }
                            _ => {
                                return Some(Err(LexerError {
                                    char_idx: self.current_match_start,
                                }));
                            }
                        },
                    },
                    52usize => {
                        let rhs: Token = Token::Ellipsis;
                        self.state = self.initial_state;
                        return Some(Ok((self.current_match_start, rhs, self.current_match_end)));
                    }
                    53usize => match self.iter.peek().copied() {
                        None => {
                            let rhs: Token = Token::OpEquality;
                            self.state = self.initial_state;
                            return Some(Ok((
                                self.current_match_start,
                                rhs,
                                self.current_match_end,
                            )));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{3d}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 55usize;
                            }
                            _ => {
                                let rhs: Token = Token::OpEquality;
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    rhs,
                                    self.current_match_end,
                                )));
                            }
                        },
                    },
                    54usize => {
                        let rhs: Token = Token::FatArrow;
                        self.state = self.initial_state;
                        return Some(Ok((self.current_match_start, rhs, self.current_match_end)));
                    }
                    55usize => {
                        let rhs: Token = Token::OpStrictEquality;
                        self.state = self.initial_state;
                        return Some(Ok((self.current_match_start, rhs, self.current_match_end)));
                    }
                    56usize => match self.iter.peek().copied() {
                        None => {
                            return Some(Err(LexerError {
                                char_idx: self.current_match_start,
                            }));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{5c}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 58usize;
                            }
                            '\u{22}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 59usize;
                            }
                            _ => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 57usize;
                            }
                        },
                    },
                    57usize => {
                        self.state = self.initial_state;
                        continue;
                    }
                    58usize => {
                        let rhs: fn(LexerHandle<'_, 'input>) -> LexerAction<Token> =
                            |lexer| lexer.switch(LexerRule::StringEscape);
                        let str = &self.input[self.current_match_start..self.current_match_end];
                        let handle = LexerHandle {
                            iter: &mut self.iter,
                            match_: str,
                            user_state: &mut self.user_state,
                        };
                        match rhs(handle) {
                            LexerAction::Continue => {
                                self.state = self.initial_state;
                                continue;
                            }
                            LexerAction::Return(tok) => {
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    tok,
                                    self.current_match_end,
                                )));
                            }
                            LexerAction::Switch(rule_set) => {
                                self.switch(rule_set);
                                continue;
                            }
                            LexerAction::SwitchAndReturn(tok, rule_set) => {
                                self.switch(rule_set);
                                return Some(Ok((
                                    self.current_match_start,
                                    tok,
                                    self.current_match_end,
                                )));
                            }
                        }
                    }
                    59usize => {
                        let rhs: fn(LexerHandle<'_, 'input>) -> LexerAction<Token> =
                            |lexer| lexer.switch_and_return(LexerRule::Init, Token::String);
                        let str = &self.input[self.current_match_start..self.current_match_end];
                        let handle = LexerHandle {
                            iter: &mut self.iter,
                            match_: str,
                            user_state: &mut self.user_state,
                        };
                        match rhs(handle) {
                            LexerAction::Continue => {
                                self.state = self.initial_state;
                                continue;
                            }
                            LexerAction::Return(tok) => {
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    tok,
                                    self.current_match_end,
                                )));
                            }
                            LexerAction::Switch(rule_set) => {
                                self.switch(rule_set);
                                continue;
                            }
                            LexerAction::SwitchAndReturn(tok, rule_set) => {
                                self.switch(rule_set);
                                return Some(Ok((
                                    self.current_match_start,
                                    tok,
                                    self.current_match_end,
                                )));
                            }
                        }
                    }
                    60usize => match self.iter.peek().copied() {
                        None => {
                            return Some(Err(LexerError {
                                char_idx: self.current_match_start,
                            }));
                        }
                        Some((char_idx, char)) => match char {
                            '\u{75}' | '\u{22}' | '\u{74}' | '\u{6e}' => {
                                self.current_match_end += char.len_utf8();
                                let _ = self.iter.next();
                                self.state = 61usize;
                            }
                            _ => {
                                return Some(Err(LexerError {
                                    char_idx: self.current_match_start,
                                }));
                            }
                        },
                    },
                    _ => {
                        let rhs: fn(LexerHandle<'_, 'input>) -> LexerAction<Token> =
                            |lexer| lexer.switch(LexerRule::String);
                        let str = &self.input[self.current_match_start..self.current_match_end];
                        let handle = LexerHandle {
                            iter: &mut self.iter,
                            match_: str,
                            user_state: &mut self.user_state,
                        };
                        match rhs(handle) {
                            LexerAction::Continue => {
                                self.state = self.initial_state;
                                continue;
                            }
                            LexerAction::Return(tok) => {
                                self.state = self.initial_state;
                                return Some(Ok((
                                    self.current_match_start,
                                    tok,
                                    self.current_match_end,
                                )));
                            }
                            LexerAction::Switch(rule_set) => {
                                self.switch(rule_set);
                                continue;
                            }
                            LexerAction::SwitchAndReturn(tok, rule_set) => {
                                self.switch(rule_set);
                                return Some(Ok((
                                    self.current_match_start,
                                    tok,
                                    self.current_match_end,
                                )));
                            }
                        }
                    }
                }
            }
        }
    }
    fn identifiers(s: &str) {
        let mut lex = Lexer::new(s);
        while let Some(token) = lex.next() {
            token;
        }
    }
    pub fn identifiers_bench(c: &mut Criterion) {
        c.bench_function("identifiers", |b| {
            b.iter(|| identifiers(black_box(IDENTIFIERS)))
        });
    }
}
pub fn benches() {
    let mut criterion: ::criterion::Criterion<_> =
        ::criterion::Criterion::default().configure_from_args();
    logos::identifiers_bench(&mut criterion);
}
fn main() {
    ::criterion::__warn_about_html_reports_feature();
    ::criterion::__warn_about_cargo_bench_support_feature();
    benches();
    ::criterion::Criterion::default()
        .configure_from_args()
        .final_summary();
}
