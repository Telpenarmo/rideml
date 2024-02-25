use crate::{hir::*, LetDecl};
use la_arena::Arena;
use parser::{nodes as ast, AstToken};

#[allow(unused)]
pub struct Database {
    declarations: Arena<Declaration>,
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
        let declarations = Arena::new();
        let mut expressions = Arena::new();
        let mut type_expressions = Arena::new();

        expressions.alloc(Expr::Missing);
        type_expressions.alloc(TypeExpr::Missing);

        Self {
            declarations,
            expressions,
            type_expressions,
        }
    }

    pub(crate) fn lower_module(&mut self, ast: ast::Module) -> Module {
        let declarations: Vec<DeclarationIdx> = ast
            .decls()
            .filter_map(|ast| {
                self.lower_decl(ast)
                    .map(|decl| self.declarations.alloc(decl))
            })
            .collect();
        let declarations = declarations.into_boxed_slice();
        Module { declarations }
    }

    fn lower_decl(&mut self, ast: ast::Decl) -> Option<Declaration> {
        Some(match ast {
            ast::Decl::LetDecl(ast) => {
                let params = self.lower_params(ast.params());
                let defn = self.lower_expr(ast.expr());
                let defn = self.alloc_expr(defn);
                Declaration::LetDecl(Box::new(LetDecl {
                    name: self.lower_ident(ast.ident_lit())?,
                    params,
                    defn,
                }))
            }
            ast::Decl::OpenDecl(ast) => Declaration::OpenDecl {
                path: self.lower_ident(ast.ident_lit())?,
            },
            ast::Decl::TypeDecl(ast) => {
                let defn = self.lower_type_expr(ast.type_expr());
                let defn = self.alloc_type_expr(defn);
                Declaration::TypeDecl {
                    name: self.lower_ident(ast.ident_lit())?,
                    defn,
                }
            }
        })
    }

    fn lower_params(&mut self, ast: Option<ast::Params>) -> Box<[Param]> {
        ast.map(|ast| ast.params().map(|ast| self.lower_param(ast)).collect())
            .unwrap_or_default()
    }

    fn lower_param(&mut self, ast: ast::Param) -> Param {
        let typ = ast.type_expr().map(|typ| self.lower_type_expr(Some(typ)));
        let typ = typ.map(|typ| self.alloc_type_expr(typ));
        Param {
            name: ast.ident_lit().expect("Empty param. How?").text().into(),
            typ,
        }
    }

    fn lower_type_expr(&mut self, type_expr: Option<ast::TypeExpr>) -> TypeExpr {
        if let None = type_expr {
            return TypeExpr::Missing;
        }
        match type_expr.unwrap() {
            ast::TypeExpr::TypeIdent(ast) => ast.ident_lit().map_or(TypeExpr::Missing, |name| {
                let name = name.text().into();
                TypeExpr::IdentTypeExpr { name }
            }),
            ast::TypeExpr::TypeArrow(ast) => {
                let from = self.lower_type_expr(ast.from());
                let from = self.alloc_type_expr(from);

                let to = self.lower_type_expr(ast.to());
                let to = self.alloc_type_expr(to);
                TypeExpr::TypeArrow { from, to }
            }
            ast::TypeExpr::TypeParen(ast) => self.lower_type_expr(ast.type_expr()),
        }
    }

    fn lower_expr(&mut self, expr: Option<ast::Expr>) -> Expr {
        if let None = expr {
            return Expr::Missing;
        }
        match expr.unwrap() {
            ast::Expr::IdentExpr(ast) => {
                ast.ident_lit()
                    .map_or(Expr::Missing, |ident| Expr::IdentExpr {
                        name: ident.text().into(),
                    })
            }
            ast::Expr::ParenExpr(ast) => self.lower_expr(ast.expr()),
            ast::Expr::AppExpr(ast) => {
                let func = self.lower_expr(ast.func());
                let func = self.alloc_expr(func);

                let arg = self.lower_expr(ast.arg());
                let arg = self.alloc_expr(arg);

                Expr::AppExpr { func, arg }
            }
            ast::Expr::LiteralExpr(ast) => ast.literal().map_or(Expr::Missing, |lit| {
                Expr::LiteralExpr(match lit.kind() {
                    ast::LiteralKind::Int => Literal::IntLiteral(
                        lit.syntax().text().parse().expect("Invalid int literal"),
                    ),

                    ast::LiteralKind::EmptyParen => Literal::UnitLiteral,
                })
            }),
            ast::Expr::LambdaExpr(ast) => {
                let params = self.lower_params(ast.params());
                let body = self.lower_expr(ast.body());
                let body = self.alloc_expr(body);
                Expr::LambdaExpr { params, body }
            }
            ast::Expr::LetExpr(ast) => ast.ident_lit().map_or(Expr::Missing, |ident| {
                let name = ident.text().into();

                let params = self.lower_params(ast.params());

                let defn = self.lower_expr(ast.def());
                let defn = self.alloc_expr(defn);

                let body = self.lower_expr(ast.body());
                let body = self.alloc_expr(body);

                Expr::LetExpr(Box::new(LetExpr {
                    name,
                    params,
                    defn,
                    body,
                }))
            }),
            ast::Expr::BinaryExpr(_) => todo!("Binary expressions are not yet supported"),
        }
    }

    fn lower_ident(&mut self, ident: Option<parser::SyntaxToken>) -> Option<String> {
        ident.map(|ident| ident.text().into())
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

#[cfg(test)]
mod tests {
    use crate::Expr;

    fn check_expr(text: &str, expected_database: super::Database) {
        let module_syntax = parser::parse(&format!("let x = {};", text))
            .module()
            .unwrap();
        let mut db = super::Database::default();

        db.lower_module(module_syntax);

        assert_eq!(db.expressions, expected_database.expressions);
        assert_eq!(db.type_expressions, expected_database.type_expressions);
    }

    #[test]
    fn lower_ident() {
        let mut database = super::Database::default();
        database.alloc_expr(Expr::IdentExpr { name: "x".into() });
        check_expr("x", database);
    }

    #[test]
    fn lower_app() {
        let mut database = super::Database::default();
        let func = database.alloc_expr(Expr::IdentExpr { name: "f".into() });
        let arg = database.alloc_expr(Expr::IdentExpr { name: "y".into() });
        database.alloc_expr(Expr::AppExpr { func, arg });
        check_expr("f y", database);
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
        database.alloc_expr(Expr::LambdaExpr { params, body });

        check_expr("\\x y -> x", database);
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

        database.alloc_expr(Expr::LetExpr(Box::new(crate::LetExpr {
            name: "a".into(),
            params,
            defn,
            body,
        })));

        check_expr("let a b = c in d", database);
    }

    #[test]
    fn lower_lambda_missing_type() {
        let mut database = super::Database::default();
        let param = crate::Param {
            name: "x".into(),
            typ: None,
        };
        let params = vec![param].into_boxed_slice();
        let body = database.alloc_expr(Expr::IdentExpr { name: "x".into() });
        database.alloc_expr(Expr::LambdaExpr { params, body });

        check_expr("\\x -> x", database);
    }

    #[test]
    fn lower_lambda_missing_body() {
        let mut database = super::Database::default();
        let param = crate::Param {
            name: "x".into(),
            typ: None,
        };
        let params = vec![param].into_boxed_slice();

        database.alloc_expr(Expr::LambdaExpr {
            params,
            body: database.missing_expr_id(),
        });

        check_expr("\\x ->", database);
    }

    #[test]
    fn lower_lambda_missing_params() {
        let mut database = super::Database::default();
        let body = database.alloc_expr(Expr::IdentExpr { name: "x".into() });
        let params = vec![].into_boxed_slice();
        database.alloc_expr(Expr::LambdaExpr { params, body });

        check_expr("\\-> x", database);
    }

    #[test]
    fn lower_let_missing_defn() {
        let mut database = super::Database::default();
        let param = crate::Param {
            name: "b".into(),
            typ: None,
        };
        let params = vec![param].into_boxed_slice();
        let body = database.alloc_expr(Expr::IdentExpr { name: "d".into() });
        database.alloc_expr(Expr::LetExpr(Box::new(crate::LetExpr {
            name: "a".into(),
            params,
            defn: database.missing_expr_id(),
            body,
        })));

        check_expr("let a b = in d", database);
    }

    #[test]
    fn lower_let_missing_body() {
        let mut database = super::Database::default();
        let param = crate::Param {
            name: "b".into(),
            typ: None,
        };
        let params = vec![param].into_boxed_slice();
        let defn = database.alloc_expr(Expr::IdentExpr { name: "c".into() });
        database.alloc_expr(Expr::LetExpr(Box::new(crate::LetExpr {
            name: "a".into(),
            params,
            defn,
            body: database.missing_expr_id(),
        })));

        check_expr("let a b = c in", database);
    }
}
