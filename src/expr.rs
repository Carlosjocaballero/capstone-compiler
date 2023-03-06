use crate::LoxError::*;
use crate::scanner::Scanner;
use crate::token::*;
use std::any::{Any, TypeId};

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Variable(VariableExpr),
    Assign(AssignExpr),
    Clone(CloneExpr),
    None
}

impl Expr {
    pub fn accept<T>(&self, expr_visitor: &mut dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        match self {
            Expr::Binary(v) => v.accept(expr_visitor), 
            Expr::Grouping(v) => v.accept(expr_visitor), 
            Expr::Literal(v) => v.accept(expr_visitor), 
            Expr::Unary(v) => v.accept(expr_visitor),
            Expr::Variable(v) => v.accept(expr_visitor),
            Expr::Assign(v) => v.accept(expr_visitor),
            Expr::Clone(v) => v.accept(expr_visitor),
            _ => Err(ScannerError{is_error : true})
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}
#[derive(PartialEq, Clone, Debug)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct LiteralExpr {
    pub value: Option<Literal>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct VariableExpr{
    pub name: Token
}

#[derive(PartialEq, Clone, Debug)]
pub struct CloneExpr {
    pub clone: Box<Expr>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct AssignExpr{
    pub name: Token,
    pub value: Box<Expr>
}

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<T, ScannerError>;
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<T, ScannerError>;
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<T, ScannerError>;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<T, ScannerError>;
    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> Result<T, ScannerError>;
    fn visit_clone_expr(&mut self, expr: &CloneExpr) -> Result<T, ScannerError>;
    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> Result<T, ScannerError>;
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

impl VariableExpr{
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_variable_expr(self)
    }
}

impl AssignExpr{
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_assign_expr(self)
    }
}

impl CloneExpr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_clone_expr(self)
    }
}
