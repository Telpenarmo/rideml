use la_arena::Idx;

pub(crate) type ExprIdx = Idx<Expr>;
pub(crate) type TypeExprIdx = Idx<TypeExpr>;

#[allow(unused)]
pub struct Module {
    pub declarations: Box<[Declaration]>,
}

#[derive(PartialEq, Debug)]
pub struct LetDecl {
    pub name: String,
    pub params: Box<[Param]>,
    pub defn: Box<Expr>,
}

#[derive(PartialEq, Debug)]
pub enum Declaration {
    TypeDecl { name: String, defn: Box<TypeExpr> },
    LetDecl(Box<LetDecl>),
    OpenDecl { path: String },
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Missing,
    LetExpr(Box<LetExpr>),
    IdentExpr { name: String },
    LambdaExpr { params: Box<[Param]>, body: ExprIdx },
    AppExpr { func: ExprIdx, arg: ExprIdx },
    LiteralExpr(Literal),
}

#[allow(unused)]
#[derive(PartialEq, Debug)]
pub struct LetExpr {
    pub name: String,
    pub params: Box<[Param]>,
    pub defn: ExprIdx,
    pub body: ExprIdx,
}

#[derive(PartialEq, Debug)]
pub enum TypeExpr {
    Missing,
    IdentTypeExpr { name: String },
    TypeArrow { from: TypeExprIdx, to: TypeExprIdx },
}

#[derive(PartialEq, Debug)]
pub enum Literal {
    IntLiteral(i64),
    BoolLiteral(bool),
    UnitLiteral,
}

#[allow(unused)]
#[derive(PartialEq, Debug)]
pub struct Param {
    pub(crate) name: String,
    pub(crate) typ: Option<TypeExprIdx>,
}
