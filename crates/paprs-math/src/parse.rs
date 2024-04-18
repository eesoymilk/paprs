use crate::math::ast::{Delimiters, Expression, Operator, Statement};
use crate::math::parser::Rule;
use crate::math::pratt_parser::PRATT_PARSER;
use crate::math::utils::{valid_rule, valid_rules};
use pest::error::Error;
use pest::iterators::Pair;

pub fn parse_group(group: Pair<Rule>) -> Result<Expression, Error<Rule>> {
    valid_rules(
        &group,
        &[
            Rule::parenthesized,
            Rule::bracketed,
            Rule::braced,
            Rule::barred,
        ],
    )?;

    let delimiters = match group.as_rule() {
        Rule::parenthesized => Delimiters::Parentheses,
        Rule::bracketed => Delimiters::Brackets,
        Rule::braced => Delimiters::Braces,
        rule => unreachable!("Expr::parse expected grouped expression, found {:?}", rule),
    };
    let stmt_pair = group.into_inner().next().unwrap();
    Ok(Expression::Group {
        statement: Box::new(parse_statement(stmt_pair).unwrap()),
        delimiters,
    })
}

pub fn parse_expression(expr: Pair<Rule>) -> Result<Expression, Error<Rule>> {
    valid_rule(&expr, Rule::expression)?;

    let parsed_expr = PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::number => Ok(Expression::Value(primary.as_str().parse().unwrap())),
            Rule::variable => Ok(Expression::Variable(primary.as_str().to_string())),
            Rule::parenthesized | Rule::bracketed | Rule::braced | Rule::barred => {
                Ok(parse_group(primary).unwrap())
            }
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => Operator::Add,
                Rule::subtract => Operator::Subtract,
                Rule::multiply => Operator::Multiply,
                Rule::divide => Operator::Divide,
                Rule::fraction => Operator::Fraction,
                Rule::superscript => Operator::Superscript,
                Rule::over => Operator::Over,
                Rule::pre_superscript => Operator::PreSuperscript,
                Rule::subscript => Operator::Subscript,
                Rule::under => Operator::Under,
                Rule::pre_subscript => Operator::PreSubscript,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Ok(Expression::BinOp {
                left: Box::new(lhs?),
                op,
                right: Box::new(rhs?),
            })
        })
        .map_prefix(|op, rhs| {
            let op = match op.as_rule() {
                Rule::negative => Operator::Subtract,
                rule => unreachable!("Expr::parse expected prefix operation, found {:?}", rule),
            };
            Ok(Expression::BinOp {
                left: Box::new(Expression::Value(0.0)),
                op,
                right: Box::new(rhs?),
            })
        })
        .map_postfix(|lhs, op| {
            let op = match op.as_rule() {
                Rule::factorial => Operator::Multiply,
                rule => unreachable!("Expr::parse expected postfix operation, found {:?}", rule),
            };
            Ok(Expression::BinOp {
                left: Box::new(lhs?),
                op,
                right: Box::new(Expression::Value(0.0)),
            })
        })
        .parse(expr.into_inner());

    Ok(parsed_expr?)
}

pub fn parse_statement(stmt_pair: Pair<Rule>) -> Result<Statement, Error<Rule>> {
    valid_rule(&stmt_pair, Rule::statement)?;

    // Get the inner pair which we'll match against specific rules
    let inner = stmt_pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::expression => {
            let expr = parse_expression(inner)?;
            Ok(Statement::Expr(expr))
        }
        
        Rule::matrix => {
            let mut rows = Vec::new();
            for row_pair in inner.into_inner() {
                let mut row = Vec::new();
                for expr_pair in row_pair.into_inner() {
                    row.push(parse_expression(expr_pair)?);
                }
                rows.push(row);
            }
            Ok(Statement::Matrix(rows))
        }
        rule => unreachable!(
            "parse_statement expected expression or matrix, found {:?}",
            rule
        ),
    }
}
