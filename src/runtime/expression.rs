use crate::runtime::literal::Literal;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    And,
    Or,
    Xor,
    Mod,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Tuple(Vec<Expression>),
    BinaryOperator(BinaryOperator, Box<Expression>, Box<Expression>),
    UnaryOperator(UnaryOperator, Box<Expression>),
    FunctionCall(String, Box<Expression>),
    Variable(String),
}

impl Expression {
    pub fn evaluate(&self, vars: &HashMap<String, Literal>) -> Literal {
        match self {
            Expression::Literal(v) => v.clone(),
            Expression::Tuple(v) => Literal::Tuple(v.iter().map(|i| i.evaluate(vars)).collect()),
            Expression::Variable(name) => {
                if name == "N" {
                    Literal::Null
                } else {
                    vars.get(name).expect("Undefined variable {name}").clone()
                }
            }
            _ => todo!("Other expression types not implemented yet"),
        }
    }
}
