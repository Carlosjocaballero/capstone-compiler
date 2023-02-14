use std::io;
use std::io::Write;
use std::fs;

mod token_type;
mod token;

// use token_type::TokenType;
// use token::Token;
// use token::Literal;

pub struct Lox{}

impl Lox{
    pub fn new(args: &Vec<String>) {
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
    }
}

// NOTE: Unfinished. Tokenizing needed in run() function
fn run(source: String){
    println!("{}", source);
}

// fn error(line: i16, message: String){
//     report(line, "", message);
// }

// fn report(line: i16, location: &str, message: String){
//     let error: bool = true;
//     match error{
//         false => println!(""),
//         true => {
//             eprintln!("[line {}] Error {}: {}", line, location, message);
//             std::process::exit(1);
//         }
//     }
// }
