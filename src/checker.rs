use crate::errors;
use crate::parser;
use crate::runtime::expression;
use crate::runtime::pattern;

#[derive(Copy, Clone)]
enum CheckerMode<'a> {
    Function,
    Pattern { function_names: &'a Vec<String> },
}

fn check_pattern(
    pat: &pattern::Pattern,
    mode: CheckerMode,
    variables: &mut Vec<String>,
) -> errors::CellTailResult<()> {
    match pat {
        pattern::Pattern::Any => Ok(()),
        pattern::Pattern::Identifier(val) => Ok(variables.push(val.clone())),
        pattern::Pattern::Literal(_) => Ok(()),
        pattern::Pattern::Expression(expr) => check_expression(expr, variables, mode),
        pattern::Pattern::Tuple(tup) => {
            for var in tup {
                check_pattern(var, mode, variables)?
            }
            Ok(())
        }
    }
}

fn check_expression(
    expr: &expression::Expression,
    variables: &Vec<String>,
    mode: CheckerMode,
) -> errors::CellTailResult<()> {
    match expr {
        expression::Expression::Literal(_) => Ok(()),
        expression::Expression::BinaryOperator(_, a, b) => {
            check_expression(a, variables, mode).and(check_expression(b, variables, mode))
        }
        expression::Expression::Tuple(z) => z
            .iter()
            .map(|i| check_expression(i, variables, mode))
            .collect::<errors::CellTailResult<()>>(),
        expression::Expression::UnaryOperator(_, z) => check_expression(z, variables, mode),
        expression::Expression::FunctionCall(funk, arguments) => match mode {
            CheckerMode::Function => Err(errors::CellTailError::new(
                &errors::UnkownLocationError,
                format!("Can't call function {funk:?} inside of a function"),
            )),
            CheckerMode::Pattern { function_names } => {
                if function_names.contains(funk) {
                    Ok(())
                } else {
                    Err(errors::CellTailError::new(
                        &errors::UnkownLocationError,
                        format!("Call to undefined function {funk:?}"),
                    ))
                }
            }
        }
        .and_then(|_| check_expression(arguments, variables, mode)),
        expression::Expression::Variable(var) => {
            if var == "N" {
                Ok(())
            } else if variables.contains(var) {
                Ok(())
            } else {
                Err(errors::CellTailError::new(
                    &errors::UnkownLocationError,
                    format!("Reference to unkown variable {var:?}\nNote: In patterns variables must be used outside complex expressions before they can be used inside complex expressions.\nNote: Existing vars: {variables:?}"),
                ))
            }
        }
    }
}

pub fn check_program(program: &parser::Program) -> errors::CellTailResult<()> {
    let function_names: Vec<_> = program.functions.iter().map(|i| i.0.clone()).collect();

    for rule in &program.rules.0 {
        let mut vars = vec![];

        if let pattern::Pattern::Tuple(a) = &rule.0 {
            if a.len() != 3 {
                Err(errors::CellTailError::new(
                    &errors::UnkownLocationError,
                    format!(
                        "One rule is matches {} elements instead of the required 3 (left, center, right) elements, but matches {:?}",
                        a.len(),
                        rule.0
                    ),
                ))?
            }
        } else {
            Err(errors::CellTailError::new(
                &errors::UnkownLocationError,
                format!(
                    "One rule is not a tuple that matches exactly 3 elements, but matches {:?}",
                    rule.0
                ),
            ))?
        }

        check_pattern(
            &rule.0,
            CheckerMode::Pattern {
                function_names: &function_names,
            },
            &mut vars,
        )?;
        check_expression(
            &rule.1,
            &vars,
            CheckerMode::Pattern {
                function_names: &function_names,
            },
        )?
    }

    for function in &program.functions {
        for rule in &function.1 .0 {
            let mut vars = vec![];
            check_pattern(&rule.0, CheckerMode::Function, &mut vars)?;
            check_expression(&rule.1, &vars, CheckerMode::Function)?
        }
    }

    Ok(())
}