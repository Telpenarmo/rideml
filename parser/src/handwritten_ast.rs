use crate::{
    ast::support,
    nodes::{Expr, TypeExpr},
    AstNode,
};

pub(crate) fn app_expr_func(parent: &rowan::SyntaxNode<crate::RideMLLanguage>) -> Option<Expr> {
    support::child(parent)
}

pub(crate) fn app_expr_arg(parent: &rowan::SyntaxNode<crate::RideMLLanguage>) -> Option<Expr> {
    parent.children().skip(1).find_map(Expr::cast)
}

pub(crate) fn type_arrow_from(
    parent: &rowan::SyntaxNode<crate::RideMLLanguage>,
) -> Option<crate::nodes::TypeExpr> {
    support::child(parent)
}

pub(crate) fn type_arrow_to(
    parent: &rowan::SyntaxNode<crate::RideMLLanguage>,
) -> Option<crate::nodes::TypeExpr> {
    parent
        .children_with_tokens()
        .skip_while(|child| child.kind() != crate::SyntaxKind::ARROW)
        .skip(1)
        .find_map(|child| match child {
            rowan::NodeOrToken::Node(n) => TypeExpr::cast(n),
            rowan::NodeOrToken::Token(_) => None,
        })
}

pub(crate) fn let_expr_def(parent: &rowan::SyntaxNode<crate::RideMLLanguage>) -> Option<Expr> {
    parent
        .children_with_tokens()
        .skip_while(|child| match child {
            rowan::NodeOrToken::Node(_) => false,
            rowan::NodeOrToken::Token(t) => t.kind() != crate::SyntaxKind::LET_KW,
        })
        .take_while(|child| match child {
            rowan::NodeOrToken::Node(_) => true,
            rowan::NodeOrToken::Token(t) => t.kind() != crate::SyntaxKind::IN_KW,
        })
        .find_map(|child| match child {
            rowan::NodeOrToken::Node(n) => Expr::cast(n),
            rowan::NodeOrToken::Token(_) => None,
        })
}

pub(crate) fn let_expr_body(parent: &rowan::SyntaxNode<crate::RideMLLanguage>) -> Option<Expr> {
    parent
        .children_with_tokens()
        .skip_while(|child| match child {
            rowan::NodeOrToken::Node(_) => true,
            rowan::NodeOrToken::Token(t) => t.kind() != crate::SyntaxKind::IN_KW,
        })
        .find_map(|child| match child {
            rowan::NodeOrToken::Node(n) => Expr::cast(n),
            rowan::NodeOrToken::Token(_) => None,
        })
}

pub(crate) fn binary_expr_lhs(parent: &rowan::SyntaxNode<crate::RideMLLanguage>) -> Option<Expr> {
    support::child(parent)
}

pub(crate) fn binary_expr_rhs(parent: &rowan::SyntaxNode<crate::RideMLLanguage>) -> Option<Expr> {
    parent
        .children_with_tokens()
        .skip_while(|child| match child {
            rowan::NodeOrToken::Node(_) => true,
            rowan::NodeOrToken::Token(_) => false,
        })
        .find_map(|child| match child {
            rowan::NodeOrToken::Node(n) => Expr::cast(n),
            rowan::NodeOrToken::Token(_) => None,
        })
}
