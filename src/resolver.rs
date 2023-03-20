use std::collections::HashMap;
use std::hash::Hash;

use crate::LoxError;
use crate::expr::*;
use crate::new;
use crate::token::*;
use crate::interpreter::*;
use crate::stmt::*;
use crate::LoxError::*;
use crate::environment::*;

pub struct Resolver{
    pub interpreter: Interpreter,
    pub scopes: Vec<HashMap<String, bool>>, //scopes should be treated as a stack of hashmaps
    pub error: ResolverError,
    pub currentFunction: FunctionType //Default value is FunctionType::NONE
}

#[derive(Clone, Debug, PartialEq)]
pub enum FunctionType{
    NONE,
    FUNCTION
}

impl Default for Resolver{
    fn default() -> Self {
        let defaultScopes: Vec<HashMap<String, bool>> = Vec::new();
        Resolver { 
            interpreter: Interpreter::new(), 
            scopes: defaultScopes, 
            error: ResolverError { is_error: false }, 
            currentFunction: FunctionType::NONE 
        }
    }
}

impl Resolver{
    fn Resolver(&mut self, interpreter: Interpreter){
        self.interpreter = interpreter;
    }

    /*
    In book, resolve_stmts, resolve, and resolve_expr are all named resolve and are overloaded, however Rust does not
    seem to have funtion overloading. 
    
    Considering making a trait for resolve but for now, renamed some of the functions
    (resolve_stmts and resolve_expr) to handle this issue.

    If need to call resolve functions without knowing what your calling resolve for, then will need to do some type 
    of function overloading
     */
    pub fn resolve_stmts(&mut self, statements: &mut Vec<Box<Stmt>>){
        for statement in statements{
            self.resolve(*statement.clone()); 
        }
    }

    pub fn resolve(&mut self, mut statement: Stmt){
        statement.accept(self);
    }

    pub fn resolve_expr(&mut self, expr: Expr){
        expr.accept(self);
    }

    //11.3.5 - resolveFunction(function: &FunctionStmt)

    fn beginScope(&mut self){
        let mut x: HashMap<String, bool> = HashMap :: new();
        self.scopes.push(x);
    }

    fn endScope(&mut self){
        self.scopes.pop();
    }

    /*
    Declaration adds the variable to the innermost scope so that it shadows any outer scope and so that we know the variable exists.
    Binding the variable name to false lets us mark it as "not ready yet"
     */
    fn declare(&mut self, name: &Token){
        if self.scopes.is_empty(){return;}

        if let Some(scope) = self.scopes.last_mut(){
            if scope.contains_key(&name.lexeme.clone()){
                self.error.error(name.clone(), "Already a variable with this name in this scope.".to_string());
            }
            scope.insert(name.lexeme.clone(), false);
        }
    }

    /*
    Set the variable's value in the scope map to indicate that it is ready to be used
     */
    fn define(&mut self, name: Token){
        if self.scopes.is_empty(){return;}
        
        if let Some(top) = self.scopes.last_mut(){
            top.insert(name.lexeme, true);
        }
    }

    fn resolveLocal(&mut self, expr: Expr, name: Token){
        for (idx, scope) in self.scopes.iter().enumerate(){
            if scope.contains_key(&name.lexeme){
                println!("********RESOLVING LOCAL********");
                self.interpreter.resolve(expr, (self.scopes.len() - 1 - idx).try_into().unwrap());
                return;
            }
        }
    }
    
    fn resolveFunction(&mut self, function: FunctionStmt, _type: FunctionType){
        let enclosingFunction = self.currentFunction.clone();
        self.currentFunction = _type;
        
        self.beginScope();

        for param in function.parameters.iter(){
            self.declare(&param.clone());
            self.define(param.clone());
        }
        let mut body = function.body;
        self.resolve_stmts(&mut body);
        self.endScope();

        self.currentFunction = enclosingFunction;
    }

}

impl ExprVisitor<Literal> for Resolver{
    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> Result<Literal, ScannerError> {
        if let Some(top) = self.scopes.last(){
            if !self.scopes.is_empty() && top.get(&expr.name.lexeme) == Some(&false){
                self.error.error(expr.name.clone(), "Can't read local variable in its own initializer.".to_string());
            }
        }

        self.resolveLocal(Expr::Variable(expr.clone()), expr.name.clone());
        return Ok(Literal::None);
    }

    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> Result<Literal, ScannerError> {
        self.resolve_expr(*expr.value.clone());
        self.resolveLocal(Expr::Assign(expr.clone()), expr.name.clone());
        return Ok(Literal::None);
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<Literal, ScannerError> {
        self.resolve_expr(*expr.left.clone());
        self.resolve_expr(*expr.right.clone());
        return Ok(Literal::None);
    }

    fn visit_calling_expr(&mut self, expr: &CallingExpr) -> Result<Literal, ScannerError> {
        self.resolve_expr(*expr.callee.clone());

        for argument in expr.arguments.iter(){
            self.resolve_expr(*argument.clone());
        }

        return Ok(Literal::None);
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<Literal, ScannerError> {
        self.resolve_expr(*expr.expression.clone());
        return Ok(Literal::None);
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<Literal, ScannerError> {
        return Ok(Literal::None);
    }

    fn visit_logical_expr(&mut self, expr: &LogicalExpr) -> Result<Literal, ScannerError> {
        self.resolve_expr(*expr.left.clone());
        self.resolve_expr(*expr.right.clone());
        return Ok(Literal::None);
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<Literal, ScannerError> {
        self.resolve_expr(*expr.right.clone());
        return Ok(Literal::None);
    }

    //this function is not mentioned in the chapter so for now return None
    fn visit_clone_expr(&mut self, expr: &CloneExpr) -> Result<Literal, ScannerError> {
        return Ok(Literal::None);
    }
}

impl StmtVisitor<Literal> for Resolver{
    fn visit_block_stmt(&mut self, expr: &BlockStmt) -> Result<Literal, ScannerError> {
        self.beginScope();
        self.resolve_stmts(&mut expr.statements.clone());
        self.endScope();
        
        return Ok(Literal::None);
    }

    fn visit_expression_stmt(&mut self, expr: &ExpressionStmt) -> Result<Literal, ScannerError> {
        self.resolve_expr(*expr.expression.clone());
        return Ok(Literal::None);
    }

    fn visit_function_stmt(&mut self, stmt: &FunctionStmt) -> Result<Literal, ScannerError> {
        self.declare(&stmt.name.clone());
        self.define(stmt.name.clone());
        self.resolveFunction(stmt.clone(), FunctionType::FUNCTION);
        return Ok(Literal::None);
    }
    
    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Result<Literal, ScannerError> {
        self.resolve_expr(*stmt.condition.clone());
        self.resolve(*stmt.then_branch.clone());

        match stmt.else_branch.clone(){
            None => (),
            Some(x) => self.resolve(*x)
        }

        return Ok(Literal::None);
    }
    
    fn visit_print_stmt(&mut self, expr: &PrintStmt) -> Result<Literal, ScannerError> {
        self.resolve_expr(*expr.expression.clone());
        return Ok(Literal::None);
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStmt) -> Result<Literal, ScannerError> {
        if self.currentFunction == FunctionType::NONE{
            self.error.error(stmt.keyword.clone(), "Can't return from top-level code.".to_string());
        }
        
        if stmt.value != Box::new(Expr::Literal(LiteralExpr { value: Some(Literal::None) })){
            self.resolve_expr(*stmt.value.clone());
        }
        return Ok(Literal::None);
    }

    fn visit_var_stmt(&mut self, expr: &VarStmt) -> Result<Literal, ScannerError> {
        self.declare(&expr.name);
        if *expr.initializer != Expr::Literal(LiteralExpr { value: Some(Literal::None) }){
            self.resolve_expr(*expr.initializer.clone());
        }
        self.define(expr.name.clone());

        return Ok(Literal::None);
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Result<Literal, ScannerError> {
        self.resolve_expr(*stmt.condition.clone());
        self.resolve(*stmt.body.clone());
        return Ok(Literal::None);
    }
}