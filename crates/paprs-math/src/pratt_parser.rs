use crate::math::parser::{Rule, Rule::*};
use pest::pratt_parser::{Assoc::*, Op, PrattParser};

lazy_static::lazy_static! {
    pub static ref PRATT_PARSER: PrattParser<Rule> = {
        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(fraction, Left))
            .op(Op::infix(superscript, Left) | Op::infix(over, Left) | Op::infix(pre_superscript, Left))
            .op(Op::infix(subscript, Left) | Op::infix(under, Left) | Op::infix(pre_subscript, Left))
            .op(Op::prefix(negative))
            .op(Op::postfix(factorial))
    };
}
