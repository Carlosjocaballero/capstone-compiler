use crate::token::*;
use crate::expr::*;
use crate::stmt::*;
use crate::LoxError::*;

pub struct Parser {
	pub tokens: Vec<Token>,
	pub current: usize,
	pub parser_error: ParseError // Edit 3.
}

impl Parser {
	pub fn parse(&mut self) -> Vec<Box<Stmt>> {
		let mut statements = Vec::new();
		while !self.isAtEnd() {
			statements.push(self.declaration());
		}
		return statements
	}

	fn expression(&mut self) -> Box<Expr> {
		return self.assignment();
	}

	fn declaration(&mut self) -> Box<Stmt> {
		///////////////////////////////////////////////////////////
		/////////////   NEEDS CHECKING IF CORRECT /////////////////
		if self.parser_error.is_error == true {self.synchronize()};
		///////////////////////////////////////////////////////////
		

		if self.matching(&vec![TokenType::Var]){
			return self.var_declaration();
		}
		return self.statement();
	}

	fn statement(&mut self) -> Box<Stmt> {
		if self.matching(&vec![TokenType::If]) {
			return self.if_statement();
		}
		if self.matching(&vec![TokenType::Print]) {
			return self.print_statement();
		}
		if self.matching(&vec![TokenType::While]) {
			return self.while_statement();
		}
		if self.matching(&vec![TokenType::For]) {
			return self.for_statement();
		}
		if self.matching(&vec![TokenType::LeftBrace]) {
			return Box::new(Stmt::Block(BlockStmt { statements: self.block() }));
		}
		return self.expression_statement();
	}



	fn if_statement(&mut self) -> Box<Stmt> {
		self.consume(TokenType::LeftParen, "Expect '(' after 'if'.");
		let condition: Box<Expr> = self.expression();
		self.consume(TokenType::RightParen, "Expect ')' after 'if'.");

		let then_branch = self.statement();
		let else_branch = if self.matching(&vec![TokenType::Else]) {
            Some(self.statement())
        } else {
            None
        };

		return Box::new(Stmt::If(IfStmt { condition: condition, then_branch: then_branch, else_branch: else_branch}))

	}

	fn print_statement(&mut self) -> Box<Stmt> {
		let value: Box<Expr> = self.expression();
		self.consume(TokenType::Semicolon, "Expect ';' after value. ");
		Box::new(Stmt::Print(PrintStmt { expression: value }))
	}

	fn var_declaration(&mut self) -> Box<Stmt> {
		let name: Token = self.consume(TokenType::Identifier, "Expect variable name.");
		let mut initializer : Box<Expr> = Box::new(Expr::Literal(LiteralExpr { value: Some(Literal::None) }));
		if self.matching(&vec![TokenType::Equal]) {
			initializer = self.expression();
		}

		self.consume(TokenType::Semicolon, "Expect ';' after varibale declaraton.");
		return Box::new(Stmt::Var(VarStmt { name: name, initializer: initializer }));
	}
	
	fn while_statement(&mut self) -> Box<Stmt>{
		self.consume(TokenType::LeftParen, "Expect '(' after 'while'.");
		let condition: Box<Expr> =  self.expression();
		self.consume(TokenType::RightParen, "Expect ')' after condition.");
		let body: Box<Stmt> = self.statement();
		return Box::new(Stmt::While(WhileStmt{condition: condition, body: body}));
	}

	fn for_statement(&mut self) -> Box<Stmt>{
		self.consume(TokenType::LeftParen, "Expect '(' after 'for'.");

		let initializer: Option<Box<Stmt>>;
		if self.matching(&vec![TokenType::Semicolon]) {
			initializer = None;
		} else if self.matching(&vec![TokenType::Var]) {
			initializer = Some(self.var_declaration());
		} else {
			initializer = Some(self.expression_statement());
		}

		let condition: Option<Box<Expr>>;
		if !self.check(TokenType::Semicolon) {
		condition = Some(self.expression());
		} else {
		condition = None;
		}
		self.consume(TokenType::Semicolon, "Expect ';' after loop condition.");

		let increment = if !self.check(TokenType::RightParen) {
			Some(self.expression())
		} else {
			None
		};
		self.consume(TokenType::RightParen, "Expect ')' after for clauses.");

		let mut body: Box<Stmt> = self.statement();

		if let Some(increment_expr) = increment {
			body = Box::new(Stmt::Block(BlockStmt { 
				statements: vec![body, Box::new(Stmt::Expression(ExpressionStmt { expression: increment_expr }))], 
			}))
		}

		body = Box::new(Stmt::While(WhileStmt { 
			condition: if let Some(cond) = condition {
				cond 
			} else {
				Box::new(Expr::Literal(LiteralExpr { 
					value: Some(Literal::Bool(true)),
				 }))
			}, 
			body: body, 
		}));

		if let Some(initializer) = initializer {
			body = Box::new(Stmt::Block(BlockStmt {
				statements: vec![initializer, body],
			}));
		}		
	
		return body		

	}

	fn expression_statement(&mut self) -> Box<Stmt> {
		let expr: Box<Expr> = self.expression();
		self.consume(TokenType::Semicolon, "Expect ';' after expression. ");
		Box::new(Stmt::Expression(ExpressionStmt { expression: expr }))
	}

	fn block(&mut self) -> Vec<Box<Stmt>> {
		let mut statements = Vec::new();

		while !self.check(TokenType::RightBrace) && !self.isAtEnd() {
			statements.push(self.declaration());
		}

		self.consume(TokenType::RightBrace, "Expect '}' after block.");
		statements
	}

	fn assignment(&mut self) -> Box<Expr>{
		let expr = self.or();

		if self.matching(&vec![TokenType::Equal]){
			let equals : Token = self.previous();
			let value : Box<Expr> = self.assignment();
		///////////////////////////////////////////////////////////////////////////////////////////////
			if let Expr::Variable(x) = *expr{
				return Box::new(Expr::Assign(AssignExpr { name: x.name, value: value }))
			}

			self.parser_error.error(&equals, "Invalid assignment target.".to_string())
		}

		return expr;
	}

	fn or(&mut self) -> Box<Expr> {
		let mut _expr = self.and();
	
		while self.matching(&vec![TokenType::Or]) {
			let operator = self.previous().clone();
			let right = self.and();
			_expr = Box::new(Expr::Logical(LogicalExpr {left: _expr, operator, right: right}));
		}
	
		_expr
	}

	fn and(&mut self) -> Box<Expr> {
		let mut _expr = self.equality();
	
		while self.matching(&vec![TokenType::And]) {
			let operator = self.previous();
			let right = self.equality();
			_expr = Box::new(Expr::Logical(LogicalExpr {left: _expr, operator, right: right}));
		}
	
		_expr
	}
	
	

	fn equality(&mut self) -> Box<Expr> {
		let mut _expr: Box<Expr> = self.comparison();
		while self.matching(&vec![TokenType::BangEqual, TokenType::EqualEqual]) {
			let _operator: Token = self.previous();
			let _right: Box<Expr> = self.comparison();
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
	
	fn consume(&mut self, token_types: TokenType, message: &str) -> Token { 
		if self.check(token_types) { return self.advance(); }
		let peek = &self.peek();
		self.parser_error.error(peek, message.to_string());
		Token { _type: TokenType::Nil, lexeme: "".to_string(), literal: Literal::None, line: 0 }
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

	fn finishCall(&mut self, callee: Box<Expr>) -> Box<Expr> {
		let mut _arguments: Vec<Box<Expr>> = Vec::new();
		if !self.check(TokenType::RightParen) {
			while {		// do-while loop, all the contents are executed in the first {} 
				if _arguments.len() >= 255 {
					let token = self.peek();
					self.parser_error.error(&token, "Can't have more than 255 arguments.".to_string());
				}
				_arguments.push(self.expression());

				self.matching(&vec![TokenType::Comma])		// while condition
			} {}			// this second {} stays empty 
		}
		let _paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.");
		let mut _expr = Box::new(Expr::Call(CallingExpr { 
			callee: callee,
			paren: _paren,
			arguments: _arguments
		}));
		_expr
	}

	fn call(&mut self) -> Box<Expr> {
		let mut _expr = self.primary();
		loop {
			if self.matching(&vec![TokenType::LeftParen]) {
				_expr = self.finishCall(_expr);
			} else {
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
		} else if self.matching(&vec![TokenType::Identifier]) {
			//println!("Parser:Primary():351\n{:?}", self.previous());
			let varExpr = Box::new(Expr::Variable(VariableExpr { name: self.previous() }));
			varExpr
		}
		 else if self.matching(&vec![TokenType::LeftParen]) {
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
			TokenType::Class => (),
			TokenType::Fun => (),
			TokenType::Var => (),
			TokenType::For => (),
			TokenType::If => (),
			TokenType::While => (),
			TokenType::Print => (),
			TokenType::Return => return,
			_ => {self.advance(); ()}
		  };
		}
		self.parser_error.is_error = false;
	}
}