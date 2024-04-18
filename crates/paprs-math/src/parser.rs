use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "math/math.pest"]
pub struct MathParser;
