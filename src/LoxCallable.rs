use crate::token::*;
use crate::interpreter::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub trait LoxCallable {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Literal>) -> Literal;
    fn arity(&self) -> usize;
    fn toString(&self) -> String;
}

impl LoxCallable for Literal {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Literal>) -> Literal {
        todo!()
    }

    fn arity(&self) -> usize {
        todo!()
    }

    fn toString(&self) -> String {
        todo!()
    }
}

impl LoxCallable for Interpreter {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Literal>) -> Literal {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as f64;
        Literal::Number(now / 1000.0)
    }

    fn arity(&self) -> usize {
        0
    }

    fn toString(&self) -> String {
        "<native fn>".to_owned()
    }
}