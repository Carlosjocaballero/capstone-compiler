use crate::token::*;

pub struct Parser {
	pub tokens: Vec<Token>,
	pub current: u32
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
}