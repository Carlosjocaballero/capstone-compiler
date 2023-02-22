//Need to do something so that I can check the type of the variable
//What's left: Runtime errors

use std::any::{Any, TypeId}; //May not need this, may use Option<Object> instead
use std::fmt::Debug;

mod token;
use token::*;

mod expr;
use expr::*;

/*
Helper for Plus operator
Adds left and right based off if they're strings or float
 */
trait plus{
    fn add(&self);
}

impl plus for String{
    fn add(&self, left: String, right: String){
        left + right
    }
}

impl plus for f64{
    fn add(&self, left: f64, right: f64){
        left + right
    }
}
struct Interpreter;

impl ExprVisitor for Interpreter{
    fn visit_literal_expr(expression: &Expr::Literal) -> Option<Object>{
        expression::value
    }

    fn visit_unary_expr(expression: &Expr::Unary) -> Option<Object>{
        let right: Option<Object> = evaluate(expression::right);

        match expressions::operator::_type{
            TokenType::Bang => !isTruthy(right),
            TokenType::Minus =>{
                checkNumberOperand(expression::operator, right);
                -(right as f64)
            }
        }
        //Unreachable
        None
    }

    //IN PROGRESS
    //Need to implement a way to check the type of the operand
    fn checkNumberOperand(operator: Token, operand: Option<Object>){

    }

    fn isTruthy(object: Option<Object>) -> Boolean{
        match object{
            None => false,
            Some(true) => true,
            Some(false) => false,
            _ => true,
        }
    }

    fn isEqual(a: Option<Object>, b: Option<Object>) -> Boolean{
        match a{
            None => match b{
                None => true,
                _ => false,
            }
            _ => a.eq(&b)
        }
    }
    fn visitGroupingExpr(expression: &Expr::Grouping) -> Any{
        evaluate(expression::expression)
    }

    fn evaluate(expression: &Expr) -> Any{
        expression::accept(expression)
    }

    fn visit_binary_expr(expression: &Expr::Binary) -> Option<Object>{
        let left = evaluate(expression::left);
        let right = evaluate(expression::right);

        match expression::operator::_type{
            TokenType::Greater => {
                (left as f64) > (right as f64)
            },
            TokenType::GreaterEqual => {
                (left as f64) >= (right as f64)
            },
            TokenType::Less => {
                (left as f64) < (right as f64)
            },
            TokenType::LessEqual => {
                (left as f64) <= (right as f64)
            },
            TokenType::BangEqual => {
                !isEqual(left, right)
            },
            TokenType::EqualEqual => {
                isEqual(left, right)
            }
            TokenType::Minus => {
                (left as f64) - (right as f64)
            },
            TokenType::Plus =>{
                add(left, right)
            }
            TokenType::Slash =>{
                (left as f64) / (right as f64)
            },
            TokenType::Star => {
                (left as f64) * (right as f64)
            },
        }
        //Unreachable
        None
    }
}