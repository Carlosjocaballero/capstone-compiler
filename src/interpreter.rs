//Need to do something so that I can check the type of the variable
//What's left: Runtime errors

/*
Running file errors:
For the functions that aren't in ExprVisitor... need to separate it because it's not a part of the trait

expression::* doesn't work. says: use of undeclared crate or module `expression`
 */

use std::any::{Any, TypeId}; //May not need this, may use Option<Object> instead
use std::fmt::Debug;
use std::option::Option;

mod token;
use token::*;

mod expr;
use expr::*;

/*
The value can either be from the enum Literal (which is in token.rs) -> string, f64

the node you get as a result of using explicit parentheses in an expression -> expression goes to evaluate -> 
goes through accept -> returns Result<T, LoxError>

for unary -> will need to return boolean or f64

for binary -> will all be f64

SO:
Need a value for:
String
f64
Boolean

Most vague one is for grouping expressions:
That's for expressions in parentheses.
Will that recrsively go through itself to figure out when it reaches the end of the parthenses

Professor says can eval to StringLiteral, float, bool, nil
 */
pub enum Value{
    String(String),
    Double(f64),
    Bool(bool),
    Literal(Literal),
    Nil(Option<Box<Value>>),
}
/*
Helper for Plus operator
Adds left and right based off if they're strings or float
 */
trait Plus{
    fn add(&self, left: Value, right: Value);
}

impl Plus for String{
    fn add(&self, left: String, right: String){
        left + &right
    }
}

impl Plus for f64{
    fn add(&self, left: f64, right: f64){
        left + &right
    }
}
struct Interpreter;

impl Interpreter{
    //IN PROGRESS
    //Need to implement a way to check the type of the operand
    fn checkNumberOperand(&self, operator: Token, operand: Value){

    }

    fn isTruthy(&self, object: Value) -> bool{
        match object{
            Value::Nil(None) => false,
            Value::Bool(object) => object,
            _ => true,
        }
    }

    fn isEqual(&self, a: Value, b: Value) -> bool{
        match a{
            Value::Nil(None) => match b{
                Value::Nil(None) => true,
                _ => false,
            }
            _ => a.eq(&b)
        }
    }

    fn evaluate(&self, expression: &Expr) -> Value{
        expression::accept(expression)
    }

}

impl ExprVisitor<Value> for Interpreter{
    //will return the value related to the expression
    fn visit_literal_expr(&self, expression: Expr::Literal) -> Value{
        expression::value
    }

    fn visit_unary_expr(&self, expression: Expr::Unary) -> Value{
        let right: Value = evaluate(expression::right); //be Box<Expr>

        match expression::operator::_type{
            TokenType::Bang => !isTruthy(right), //returns boolean
            TokenType::Minus =>{
                checkNumberOperand(expression::operator, right);
                -(right as f64) //returns f64
            }
        }
        //Unreachable
        Value::Nil(None)
    }

    fn visit_grouping_expr(&self, expression: Expr::Grouping) -> Value{
        evaluate(expression::expression)
    }

    fn visit_binary_expr(&self, expression: Expr::Binary) -> Value{
        let left = evaluate(expression::left);
        let right = evaluate(expression::right);

        match expression::operator::_type{
            TokenType::Greater => {
                return Value::Bool((left as f64) > (right as f64));
            },
            TokenType::GreaterEqual => {
                return Value::Bool((left as f64) >= (right as f64));
            },
            TokenType::Less => {
                return Value::Bool((left as f64) < (right as f64));
            },
            TokenType::LessEqual => {
                return Value::Bool((left as f64) <= (right as f64));
            },
            TokenType::BangEqual => {
                !isEqual(left, right)
            },
            TokenType::EqualEqual => {
                isEqual(left, right)
            }
            TokenType::Minus => {
                return Value::Double((left as f64) - (right as f64));
            },
            TokenType::Plus =>{
                add(left, right)
            }
            TokenType::Slash =>{
                return Value::Double((left as f64) / (right as f64));
            },
            TokenType::Star => {
                return Value::Double((left as f64) * (right as f64));
            },
        }
        //Unreachable
        Value::Nil(None)
    }
}



fn main(){
    println!("Testing interpreters.rs");
}