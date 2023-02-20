use crate::error::*;
use crate::token::*;

pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

pub struct BinaryExpr {
    'left': 'Box<Expr>',
    'operator': 'token',
    'right': 'Box<Expr>',
}

pub struct GroupingExpr {
    'expression': 'Box<Expr>',
}

pub struct LiteralExpr {
    'value': 'Object',
}

pub struct UnaryExpr {
    'operator': 'Token',
    'right': 'Box<Expr>',
}

