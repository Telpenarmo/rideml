//! This is a generated file, please do not edit manually. Changes can be
//! made in codegeneration that lives in `xtask` top-level dir.

#![allow(non_snake_case, clippy::match_like_matches_macro)]
use crate::{
    ast::{support, AstChildren, AstNode, AstToken},
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxToken,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
    pub(crate) syntax: SyntaxNode,
}
impl Module {
    pub fn decls(&self) -> AstChildren<Decl> {
        support::children(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LetDecl {
    pub(crate) syntax: SyntaxNode,
}
impl LetDecl {
    pub fn params(&self) -> Option<Params> {
        support::child(&self.syntax)
    }
    pub fn let_kw_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, LET_KW)
    }
    pub fn ident_lit(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, IDENT)
    }
    pub fn equal_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, EQUAL)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SEMICOLON)
    }
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OpenDecl {
    pub(crate) syntax: SyntaxNode,
}
impl OpenDecl {
    pub fn open_kw_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, OPEN_KW)
    }
    pub fn ident_lit(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, IDENT)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SEMICOLON)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeDecl {
    pub(crate) syntax: SyntaxNode,
}
impl TypeDecl {
    pub fn type_kw_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, TYPE_KW)
    }
    pub fn ident_lit(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, IDENT)
    }
    pub fn equal_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, EQUAL)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SEMICOLON)
    }
    pub fn type_expr(&self) -> Option<TypeExpr> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Params {
    pub(crate) syntax: SyntaxNode,
}
impl Params {
    pub fn params(&self) -> AstChildren<Param> {
        support::children(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeIdent {
    pub(crate) syntax: SyntaxNode,
}
impl TypeIdent {
    pub fn ident_lit(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, IDENT)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeArrow {
    pub(crate) syntax: SyntaxNode,
}
impl TypeArrow {
    pub fn arrow_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, ARROW)
    }
    pub fn from(&self) -> Option<TypeExpr> {
        crate::handwritten_ast::type_arrow_from(&self.syntax)
    }
    pub fn to(&self) -> Option<TypeExpr> {
        crate::handwritten_ast::type_arrow_to(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeParen {
    pub(crate) syntax: SyntaxNode,
}
impl TypeParen {
    pub fn type_expr(&self) -> Option<TypeExpr> {
        support::child(&self.syntax)
    }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, L_PAREN)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, R_PAREN)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdentExpr {
    pub(crate) syntax: SyntaxNode,
}
impl IdentExpr {
    pub fn ident_lit(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, IDENT)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LiteralExpr {
    pub(crate) syntax: SyntaxNode,
}
impl LiteralExpr {
    pub fn literal(&self) -> Option<Literal> {
        support::token_child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AppExpr {
    pub(crate) syntax: SyntaxNode,
}
impl AppExpr {
    pub fn func(&self) -> Option<Expr> {
        crate::handwritten_ast::app_expr_func(&self.syntax)
    }
    pub fn arg(&self) -> Option<Expr> {
        crate::handwritten_ast::app_expr_arg(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LambdaExpr {
    pub(crate) syntax: SyntaxNode,
}
impl LambdaExpr {
    pub fn params(&self) -> Option<Params> {
        support::child(&self.syntax)
    }
    pub fn lambda_lit(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, LAMBDA)
    }
    pub fn arrow_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, ARROW)
    }
    pub fn body(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LetExpr {
    pub(crate) syntax: SyntaxNode,
}
impl LetExpr {
    pub fn let_kw_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, LET_KW)
    }
    pub fn ident_lit(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, IDENT)
    }
    pub fn equal_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, EQUAL)
    }
    pub fn in_kw_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, IN_KW)
    }
    pub fn params(&self) -> Option<Params> {
        support::child(&self.syntax)
    }
    pub fn def(&self) -> Option<Expr> {
        crate::handwritten_ast::let_expr_def(&self.syntax)
    }
    pub fn body(&self) -> Option<Expr> {
        crate::handwritten_ast::let_expr_body(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParenExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ParenExpr {
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, L_PAREN)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, R_PAREN)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinaryExpr {
    pub(crate) syntax: SyntaxNode,
}
impl BinaryExpr {
    pub fn lhs(&self) -> Option<Expr> {
        crate::handwritten_ast::binary_expr_lhs(&self.syntax)
    }
    pub fn rhs(&self) -> Option<Expr> {
        crate::handwritten_ast::binary_expr_rhs(&self.syntax)
    }
    pub fn infix_symbol(&self) -> Option<InfixSymbol> {
        support::token_child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    pub(crate) syntax: SyntaxNode,
}
impl Param {
    pub fn ident_lit(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, IDENT)
    }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, L_PAREN)
    }
    pub fn colon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, COLON)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, R_PAREN)
    }
    pub fn type_expr(&self) -> Option<TypeExpr> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Decl {
    LetDecl(LetDecl),
    OpenDecl(OpenDecl),
    TypeDecl(TypeDecl),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    IdentExpr(IdentExpr),
    LiteralExpr(LiteralExpr),
    AppExpr(AppExpr),
    LambdaExpr(LambdaExpr),
    LetExpr(LetExpr),
    ParenExpr(ParenExpr),
    BinaryExpr(BinaryExpr),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeExpr {
    TypeIdent(TypeIdent),
    TypeArrow(TypeArrow),
    TypeParen(TypeParen),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Literal {
    syntax: SyntaxToken,
    kind: LiteralKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LiteralKind {
    Int,
    EmptyParen,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InfixSymbol {
    syntax: SyntaxToken,
    kind: InfixSymbolKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InfixSymbolKind {
    Plus,
    Minus,
    Star,
    Slash,
}
impl AstNode for Module {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MODULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for LetDecl {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LET_DECL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for OpenDecl {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == OPEN_DECL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TypeDecl {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TYPE_DECL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Params {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PARAMS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TypeIdent {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TYPE_IDENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TypeArrow {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TYPE_ARROW
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TypeParen {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TYPE_PAREN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for IdentExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == IDENT_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for LiteralExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LITERAL_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AppExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == APP_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for LambdaExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LAMBDA_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for LetExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LET_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ParenExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PAREN_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for BinaryExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == BINARY_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Param {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PARAM
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl From<LetDecl> for Decl {
    fn from(node: LetDecl) -> Decl {
        Decl::LetDecl(node)
    }
}
impl From<OpenDecl> for Decl {
    fn from(node: OpenDecl) -> Decl {
        Decl::OpenDecl(node)
    }
}
impl From<TypeDecl> for Decl {
    fn from(node: TypeDecl) -> Decl {
        Decl::TypeDecl(node)
    }
}
impl AstNode for Decl {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            LET_DECL | OPEN_DECL | TYPE_DECL => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            LET_DECL => Decl::LetDecl(LetDecl { syntax }),
            OPEN_DECL => Decl::OpenDecl(OpenDecl { syntax }),
            TYPE_DECL => Decl::TypeDecl(TypeDecl { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Decl::LetDecl(it) => &it.syntax,
            Decl::OpenDecl(it) => &it.syntax,
            Decl::TypeDecl(it) => &it.syntax,
        }
    }
}
impl From<IdentExpr> for Expr {
    fn from(node: IdentExpr) -> Expr {
        Expr::IdentExpr(node)
    }
}
impl From<LiteralExpr> for Expr {
    fn from(node: LiteralExpr) -> Expr {
        Expr::LiteralExpr(node)
    }
}
impl From<AppExpr> for Expr {
    fn from(node: AppExpr) -> Expr {
        Expr::AppExpr(node)
    }
}
impl From<LambdaExpr> for Expr {
    fn from(node: LambdaExpr) -> Expr {
        Expr::LambdaExpr(node)
    }
}
impl From<LetExpr> for Expr {
    fn from(node: LetExpr) -> Expr {
        Expr::LetExpr(node)
    }
}
impl From<ParenExpr> for Expr {
    fn from(node: ParenExpr) -> Expr {
        Expr::ParenExpr(node)
    }
}
impl From<BinaryExpr> for Expr {
    fn from(node: BinaryExpr) -> Expr {
        Expr::BinaryExpr(node)
    }
}
impl AstNode for Expr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            IDENT_EXPR | LITERAL_EXPR | APP_EXPR | LAMBDA_EXPR | LET_EXPR | PAREN_EXPR
            | BINARY_EXPR => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            IDENT_EXPR => Expr::IdentExpr(IdentExpr { syntax }),
            LITERAL_EXPR => Expr::LiteralExpr(LiteralExpr { syntax }),
            APP_EXPR => Expr::AppExpr(AppExpr { syntax }),
            LAMBDA_EXPR => Expr::LambdaExpr(LambdaExpr { syntax }),
            LET_EXPR => Expr::LetExpr(LetExpr { syntax }),
            PAREN_EXPR => Expr::ParenExpr(ParenExpr { syntax }),
            BINARY_EXPR => Expr::BinaryExpr(BinaryExpr { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Expr::IdentExpr(it) => &it.syntax,
            Expr::LiteralExpr(it) => &it.syntax,
            Expr::AppExpr(it) => &it.syntax,
            Expr::LambdaExpr(it) => &it.syntax,
            Expr::LetExpr(it) => &it.syntax,
            Expr::ParenExpr(it) => &it.syntax,
            Expr::BinaryExpr(it) => &it.syntax,
        }
    }
}
impl From<TypeIdent> for TypeExpr {
    fn from(node: TypeIdent) -> TypeExpr {
        TypeExpr::TypeIdent(node)
    }
}
impl From<TypeArrow> for TypeExpr {
    fn from(node: TypeArrow) -> TypeExpr {
        TypeExpr::TypeArrow(node)
    }
}
impl From<TypeParen> for TypeExpr {
    fn from(node: TypeParen) -> TypeExpr {
        TypeExpr::TypeParen(node)
    }
}
impl AstNode for TypeExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TYPE_IDENT | TYPE_ARROW | TYPE_PAREN => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TYPE_IDENT => TypeExpr::TypeIdent(TypeIdent { syntax }),
            TYPE_ARROW => TypeExpr::TypeArrow(TypeArrow { syntax }),
            TYPE_PAREN => TypeExpr::TypeParen(TypeParen { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TypeExpr::TypeIdent(it) => &it.syntax,
            TypeExpr::TypeArrow(it) => &it.syntax,
            TypeExpr::TypeParen(it) => &it.syntax,
        }
    }
}
impl AstToken for Literal {
    fn can_cast(kind: SyntaxKind) -> bool {
        LiteralKind::can_cast(kind)
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        let kind = LiteralKind::cast(syntax.kind())?;
        Some(Literal { syntax, kind })
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
impl LiteralKind {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            INT | EMPTY_PAREN => true,
            _ => false,
        }
    }
    pub fn cast(kind: SyntaxKind) -> Option<Self> {
        let res = match kind {
            INT => Self::Int,
            EMPTY_PAREN => Self::EmptyParen,
            _ => return None,
        };
        Some(res)
    }
}
impl Literal {
    pub fn kind(&self) -> LiteralKind {
        self.kind
    }
}
impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for InfixSymbol {
    fn can_cast(kind: SyntaxKind) -> bool {
        InfixSymbolKind::can_cast(kind)
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        let kind = InfixSymbolKind::cast(syntax.kind())?;
        Some(InfixSymbol { syntax, kind })
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
impl InfixSymbolKind {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PLUS | MINUS | STAR | SLASH => true,
            _ => false,
        }
    }
    pub fn cast(kind: SyntaxKind) -> Option<Self> {
        let res = match kind {
            PLUS => Self::Plus,
            MINUS => Self::Minus,
            STAR => Self::Star,
            SLASH => Self::Slash,
            _ => return None,
        };
        Some(res)
    }
}
impl InfixSymbol {
    pub fn kind(&self) -> InfixSymbolKind {
        self.kind
    }
}
impl std::fmt::Display for InfixSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Decl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TypeExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for LetDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for OpenDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TypeDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Params {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TypeIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TypeArrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TypeParen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for IdentExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for LiteralExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AppExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for LambdaExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for LetExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ParenExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
