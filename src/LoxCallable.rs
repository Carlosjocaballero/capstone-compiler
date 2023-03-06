use crate::token::*;
use crate::expr::*;
use crate::LoxError::*;

pub struct LoxCallable{

}
impl LoxCallable{
    pub fn call(interpreter:Interpreter, arguments:Vec<object::Literal>);
    pub fn arity()->Number;
}