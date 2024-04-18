#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Operator {
    Add,            // `+`
    Subtract,       // `-`
    Multiply,       // `*`
    Divide,         // `//`
    Fraction,       // `/`
    Superscript,    // `^`
    Over,           // `^^`
    PreSuperscript, // `^^^`
    Subscript,      // `_`
    Under,          // `__`
    PreSubscript,   // `___`
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Delimiters {
    Parentheses,
    Brackets,
    Braces,
    Bars,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Expression {
    Value(f64),
    Variable(String),
    Group {
        statement: Box<Statement>,
        delimiters: Delimiters,
    },
    BinOp {
        op: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Statement {
    Matrix(Vec<Vec<Expression>>),
    Expr(Expression),
}
