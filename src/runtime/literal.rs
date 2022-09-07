use std::fmt;

#[derive(Debug, Clone, PartialEq, Ord, PartialOrd, Eq)]
pub enum Literal {
    Null,
    Number(isize),
    Tuple(Vec<Literal>),
}

impl Literal {
    pub fn new_string_literal(item: &[u8]) -> Literal {
        if item.len() == 0 {
            return Literal::Null;
        } else {
            return Literal::Tuple(vec![
                Literal::Number(item[0] as isize),
                Literal::new_string_literal(&item[1..]),
            ]);
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match self {
            Literal::Number(k) => {
                f.pad(&format!("{}", k))
            }
            Literal::Null => {
                write!(f, "NULL")
            }
            Literal::Tuple(tup) => {
                write!(f, "(")?;
                for elem in tup {
                    write!(f, "{}, ", elem)?;
                }
                write!(f, ")")
            }
        }
    }
}
