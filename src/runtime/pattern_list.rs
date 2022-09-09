use crate::errors;
use crate::runtime::expression::Expression;
use crate::runtime::literal::Literal;
use crate::runtime::pattern::Pattern;
use std::collections::HashMap;

#[derive(Debug)]
pub struct PatternList(pub Vec<(Pattern, Expression, PatternPosition)>);

impl PatternList {
    pub fn apply_first_matching_pattern(
        &self,
        literal: Literal,
        functions: &HashMap<String, PatternList>,
    ) -> Option<Literal> {
        for (pattern, expression, _error_range) in &self.0 {
            if let Some(caputred_variables) = (pattern).matches(&literal) {
                return Some(expression.evaluate(&caputred_variables, functions));
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct PatternPosition {
    start: Option<usize>,
    end: Option<usize>,
}

impl PatternPosition {
    pub fn new<T: errors::SourceCodePosition>(m: &T) -> PatternPosition {
        PatternPosition {
            start: m.get_start(),
            end: m.get_end(),
        }
    }
}

impl errors::SourceCodePosition for PatternPosition {
    fn get_start(&self) -> Option<usize> {
        self.start
    }
    fn get_end(&self) -> Option<usize> {
        self.end
    }
}
