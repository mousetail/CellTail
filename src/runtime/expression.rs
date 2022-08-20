use crate::runtime::literal::Literal;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
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

impl BinaryOperator {
    pub fn apply(self, op1: Literal, op2: Literal) -> Literal {
        match op1 {
            Literal::Null => op2,
            Literal::Number(n) => match op2 {
                Literal::Null => op1,
                Literal::Number(n2) => match self {
                    BinaryOperator::Add => Literal::Number(Self::apply_add(n, n2)),
                    BinaryOperator::Subtract => Literal::Number(Self::apply_sub(n, n2)),
                    BinaryOperator::Multiply => Literal::Number(Self::apply_mul(n, n2)),
                    BinaryOperator::Divide => Self::apply_div(n, n2),
                    BinaryOperator::And => Literal::Number(Self::apply_and(n, n2)),
                    BinaryOperator::Or => Literal::Number(Self::apply_or(n, n2)),
                    BinaryOperator::Xor => Literal::Number(Self::apply_xor(n, n2)),
                    BinaryOperator::Mod => Self::apply_mod(n, n2),
                },

                // This is provisional, probably want to do something actually useful with this combination of types
                b @ Literal::Tuple(_) => Literal::Tuple(vec![Literal::Number(n), b]),
            },
            Literal::Tuple(b) => Literal::Tuple(
                [
                    b[..b.len() - 1].to_vec(),
                    vec![self.apply(b[b.len() - 1].clone(), op2)],
                ]
                .concat(),
            ),
        }
    }

    fn apply_add(op1: isize, op2: isize) -> isize {
        op1 + op2
    }

    fn apply_sub(op1: isize, op2: isize) -> isize {
        op1 - op2
    }

    fn apply_mul(op1: isize, op2: isize) -> isize {
        op1 * op2
    }
    fn apply_div(op1: isize, op2: isize) -> Literal {
        if op2 == 0 {
            Literal::Null
        } else {
            Literal::Number(op1 / op2)
        }
    }
    fn apply_mod(op1: isize, op2: isize) -> Literal {
        if op2 == 0 {
            Literal::Null
        } else {
            Literal::Number(op1 % op2)
        }
    }

    fn apply_and(op1: isize, op2: isize) -> isize {
        op1 & op2
    }

    fn apply_or(op1: isize, op2: isize) -> isize {
        op1 | op2
    }
    fn apply_xor(op1: isize, op2: isize) -> isize {
        op1 ^ op2
    }
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
            Expression::BinaryOperator(op, ex1, ex2) => {
                op.apply(Self::evaluate(&**ex1, vars), Self::evaluate(&**ex2, vars))
            }
            _ => todo!("Other expression types not implemented yet"),
        }
    }
}