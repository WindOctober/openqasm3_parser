use oq3_syntax::{ast, AstNode, SourceFile};

// Test tree structure rather than an evaluator that could mask parser errors.
fn shape(expr: ast::Expr) -> String {
    match expr {
        ast::Expr::BinExpr(e) => format!(
            "({} {} {})",
            shape(e.lhs().unwrap()),
            e.op_token().unwrap().text(),
            shape(e.rhs().unwrap())
        ),
        ast::Expr::PrefixExpr(e) => format!(
            "({}{})",
            e.op_token().unwrap().text(),
            shape(e.expr().unwrap())
        ),
        ast::Expr::ParenExpr(e) => shape(e.expr().unwrap()),
        e => e.syntax().text().to_string(),
    }
}

#[test]
fn power_precedence_and_associativity() {
    for (source, expected) in [
        ("2 ** 3 ** 2", "(2 ** (3 ** 2))"),
        ("(2 ** 3) ** 2", "((2 ** 3) ** 2)"),
        ("2 ** 3 + 1", "((2 ** 3) + 1)"),
        ("1 + 2 ** 3", "(1 + (2 ** 3))"),
        ("2 * 3 ** 2", "(2 * (3 ** 2))"),
        ("2 ** 3 * 4", "((2 ** 3) * 4)"),
        ("-2 ** 2", "(-(2 ** 2))"),
        ("(-2) ** 2", "((-2) ** 2)"),
        ("2 ** -3 ** 2", "(2 ** (-(3 ** 2)))"),
        ("-2 ** -3 * 4", "((-(2 ** (-3))) * 4)"),
        ("~a ** b", "(~(a ** b))"),
        ("!a ** b", "(!(a ** b))"),
        ("f(a) ** b[0]", "(f(a) ** b[0])"),
        ("float[64](a) ** 2", "(float[64](a) ** 2)"),
        ("a ** 2 < b", "((a ** 2) < b)"),
        ("a ^ b ** 2", "(a ^ (b ** 2))"),
        // Neighboring operators retain their existing associativity.
        ("a - b - c", "((a - b) - c)"),
        ("a / b * c", "((a / b) * c)"),
        ("-a * b", "((-a) * b)"),
    ] {
        // Both initializers and assignment RHS use the same precedence.
        for statement in [format!("float[64] x = {source};"), format!("x = {source};")] {
            let parse = SourceFile::parse(&statement);
            assert!(
                parse.errors().is_empty(),
                "{statement}: {:?}",
                parse.errors()
            );
            let stmt = parse.tree().statements().next().unwrap();
            let expr = stmt
                .syntax()
                .children()
                .filter_map(ast::Expr::cast)
                .last()
                .unwrap();
            assert_eq!(shape(expr), expected, "{statement}");
        }
    }
}
