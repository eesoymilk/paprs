use crate::math::parser::Rule;
use pest::error::Error;
use pest::iterators::Pair;

pub fn valid_rule(pair: &Pair<Rule>, rule: Rule) -> Result<(), Error<Rule>> {
    if pair.as_rule() != rule {
        Err(Error::new_from_span(
            pest::error::ErrorVariant::ParsingError {
                positives: vec![rule],
                negatives: vec![],
            },
            pair.as_span(),
        ))
    } else {
        Ok(())
    }
}

pub fn valid_rules(pair: &Pair<Rule>, rules: &[Rule]) -> Result<(), Error<Rule>> {
    if !rules.contains(&pair.as_rule()) {
        Err(Error::new_from_span(
            pest::error::ErrorVariant::ParsingError {
                positives: rules.to_vec(),
                negatives: vec![],
            },
            pair.as_span(),
        ))
    } else {
        Ok(())
    }
}
