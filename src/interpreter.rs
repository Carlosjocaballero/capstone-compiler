// Need to do something so that I can check the type of the variable
// What's left: Runtime errors

/*
Running file errors:
For the functions that aren't in ExprVisitor... need to separate it because it's not a part of the trait

expression::* doesn't work. says: use of undeclared crate or module `expression`
 */

use std::any::{Any, TypeId}; //May not need this, may use Option<Object> instead
use std::fmt::Debug;
use std::option::Option;
use crate::scanner::Scanner;
use crate::{token::*, LoxError};
use crate::expr::*;
use crate::LoxError::*;

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
// pub enum Literal{
//     String(String),
//     Double(f64),
//     Bool(bool),
//     Literal(Literal),
//     Nil(Option<Box<Literal>>),
// }


pub struct Interpreter{}

impl Interpreter{
    pub fn interpret(&self, expression: &Box<Expr>){
        let value = self.evaluate(&expression);
        if let Ok(value) = value{
        println!("{}", self.stringify(&value))
        }
    }

    fn stringify(&self, expression: &Literal) -> String{
        if expression == &Literal::None {return "nill".to_string()};
        if let Literal::Number(_num) = expression{
            let mut text : String = expression.to_string();
            if text.ends_with(".0") {
                text = text.substring(0, text.len() - 2);
            }
            return text;
        } else{
            return expression.to_string();
        }
    }
    //IN PROGRESS
    //Need to implement a way to check the type of the operand
    fn check_number_operand(&self, _operator: &Token, _operand: &Literal){
        let mut err = ScannerError{
            is_error: false
        };
        if let Literal::Number(_x) = _operand{return;} else{ScannerError::run_time_error(&mut err, _operator, "Operand must be a number.".to_string())}
    }

    fn check_number_operands(&self, _operator: &Token, _left: &Literal, _right: &Literal){
        let mut err: ScannerError = ScannerError { is_error: false };
        if let (Literal::Number(_x), Literal::Number(_y)) = (&_left,&_right){return;} else{
            ScannerError::run_time_error(&mut err, _operator, "Operands must be numbers.".to_string());
        }
    }

    fn is_truthy(&self, object: Literal) -> bool{
        match object{
            Literal::None => false,
            Literal::Bool(x) => x,
            _ => true
        }
    }

    // CHECK IF IT WORKS
    fn is_equal(&self, a: Literal, b: Literal) -> bool{
            if a == Literal::None && b == Literal::None {return true};
            if a == Literal::None {return false};
            return a == b;    
    }   

    fn evaluate(&self, expression: &Box<Expr>) -> Result<Literal, ScannerError>{
        return expression.accept(self)
    }

}

impl ExprVisitor<Literal> for Interpreter{
    //will return the value related to the expression
    fn visit_literal_expr(&self, expression: &LiteralExpr) -> Result<Literal, ScannerError>{
        match &expression.value{
            Some(x) => Ok(x.clone()),
            None => {
                let result : Result<Literal, ScannerError> = Ok(Literal::None);
                result
            }
        }
    }

    fn visit_unary_expr(&self, expression: &UnaryExpr) -> Result<Literal, ScannerError>{
        let right = self.evaluate(&expression.right); //be Box<Expr>

        let right = match right{
            Ok(x) => x,
            Err(_) => Literal::None,
        };

        // RETURN VALUE ACCORDING TO LITERAL TYPE
        let right_num = if let Literal::Number(x) = right{x} else{ 0.0 };

        match expression.operator._type{
            TokenType::Bang => return Ok::<Literal, ScannerError>(Literal::Bool(!self.is_truthy(right))),
            TokenType::Minus => {
                self.check_number_operand(&expression.operator, &right);
                return Ok::<Literal, ScannerError>(Literal::Number(-1.0 * right_num))
            },
            _ => return Ok(Literal::None)
        };
        //Unreachable
    }

    fn visit_grouping_expr(&self, expression: &GroupingExpr) -> Result<Literal, ScannerError>{
        match self.evaluate(&expression.expression){
            Ok(x) => Ok(x),
            Err(_) => Ok(Literal::None),
        }
    }

    // TEST TO SEE IF THIS WORKS. MIGHT NOT WORK
    fn visit_binary_expr(&self, expression: &BinaryExpr) -> Result<Literal, ScannerError>{
        let left = match self.evaluate(&expression.left){
            Ok(x) => x,
            Err(_) => Literal::None
        };
        let right = match self.evaluate(&expression.right){
            Ok(x) => x,
            Err(_) => Literal::None
        };

        let left_num = if let Literal::Number(x) = left{x} else {0.0};
        let right_num = if let Literal::Number(x) = right{x} else {0.0};

        match expression.operator._type{
            TokenType::Greater => {
                self.check_number_operands(&expression.operator, &left, &right);
                return Ok(Literal::Bool(left_num > right_num))
            },
            TokenType::GreaterEqual => {
                self.check_number_operands(&expression.operator, &left, &right);
                return Ok(Literal::Bool(left_num >= right_num))
            },
            TokenType::Less => {
                self.check_number_operands(&expression.operator, &left, &right);
                return Ok(Literal::Bool(left_num < right_num))
            },
            TokenType::LessEqual => {
                self.check_number_operands(&expression.operator, &left, &right);
                return Ok(Literal::Bool(left_num <= right_num))
            },
            TokenType::BangEqual => return Ok(Literal::Bool(!self.is_equal(left, right))),
            TokenType::EqualEqual => return Ok(Literal::Bool(self.is_equal(left, right))),
            TokenType::Minus => {
                self.check_number_operands(&expression.operator, &left, &right);
                return Ok(Literal::Number(left_num - right_num))
            },
            TokenType::Plus => {
                if let (Literal::Number(x), Literal::Number(y)) = (&left, &right){
                    return Ok(Literal::Number(x + y))
                } else if let (Literal::StringLiteral(x), Literal::StringLiteral(y)) = (&left, &right){
                    return Ok(Literal::StringLiteral(format!("{}{}", x, y)))
                } else {
                    let mut err : ScannerError = ScannerError { is_error: false };
                    ScannerError::run_time_error(&mut err, &expression.operator, "Operands must be two numbers or two strings.".to_string());
                    return Err(err);
                }
            }
            TokenType::Slash => {
                self.check_number_operands(&expression.operator, &left, &right);
                return Ok(Literal::Number(left_num / right_num))
            },
            TokenType::Star => {
                self.check_number_operands(&expression.operator, &left, &right);
                return Ok(Literal::Number(left_num * right_num))
            },
            _ => return Err(ScannerError { is_error: true })
        }
    }
}

trait StringUtils{
    // Trait and implementation for a method for String that returns
    // a substring, which begins at the specified begin_index and extends
    // to the character at index end_index - 1
    fn substring(&self, begin_index: usize, end_index: usize) -> Self;
    // Gets the character in a position
    fn char_at(&mut self, index_pos: usize) -> char;
}


impl StringUtils for String{
    fn substring(&self, begin_index: usize, end_index: usize) -> Self {
        if begin_index + (end_index - begin_index) > self.len(){
            panic!("substring(): index out of bounds");
        }
        self.chars().skip(begin_index).take(end_index - begin_index).collect()
    }

    fn char_at(&mut self, index_pos: usize) -> char {
        let curr_source_char : char =  match self.chars().nth(index_pos){
            Some(x) => x,
            None => {
                println!("advance(): char not accessible by index. Returning empty space");
                ' '
            }
        };
        return curr_source_char;
    }
}