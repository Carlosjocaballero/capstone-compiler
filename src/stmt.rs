use crate::LoxError::*;
use crate::token::*;
use crate::expr::*;

#[derive(Clone, Debug)]
pub enum Stmt {
    Expression(ExpressionStmt),
    Function(FunctionStmt),
    Print(PrintStmt),
    Var(VarStmt),
    Block(BlockStmt),
    If(IfStmt),
    While(WhileStmt)
}

impl Stmt {
    pub fn accept<T>(&mut self, stmt_visitor: &mut dyn StmtVisitor<T>) -> Result<T, ScannerError> {
        match self {
            Stmt::Expression(v) => v.accept(stmt_visitor), 
            Stmt::Function(v) => v.accept(stmt_visitor),
            Stmt::Print(v) => v.accept(stmt_visitor),
            Stmt::Var(v) => v.accept(stmt_visitor),
            Stmt::Block(v) => v.accept(stmt_visitor),
            Stmt::If(v) => v.accept(stmt_visitor),
            Stmt::While(v) => v.accept(stmt_visitor)
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExpressionStmt {
    pub expression: Box<Expr>,
}
#[derive(Clone, Debug)]
pub struct FunctionStmt{
    pub name: Token,
    pub parameters:Vec<Token>,
    pub body:Vec<Box<Stmt>>
}

#[derive(Clone, Debug)]
pub struct PrintStmt {
    pub expression: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct VarStmt{
    pub name : Token,
    pub initializer: Box<Expr>
}

#[derive(Clone, Debug)]
pub struct BlockStmt{
    pub statements : Vec<Box<Stmt>>
}

#[derive(Clone, Debug)]
pub struct IfStmt {
    pub condition: Box<Expr>,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>
}

#[derive(Clone, Debug)]
pub struct WhileStmt{
    pub condition: Box<Expr>,
    pub body: Box<Stmt>
}

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> Result<T, ScannerError>;
    fn visit_function_stmt(&mut self, stmt: &FunctionStmt) -> Result<T, ScannerError>;
    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Result<T, ScannerError>;
    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> Result<T, ScannerError>;
    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Result<T, ScannerError>;
    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Result<T, ScannerError>;
    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Result<T, ScannerError>;
}

impl ExpressionStmt {
    pub fn accept<T>(&mut self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_expression_stmt(self)
    }
}

impl FunctionStmt {
    pub fn accept<T>(&mut self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_function_stmt(self)
    }
}

impl PrintStmt {
    pub fn accept<T>(&mut self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_print_stmt(self)
    }
}

impl VarStmt{
    pub fn accept<T>(&mut self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_var_stmt(self)
    }
}

impl BlockStmt{
    pub fn accept<T>(&mut self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_block_stmt(self)
    }
}

impl IfStmt{
    pub fn accept<T>(&mut self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_if_stmt(self)
    }
}

impl WhileStmt{
    pub fn accept<T>(&mut self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, ScannerError> {
        visitor.visit_while_stmt(self)
    }
}

