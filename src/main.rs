use std::env;
use std::io;
use std::io::Write;
use std::fs;

mod token;
mod interpreter;
mod scanner;
mod expr;
mod stmt;
mod generate_ast;
mod ast_printer;
mod parser;
mod environment;
pub mod LoxError;


use LoxError::*;

use crate::environment::Environment;
use crate::scanner::Scanner;
use crate::stmt::Stmt;
use parser::*;

fn main(){
    let args : Vec<_> = env::args().collect();
    new(&args);
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
    // generate_ast(&"src".to_string());
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
    let mut scanner: Scanner = Scanner::new(source);
    
    scanner.scan_tokens();
    // println!("{:?}", scanner.tokens);

    let mut parser = Parser{
        tokens: scanner.tokens,
        current: 0,
        parser_error: ParseError { is_error: false }
    };

    // let expression: Box<Expr> = parser.parse();

    let statements: Vec<Box<Stmt>> = parser.parse();

    // println!("{:?}", statements);
    
    // let mut printer= crate::ast_printer::AstPrinter{};

    // let tree_string: String = match printer.print(&expression){
    //     Ok(x) => x,
    //     _ => String::from("Coudn't print tree")
    // };

    // println!("{}", tree_string);


    let mut interpreter = interpreter::Interpreter{
        environment: Environment::new(),
        error: InterpreterError { is_error: false }
    };
    interpreter.interpret(statements);
    if interpreter.error.is_error == true {std::process::exit(70);}

    // //----------- Expr for TESTING PURPOSES --------------
    // //generate_ast(&"src".to_string());
    // let expression: Expr = Expr::Binary(
    //     BinaryExpr{
    //         left: Box::new(Expr::Unary(
    //             UnaryExpr{
    //                 operator: Token { 
    //                     _type: TokenType::Minus,
    //                     lexeme: "-".to_string(), 
    //                     literal: Literal::None,
    //                     line: 1 
    //                 },
    //                 right: Box::new(Expr::Literal(
    //                     LiteralExpr{ value : Some(Literal::Number(123.0))}
    //                 ))  
    //             }
    //         )),
    //         operator: Token { 
    //             _type: TokenType::Star,
    //             lexeme: "*".to_string(),
    //             literal: Literal::None,
    //             line: 1
    //         },
    //         right: Box::new(Expr::Grouping(
    //             expr::GroupingExpr { 
    //                 expression: Box::new(Expr::Literal(LiteralExpr {value: Some(Literal::Number(45.67))}))
    //             }
    //         ))
    //     }
    // );

    // let printer= crate::ast_printer::AstPrinter{};

    // let tree_string: String = match printer.print(&expression){
    //     Ok(x) => x,
    //     _ => String::from("Coudn't print tree")
    // };

    // println!("{}", tree_string);

      
}
