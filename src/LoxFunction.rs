use crate::LoxCallable::*;
use crate::stmt::FunctionStmt;
use crate::token::*;
use crate::interpreter::*;
use crate::Environment;

pub struct LoxFunction{
    declaration: FunctionStmt
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Literal>) -> Literal {
        let environment= Box::new(Environment::new_enclosed(&interpreter.globals));
        let i = 0;
        while i < self.declaration.parameters.len() {
            environment.define(self.declaration.parameters[i].lexeme, arguments[i]);
            i = i + 1;
        }

        interpreter.execute_block(&self.declaration.body, environment);
        return Literal::None;
    }

    fn arity(&self) -> usize {
        return self.declaration.parameters.len();
    }

    fn toString(&self) -> String {
        return "<fn".to_string() + &self.declaration.name.lexeme + ">";
    }
}