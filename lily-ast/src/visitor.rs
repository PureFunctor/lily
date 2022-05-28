use super::expr::{Expr, Literal};
use super::r#type::Type;

pub mod common;

pub trait Visitor<'ast, Ann>: Sized {
    fn visit_expr(&mut self, e: &'ast Expr<Ann>) {
        walk_expr(self, e)
    }

    fn visit_type(&mut self, t: &'ast Type<Ann>) {
        walk_type(self, t)
    }
}

pub fn walk_expr<'ast, V, Ann>(visitor: &mut V, e: &'ast Expr<Ann>)
where
    V: Visitor<'ast, Ann>,
{
    match e {
        Expr::Literal { ann: _, literal } => match literal {
            Literal::Character(_) => (),
            Literal::String(_) => (),
            Literal::Int(_) => (),
            Literal::Float(_) => (),
            Literal::Array(es) => {
                for e in es.iter() {
                    visitor.visit_expr(e);
                }
            }
            Literal::Object(es) => {
                for (_, e) in es.iter() {
                    visitor.visit_expr(e);
                }
            }
        },
        Expr::Variable { ann: _, name: _ } => (),
        Expr::Lambda {
            ann: _,
            argument: _,
            expr,
        } => visitor.visit_expr(expr),
        Expr::Application {
            ann: _,
            function,
            argument,
        } => {
            visitor.visit_expr(function);
            visitor.visit_expr(argument);
        }
        Expr::Annotation {
            ann: _,
            expr,
            r#type,
            checked: _,
        } => {
            visitor.visit_expr(expr);
            visitor.visit_type(r#type);
        }
        Expr::Let {
            ann: _,
            name: _,
            value,
            expr,
        } => {
            visitor.visit_expr(value);
            visitor.visit_expr(expr);
        }
    }
}

pub fn walk_type<'ast, V, Ann>(visitor: &mut V, t: &'ast Type<Ann>)
where
    V: Visitor<'ast, Ann>,
{
    match t {
        Type::Forall {
            ann: _,
            name: _,
            kind,
            r#type,
        } => {
            if let Some(kind) = kind {
                visitor.visit_type(kind);
            };
            visitor.visit_type(r#type);
        }
        Type::Skolem { ann: _, name: _ } => {}
        Type::Unsolved { ann: _, name: _ } => {}
        Type::Variable { ann: _, name: _ } => {}
        Type::Constructor { ann: _, name: _ } => {}
        Type::Application {
            ann: _,
            variant: _,
            function,
            argument,
        } => {
            visitor.visit_type(function);
            visitor.visit_type(argument);
        }
        Type::Function {
            ann: _,
            argument,
            result,
        } => {
            visitor.visit_type(argument);
            visitor.visit_type(result);
        }
    }
}
