use std::string::ParseError;

use crate::token::*;
use crate::expr::*;
use crate::stmt::*;

pub struct Parser {
	pub tokens: Vec<Token>,
	pub current: usize,
	// pub extends: ParseError // Edit 3.
}

impl Parser {
	pub fn parse(&mut self) -> Vec<Box<Stmt>> {
		let mut statements = Vec::new();
		while !self.isAtEnd() {
			statements.push(self.statement());
			//statements.add(self.declaration());
		}
		statements
	}

	fn expression(&mut self) -> Box<Expr> {
		return self.assignment()?;
	}

	fn declaration(&mut self) -> Box<Stmt> {
		match self.peek().token_type {
			TokenType::Var => {
				self.advance();
				self.var_declaration()
			}
			_=>{
				match self.statement() {
					Ok(stmt) => Some(stmt),
					Err(ParseError { .. }) => {
						self.synchronize();
						None
					}
				}
			}
		}
	}

	fn statement(&mut self) -> Box<Stmt> {
		if self.matching(&vec![TokenType::Print]) {
			return self.print_statement();
		}			
		return self.expression_statement();
		// if self.matching(TokenType::LeftBrace) {
		// 	return Stmt::Block(self.block()?);
		// }
	}

	fn print_statement(&mut self) -> Box<Stmt> {
		let value: Box<Expr> = self.expression();
		self.consume(TokenType::Semicolon, "Expect ';' after value. ");
		Stmt::Print(value)
		//////////////////////////////////////////////////////////////////////////////////////////
	}

	fn var_declaration(&mut self) -> Box<Stmt> {
		let name = self.consume(TokenType::Identifier, "Expect variable name.")?;
		let initializer = if self.matching(&[TokenType::Equal]) {
			Some(self.expression()?)
		} else {
			None
		};
		self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.")?;
		Some(Stmt::Var(name, initializer))
	}

	fn expression_statement(&mut self) -> Box<Stmt> {
		let expr: Box<Expr> = self.expression();
		self.consume(TokenType::Semicolon, "Expect ';' after expression. ");
		Stmt::Expression(expr)
	}

	fn block(&mut self) -> Vec<Stmt> {
		let mut statements = Vec::new();

		while !self.check(TokenType::RightBrace) && !self.isAtEnd() {
			statements.push(self.declaration());
		}

		self.consume(TokenType::RightBrace, "Expect '}' after block.");
		statements
	}

	fn assignment(&mut self) -> Result<Expr, RuntimeError> {
		let expr = self.equality()?;
		if self.matching(&[Equal]) {
			let equals = self.previous();
			let value = self.assignment();

			if let Expr::Variable(name) = &expr {
				return Ok(Expr::Assign(name.clone(), Box::new(value)));
			}

			self.error(&equals, "Invalid assignment target.");
			return Err(RuntimeError::new("Invalid assignment target."));
		}

		Ok(expr)
	}

	fn equality(&mut self) -> Box<Expr> {
		let mut _expr: Box<Expr> = self.comparison();
		//let mut binaryExpr: Box<Expr> = _expr.clone();
		while self.matching(&vec![TokenType::BangEqual, TokenType::EqualEqual]) {
			let _operator: Token = self.previous();
			let _right: Box<Expr> = self.comparison();
			// expr = new Expr.Binary(expr, operator, right); 
			_expr = Box::new(Expr::Binary(BinaryExpr { 
				left: _expr.clone(),
				operator: _operator,
				right: _right 
			}));
		}
		_expr
	}

	fn matching(&mut self, tokenTypes: &Vec<TokenType>) -> bool { 
		for tokenType in tokenTypes {
			if self.check(*tokenType) {
				self.advance();
				return true;
			}
		}
		false
	}
	
	fn consume(&mut self, tokenType: TokenType, message:&str) -> Token { 
		// for tokenType in tokenTypes {
		if self.check(tokenType) { self.advance() }
		// }
		else { self.advance() }		// placeholder return, need to return error
		// throw!(self.error(self.peek(), message));
	}

	fn check(&mut self, tokenType: TokenType) -> bool {
		if self.isAtEnd() { 
			false 
		} else {
			let temp_token_type = self.peek()._type;
			return temp_token_type == tokenType; 
		}
	}

	fn advance(&mut self) -> Token {
		if !self.isAtEnd() { self.current += 1; }
		self.previous()
	}

	fn isAtEnd(&mut self) -> bool {
		let temp_token_type = self.peek()._type;
		return temp_token_type == TokenType::Eof;
	}

	fn peek(&mut self) -> Token {
		self.tokens[self.current].clone()
	}

	fn previous(&mut self) -> Token {
		self.tokens[self.current - 1].clone()
	}

	fn comparison(&mut self) -> Box<Expr> {
		let mut _expr: Box<Expr> = self.term();
		while self.matching(&vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
			let _operator: Token = self.previous();
			let _right: Box<Expr> = self.term();
			_expr = Box::new(Expr::Binary(BinaryExpr {
				left: _expr,
				operator: _operator,
				right: _right
			}));
		}
		_expr
	}

	fn term(&mut self) -> Box<Expr> {
		let mut _expr: Box<Expr> = self.factor();
		while self.matching(&vec![TokenType::Minus, TokenType::Plus]) {
			let _operator: Token = self.previous();
			let _right: Box<Expr> = self.factor();
			_expr = Box::new(Expr::Binary(BinaryExpr {
				left: _expr,
				operator: _operator,
				right: _right
			}));
		}
		_expr
	}

	fn factor(&mut self) -> Box<Expr> {
		let mut _expr: Box<Expr> = self.unary();
		while self.matching(&vec![TokenType::Slash, TokenType::Star]) {
			let _operator: Token = self.previous();
			let _right: Box<Expr> = self.unary();
			_expr = Box::new(Expr::Binary(BinaryExpr {
				left: _expr,
				operator: _operator,
				right: _right
			}));
		}
		_expr
	}

	fn unary(&mut self) -> Box<Expr> {
		if self.matching(&vec![TokenType::Bang, TokenType::Minus]) {
			let _operator: Token = self.previous();
			let _right: Box<Expr> = self.unary();

			let unaryExpr = Box::new(Expr::Unary(UnaryExpr {
				operator: _operator,
				right: _right
			}));
			unaryExpr
		} else {
			self.primary()
		}
	}

	fn primary(&mut self) -> Box<Expr> {
		if self.matching(&vec![TokenType::False]) { 
			let literalExpr = Box::new(Expr::Literal(LiteralExpr {
				value: Some(Literal::Bool(false))
			}));
			literalExpr
		} else if self.matching(&vec![TokenType::True]) { 
			let literalExpr = Box::new(Expr::Literal(LiteralExpr {
				value: Some(Literal::Bool(true))
			}));
			literalExpr
		} else if self.matching(&vec![TokenType::Nil]) { 
			let literalExpr = Box::new(Expr::Literal(LiteralExpr {
				value: Some(Literal::None)
			}));
			literalExpr
		} else if self.matching(&vec![TokenType::Number, TokenType::String]) {
			let literalExpr = Box::new(Expr::Literal(LiteralExpr {
				value: Some(self.previous().literal)
			}));
			literalExpr
		} else if self.matching(&vec![TokenType::Identifier]) {
			return Ok(Expr::Variable(self.previous()));
		} else if self.matching(&vec![TokenType::LeftParen]) {
			let expr: Box<Expr> = self.expression();
			self.consume(TokenType::RightParen, "Expect ')' after expression.");
			let groupingExpr = Box::new(Expr::Grouping(GroupingExpr {
				expression: expr
			}));
			groupingExpr
		} else {
			let literalExpr = Box::new(Expr::Literal(LiteralExpr {
				value: Some(Literal::None)
			}));
			literalExpr
		}
	}

	// fn error(&mut self, tokenType: TokenType, message:&str) -> ParseError {
	// 	Lox.error(token, message);
	// 	return new ParseError();
	// }

	// fn error(&mut self, tokenType: TokenType, message: &str) { // Edit 2
	// 	for tokenType in tokenTypes {
	// 		let temp_token = self.tokens;
	// 		if temp_token == TokenType::Eof {
	// 	  	self.report(token.line, " at end", message);
	// 		} else {
	// 			self.report(token.line, " at '" + token.lexeme + "'", message);
	// 		}
	// 	}
	// }
	  
	fn synchronize(&mut self) { // edit 4.
		self.advance();
	
		while !self.isAtEnd() {
		let temp_token_type = self.previous()._type;
		  if temp_token_type == TokenType::Semicolon { return };
		  match self.peek()._type {
			Class => (),
			FUN => (),
			VAR => (),
			FOR => (),
			IF => (),
			WHILE => (),
			PRINT => (),
			RETURN => return
		  };
		  self.advance();
		}
	}
}