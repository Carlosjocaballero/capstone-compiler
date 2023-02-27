use std::fmt;
#[derive(Debug, PartialEq, Clone)]
pub enum Literal{
    Identifier(String),
    StringLiteral(String),
    Number(f64),
    Bool(bool),
    None
}

// Implementation to print enum value as String
impl fmt::Display for Literal{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Literal::Identifier(identifier_string) => write!(f, "{}", identifier_string),
            Literal::StringLiteral(literal_string) => write!(f, "{}", literal_string),
            Literal::Number(num) => write!(f, "{}", num),
            Literal::Bool(bool) => write!(f, "{}", bool),
            Literal::None => write!(f, "None") 
        }
    }
}

#[derive(Debug)]
pub struct Token{
    pub _type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: u32,
}

impl Token{
    pub fn to_string(&self) -> String{
        return self._type.to_string() + " " + self.lexeme.as_str() + " " + self.literal.to_string().as_str();
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TokenType{
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    
    // One or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // literals
    Identifier, String, Number,


    // keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof
}

// Turns the TokenType enum to string. use .to_string() to use method
impl fmt::Display for TokenType{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            TokenType::LeftParen => write!(f, "LeftParen"),
            TokenType::RightParen => write!(f, "RightParen"),
            TokenType::LeftBrace => write!(f, "LeftBrace"),
            TokenType::RightBrace => write!(f, "RightBrace"),
            TokenType::Comma => write!(f, "Comma"),
            TokenType::Dot => write!(f, "Dot"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::Slash => write!(f, "Slash"),
            TokenType::Star => write!(f, "Star"),
            TokenType::Bang => write!(f, "Bang"),
            TokenType::BangEqual => write!(f, "BangEqual"),
            TokenType::Equal => write!(f, "Equal"),
            TokenType::EqualEqual => write!(f, "EqualEqual"),
            TokenType::Greater => write!(f, "Greater"),
            TokenType::GreaterEqual => write!(f, "GreaterEqual"),
            TokenType::Less => write!(f, "Less"),
            TokenType::LessEqual => write!(f, "LessEqual"),
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::String => write!(f, "String"),
            TokenType::Number => write!(f, "Number"),
            TokenType::And => write!(f, "And"),
            TokenType::Class => write!(f, "Class"),
            TokenType::Else => write!(f, "Else"),
            TokenType::False => write!(f, "False"),
            TokenType::Fun => write!(f, "Fun"),
            TokenType::For => write!(f, "For"),
            TokenType::If => write!(f, "If"),
            TokenType::Nil => write!(f, "Nil"),
            TokenType::Or => write!(f, "Or"),
            TokenType::Print => write!(f, "Print"),
            TokenType::Return => write!(f, "Return"),
            TokenType::Super => write!(f, "Super"),
            TokenType::This => write!(f, "This"),
            TokenType::True => write!(f, "True"),
            TokenType::Var => write!(f, "Var"),
            TokenType::While => write!(f, "While"),
            TokenType::Eof => write!(f, "Eof"),
        }
    }
}

