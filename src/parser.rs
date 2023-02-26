use crate::token::*;

pub struct Parser {
	pub tokens: Vec<Token>,
	pub current: u32,
	pub extends:ParseError // Edit 3.
}

impl Parser {
	fn expression(&self) -> Expr {
		equality()
	}

	fn equality(&self) -> Expr {
		let expr: Expr = comparison();
		while (matching(BANG_EQUAL, EQUAL_EQUAL)) {
			let operator: Token = previous();
			let right: Expr = comparison();
			expr = new Expr.Binary(expr, operator, right); // FIX NEW
		}
		expr
	}

	fn matching(&self, tokenTypes: TokenType) -> boolean {	// FIX FOR MULTIPLE 
		for tokenType in tokenTypes {
			if (check(tokenType)) {
				advance();
				true
			}
		}
		false
	}
	fn consume(&self, tokenTypes: TokenType, message:&str) { // Edit
		for tokenType in tokenTypes{
			if (check(tokenType)){ return advance()};
		}
		throw; error(peek(), message);
	}

	fn check(&self, tokenType: TokenType) -> boolean {
		if (isAtEnd()) { false }
		let temp_token = peek();
		return temp_token.TokenType == tokenType;
	}

	fn advance(&mut self) -> Token {
		if (!isAtEnd()) { current += 1; }
		previous()
	}

	fn isAtEnd(&self) -> boolean {
		let temp_token = peek();
		return temp_token.TokenType == EOF;
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
	
		  switch (peek()) {
			CLASS;
			FUN;
			VAR;
			FOR;
			IF;
			WHILE;
			PRINT;
			RETURN;
			return;
		  }
	
		  advance();
		}
	  }
}