use crate::token::*;
use crate::expr::*;
use crate::LoxError::*;
extern crate arraylist;
use arraylist::arl::ArrayList;

pub struct Parser {
	pub tokens: Vec<Token>,
	pub current: usize,
	pub parser_error: ParseError // Edit 3.
}

impl Parser {
	pub fn parse(&mut self) -> Box<Expr> {
		self.expression()
	}

	fn expression(&mut self) -> Box<Expr> {
		self.equality()
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
	
	fn consume(&mut self, token_types: TokenType, message:&str) -> Token { 
		// for token_types in token_typess {
		if self.check(token_types) { self.advance(); }
		// }
		let peek = &self.peek();
		self.parser_error.error(peek, message.to_string());
		panic!()
	}

	fn check(&mut self, token_types: TokenType) -> bool {
		if self.isAtEnd() { 
			false 
		} else {
			let temp_token_type = self.peek()._type;
			return temp_token_type == token_types; 
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
			self.call()
		}
	}
	fn finishCall(&mut self, callee:Box<Expr>)->Box<Expr>{
		let mut arguments = ArrayList::new();
		if !self.check(TokenType::RightParen){
			while self.matching(&vec![tokenType::RightParen]){
				arguments.add(self.expression());
			}
		}
		let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.");
		return Box::new(Expr::Call())
	}
	fn call(&mut self) -> Box<Expr>{
		let mut _expr = self.primary();
		loop{
			if(self.matching(&vec![TokenType::LeftParen])){
				_expr = self.finishCall(_expr);
			}
			else{
				break;
			}
		}
		_expr
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
			let peek = self.peek();
			self.parser_error.error(&peek, "Expect expression.".to_string());
			literalExpr
		}
	}

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