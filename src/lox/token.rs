use crate::lox::token_type::TokenType;
use std::fmt;
pub enum Literal{
    Identifier(String),
    StringLiteral(String),
    Number(f64),
}

impl fmt::Display for Literal{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Literal::Identifier(String) => write!(f, "Identifier"),
            Literal::StringLiteral(String) => write!(f, "StringLiteral"),
            Literal::Number(f64) => write!(f, "Number"),
        }
    }
}

pub struct Token{
    pub _type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: i64,
}

impl Token{
    pub fn toString(&self) -> String{
        return self._type.to_string() + " " + self.lexeme.as_str() + " " + self.literal.to_string().as_str();
    }
}

