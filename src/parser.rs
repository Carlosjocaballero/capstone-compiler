use crate::token::*;
use crate::expr::*;

pub struct Parser {
	pub tokens: Vec<Token>,
	pub current: u32,
	pub extends: ParseError // Edit 3.
}

impl Parser {
	fn expression(&self) -> Box<Expr> {
		self.equality()
	}

	fn equality(&self) -> Box<Expr> {
		let _expr: Expr = self.comparison();
		let mut binaryExpr: BinaryExpr;
		while self.matching(&vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
			let _operator: Token = self.previous();
			let _right: Expr = self.comparison();
			// expr = new Expr.Binary(expr, operator, right); // FIX NEW
			binaryExpr = BinaryExpr {
				 left: _expr,
				 operator: _operator,
				 right: _right
			};
		}
		binaryExpr
	}

	fn matching(&self, tokenTypes: &Vec<TokenType>) -> bool {	// FIX FOR MULTIPLE 
		for tokenType in tokenTypes {
			if (self.check(tokenType)) {
				self.advance();
				true
			}
		}
		false
	}
	fn consume(&self, tokenTypes: TokenType, message:&str) { // Edit
		for tokenType in tokenTypes{
			if check(tokenType) { return advance()};
		}
		throw; error(peek(), message);
	}

	fn check(&self, tokenType: TokenType) -> bool {
		if (isAtEnd()) { false }
		let temp_token = peek();
		return temp_token.TokenType == tokenType;
	}

	fn advance(&mut self) -> Token {
		if (!isAtEnd()) { current += 1; }
		previous()
	}

	fn isAtEnd(&self) -> bool {
		let temp_token = peek();
		return temp_token.TokenType == TokenEOF;
	}

	fn peek(&self) -> Token {
		tokens.get(current)
	}

	fn previous(&self) -> Token {
		tokens.get(current - 1)
	}
	fn error(tokenType: TokenType, message:&str)->ParseError {
		Lox.error(token, message);
		return new ParseError();
	}
	fn error(tokenType: TokenType, message:&str) {// Edit 2
		for tokenType in tokenTypes{
			let temp_token = tokens;
			if (temp_token == tokenType.EOF) {
		  	report(token.line, " at end", message);
		} else {
		  report(token.line, " at '" + token.lexeme + "'", message);
		}
		}
	}
	  fn synchronize() { // edit 4.
		advance();
	
		while (!isAtEnd()) {
		let temp_token = previous();
		  if (temp_token == SEMICOLON){return};
		  match (peek()) {
			peek()=>CLASS,
			peek()=>FUN,
			peek()=>VAR,
			peek()=>FOR,
			peek()=>IF,
			peek()=>WHILE,
			peek()=>PRINT,
			peek()=>RETURN,
			_=>return,
		  };
		  advance();
		}
	  }
}