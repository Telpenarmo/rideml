use crate::{
    parser::{CompletedMarker, Parser},
    SyntaxKind,
};

pub(crate) fn delimited_type_expr(parser: &mut Parser) -> CompletedMarker {
    if parser.at(SyntaxKind::IDENT) {
        let mark = parser.open();
        parser.advance();
        parser.close(mark, SyntaxKind::TYPE_IDENT)
    } else if parser.at(SyntaxKind::L_PAREN) {
        let mark = parser.open();
        parser.advance();
        type_expr(parser);
        parser.expect(SyntaxKind::R_PAREN);
        parser.close(mark, SyntaxKind::TYPE_PAREN)
    } else {
        parser.error("Expected type expression".into())
    }
}

pub(crate) fn type_expr(parser: &mut Parser) -> CompletedMarker {
    let lhs_mark = delimited_type_expr(parser);

    if parser.eat(SyntaxKind::ARROW) {
        let marker = parser.open_before(lhs_mark);
        type_expr(parser);
        parser.close(marker, SyntaxKind::TYPE_ARROW)
    } else {
        lhs_mark
    }
}

pub(crate) fn type_annotation(parser: &mut Parser) -> Option<CompletedMarker> {
    if parser.at(SyntaxKind::COLON) {
        let marker = parser.open();
        parser.advance();
        type_expr(parser);
        Some(parser.close(marker, SyntaxKind::TYPE_ANNOTATION))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{check, check_err, PrefixEntryPoint};
    use expect_test::expect;

    #[test]
    fn parse_type_ident() {
        check(
            PrefixEntryPoint::TypeExpr,
            "a",
            &expect![[r#"
                TYPE_IDENT@0..1
                  IDENT@0..1 "a"
            "#]],
        );
    }

    #[test]
    fn parse_type_paren() {
        check(
            PrefixEntryPoint::TypeExpr,
            "(a)",
            &expect![[r#"
                TYPE_PAREN@0..3
                  L_PAREN@0..1 "("
                  TYPE_IDENT@1..2
                    IDENT@1..2 "a"
                  R_PAREN@2..3 ")"
            "#]],
        );
    }

    #[test]
    fn parse_type_arrow() {
        check(
            PrefixEntryPoint::TypeExpr,
            "a -> b",
            &expect![[r#"
                TYPE_ARROW@0..6
                  TYPE_IDENT@0..2
                    IDENT@0..1 "a"
                    WHITESPACE@1..2 " "
                  ARROW@2..4 "->"
                  WHITESPACE@4..5 " "
                  TYPE_IDENT@5..6
                    IDENT@5..6 "b"
            "#]],
        );
    }

    #[test]
    fn type_arrow_is_right_associative() {
        check(
            PrefixEntryPoint::TypeExpr,
            "a -> b -> c",
            &expect![[r#"
                TYPE_ARROW@0..11
                  TYPE_IDENT@0..2
                    IDENT@0..1 "a"
                    WHITESPACE@1..2 " "
                  ARROW@2..4 "->"
                  WHITESPACE@4..5 " "
                  TYPE_ARROW@5..11
                    TYPE_IDENT@5..7
                      IDENT@5..6 "b"
                      WHITESPACE@6..7 " "
                    ARROW@7..9 "->"
                    WHITESPACE@9..10 " "
                    TYPE_IDENT@10..11
                      IDENT@10..11 "c"
            "#]],
        );
    }

    #[test]
    fn parse_type_arrow_nested() {
        check(
            PrefixEntryPoint::TypeExpr,
            "(a -> b) -> c",
            &expect![[r#"
                TYPE_ARROW@0..13
                  TYPE_PAREN@0..9
                    L_PAREN@0..1 "("
                    TYPE_ARROW@1..7
                      TYPE_IDENT@1..3
                        IDENT@1..2 "a"
                        WHITESPACE@2..3 " "
                      ARROW@3..5 "->"
                      WHITESPACE@5..6 " "
                      TYPE_IDENT@6..7
                        IDENT@6..7 "b"
                    R_PAREN@7..8 ")"
                    WHITESPACE@8..9 " "
                  ARROW@9..11 "->"
                  WHITESPACE@11..12 " "
                  TYPE_IDENT@12..13
                    IDENT@12..13 "c"
            "#]],
        );
    }

    #[test]
    fn parse_type_arrow_without_lhs() {
        check_err(
            PrefixEntryPoint::TypeExpr,
            "-> a",
            &expect![[r#"
                TYPE_ARROW@0..4
                  ERROR@0..0
                  ARROW@0..2 "->"
                  WHITESPACE@2..3 " "
                  TYPE_IDENT@3..4
                    IDENT@3..4 "a"
            "#]],
            &["Expected type expression"],
        );
    }
}
