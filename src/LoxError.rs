use std::{error::Error, iter::Scan};

use crate::{token:: *, scanner::Scanner};

pub trait ErrorTools{
    fn report(&mut self, line: u32, _where: String, message:String);
    fn _error(&mut self, line: u32, message:String);
}

impl ErrorTools for ScannerError{
    fn report(&mut self, line: u32, _where: String, message:String) {
        println!("[line {}] Error {}: {}", line, _where, message);
        self.is_error = true;
    }

    fn _error(&mut self, line: u32, message:String) {
        self.report(line, "".to_string(), message) 
    }
}

impl ErrorTools for ParseError{
    fn report(&mut self, line: u32, _where: String, message:String) {
        println!("[line {}] Error {}: {}", line, _where, message);
        self.is_error = true;
    }

    fn _error(&mut self, line: u32, message:String) {
        self.report(line, "".to_string(), message)
    }
}



pub struct ScannerError{
    pub is_error: bool
}

pub struct ParseError{
    pub is_error: bool
}

impl ParseError{
    pub fn error(&mut self, token: &Token, message: String){
        if token._type == TokenType::Eof{
            self.report(token.line, " at end ".to_string(), message)
        } else{
            self.report(token.line, "at '".to_string() + &token.lexeme + "'", message);
        }
    }
}

pub struct InterpreterError{
    pub is_error: bool
}

impl InterpreterError{
    pub fn run_time_error(&mut self, operator: &Token, message: String){
        println!("{} \n[line {}]", message, operator.line);
        self.is_error = true;
    }
}
