use crate::runtime::expression::Expression;
use crate::runtime::literal::Literal;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Pattern {
    Literal(Literal),
    Identifier(String),
    Any,
    Tuple(Vec<Pattern>),
    Expression(Expression),
    And(Vec<Pattern>),
    Or(Vec<Pattern>),
}

impl Pattern {
    pub fn match_dict(&self, value: &Literal, variables: &mut HashMap<String, Literal>) -> bool {
        match self {
            Pattern::Literal(lit) => {
                if lit == value {
                    true
                } else {
                    //println!("No match: literals {:?} and {:?} don't match", lit, value);
                    false
                }
            }
            Pattern::Identifier(ident) => {
                if variables.contains_key(ident) {
                    variables.get(ident) == Some(value)
                } else {
                    variables.insert(ident.clone(), value.clone());
                    true
                }
            }
            Pattern::Tuple(tup1) => {
                if let Literal::Tuple(tup2) = value {
                    if tup2.len() != tup1.len() {
                        //println!("No match: length is different {:?} vs {:?}", tup2, tup1);
                        false
                    } else {
                        tup1.iter()
                            .zip(tup2)
                            .all(|(pat, val)| pat.match_dict(val, variables))
                    }
                } else {
                    // println!("Matching tuple with non tuple");
                    false
                }
            }
            Pattern::Expression(expr) => {
                let new_value = expr.evaluate(variables, &HashMap::new());
                &new_value == value
            }
            Pattern::And(parts) => {
                todo!();
            }
            Pattern::Or(parts) => {
                todo!();
            }
            Pattern::Any => true,
        }
    }

    pub fn matches(&self, value: &Literal) -> Option<HashMap<String, Literal>> {
        let mut result = HashMap::new();

        if self.match_dict(value, &mut result) {
            Some(result)
        } else {
            None
        }
    }
}
