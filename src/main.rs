use std::io;
mod generate_ast;
mod ast_printer;
use generate_ast::*;

fn main() -> io::Result<()> {
    generate_ast(&"src".to_string())
}