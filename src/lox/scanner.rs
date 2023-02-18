use std::{collections::HashMap, vec};
use crate::lox::token::*;

pub struct Scanner{
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: i32
}

impl Scanner{
    fn is_at_end(&self) -> bool{
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self){
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        // Adds an end of line token
        let new_token: Token = Token{
            _type: TokenType::Eof,
            lexeme: "".to_string(),
            line: self.line,
            literal: Literal::None
        };
        self.tokens.push(new_token);
    }

    fn scan_token(&mut self){
        let c : char = self.advance();
        match c{
            '(' => self.add_token_token(TokenType::LeftParen),
            ')' => self.add_token_token(TokenType::RightParen),
            '{' => self.add_token_token(TokenType::LeftBrace),
            '}' => self.add_token_token(TokenType::RightBrace),
            ',' => self.add_token_token(TokenType::Comma),
            '.' => self.add_token_token(TokenType::Dot),
            '-' => self.add_token_token(TokenType::Minus),
            '+' => self.add_token_token(TokenType::Plus),
            ';' => self.add_token_token(TokenType::Semicolon),
            '*' => self.add_token_token(TokenType::Star),
            '!' =>{
                match self.token_match('='){
                    true => self.add_token_token(TokenType::BangEqual),
                    false => self.add_token_token(TokenType::Bang)
                }
            },
            '=' =>{
                match self.token_match('='){
                    true => self.add_token_token(TokenType::EqualEqual),
                    false => self.add_token_token(TokenType::Equal),
                }
            },
            '<' =>{
                match self.token_match('=') {
                    true => self.add_token_token(TokenType::LessEqual),
                    false => self.add_token_token(TokenType::Less),
                }
            },
            '>' =>{
                match self.token_match('=') {
                    true => self.add_token_token(TokenType::GreaterEqual),
                    false => self.add_token_token(TokenType::GreaterEqual),
                }
            },
            _ => {()}
        }
    }

    fn token_match(&mut self, expected: char) -> bool{
        if self.is_at_end() {return false}        
        let curr_source_char : char = match self.source.chars().nth(self.current){
            Some(x) => x,
            None => {
                println!("token_match(expected: char): char not accessible by index. Returning empty space");
                ' '
            }
        };
        if curr_source_char != expected{
            return false;
        } 
        self.current += 1;
        return true;
    }

    fn advance(&mut self) -> char{
        let curr_source_char: char = match self.source.chars().nth(self.current){
            Some(x) => x,
            None => {
                println!("advance(): char not accessible by index. Returning empty space");
                ' '
            }
        };
        self.current += 1;
        return curr_source_char;
    }

    fn add_token_token(&mut self, _type : TokenType){
        self.add_token_token_literal(_type, Literal::None);
    }

    fn add_token_token_literal(&mut self, _type : TokenType, literal: Literal){
        let _text : String =  self.source.substring(self.start, self.current);
        let new_token: Token = Token{
            _type: _type,
            lexeme: _text,
            literal: literal,
            line: self.line
        };
        self.tokens.push(new_token);
    }
}

// Trait and implementation for a method for String that returns
// a substring, which begins at the specified begin_index and extends
// to the character at index end_index - 1
trait StringUtils{
    fn substring(&self, begin_index: usize, end_index: usize) -> Self;
}

impl StringUtils for String{
    fn substring(&self, begin_index: usize, end_index: usize) -> Self {
        if begin_index + (end_index - begin_index) > self.len(){
            panic!("substring(): index out of bounds");
        }
        self.chars().skip(begin_index).take(end_index - begin_index).collect()
    }
}