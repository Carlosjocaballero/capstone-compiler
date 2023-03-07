use crate::token::*;
use crate::expr::*;
use crate::interpreter::*;
use crate::LoxError::*;

pub struct LoxCallable {

}

impl LoxCallable {
    pub fn call(interpreter: Interpreter, arguments: Vec<Literal>) -> Literal {
        todo!()
    }

    pub fn arity() -> Literal {
        todo!()
    }
}