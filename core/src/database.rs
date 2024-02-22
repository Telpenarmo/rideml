use crate::{hir::*, LetDecl};
use la_arena::Arena;
use parser::{nodes as ast, AstToken};

#[allow(unused)]
pub struct Database {
    expressions: Arena<Expr>,
    type_expressions: Arena<TypeExpr>,
}

#[allow(unreachable_code, unused)]
impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(unused)]
impl Database {
    pub fn new() -> Self {
        let mut expressions = Arena::new();
        let mut type_expressions = Arena::new();

        expressions.alloc(Expr::Missing);
        type_expressions.alloc(TypeExpr::Missing);

        Self {
            expressions,
            type_expressions,
        }
    }

    pub(crate) fn lower_module(&mut self, ast: ast::Module) -> Option<Module> {
        let declarations: Vec<Declaration> =
            ast.decls().filter_map(|ast| self.lower_decl(ast)).collect();
        let declarations = declarations.into_boxed_slice();
        Some(Module { declarations })
    }

    fn lower_decl(&mut self, ast: ast::Decl) -> Option<Declaration> {
        Some(match ast {
            ast::Decl::LetDecl(ast) => {
                let params = ast.params().map(|ast| {
                    ast.params()
                        .filter_map(|param| self.lower_param(param))
                        .collect()
                })?;
                let defn = ast.expr().and_then(|expr| self.lower_expr(expr))?;
                let defn = Box::new(defn);
                Declaration::LetDecl(Box::new(LetDecl {
                    name: ast.ident_lit()?.text().into(),
                    params,
                    defn,
                }))
            }
            ast::Decl::OpenDecl(ast) => Declaration::OpenDecl {
                path: ast.ident_lit()?.text().into(),
            },
            ast::Decl::TypeDecl(ast) => {
                let defn = ast.type_expr().and_then(|typ| self.lower_type_expr(typ))?;
                let defn = Box::new(defn);
                Declaration::TypeDecl {
                    name: ast.ident_lit()?.text().into(),
                    defn,
                }
            }
        })
    }

    fn lower_param(&mut self, ast: ast::Param) -> Option<Param> {
        let typ = ast.type_expr().and_then(|typ| self.lower_type_expr(typ));
        let typ = typ.map(|typ| self.alloc_type_expr(typ));
        Some(Param {
            name: ast.ident_lit()?.text().into(),
            typ,
        })
    }

    fn lower_type_expr(&mut self, type_expr: ast::TypeExpr) -> Option<TypeExpr> {
        Some(match type_expr {
            ast::TypeExpr::TypeIdent(ast) => TypeExpr::IdentTypeExpr {
                name: ast.ident_lit()?.text().into(),
            },
            ast::TypeExpr::TypeArrow(ast) => {
                let from = ast.from().and_then(|from| self.lower_type_expr(from))?;
                let from = self.alloc_type_expr(from);

                let to = ast.to().and_then(|to| self.lower_type_expr(to))?;
                let to = self.alloc_type_expr(to);
                TypeExpr::TypeArrow { from, to }
            }
            ast::TypeExpr::TypeParen(ast) => self.lower_type_expr(ast.type_expr()?)?,
        })
    }

    fn lower_expr(&mut self, expr: ast::Expr) -> Option<Expr> {
        Some(match expr {
            ast::Expr::IdentExpr(ast) => Expr::IdentExpr {
                name: ast.ident_lit()?.text().into(),
            },
            ast::Expr::ParenExpr(ast) => self.lower_expr(ast.expr()?)?,
            ast::Expr::AppExpr(ast) => {
                let func = self.lower_expr(ast.func()?)?;
                let func = self.alloc_expr(func);

                let arg = self.lower_expr(ast.arg()?)?;
                let arg = self.alloc_expr(arg);

                Expr::AppExpr { func, arg }
            }
            ast::Expr::LiteralExpr(ast) => {
                let lit = ast.literal()?;
                Expr::LiteralExpr(match lit.kind() {
                    ast::LiteralKind::Int => Literal::IntLiteral(
                        lit.syntax().text().parse().expect("Invalid int literal"),
                    ),

                    ast::LiteralKind::EmptyParen => Literal::UnitLiteral,
                })
            }
            ast::Expr::LambdaExpr(ast) => {
                let params = ast.params().map(|ast| {
                    ast.params()
                        .filter_map(|param| self.lower_param(param))
                        .collect()
                })?;
                let body = self.lower_expr(ast.body()?)?;
                let body = self.alloc_expr(body);
                Expr::LambdaExpr { params, body }
            }
            ast::Expr::LetExpr(ast) => {
                let name = ast.ident_lit()?.text().into();

                let params = ast.params().map(|ast| {
                    ast.params()
                        .filter_map(|param| self.lower_param(param))
                        .collect()
                })?;

                let defn = self.lower_expr(ast.def()?)?;
                let defn = self.alloc_expr(defn);

                let body = self.lower_expr(ast.body()?)?;
                let body = self.alloc_expr(body);

                Expr::LetExpr(Box::new(LetExpr {
                    name,
                    params,
                    defn,
                    body,
                }))
            }
            ast::Expr::BinaryExpr(_) => todo!("Binary expressions are not yet supported"),
        })
    }

    fn alloc_expr(&mut self, expr: Expr) -> ExprIdx {
        if let Expr::Missing = expr {
            self.missing_expr_id()
        } else {
            self.expressions.alloc(expr)
        }
    }

    fn alloc_type_expr(&mut self, type_expr: TypeExpr) -> TypeExprIdx {
        if let TypeExpr::Missing = type_expr {
            self.missing_type_expr_id()
        } else {
            self.type_expressions.alloc(type_expr)
        }
    }

    fn missing_expr_id(&self) -> ExprIdx {
        let raw = la_arena::RawIdx::from_u32(0);
        ExprIdx::from_raw(raw)
    }

    fn missing_type_expr_id(&self) -> TypeExprIdx {
        let raw = la_arena::RawIdx::from_u32(0);
        TypeExprIdx::from_raw(raw)
    }
}

// #[cfg(test)]
mod tests {
    use crate::Expr;

    fn check_expr(text: &str, expected: Expr, expected_database: super::Database) {
        let module_syntax = parser::parse(&format!("let x = {};", text))
            .module()
            .unwrap();
        let mut db = super::Database::default();
        let module = db.lower_module(module_syntax).unwrap();

        assert_eq!(db.expressions, expected_database.expressions);

        assert_eq!(module.declarations.len(), 1);
        assert_eq!(
            module.declarations[0],
            crate::Declaration::LetDecl(Box::new(crate::LetDecl {
                name: "x".into(),
                params: vec![].into_boxed_slice(),
                defn: Box::new(expected),
            }))
        )
    }

    #[test]
    fn lower_ident() {
        check_expr(
            "x",
            Expr::IdentExpr { name: "x".into() },
            super::Database::default(),
        );
    }

    #[test]
    fn lower_app() {
        let mut database = super::Database::default();
        let func = database.alloc_expr(Expr::IdentExpr { name: "f".into() });
        let arg = database.alloc_expr(Expr::IdentExpr { name: "y".into() });
        check_expr("f y", Expr::AppExpr { func, arg }, database);
    }

    #[test]
    fn lower_lambda() {
        let mut database = super::Database::default();
        let param = crate::Param {
            name: "x".into(),
            typ: None,
        };
        let param2 = crate::Param {
            name: "y".into(),
            typ: None,
        };
        let params = vec![param, param2].into_boxed_slice();
        let body = database.alloc_expr(Expr::IdentExpr { name: "x".into() });
        check_expr("\\x y -> x", Expr::LambdaExpr { params, body }, database);
    }

    #[test]
    fn lower_let() {
        let mut database = super::Database::default();
        let param = crate::Param {
            name: "b".into(),
            typ: None,
        };
        let params = vec![param].into_boxed_slice();
        let defn = database.alloc_expr(Expr::IdentExpr { name: "c".into() });
        let body = database.alloc_expr(Expr::IdentExpr { name: "d".into() });

        check_expr(
            "let a b = c in d",
            Expr::LetExpr(Box::new(crate::LetExpr {
                name: "a".into(),
                params,
                defn,
                body,
            })),
            database,
        );
    }
}
