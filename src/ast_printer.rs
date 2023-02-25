use crate::LoxError::*;
use crate::token::*;
use crate::expr::*;

struct AstPrinter;

impl AstPrinter {
    fn print(&self, expr: &Expr) -> Result<String, ScannerError> {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &String, exprs: &[&Box<Expr>]) -> Result<String, ScannerError> {
        let mut builder = format!("({name}");

        for expr in exprs {
            builder = format!("{builder} {}", expr.accept(self)?);
        }
        builder = format!("{builder})");

        Ok(builder)
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<String, ScannerError> {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<String, ScannerError> {
        self.parenthesize(&"group".to_string(), &[&expr.expression])
    }
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<String, ScannerError> {
        if &Some(value) = &expr.value {
            Ok(value.to_string())
        } else {
            Ok("nil".to_string())
        }
    }
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<String, ScannerError> {
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