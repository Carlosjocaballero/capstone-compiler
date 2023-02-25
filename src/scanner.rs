use std::collections::HashMap;
use crate::token::*;
use crate::LoxError::*;

pub struct Scanner{
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: u32,
    pub keywords: HashMap<String, TokenType>,
    pub error : ScannerError,
}

impl Scanner{

    pub fn new(source : String) -> Self{
        
        let mut keywords: HashMap<String, TokenType> = HashMap::new();

        keywords.insert(String::from("and"), TokenType::And);
        keywords.insert(String::from("class"), TokenType::Class);
        keywords.insert(String::from("else"), TokenType::Else);
        keywords.insert(String::from("false"), TokenType::False);
        keywords.insert(String::from("for"), TokenType::For);
        keywords.insert(String::from("fun"), TokenType::Fun);
        keywords.insert(String::from("if"), TokenType::If);
        keywords.insert(String::from("nil"), TokenType::Nil);
        keywords.insert(String::from("or"), TokenType::Or);
        keywords.insert(String::from("print"), TokenType::Print);
        keywords.insert(String::from("return"), TokenType::Return);
        keywords.insert(String::from("super"), TokenType::Super);
        keywords.insert(String::from("this"), TokenType::This);
        keywords.insert(String::from("true"), TokenType::True);
        keywords.insert(String::from("var"), TokenType::Var);
        keywords.insert(String::from("while"), TokenType::While);

        Scanner { 
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: keywords,
            error: ScannerError { is_error: false } 
        }
        
    }

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
        if self.error.is_error {std::process::exit(65);}
    }

    fn scan_token(&mut self){
        let c : char = self.advance();
        match c{
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' =>{
                match self.token_match('='){
                    true => self.add_token(TokenType::BangEqual),
                    false => self.add_token(TokenType::Bang)
                }
            },
            '=' =>{
                match self.token_match('='){
                    true => self.add_token(TokenType::EqualEqual),
                    false => self.add_token(TokenType::Equal),
                }
            },
            '<' =>{
                match self.token_match('=') {
                    true => self.add_token(TokenType::LessEqual),
                    false => self.add_token(TokenType::Less),
                }
            },
            '>' =>{
                match self.token_match('=') {
                    true => self.add_token(TokenType::GreaterEqual),
                    false => self.add_token(TokenType::Greater),
                }
            },
            '/' =>{
                match self.token_match('/') {
                    // if it's a comment, keep peeking forward until the end of line
                    // ignore everything between the // and \n 
                    true => while self.peek() != '\n' && !self.is_at_end(){
                        self.advance();
                    },
                    false => self.add_token(TokenType::Slash),
                }
            },
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if self.is_digit(c){
                    self.number();
                }
                else if self.is_alpha(c){
                    self.identifier();
                }
                else{
                    self.error._error(self.line, "Unexpected Character".to_string())
                }
            }
        }
    }

    fn identifier(&mut self){
        let  mut peek: char = self.peek(); 
        while self.is_alphanumeric(peek) {self.advance(); peek = self.peek()}

        let text: String = self.source.substring(self.start, self.current);
        let _type : TokenType = match self.keywords.get(&text){
            Some(x) => *x,
            None => TokenType::Identifier
        };
        self.add_token_literal(_type, Literal::Identifier(text));
    }

    fn is_alphanumeric(&self, c: char) -> bool{
        return self.is_alpha(c) || self.is_digit(c);
    }

    fn is_alpha(&self, c: char) -> bool{
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn is_digit(&self, c: char) -> bool{
        return c >= '0' && c <= '9';
    }

    fn number(&mut self){
        let mut peek: char = self.peek();
        while self.is_digit(peek) {self.advance(); peek = self.peek();}

        // look for fraction (if exist)
        let peek_next: char = self.peek_next();
        if self.peek() == '.' && self.is_digit(peek_next){
            // consume the "."
            self.advance();

            peek = self.peek();
            while self.is_digit(peek) {self.advance(); peek = self.peek();}
        }

        let token_num : f64 = self.source.substring(self.start, self.current).parse().unwrap();
        self.add_token_literal(TokenType::Number, Literal::Number(token_num));

    }

    fn peek_next(&mut self) -> char{
        if self.current + 1 >= self.source.len() {return '\0';}
        return self.source.char_at(self.current + 1);
    }

    fn string(&mut self){
        while self.peek() != '"' && !self.is_at_end(){
            if self.peek() == '\n' {self.line+=1};
            self.advance();
        }

        if self.is_at_end(){
            // Error supposed to be called. Create panic for now
            self.error._error(self.line, "Underminated String".to_string());
            return;  
        }

        // advance() once more for closing "
        self.advance();

        let value : String = self.source.substring(self.start+1, self.current-1);

        self.add_token_literal(TokenType::String, Literal::StringLiteral(value));

    }

    fn peek(&mut self) -> char{
        if self.is_at_end() {return '\0'};
        return self.source.char_at(self.current);
    }

    fn token_match(&mut self, expected: char) -> bool{
        if self.is_at_end() {return false}        
        let curr_source_char : char = self.source.char_at(self.current);
        if curr_source_char != expected{
            return false;
        } 
        self.current += 1;
        return true;
    }

    fn advance(&mut self) -> char{
        let curr_source_char: char = self.source.char_at(self.current);
        self.current += 1;
        return curr_source_char;
    }

    fn add_token(&mut self, _type : TokenType){
        self.add_token_literal(_type, Literal::None);
    }

    fn add_token_literal(&mut self, _type : TokenType, literal: Literal){
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