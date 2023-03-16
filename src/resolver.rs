use std::collections::HashMap;

use crate::expr::*;
use crate::token::*;
use crate::interpreter::*;

pub struct Resolver{
    pub interpreter: Interpreter,
    pub scopes: Vec<HashMap<String, bool>>,
}

impl Resolver{
    fn Resolver(&mut self, interpreter: Interpreter){
        self.interpreter = interpreter;
    }

    //can't do this yet
    fn visitBlockStmt(&mut self) -> Literal{
        Resolver::beginScope(self);
        //Resolver::resolve(/*stmt.statements*/);
        Resolver::endScope(self);

        return Literal::None;
    }

    //Can't do these yet either
    /*fn resolve(statements: Vec<Stmt>){
        for statement in statements{
            resolve(statement);
        }
    }

    fn resolve(&self, statement: Stmt){
        stmt.accept(self);
    }

    fn resolve(&self, expr: Expr){
        expr.accept(self);
    }*/

    fn beginScope(&mut self){
        let mut x: HashMap<String, bool> = HashMap :: new();
        self.scopes.push(x);
    }

    fn endScope(&mut self){
        self.scopes.pop();
    }


}

/*impl ExprVisitor<Literal> for Resolver{

}*/

/*impl StmtVisitor<Literal> for Resolver{
    
}*/
