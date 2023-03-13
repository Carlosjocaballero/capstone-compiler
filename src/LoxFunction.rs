use crate::token::*;
use crate::expr::*;
use crate::interpreter::*;


//Draft

pub struct LoxFunction{
    pub declaration:Stmt::Function,

}
impl LoxFunction{
    pub fn LoxFunction(&mut self, declaration:Stmt::Function){
        self.declaration = declaration;
    }
}

pub trait LoxFunction{
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Literal>) -> Literal;
}

impl LoxFunction for LoxCallable{
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Literal>) -> Literal{
        let environment = Box::new(Environment::new_enclosed(interpreter.globals()));
        let i = 0;
        while(i<self.declaration.params.size()){
            environment.define(declaration.params.get(i).lexeme, arguments(i));
            i = i + 1;
        }
        interpreter.executeBlock(declaration.body, environment);
        return None;
    }
} 