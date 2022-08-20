use crate::runtime::literal::Literal;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Pattern {
    Literal(Literal),
    Identifier(String),
    // Any,
    Tuple(Vec<Pattern>),
}

impl Pattern {
    pub fn matches(&self, value: Literal) -> Option<HashMap<String, Literal>> {
        match self {
            Pattern::Literal(lit) => {
                if *lit == value {
                    Some(HashMap::new())
                } else {
                    // println!("No match: literals {:?} and {:?} don't match", lit, value);
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
                        // println!("No match: length is different {:?} vs {:?}", tup2, tup1);
                        return None;
                    }
                    let mut out = HashMap::new();
                    for (pat1, val1) in tup1.iter().zip(tup2) {
                        if let Some(vars) = pat1.matches(val1) {
                            out.extend(vars);
                        } else {
                            return None;
                        }
                    }

                    Some(out)
                } else {
                    None
                }
            }
        }
    }
}
