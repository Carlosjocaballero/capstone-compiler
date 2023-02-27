use crate::token::*;
use crate::expr::*;

pub struct Parser {
	pub tokens: Vec<Token>,
	pub current: usize,
	// pub extends: ParseError // Edit 3.
}

impl Parser {
	fn expression(&self) -> Box<Expr> {
		self.equality()
	}

	fn equality(&self) -> Box<Expr> {
		let _expr: Expr = self.comparison();
		let mut binaryExpr: BinaryExpr;
		while self.matching(&vec![TokenType::BangEqual, TokenType::EqualEqual]) {
			let _operator: Token = self.previous();
			let _right: Expr = self.comparison();
			// expr = new Expr.Binary(expr, operator, right); 
			binaryExpr = BinaryExpr {
				left: _expr,
				operator: _operator,
				right: _right
			};
		}
		binaryExpr
	}

	fn matching(&self, tokenTypes: Vec<TokenType>) -> bool { 
		for tokenType in tokenTypes {
			if self.check(tokenType) {
				self.advance();
				true
			}
		}
		false
	}
	
	fn consume(&self, tokenType: TokenType, message:&str) -> Token { 
		// for tokenType in tokenTypes {
		if self.check(tokenType) { self.advance() }
		// }
		else { self.advance() }		// placeholder return, need to return error
		// throw!(self.error(self.peek(), message));
	}

	fn check(&self, tokenType: TokenType) -> bool {
		if self.isAtEnd() { 
			false 
		} else {
			let temp_token = self.peek();
			return true; // temp_token == tokenType;
		}
	}

	fn advance(&mut self) -> Token {
		if !self.isAtEnd() { self.current += 1; }
		self.previous()
	}

	fn isAtEnd(&self) -> bool {
		let temp_token = self.peek();
		return true; // temp_token == TokenType::Eof;
	}

	fn peek(&self) -> Token {
		self.tokens[self.current]
	}

	fn previous(&self) -> Token {
		self.tokens[self.current - 1]
	}

	fn comparison(&self) -> Expr {
		let _expr: Expr = self.term();
		let mut binaryExpr: BinaryExpr;
		while self.matching(&vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
			let _operator: Token = self.previous();
			let _right: Expr = self.term();
			binaryExpr = BinaryExpr {
				left: _expr,
				operator: _operator,
				right: _right
			};
		}
		binaryExpr
	}

	fn term(&self) -> Expr {
		let _expr: Expr = self.factor();
		let mut binaryExpr: BinaryExpr;
		while self.matching(&vec![TokenType::Minus, TokenType::Plus]) {
			let _operator: Token = self.previous();
			let _right: Expr = self.factor();
			binaryExpr = BinaryExpr {
				left: _expr,
				operator: _operator,
				right: _right
			};
		}
		binaryExpr
	}

	fn factor(&self) -> Expr {
		let _expr: Expr = self.unary();
		let mut binaryExpr: BinaryExpr;
		while self.matching(&vec![TokenType::Slash, TokenType::Star]) {
			let _operator: Token = self.previous();
			let _right: Expr = self.unary();
			binaryExpr = BinaryExpr {
				left: _expr,
				operator: _operator,
				right: _right
			};
		}
		binaryExpr
	}

	fn unary(&self) -> Expr {
		if matching(&vec![TokenType::Bang, TokenType::Minus]) {
			let _operator: Token = self.previous();
			let _right: Expr = self.unary();

			let mut unaryExpr = UnaryExpr {
				operator: _operator,
				right: _right
			};
			unaryExpr
		} else {
			self.primary()
		}
	}

	fn primary(&self) -> Expr {
		if matching(&vec![TokenType::False]) { 
			return new Expr.Literal(false);
		} else if matching(&vec![TokenType::True]) { 
			return new Expr.Literal(true);
		} else if matching(&vec![TokenType::Nil]) { 
			return new Expr.Literal(null);
		} else if matching(&vec![TokenType::Number, TokenType::String]) {
			return new Expr.Literal(self.previous().literal);
		} else if matching(&vec![TokenType::LeftParen]) {
			Expr expr = self.expression();
			consume(TokenType::RightParen, "Expect ')' after expression.");
			return new Expr.Grouping(expr);
		} else {
			expr::Expr::new(self)
			// expr::Expr::Literal = { value: Literal::None };
		}
	}

	// fn error(&self, tokenType: TokenType, message:&str) -> ParseError {
	// 	Lox.error(token, message);
	// 	return new ParseError();
	// }

	// fn error(&self, tokenType: TokenType, message: &str) { // Edit 2
	// 	for tokenType in tokenTypes {
	// 		let temp_token = self.tokens;
	// 		if temp_token == TokenType::Eof {
	// 	  	self.report(token.line, " at end", message);
	// 		} else {
	// 			self.report(token.line, " at '" + token.lexeme + "'", message);
	// 		}
	// 	}
	// }
	  
	fn synchronize(&self) { // edit 4.
		self.advance();
	
		while !self.isAtEnd() {
		let temp_token = self.previous();
		  if temp_token == TokenType::Semicolon { return };
		  match self.peek() {
				Class => self.peek().to_string(),
				FUN => self.peek().to_string(),
				VAR => self.peek().to_string(),
				FOR => self.peek().to_string(),
				IF => self.peek().to_string(),
				WHILE => self.peek().to_string(),
				PRINT => self.peek().to_string(),
				RETURN => return
		  };
		  self.advance();
		}
	}
}