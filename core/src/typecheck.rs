use std::collections::HashMap;

use crate::{types::TypeIdx, Database, Expr, ExprIdx};

pub fn typecheck() {}

type Context<'a> = HashMap<&'a str, TypeIdx>;

fn infer_var_type(var: &str, ctx: &Context) -> Result<TypeIdx, ()> {
    if let Some(idx) = ctx.get(var) {
        Ok(*idx)
    } else {
        Err(())
    }
}

fn check_expr_against_type(
    db: &Database,
    expr: ExprIdx,
    ty: TypeIdx,
    ctx: &Context,
) -> Result<(), String> {
    let expr = &db.expressions[expr];
    match expr {
        Expr::Missing => Err("Missing expression".into()),
        Expr::LetExpr(_) => todo!(),
        Expr::IdentExpr { name } => todo!(),
        Expr::LambdaExpr { params, body } => todo!(),
        Expr::AppExpr { func, arg } => todo!(),
        Expr::LiteralExpr(_) => todo!(),
    }
}
