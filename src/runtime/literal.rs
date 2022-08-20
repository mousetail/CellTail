#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(isize),
    Tuple(Vec<Literal>),
    Null,
}
