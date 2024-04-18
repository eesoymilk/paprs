use super::{ast::*, parse::*, parser::*};
use pest::Parser;

#[test]
fn test_parse_stmt_1() {
    let input = "1 + 2";
    let parsed = MathParser::parse(Rule::statement, input)
        .unwrap()
        .next()
        .unwrap();
    println!("{:#?}", parsed);

    let stmt = parse_statement(parsed).unwrap();
    println!("{:#?}", stmt);

    assert_eq!(
        stmt,
        Statement::Expr(Expression::BinOp {
            op: Operator::Add,
            left: Box::new(Expression::Value(1.0)),
            right: Box::new(Expression::Value(2.0)),
        })
    );
}

#[test]
fn test_parse_stmt_2() {
    let input = "1 + 2 * 3";
    let parsed = MathParser::parse(Rule::statement, input)
        .unwrap()
        .next()
        .unwrap();
    println!("{:#?}", parsed);

    let stmt = parse_statement(parsed).unwrap();
    println!("{:#?}", stmt);

    assert_eq!(
        stmt,
        Statement::Expr(Expression::BinOp {
            op: Operator::Add,
            left: Box::new(Expression::Value(1.0)),
            right: Box::new(Expression::BinOp {
                op: Operator::Multiply,
                left: Box::new(Expression::Value(2.0)),
                right: Box::new(Expression::Value(3.0)),
            }),
        })
    );
}

#[test]
fn test_parse_stmt_3() {
    let input = "1 + 2 * 3 ^ 4";
    let parsed = MathParser::parse(Rule::statement, input)
        .unwrap()
        .next()
        .unwrap();
    println!("{:#?}", parsed);

    let stmt = parse_statement(parsed).unwrap();
    println!("{:#?}", stmt);

    assert_eq!(
        stmt,
        Statement::Expr(Expression::BinOp {
            op: Operator::Add,
            left: Box::new(Expression::Value(1.0)),
            right: Box::new(Expression::BinOp {
                op: Operator::Multiply,
                left: Box::new(Expression::Value(2.0)),
                right: Box::new(Expression::BinOp {
                    op: Operator::Superscript,
                    left: Box::new(Expression::Value(3.0)),
                    right: Box::new(Expression::Value(4.0)),
                }),
            }),
        })
    );
}

#[test]
fn test_parse_stmt_4() {
    let input = "(a_123 + 2) * 3";
    let parsed = MathParser::parse(Rule::statement, input)
        .unwrap()
        .next()
        .unwrap();
    println!("{:#?}", parsed);

    let stmt = parse_statement(parsed).unwrap();
    println!("{:#?}", stmt);

    assert_eq!(
        stmt,
        Statement::Expr(Expression::BinOp {
            op: Operator::Multiply,
            left: Box::new(Expression::Group {
                statement: Box::new(Statement::Expr(Expression::BinOp {
                    op: Operator::Add,
                    left: Box::new(Expression::BinOp {
                        op: Operator::Subscript,
                        left: Box::new(Expression::Variable("a".to_string())),
                        right: Box::new(Expression::Value(123.0)),
                    }),
                    right: Box::new(Expression::Value(2.0)),
                })),
                delimiters: Delimiters::Parentheses,
            }),
            right: Box::new(Expression::Value(3.0)),
        })
    );
}


#[test]
fn test_parse_stmt_5() {
    let input = "[1, 2, 3 ; 4, 5]";
    let parsed = MathParser::parse(Rule::statement, input)
        .unwrap()
        .next()
        .unwrap();
    println!("{:#?}", parsed);

    let stmt = parse_statement(parsed).unwrap();
    println!("{:#?}", stmt);

    assert_eq!(
        stmt,
        Statement::Expr(Expression::Group {
            statement: Box::new(Statement::Matrix(vec![
                vec![
                    Expression::Value(1.0),
                    Expression::Value(2.0),
                    Expression::Value(3.0),
                ],
                vec![
                    Expression::Value(4.0),
                    Expression::Value(5.0),
                ],
            ])),
            delimiters: Delimiters::Brackets,
        })
    );
    
}