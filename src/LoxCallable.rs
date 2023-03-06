use crate::token::*;
use crate::expr::*;
use crate::LoxError::*;

pub struct LoxCallable{

}
impl LoxCallable{
    pub fn call(Interpreter:Interpreter, arguments:Vec<object::Literal>);
}