use std::io;
use std::fs;
use std::io::Write;
use std::env;

fn main() {
    let args : Vec<_> = env::args().collect();

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

fn run_prompt(){
    loop{
        print!(">> ");
        io::stdout().flush().unwrap();
        let mut line: String = String::new();

        io::stdin().read_line(&mut line).expect("Could not read the line");
        run(line);
    }
}

fn run(source: String){
    println!("{}", source);
}


