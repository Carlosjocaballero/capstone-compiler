use crate::LoxError::*;
use crate::token::*;

pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
}

impl Stmt {
    pub fn accept<T>(&self, stmt_visitor: &dyn StmtVisitor<T>) -> Result<T, ScannerError> {
        match self {
            Stmt::Expression(v) => v.accept(stmt_visitor), 
            Stmt::Print(v) => v.accept(stmt_visitor), 
        }
    }
}

pub struct ExpressionStmt {
    pub expression: Box<Stmt>,
}

pub struct PrintStmt {
    pub expression: Box<Stmt>,
}

pub trait ExprVisitor<T> {
    fn visit_expression_stmt(&self, expr: &ExpressionStmt) -> Result<T, ScannerError>;
    fn visit_print_stmt(&self, expr: &PrintStmt) -> Result<T, ScannerError>;
}

impl ExpressionStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_expression_stmt(self)
    }
}

impl PrintStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_print_stmt(self)
    }
}

