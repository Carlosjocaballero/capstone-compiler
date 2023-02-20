use std::env::args;
use std::fs::File;
use std::io::{self, Write};

#[derive(Debug)]
struct TreeType {
    //base_name: String,
    class_name: String,
    fields: Vec<String>,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        eprintln!("Usage: generate-ast <output directory>");
        std::process::exit(64);
    }

    let output_dir = args.get(1).unwrap().to_string();

    define_ast(&output_dir, &"Expr".to_string(), &vec![
        "Binary   : Box<Expr> left, token operator, Box<Expr> right".to_string(),
        "Grouping : Box<Expr> expression".to_string(),
        "Literal  : Object value".to_string(),
        "Unary    : Token operator, Box<Expr> right".to_string(),
    ])?;
    Ok(())
}

fn define_ast(output_dir: &String, base_name: &String, types: &[String]) -> io::Result<()> {
    let path = format!("{output_dir}/{}.rs", base_name.to_lowercase());
    let mut file = File::create(path)?;
    let mut tree_types = Vec::new();

    write!(file, "{}", "use crate::error::*;\n")?;
    write!(file, "{}", "use crate::token::*;\n")?;

    for treetype in types {
        let (base_class_name, args) = treetype.split_once(":").unwrap();
        let class_name = format!("{}{}", base_class_name.trim(), base_name); //binary + expr for exampel
        let arg_split = args.split(",");
        let mut fields = Vec::new();
        for arg in arg_split {
            let (t2type, name) = arg.trim().split_once(" ").unwrap(); //flip the order so that it prints letf then box
            fields.push(format!("'{}': '{}'", name, t2type));
        }
        tree_types.push(TreeType {
            //base_name, 
            class_name, 
            fields });
    }
    print!("{:?}", tree_types);

    Ok(())
}