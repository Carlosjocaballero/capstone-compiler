use crate::token::*;
use crate::expr::*;
use crate::interpreter::*;
use crate::LoxCallable::LoxCallable;
use crate::Environment;
use crate::Stmt;

//Draft

pub struct LoxFunction{
    pub declaration:Stmt::Function,
}

impl LoxFunction{
    pub fn LoxFunction(&mut self, declaration:Stmt::Function){
        self.declaration = declaration;
    }
}

dyn impl LoxFunction for LoxCallable{
    fn call(&mut self, interpreter: &Interpreter, arguments: Vec<Literal>) -> Literal{
        let environment= Box::new(Environment::new_enclosed(&interpreter.globals));
        let i = 0;
        while i<self.declaration.params.size(){
            environment.define(self.declaration.params.get(i).lexeme, arguments(i));
            i = i + 1;
        }
        interpreter.execute_block(self.declaration.body, environment);
        return None;
    }
    fn arity(&mut self) -> Literal{
        return self.declaration.params.size();
    }
    fn toString(&mut self) -> String{
        return "<fn" + self.declaration.name.lexeme + ">";
    }
} 