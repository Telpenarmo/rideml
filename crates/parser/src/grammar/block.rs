use super::{
    decl::DECL_START,
    expr::{self, expr},
    params::params,
};
use crate::{
    parser::{CompletedMarker, Parser},
    token_set::TokenSet,
    SyntaxKind,
};

const BLOCK_END: TokenSet =
    DECL_START.union(TokenSet::new(&[SyntaxKind::R_BRACE, SyntaxKind::EOF]));
const STMT_FIRST: TokenSet = TokenSet::new(&[SyntaxKind::LET_KW]).union(expr::EXPR_FIRST);

pub(crate) fn block(parser: &mut Parser) -> CompletedMarker {
    assert!(parser.at(SyntaxKind::L_BRACE));
    let mark = parser.open();
    parser.expect(SyntaxKind::L_BRACE);

    while !parser.at_any(BLOCK_END) {
        if parser.at(SyntaxKind::LET_KW) {
            let_stmt(parser);
        } else if parser.at_any(expr::EXPR_FIRST) {
            let expr_marker = expr::expr(parser);
            if parser.at_any(BLOCK_END) {
                break;
            }
            let marker = parser.open_before(expr_marker);
            parser.expect(SyntaxKind::SEMICOLON);
            parser.close(marker, SyntaxKind::EXPR_STMT);
        } else {
            parser.eat_error_until(BLOCK_END.union(STMT_FIRST), "Expected statement".into());
        }
    }

    parser.expect(SyntaxKind::R_BRACE);
    parser.close(mark, SyntaxKind::BLOCK_EXPR)
}

fn let_stmt(parser: &mut Parser) -> CompletedMarker {
    assert!(parser.at(SyntaxKind::LET_KW));

    let mark = parser.open();
    parser.expect(SyntaxKind::LET_KW);
    parser.expect(SyntaxKind::IDENT);
    params(parser);
    parser.expect(SyntaxKind::EQUAL);
    expr(parser);
    parser.expect(SyntaxKind::SEMICOLON);
    parser.close(mark, SyntaxKind::LET_STMT)
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, expect_file};

    use crate::{check, check_err, check_file, PrefixEntryPoint};

    #[test]
    fn def_block_directly() {
        check_file(
            "def x { 0 }",
            &expect_file!["../../test_data/def_block_directly.rml_cst"],
        );
    }

    #[test]
    fn empty_block() {
        check(
            PrefixEntryPoint::Expr,
            "{}",
            &expect![[r#"
                BLOCK_EXPR@0..2
                  L_BRACE@0..1 "{"
                  R_BRACE@1..2 "}"
            "#]],
        );
    }

    #[test]
    fn def_block_as_expr() {
        check_file(
            r"def x = { 0 };",
            &expect_file!["../../test_data/def_block_as_expr.rml_cst"],
        );
    }

    #[test]
    fn block_with_expr_stmt_and_trailing_expr() {
        check(
            PrefixEntryPoint::Expr,
            r"{ x; 42 }",
            &expect![[r#"
                BLOCK_EXPR@0..9
                  L_BRACE@0..1 "{"
                  WHITESPACE@1..2 " "
                  EXPR_STMT@2..5
                    IDENT_EXPR@2..3
                      IDENT@2..3 "x"
                    SEMICOLON@3..4 ";"
                    WHITESPACE@4..5 " "
                  LITERAL_EXPR@5..8
                    INT@5..7 "42"
                    WHITESPACE@7..8 " "
                  R_BRACE@8..9 "}"
            "#]],
        );
    }

    #[test]
    fn block_with_let_stmt() {
        check(
            PrefixEntryPoint::Expr,
            r"{ let x = 42; }",
            &expect![[r#"
                BLOCK_EXPR@0..15
                  L_BRACE@0..1 "{"
                  WHITESPACE@1..2 " "
                  LET_STMT@2..14
                    LET_KW@2..5 "let"
                    WHITESPACE@5..6 " "
                    IDENT@6..7 "x"
                    WHITESPACE@7..8 " "
                    PARAMS@8..8
                    EQUAL@8..9 "="
                    WHITESPACE@9..10 " "
                    LITERAL_EXPR@10..12
                      INT@10..12 "42"
                    SEMICOLON@12..13 ";"
                    WHITESPACE@13..14 " "
                  R_BRACE@14..15 "}"
            "#]],
        );
    }

    #[test]
    fn block_with_let_stmt_and_trailing_expr() {
        check_file(
            r"def x { let x = 42; 42 }",
            &expect_file!["../../test_data/block_with_let_stmt_and_trailing_expr.rml_cst"],
        );
    }

    #[test]
    fn block_with_let_stmt_and_expr_stmt() {
        check_file(
            r"def x { let x = 42; x; }",
            &expect_file!["../../test_data/block_with_let_stmt_and_expr_stmt.rml_cst"],
        );
    }

    #[test]
    fn block_with_nested_block() {
        check(
            PrefixEntryPoint::Expr,
            r"{{ 0 }}",
            &expect![[r#"
                BLOCK_EXPR@0..7
                  L_BRACE@0..1 "{"
                  BLOCK_EXPR@1..6
                    L_BRACE@1..2 "{"
                    WHITESPACE@2..3 " "
                    LITERAL_EXPR@3..5
                      INT@3..4 "0"
                      WHITESPACE@4..5 " "
                    R_BRACE@5..6 "}"
                  R_BRACE@6..7 "}"
            "#]],
        );
    }

    #[test]
    fn unclosed_block() {
        check_err(
            PrefixEntryPoint::Expr,
            r"{",
            &expect![[r#"
                BLOCK_EXPR@0..1
                  L_BRACE@0..1 "{"
                  ERROR@1..1
            "#]],
            &["Expected R_BRACE but found EOF"],
        );
    }
}
