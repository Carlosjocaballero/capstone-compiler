use std::collections::HashMap;
use std::hash::Hash;

use crate::LoxError;
use crate::expr::*;
use crate::token::*;
use crate::interpreter::*;
use crate::stmt::*;
use crate::LoxError::*;

pub struct Resolver{
    pub interpreter: Interpreter,
    pub scopes: Vec<HashMap<String, bool>>, //scopes should be treated as a stack of hashmaps
}

impl Resolver{
    fn Resolver(&mut self, interpreter: Interpreter){
        self.interpreter = interpreter;
    }

    fn resolve(statements: Vec<Stmt>){
        for statement in statements{
            resolve(statement);
        }
    }

    fn resolve(&self, statement: Stmt){
        stmt.accept(self);
    }

    fn resolve(&self, expr: Expr){
        expr.accept(self);
    }

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
    fn declare(&self, name: Token){
        if self.scopes.is_empty(){return;}

        let mut scope: HashMap<String, bool> = self.scopes.last(); 
        scope.insert(name.lexeme, false);
    }

    /*
    Set the variable's value in the scope map to indicate that it is ready to be used
     */
    fn define(&mut self, name: Token){
        if self.scopes.is_empty(){return;}
        
        let Some(top) = self.scopes.last();
        top.insert(name.lexeme, true);
    }

    /*
    CAN'T DO THIS YET
    fn resolveLocal(&self, expr: Expr, name: Token){
        for scope in self.scopes{
            if scope.contains_key(&name.lexeme){
                self.interpreter.resolve(expr, self.scopes.len());
                return;
            }
        }
    }*/


}

impl ExprVisitor<Literal> for Resolver{
    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> Result<Literal, ScannerError> {
        let Some(top) = self.scopes.last();
        if !self.scopes.is_empty() &&  top.get(&expr.name.lexeme) == false /*Boolean.FALSE*/{
            ErrorTools::_error(self, expr.name, "Can't read local variable in its own initializer.".to_string());;
        }

        //Can't do yet
        /*self.resolveLocal(expr.name);*/
        return Ok(Literal::None);
    }
}

impl StmtVisitor<Literal> for Resolver{
    fn visit_block_stmt(&mut self, expr: &BlockStmt) -> Result<Literal, ScannerError> {
        self.beginScope();
        self.resolve(expr.statements);
        self.endScope();
        
        return Ok(Literal::None);
    }

    fn visit_var_stmt(&mut self, expr: &VarStmt) -> Result<Literal, ScannerError> {
        declare(expr.name);
        if expr.initializer != Literal::None{
            self.resolve(expr.initializer);
        }
        define(expr.name);

        return Ok(Literal::None);
    }
}
