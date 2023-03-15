// Need to do something so that I can check the type of the variable
// What's left: Runtime errors

/*
Running file errors:
For the functions that aren't in ExprVisitor... need to separate it because it's not a part of the trait
expression::* doesn't work. says: use of undeclared crate or module `expression`
*/

use core::panic;
use std::any::{Any, TypeId}; use std::env;
//May not need this, may use Option<Object> instead
use std::fmt::Debug;
use std::option::Option;
use crate::scanner::Scanner;
use environment::*;
use crate::stmt::*;
use crate::{env::*, expr};
use crate::{token::*, LoxError};
use crate::expr::*;
use crate::LoxError::*;
use crate::environment;
use crate::LoxCallable::*;
use crate::LoxFunction;

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


pub struct Interpreter{
    pub globals: Box<Environment>,
    pub environment: Box<Environment>,
    pub error: InterpreterError
}

impl Interpreter{
    // needs checking for 10.2.1 
    pub fn new() -> Self {
        let mut _globals = Box::new(Environment::new());
        _globals.define("clock".to_owned(), Literal::None);
        Self { 
            globals: _globals, 
            environment: Box::new(Environment::new()), 
            error: InterpreterError { is_error: false }
        }
    }

    pub fn interpret(&mut self, statements: Vec<Box<Stmt>>){
        // let value = self.evaluate(&expression);
        // if let Ok(value) = value{
        // println!("{}", self.stringify(&value))
        // }
        for statement in statements{
            self.execute(statement)
        }
    }

    fn stringify(&self, expression: &Literal) -> String{
        if expression == &Literal::None {return "nil".to_string()};
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
    fn check_number_operand(&mut self, _operator: &Token, _operand: &Literal){
        if let Literal::Number(_x) = _operand{return;} else{self.error.run_time_error( _operator, "Operand must be a number.".to_string())}
    }

    fn check_number_operands(&mut self, _operator: &Token, _left: &Literal, _right: &Literal){
        if let (Literal::Number(_x), Literal::Number(_y)) = (&_left,&_right){return;} else{
            self.error.run_time_error(_operator, "Operands must be numbers.".to_string());
        }
    }

    fn is_truthy(&mut self, object: &Literal) -> bool{
        match object{
            Literal::None => false,
            Literal::Bool(x) => *x,
            _ => true
        }
    }

    // CHECK IF IT WORKS
    fn is_equal(&self, a: Literal, b: Literal) -> bool{
            if a == Literal::None && b == Literal::None {return true};
            if a == Literal::None {return false};
            return a == b;    
    }   

    fn evaluate(&mut self, expression: &Box<Expr>) -> Result<Literal, ScannerError>{
        return expression.accept(self)
    }

    fn execute(&mut self, mut stmt: Box<Stmt>) {
        stmt.accept(self);
    }

    pub fn execute_block(&mut self, statement: &Vec<Box<Stmt>>, environment: Box<Environment>){
        // Changed to pub
        let mut previous = self.environment.clone();

        self.environment = environment;

        for stmt in statement{
            self.execute(stmt.clone())
        }
        // println!("outter environment:");
        // previous.print_map();
        // println!("inner environment: ");
        // self.environment.print_map();

        // for (key,value) in previous.values.clone(){
        //     if self.environment.values.contains_key(&key){
        //         //println!("works!");
        //         let new_value = match self.environment.values.get(&key){
        //             Some(value) => value,
        //             None => &Literal::None
        //         };
        //         previous.values.insert(key, new_value.clone());
        //     }
        // }

        //println!("outter updated:");
        //previous.print_map();
        
        self.environment = previous;
    }

    // fn visit_block_stmt(&mut self, stmt: &Stmt::Block) -> Result<(), RuntimeError> {
    //     let environment = Environment::new_enclosed(&self.environment);
    //     self.execute_block(&stmt.statements, environment)?;
    //     Ok(())
    // }






}

impl StmtVisitor<Literal> for Interpreter{
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> Result<Literal, ScannerError> {
        self.evaluate(&stmt.expression);
        return Ok(Literal::None);
    }

    fn visit_function_stmt(&mut self, stmt: &FunctionStmt) -> Result<Literal, ScannerError> {
        let function = Box::new(stmt);
        self.environment.define(stmt.name.lexeme.clone(), function.name.literal.clone());
        Ok(Literal::None)
    }

    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Result<Literal, ScannerError> {
        let x = match self.evaluate(&stmt.condition){
            Ok(x) => x,
            Err(_) => Literal::None
        };
        if self.is_truthy(&x) {
            self.execute(stmt.then_branch.clone());
        } else if let Some(ref else_branch) = stmt.else_branch {
            self.execute(else_branch.clone());
        }
        Ok(Literal::None)    
    }


    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Result<Literal, ScannerError> {
        match self.evaluate(&stmt.expression){
            Ok(value) => println!("{}", self.stringify(&value)),
            Err(_) => ()
        }
        return Ok(Literal::None);
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> Result<Literal, ScannerError> {
        let mut value : Literal = Literal::None;
        if *stmt.initializer != Expr::Literal(LiteralExpr { value: Some(Literal::None) }){
            match self.evaluate(&stmt.initializer){
                Ok(val) => value = val,
                Err(_) => ()
            }
        }
        self.environment.define(stmt.name.lexeme.clone(), value);
        Ok(Literal::None)
    }

    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Result<Literal, ScannerError> {
        let mut new_environment = Box::new(Environment::new_enclosed(&self.environment));
        self.execute_block(&stmt.statements, new_environment);
        return Ok(Literal::None)
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Result<Literal, ScannerError> {
        let mut eval_condition = match self.evaluate(&stmt.condition){
            Ok(literal) => literal,
            Err(_) => Literal::None
        };

        //////////////////////////////////////////////////////
        ///////////////////CHECK IF LOOP WORKS////////////////
        //////////////////////////////////////////////////////
        while self.is_truthy(&eval_condition){
            self.execute(stmt.body.clone());
            eval_condition = match self.evaluate(&stmt.condition){
                Ok(literal) => literal,
                Err(_) => Literal::None
            };
        }
        return Ok(Literal::None);
    }

}

impl ExprVisitor<Literal> for Interpreter{
    //will return the value related to the expression
    fn visit_literal_expr(&mut self, expression: &LiteralExpr) -> Result<Literal, ScannerError>{
        match &expression.value{
            Some(x) => Ok(x.clone()),
            None => {
                Ok(Literal::None)
            }
        }
    }

    fn visit_unary_expr(&mut self, expression: &UnaryExpr) -> Result<Literal, ScannerError>{
        let right = self.evaluate(&expression.right); //be Box<Expr>

        let right = match right{
            Ok(x) => x,
            Err(_) => Literal::None,
        };

        // RETURN VALUE ACCORDING TO LITERAL TYPE
        let right_num = if let Literal::Number(x) = right{x} else{ 0.0 };

        match expression.operator._type{
            TokenType::Bang => return Ok::<Literal, ScannerError>(Literal::Bool(!self.is_truthy(&right))),
            TokenType::Minus => {
                self.check_number_operand(&expression.operator, &right);
                return Ok::<Literal, ScannerError>(Literal::Number(-1.0 * right_num))
            },
            _ => return Ok(Literal::None)
        };
        //Unreachable
    }

    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> Result<Literal, ScannerError>{
        //println!("interpreter:visit_variable_expr():250");
        //self.environment.print_map();
        return Ok(self.environment.get(&expr.name));
    }

    fn visit_grouping_expr(&mut self, expression: &GroupingExpr) -> Result<Literal, ScannerError>{
        match self.evaluate(&expression.expression){
            Ok(x) => Ok(x),
            Err(_) => Ok(Literal::None),
        }
    }

    // TEST TO SEE IF THIS WORKS. MIGHT NOT WORK
    fn visit_binary_expr(&mut self, expression: &BinaryExpr) -> Result<Literal, ScannerError>{
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
                    let err : ScannerError = ScannerError { is_error: false };
                    self.error.run_time_error(&expression.operator, "Operands must be two numbers or two strings.".to_string());
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

    fn visit_calling_expr(&mut self, expr: &CallingExpr) -> Result<Literal, ScannerError> {
        // the match gets the literal from the self.evaluate()
        let callee = match self.evaluate(&expr.callee) {
            Ok(literal) => literal,
            Err(_) => Literal::None
        };

        let mut arguments: Vec<Literal> = Vec::new();
        for arg in &expr.arguments{
            arguments.push(match self.evaluate(&arg){
                Ok(literal) => literal,
                Err(_) => Literal::None
            });
        }

        if arguments.len() != callee.arity() {
            return Err(ScannerError { is_error: true });
            // RuntimeError(expr.paren, "Expected " + function.arity() + " arguments but got " + arguments.size() + ".");
        };

        // returns an Ok Result with the Literal from call() 
        Ok(callee.call(self, arguments))
    }

    fn visit_clone_expr(&mut self, expr: &CloneExpr) -> Result<Literal, ScannerError> {
        todo!()
    }

    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> Result<Literal, ScannerError>{
        let value = match self.evaluate(&expr.value){
            Ok(val) => val,
            Err(_) => Literal::None
        };
        // self.environment.print_map();
        self.environment.assign(expr.name.clone(), &value);
        // self.environment.print_map();
        return Ok(value);
    }

    fn visit_logical_expr(&mut self, expr: &LogicalExpr) -> Result<Literal, ScannerError> {
        let left = match self.evaluate(&expr.left){
            Ok(literal) => literal,
            Err(_) => Literal::None
        };

        if expr.operator._type == TokenType::Or{
            if self.is_truthy(&left) {return Ok(left);}
            else {
                let rtrn = match self.evaluate(&expr.right){
                    Ok(literal) => literal,
                    Err(_) => Literal::None 
                };
                return Ok(rtrn)
            }
        } else{
            if !self.is_truthy(&left) {return Ok(left);}
            else{
                let rtrn = match self.evaluate(&expr.right){
                    Ok(literal) => literal,
                    Err(_) => Literal::None 
                };
                return Ok(rtrn)
            }
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