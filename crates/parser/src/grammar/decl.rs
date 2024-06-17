use crate::{
    grammar::{expr, params, type_expr},
    parser::Parser,
    SyntaxKind,
};

pub(crate) fn decl(parser: &mut Parser) {
    fn eat_error(parser: &mut Parser) {
        loop {
            if parser.at(SyntaxKind::EOF)
                || parser.at(SyntaxKind::DEF_KW)
                || parser.at(SyntaxKind::TYPE_KW)
                || parser.at(SyntaxKind::OPEN_KW)
            {
                break;
            }
            if parser.at(SyntaxKind::SEMICOLON) {
                parser.advance();
                break;
            }
            parser.unexpected();
        }
    }

    if parser.at(SyntaxKind::DEF_KW) {
        def_decl(parser);
    } else if parser.at(SyntaxKind::TYPE_KW) {
        type_decl(parser);
    } else if parser.at(SyntaxKind::OPEN_KW) {
        open_decl(parser);
    } else {
        eat_error(parser);
    }
}

fn open_decl(parser: &mut Parser) {
    assert!(parser.at(SyntaxKind::OPEN_KW));

    let mark = parser.open();
    parser.expect(SyntaxKind::OPEN_KW);
    parser.expect(SyntaxKind::IDENT);
    parser.expect(SyntaxKind::SEMICOLON);

    parser.close(mark, SyntaxKind::OPEN_DECL);
}

fn type_decl(parser: &mut Parser) {
    assert!(parser.at(SyntaxKind::TYPE_KW));

    let mark = parser.open();
    parser.expect(SyntaxKind::TYPE_KW);
    parser.expect(SyntaxKind::IDENT);
    parser.expect(SyntaxKind::EQUAL);
    type_expr::type_expr(parser);
    parser.expect(SyntaxKind::SEMICOLON);

    parser.close(mark, SyntaxKind::TYPE_DECL);
}

fn def_decl(parser: &mut Parser) {
    assert!(parser.at(SyntaxKind::DEF_KW));

    let mark = parser.open();
    parser.expect(SyntaxKind::DEF_KW);
    parser.expect(SyntaxKind::IDENT);
    params::params(parser);
    parser.expect(SyntaxKind::EQUAL);
    expr::expr(parser);
    parser.expect(SyntaxKind::SEMICOLON);

    parser.close(mark, SyntaxKind::DEF_DECL);
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use crate::{check, check_err, PrefixEntryPoint};

    #[test]
    fn parse_decl() {
        check(
            PrefixEntryPoint::Module,
            "def f = x;",
            &expect![[r#"
            MODULE@0..10
              DEF_DECL@0..10
                DEF_KW@0..3 "def"
                WHITESPACE@3..4 " "
                IDENT@4..5 "f"
                WHITESPACE@5..6 " "
                PARAMS@6..6
                EQUAL@6..7 "="
                WHITESPACE@7..8 " "
                IDENT_EXPR@8..9
                  IDENT@8..9 "x"
                SEMICOLON@9..10 ";"
        "#]],
        );
    }

    #[test]
    fn parse_def_missing_body() {
        check_err(
            PrefixEntryPoint::Module,
            "def f =",
            &expect![[r#"
                MODULE@0..7
                  DEF_DECL@0..7
                    DEF_KW@0..3 "def"
                    WHITESPACE@3..4 " "
                    IDENT@4..5 "f"
                    WHITESPACE@5..6 " "
                    PARAMS@6..6
                    EQUAL@6..7 "="
            "#]],
            &["Expected expression", "Expected SEMICOLON but found EOF"],
        );
    }

    #[test]
    fn parse_def_only() {
        check_err(
            PrefixEntryPoint::Module,
            "def",
            &expect![[r#"
                MODULE@0..3
                  DEF_DECL@0..3
                    DEF_KW@0..3 "def"
                    PARAMS@3..3
            "#]],
            &[
                "Expected IDENT but found EOF",
                "Expected EQUAL but found EOF",
                "Expected expression",
                "Expected SEMICOLON but found EOF",
            ],
        );
    }
}
