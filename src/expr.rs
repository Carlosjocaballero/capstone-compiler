use crate::LoxError::*;
use crate::token::*;

pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Clone(CloneExpr),
}

impl Expr {
    pub fn accept<T>(&self, expr_visitor: &dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        match self {
            Expr::Binary(v) => v.accept(expr_visitor), 
            Expr::Grouping(v) => v.accept(expr_visitor), 
            Expr::Literal(v) => v.accept(expr_visitor), 
            Expr::Unary(v) => v.accept(expr_visitor), 
            Expr::Clone(v) => v.accept(expr_visitor), 
        }
    }
}

pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

pub struct LiteralExpr {
    pub value: Option<Literal>,
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct CloneExpr {
    pub clone: Box<Expr>,
}

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T, ScannerError>;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<T, ScannerError>;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<T, ScannerError>;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<T, ScannerError>;
    fn visit_clone_expr(&self, expr: &CloneExpr) -> Result<T, ScannerError>;
}

impl BinaryExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_binary_expr(self)
    }
}

impl GroupingExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_grouping_expr(self)
    }
}

impl LiteralExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_literal_expr(self)
    }
}

impl UnaryExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_unary_expr(self)
    }
}

impl CloneExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_clone_expr(self)
    }
}

