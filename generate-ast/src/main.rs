use std::env::args;
use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        eprintln!("Usage: generate-ast <output directory>");
        std::process::exit(64);
    }

    let output_dir = args.get(1).unwrap().to_string();

    define_ast(output_dir, "Expr".to_string(), &vec![
        "Binary   : Expr left, token operator, Expr right".to_string(),
        "Grouping : Expr expression".to_string(),
        "Literal  : Object value".to_string(),
        "Unary    : Token operator, Expr right".to_string(),
    ])?;
    Ok(())
}

fn define_ast(output_dir: String, base_name: String, types: &[String]) -> io::Result<()> {
    let path = format!("{output_dir}/{}.rs", base_name.to_lowercase());
    let mut file = File::create(path)?;

    write!(file, "{}", "use crate::error::*;\n")?;
    write!(file, "{}", "use crate::token::*;\n")?;

    Ok(())
}