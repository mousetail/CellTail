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
}

impl Pattern {
    pub fn matches(&self, value: &Literal) -> Option<HashMap<String, Literal>> {
        match self {
            Pattern::Literal(lit) => {
                if lit == value {
                    Some(HashMap::new())
                } else {
                    //println!("No match: literals {:?} and {:?} don't match", lit, value);
                    None
                }
            }
            Pattern::Identifier(ident) => {
                let mut out = HashMap::new();
                out.insert(ident.clone(), value.clone());
                Some(out)
            }
            Pattern::Tuple(tup1) => {
                if let Literal::Tuple(tup2) = value {
                    if tup2.len() != tup1.len() {
                        //println!("No match: length is different {:?} vs {:?}", tup2, tup1);
                        return None;
                    }
                    let mut out = HashMap::new();
                    for (pat1, val1) in tup1.iter().zip(tup2) {
                        if let Some(vars) = pat1.matches(val1) {
                            for var in vars {
                                if let Some(previous_value) = out.get(&var.0) {
                                    if previous_value != &var.1 {
                                        //println!("Tuple doesn't match vars");
                                        return None;
                                    }
                                } else {
                                    out.insert(var.0, var.1);
                                }
                            }
                        } else {
                            //println!("Contents doesn't match");
                            return None;
                        }
                    }

                    Some(out)
                } else {
                    // println!("Matching tuple with non tuple");
                    None
                }
            }
            Pattern::Expression(expr) => {
                let new_value = expr.evaluate(&HashMap::new(), &HashMap::new());
                if &new_value == value {
                    Some(HashMap::new())
                } else {
                    //println!("Expression doesn't match");
                    None
                }
            }
            Pattern::Any => Some(HashMap::new()),
        }
    }
}
