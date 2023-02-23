private Token consume(tokenTypes: TokenType type, String message) {
    if (check(type)) return advance();
    throw error(peek(), message);
}
private ParseError error(Token token, String message) {
    Lox.error(token, message);
    return new ParseError();
}
static void error(Token token, String message) {
    if (token.type == TokenType.EOF) {
    report(token.line, " at end", message);
    } else {
    report(token.line, " at '" + token.lexeme + "'", message);
    }
}
class Parser {
    private static class ParseError extends RuntimeException {}
    private final List<Token> tokens;
}
private void synchronize() {
    advance();
    while (!isAtEnd()) {
    if (previous().type == SEMICOLON) return;
    switch (peek().type) {
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
if (match(LEFT_PAREN)) {
    Expr expr = expression();
    consume(RIGHT_PAREN, "Expect ')' after expression.");
    return new Expr.Grouping(expr);
}
    throw error(peek(), "Expect expression.");
}
Expr parse() {
    try {
    return expression();
    } catch (ParseError error) {
    return null;
    }
    }
List<Token> tokens = scanner.scanTokens();
Parser parser = new Parser(tokens);
Expr expression = parser.parse();
// Stop if there was a syntax error.
if (hadError) return;
System.out.println(new AstPrinter().print(expression));
}