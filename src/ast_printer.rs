mod error;
use error::*;

mod token;
use token::*;

mod expr;
use expr::*;

struct AstPrinter;

impl AstPrinter {
    fn print(&self, expr: &Expr) -> Result<String, LoxError> {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &String, exprs: &[&Box<Expr>]) -> Result<String, LoxError> {
        let mut builder = format!("({name}");

        for expr in exprs {
            builder = format!("{builder} {}", expr.accept(self)?);
        }
        bulder = format!("{builder})");

        Ok(builder)
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<String, LoxError> {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, e&xpr.right])
    }
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<String, LoxError> {
        self.parenthesize(&"group".to_string(), &[&expr.expression])
    }
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<String, LoxError> {
        if &Some(value) = &expr.value {
            Ok(value.to_string())
        } else {
            Ok("nil".to_string())
        }
    }
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<String, LoxError> {
        self.parenthesize(&expr.operator.lexeme, &[&expr.right]) 
    }
}

//main method that manually instantiates a tree and prints it since theres no parser
/* 
fn main() {
    let expression = Expr::Binary(
        BinaryExpr {
            left: Box::new(Expr::Unary(
                UnaryExpr {
                    operator: Token {
                        ttype: TokenType::Minus,
                        lexeme: "-".to_string(),
                        literal: None,
                        line: 1
                    },
                    right: Box::new(Expr::Literal(
                        LiteralExpr { value: Some(Object::Num(123.0)) }
                    ))
                }
            )),
            operator: Token {
                ttype: TokenType::Star,
                lexeme: "*".to_string(),
                literal: None,
                line: 1
            },
            right: Box::new(Expr::Grouping(
                GroupingExpr{ expression:
                    Box::new(Expr::Literal( LiteralExpr {value: Some(Object::Num(45.67))}))}
            ))
        }
    );

    let printer = AstPrinter {};
    println!("{}", printer.print(&expression).unwrap());
}
*/