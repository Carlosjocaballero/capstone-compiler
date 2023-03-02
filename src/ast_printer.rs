use crate::LoxError::*;
use crate::expr::*;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&mut self, expr: &Expr) -> Result<String, ScannerError> {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: &String, exprs: &[&Box<Expr>]) -> Result<String, ScannerError> {
        let mut builder = format!("({name}");

        for expr in exprs {
            builder = format!("{builder} {}", expr.accept(self)?);
        }
        builder = format!("{builder})");

        Ok(builder)
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<String, ScannerError> {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<String, ScannerError> {
        self.parenthesize(&"group".to_string(), &[&expr.expression])
    }
    fn visit_calling_expr(&mut self, expr: &CallingExpr) -> Result<String, ScannerError> {
        self.parenthesize(&"group".to_string(), &[&expr.expression])
    }
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<String, ScannerError> {
        match &expr.value{
            Some(x) => Ok(x.to_string()),
            None => Ok(String::from("nil"))
        }
    }
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<String, ScannerError> {
        self.parenthesize(&expr.operator.lexeme, &[&expr.right]) 
    }
}
