use crate::token::*;
use crate::expr::*;
use crate::interpreter::*;

pub trait LoxCallable {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Literal>) -> Literal;
    fn arity(&self) -> usize;
}

impl LoxCallable for Literal {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Literal>) -> Literal {
        todo!()
    }

    fn arity(&self) -> usize {
        todo!()
    }
}