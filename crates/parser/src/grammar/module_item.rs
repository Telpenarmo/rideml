use super::{block, expr, params, type_expr};
use crate::{parser::Parser, token_set::TokenSet, SyntaxKind};

pub(crate) const MODULE_ITEM_START: TokenSet =
    TokenSet::new(&[SyntaxKind::DEF_KW, SyntaxKind::TYPE_KW, SyntaxKind::OPEN_KW]);
pub(crate) const MODULE_ITEM_END: TokenSet =
    TokenSet::new(&[SyntaxKind::SEMICOLON, SyntaxKind::R_BRACE]);

pub(crate) fn module_item(parser: &mut Parser) {
    if parser.at(SyntaxKind::DEF_KW) {
        def(parser);
    } else if parser.at(SyntaxKind::TYPE_KW) {
        type_definition(parser);
    } else if parser.at(SyntaxKind::OPEN_KW) {
        open(parser);
    } else {
        parser.eat_error_until(
            MODULE_ITEM_START.union(MODULE_ITEM_END),
            "Expected declaration".into(),
        );
        if parser.at_any(MODULE_ITEM_END) {
            assert!(parser.eat_any(MODULE_ITEM_END));
        }
    }
}

fn open(parser: &mut Parser) {
    assert!(parser.at(SyntaxKind::OPEN_KW));

    let mark = parser.open();
    parser.expect(SyntaxKind::OPEN_KW);
    parser.expect(SyntaxKind::IDENT);
    parser.expect(SyntaxKind::SEMICOLON);

    parser.close(mark, SyntaxKind::OPEN);
}

fn type_definition(parser: &mut Parser) {
    assert!(parser.at(SyntaxKind::TYPE_KW));

    let mark = parser.open();
    parser.expect(SyntaxKind::TYPE_KW);
    parser.expect(SyntaxKind::IDENT);
    parser.expect(SyntaxKind::EQUAL);
    type_expr::type_expr(parser);
    parser.expect(SyntaxKind::SEMICOLON);

    parser.close(mark, SyntaxKind::TYPE_DEFINITION);
}

fn def(parser: &mut Parser) {
    assert!(parser.at(SyntaxKind::DEF_KW));

    let mark = parser.open();
    parser.expect(SyntaxKind::DEF_KW);
    parser.expect(SyntaxKind::IDENT);
    params::params(parser);
    type_expr::type_annotation(parser);

    let body = parser.open();
    if parser.eat(SyntaxKind::EQUAL) {
        expr::expr(parser);
        parser.expect(SyntaxKind::SEMICOLON);
    } else if parser.at(SyntaxKind::L_BRACE) {
        block::block(parser);
    }
    parser.close(body, SyntaxKind::DEF_BODY);

    parser.close(mark, SyntaxKind::DEFINITION);
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use crate::{check, check_err, PrefixEntryPoint};

    #[test]
    fn parse_definition() {
        check(
            PrefixEntryPoint::Module,
            "def f = x;",
            &expect![[r#"
            MODULE@0..10
              DEFINITION@0..10
                DEF_KW@0..3 "def"
                WHITESPACE@3..4 " "
                IDENT@4..5 "f"
                WHITESPACE@5..6 " "
                PARAMS@6..6
                DEF_BODY@6..10
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
                  DEFINITION@0..7
                    DEF_KW@0..3 "def"
                    WHITESPACE@3..4 " "
                    IDENT@4..5 "f"
                    WHITESPACE@5..6 " "
                    PARAMS@6..6
                    DEF_BODY@6..7
                      EQUAL@6..7 "="
                      ERROR@7..7
                      ERROR@7..7
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
                  DEFINITION@0..3
                    DEF_KW@0..3 "def"
                    ERROR@3..3
                    PARAMS@3..3
                    DEF_BODY@3..3
            "#]],
            &["Expected IDENT but found EOF"],
        );
    }

    #[test]
    fn parse_def_with_param_type_annotation() {
        check(
            PrefixEntryPoint::Module,
            "def f(x: Int) = x;",
            &expect![[r#"
                MODULE@0..18
                  DEFINITION@0..18
                    DEF_KW@0..3 "def"
                    WHITESPACE@3..4 " "
                    IDENT@4..5 "f"
                    PARAMS@5..14
                      PARAM@5..14
                        L_PAREN@5..6 "("
                        IDENT@6..7 "x"
                        TYPE_ANNOTATION@7..12
                          COLON@7..8 ":"
                          WHITESPACE@8..9 " "
                          TYPE_IDENT@9..12
                            IDENT@9..12 "Int"
                        R_PAREN@12..13 ")"
                        WHITESPACE@13..14 " "
                    DEF_BODY@14..18
                      EQUAL@14..15 "="
                      WHITESPACE@15..16 " "
                      IDENT_EXPR@16..17
                        IDENT@16..17 "x"
                      SEMICOLON@17..18 ";"
            "#]],
        );
    }

    #[test]
    fn parse_def_with_return_type_annotation() {
        check(
            PrefixEntryPoint::Module,
            "def f: Int = 42;",
            &expect![[r#"
                MODULE@0..16
                  DEFINITION@0..16
                    DEF_KW@0..3 "def"
                    WHITESPACE@3..4 " "
                    IDENT@4..5 "f"
                    PARAMS@5..5
                    TYPE_ANNOTATION@5..11
                      COLON@5..6 ":"
                      WHITESPACE@6..7 " "
                      TYPE_IDENT@7..11
                        IDENT@7..10 "Int"
                        WHITESPACE@10..11 " "
                    DEF_BODY@11..16
                      EQUAL@11..12 "="
                      WHITESPACE@12..13 " "
                      LITERAL_EXPR@13..15
                        INT@13..15 "42"
                      SEMICOLON@15..16 ";"
            "#]],
        );
    }

    #[test]
    fn parse_annotated_def_lambda() {
        check(
            PrefixEntryPoint::Module,
            "def f : int -> int = \\x -> x;",
            &expect![[r#"
                MODULE@0..29
                  DEFINITION@0..29
                    DEF_KW@0..3 "def"
                    WHITESPACE@3..4 " "
                    IDENT@4..5 "f"
                    WHITESPACE@5..6 " "
                    PARAMS@6..6
                    TYPE_ANNOTATION@6..19
                      COLON@6..7 ":"
                      WHITESPACE@7..8 " "
                      TYPE_ARROW@8..19
                        TYPE_IDENT@8..12
                          IDENT@8..11 "int"
                          WHITESPACE@11..12 " "
                        ARROW@12..14 "->"
                        WHITESPACE@14..15 " "
                        TYPE_IDENT@15..19
                          IDENT@15..18 "int"
                          WHITESPACE@18..19 " "
                    DEF_BODY@19..29
                      EQUAL@19..20 "="
                      WHITESPACE@20..21 " "
                      LAMBDA_EXPR@21..28
                        BACKSLASH@21..22 "\\"
                        PARAMS@22..24
                          PARAM@22..24
                            IDENT@22..23 "x"
                            WHITESPACE@23..24 " "
                        ARROW@24..26 "->"
                        WHITESPACE@26..27 " "
                        IDENT_EXPR@27..28
                          IDENT@27..28 "x"
                      SEMICOLON@28..29 ";"
            "#]],
        );
    }
}
