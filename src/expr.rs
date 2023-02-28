use crate::LoxError::*;
use crate::token::*;

#[derive(Clone)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

impl Expr {
    pub fn accept<T>(&self, expr_visitor: &mut dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        match self {
            Expr::Binary(v) => v.accept(expr_visitor), 
            Expr::Grouping(v) => v.accept(expr_visitor), 
            Expr::Literal(v) => v.accept(expr_visitor), 
            Expr::Unary(v) => v.accept(expr_visitor), 
        }
    }
}

#[derive(Clone)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Clone)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

#[derive(Clone)]
pub struct LiteralExpr {
    pub value: Option<Literal>,
}

#[derive(Clone)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<T, ScannerError>;
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<T, ScannerError>;
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<T, ScannerError>;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<T, ScannerError>;
}

impl BinaryExpr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_binary_expr(self)
    }
}

impl GroupingExpr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_grouping_expr(self)
    }
}

impl LiteralExpr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_literal_expr(self)
    }
}

impl UnaryExpr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_unary_expr(self)
    }
}

