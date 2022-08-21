use crate::runtime::expression::Expression;
use crate::runtime::literal::Literal;
use crate::runtime::pattern::Pattern;

pub struct PatternList(pub Vec<(Pattern, Expression)>);

impl PatternList {
    pub fn apply_first_matching_pattern(&self, literal: Literal) -> Option<Literal> {
        for (pattern, expression) in &self.0 {
            if let Some(caputred_variables) = (pattern).matches(&literal) {
                return Some(expression.evaluate(&caputred_variables));
            }
        }

        None
    }
}
