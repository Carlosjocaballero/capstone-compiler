use crate::stmt::FunctionStmt;
use crate::token::*;
use crate::interpreter::*;
use crate::Environment;

pub struct LoxFunction{
    pub declaration:FunctionStmt,
}
impl LoxFunction{
    pub fn LoxFunction(&mut self, declaration:FunctionStmt){
        self.declaration = declaration;
    }
    pub fn call(&mut self, interpreter: &Interpreter, arguments: Vec<Literal>) -> Literal{
        let environment= Box::new(Environment::new_enclosed(&interpreter.globals));
        let i = 0;
        while i<self.declaration.parameters.len(){
            environment.define(self.declaration.parameters.get(i).lexeme, arguments(i));
            i = i + 1;
        }
        interpreter.execute_block(&self.declaration.body, environment);
        return Literal::None;
    }
    pub fn arity(&mut self) -> usize{
        return self.declaration.parameters.len();
    }
    pub fn toString(&mut self) -> String{
        return "<fn".to_string() + &self.declaration.name.lexeme + ">";
    }
}