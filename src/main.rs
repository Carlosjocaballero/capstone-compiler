use std::env;

// NOTE: This should be part of a struct. It was made this way for testing purposes

mod lox;
use lox::Lox;
use lox::error::*;

fn main() {
    let args : Vec<_> = env::args().collect();
    let _lox: () = Lox::new(&args);
}