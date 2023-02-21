use std::env;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::fs;

mod token;
mod scanner;
pub mod LoxError;

// use token_type::TokenType;
// use token::Token;
// use token::Literal;
use LoxError::*;
use token::*;
use crate::scanner::Scanner;

fn main() {
    let args : Vec<_> = env::args().collect();
    new(&args)
}

fn new(args: &Vec<String>) {
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        std::process::exit(64);
    }
    else if args.len() == 2{
        run_file(&args[1]);
    }
    else{
        run_prompt();
    }
}

fn run_file(path: &String){
    run(fs::read_to_string(path).expect("ERROR: Could not read file. Check directory is right or that the file is in the root folder"));
    
}

// Opens up the REPL
fn run_prompt(){
    loop{
        print!(">> ");
        io::stdout().flush().unwrap();
        let mut line: String = String::new();

        io::stdin().read_line(&mut line).expect("Could not read the line");
        run(line);
        println!("\n");
    }
}

fn run(source: String){
    let mut scanner: Scanner = Scanner{
        source: source,
        tokens: Vec::new(),
        start: 0,
        current: 0,
        line : 1,
        keywords: HashMap::new(),
        error: ScannerError { is_error: false }
    };

        
    scanner.keywords.insert(String::from("and"), TokenType::And);
    scanner.keywords.insert(String::from("class"), TokenType::Class);
    scanner.keywords.insert(String::from("else"), TokenType::Else);
    scanner.keywords.insert(String::from("false"), TokenType::False);
    scanner.keywords.insert(String::from("for"), TokenType::For);
    scanner.keywords.insert(String::from("fun"), TokenType::Fun);
    scanner.keywords.insert(String::from("if"), TokenType::If);
    scanner.keywords.insert(String::from("nil"), TokenType::Nil);
    scanner.keywords.insert(String::from("or"), TokenType::Or);
    scanner.keywords.insert(String::from("print"), TokenType::Print);
    scanner.keywords.insert(String::from("return"), TokenType::Return);
    scanner.keywords.insert(String::from("super"), TokenType::Super);
    scanner.keywords.insert(String::from("this"), TokenType::This);
    scanner.keywords.insert(String::from("true"), TokenType::True);
    scanner.keywords.insert(String::from("var"), TokenType::Var);
    scanner.keywords.insert(String::from("while"), TokenType::While);

    scanner.scan_tokens();
    println!("{:?}", scanner.tokens)
}
