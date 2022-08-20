use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(isize),
    Tuple(Vec<Literal>),
    Null,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match self {
            Literal::Number(k) => {
                write!(f, "{}", k)
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
