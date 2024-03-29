use std::fs::File;
use std::io::{self, Write};

#[derive(Debug)]
struct TreeType {
    base_class_name: String,
    class_name: String,
    fields: Vec<String>,
}

pub fn generate_ast(output_dir: &String) -> io::Result<()> {
    define_ast(output_dir, &"Expr".to_string(), &vec![
        "Assign   : Token name, Box<Expr> value".to_string(),
        "Binary   : Box<Expr> left, Token operator, Box<Expr> right".to_string(),
        "Call     : Box<Expr> callee, Token paren, Box<Expr> arguments".to_string(),
        "Grouping : Box<Expr> expression".to_string(),
        "Literal  : Option<Literal> value".to_string(),
        "Logical  : Box<Expr> left, Token operator, Box<Expr> right".to_string(),
        "Unary    : Token operator, Box<Expr> right".to_string(),
        "Clone    : Box<Expr> clone".to_string(),
    ])?;

    define_ast(output_dir, &"Stmt".to_string(), &vec![
        "Expression : Box<Expr> expression".to_string(),
        "Function   : Token name, Vec<Token> params," + " Vec<Stmt> body", // needs checking
        "If         : Box<Expr> expression, Box<Smt> then_branch, Option<Box<Stmt>> else_branch".to_string(),
        "Print      : Box<Expr> expression".to_string(),
        "Return     : Token keyword, Expr value",
        "Var        : Token name, Expr initializer",
    ])?;
    Ok(())
}

fn define_ast(output_dir: &String, base_name: &String, types: &[String]) -> io::Result<()> {
    let path = format!("{output_dir}/{}.rs", base_name.to_lowercase());
    let mut file = File::create(path)?;
    let mut tree_types = Vec::new();

    write!(file, "{}", "use crate::LoxError::*;\n")?;
    write!(file, "{}", "use crate::token::*;\n")?;

    for treetype in types {
        let (base_class_name, args) = treetype.split_once(":").unwrap();
        let class_name = format!("{}{}", base_class_name.trim(), base_name); //binary + expr for exampel
        let arg_split = args.split(",");
        let mut fields = Vec::new();
        for arg in arg_split {
            let (t2type, name) = arg.trim().split_once(" ").unwrap(); //flip the order so that it prints letf then box
            fields.push(format!("{}: {}", name, t2type));
        }
        tree_types.push(TreeType {
            base_class_name: base_class_name.trim().to_string(), 
            class_name, 
            fields });
    }

    write!(file, "\npub enum {base_name} {{\n")?;
    for t in &tree_types {
        write!(file, "    {}({}),\n", t.base_class_name, t.class_name)?;
    }
    write!(file, "}}\n\n")?;

    write!(file, "impl {} {{\n", base_name)?;
    write!(file, "    pub fn accept<T>(&self, {}_visitor: &dyn {base_name}Visitor<T>) -> Result<T, ScannerError> {{\n", base_name.to_lowercase())?;
    write!(file, "        match self {{\n")?;
    for t in &tree_types {
        write!(file, "            {}::{}(v) => v.accept({}_visitor), \n", base_name, t.base_class_name, base_name.to_lowercase())?;
    }
    write!(file, "        }}\n")?;
    write!(file, "    }}\n")?;
    write!(file, "}}\n\n")?;

    for t in &tree_types {
        write!(file, "pub struct {} {{\n", t.class_name)?;
        for f in &t.fields {
            write!(file, "    pub {},\n", f)?;
        }
        write!(file, "}}\n\n")?;
    }

    write!(file, "pub trait ExprVisitor<T> {{\n")?;
    for t in &tree_types {
        write!(file, "    fn visit_{}_{}(&self, expr: &{}) -> Result<T, ScannerError>;\n",
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase(),
            t.class_name)?;
    }
    write!(file, "}}\n\n")?;

    for t in tree_types {
        write!(file, "impl {} {{\n", t.class_name)?;
        write!(file, 
            "    pub fn accept<T>(&self, visitor: &dyn {}Visitor<T>) -> Result<T, ScannerError> {{\n", 
            base_name)?;
        write!(file, "        visitor.visit_{}_{}(self)\n", t.base_class_name.to_lowercase(),
            base_name.to_lowercase())?;
        write!(file, "    }}\n")?;
        write!(file, "}}\n\n")?;
    }

    Ok(())
}