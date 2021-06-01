use lexgen::lexer;

use criterion::{criterion_group, criterion_main, Criterion};

mod logos {
    use lexgen::lexer;

    use criterion::{black_box, Criterion};

    #[derive(Debug, Clone, Copy, PartialEq)]
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

    lexer! {
        Lexer -> Token;

        let id_start = ['a'-'z' 'A'-'Z' '_' '$'];
        let id_continue = $id_start | ['0'-'9'];

        rule Init {
            [' ' '\n' '\t' '\u{c}'],

            $id_start $id_continue* = Token::Identifier,

            '"' => |lexer| lexer.switch(LexerRule::String),

            "private" = Token::Private,
            "primitive" = Token::Primitive,
            "protected" = Token::Protected,
            "in" = Token::In,
            "instanceof" = Token::Instanceof,
            "accessor" = Token::Accessor,
            "..." = Token::Ellipsis,
            "(" = Token::ParenOpen,
            ")" = Token::ParenClose,
            "{" = Token::BraceOpen,
            "}" = Token::BraceClose,
            "+" = Token::OpAddition,
            "++" = Token::OpIncrement,
            "=" = Token::OpAssign,
            "==" = Token::OpEquality,
            "===" = Token::OpStrictEquality,
            "=>" = Token::FatArrow,
        }

        rule String {
            '"' => |lexer| lexer.switch_and_return(LexerRule::Init, Token::String),
            '\\' => |lexer| lexer.switch(LexerRule::StringEscape),
            _,
        }

        rule StringEscape {
            ['t' 'u' 'n' '"'] => |lexer| lexer.switch(LexerRule::String),
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

criterion_group!(benches, logos::identifiers_bench);
criterion_main!(benches);
