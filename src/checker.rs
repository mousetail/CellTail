use crate::errors;
use crate::parser;
use crate::runtime::expression;
use crate::runtime::pattern;
use std::collections::HashSet;

#[derive(Copy, Clone)]
enum CheckerMode<'a> {
    Function,
    Pattern { function_names: &'a Vec<String> },
}

fn check_pattern(
    pat: &pattern::Pattern,
    mode: CheckerMode,
    variables: &mut HashSet<String>,
) -> errors::CellTailResult<()> {
    match pat {
        pattern::Pattern::Any => Ok(()),
        pattern::Pattern::Identifier(val) => {
            variables.insert(val.clone());
            Ok(())
        }
        pattern::Pattern::Literal(_) => Ok(()),
        pattern::Pattern::Expression(expr) => check_expression(expr, variables, mode),
        pattern::Pattern::Tuple(tup) => {
            for var in tup {
                check_pattern(var, mode, variables)?
            }
            Ok(())
        }
        pattern::Pattern::And(tup) => {
            for i in tup {
                check_pattern(i, mode, variables)?
            }
            Ok(())
        }
        pattern::Pattern::Or(tup) => {
            let all_equal = tup
                .iter()
                .map(|b| {
                    let mut variables_clone = variables.clone();
                    match check_pattern(b, mode, &mut variables_clone) {
                        Ok(_) => Ok(variables_clone),
                        Err(e) => Err(e),
                    }
                })
                .reduce(|a, b| match (a, b) {
                    (Err(a), _) | (_, Err(a)) => Err(a),
                    (Ok(u), Ok(v)) => {
                        if u == v {
                            Ok(u)
                        } else {
                            Err(errors::CellTailError::new(
                                &errors::UnkownLocationError,
                                format!("Parts of OR expression define different variables"),
                            ))
                        }
                    }
                })
                .unwrap_or(Err(errors::CellTailError::new(
                    &errors::UnkownLocationError,
                    format!("Empty OR statement"),
                )))?;

            *variables = all_equal;

            Ok(())
        }
        pattern::Pattern::Range(ab, bc) => ab
            .clone()
            .map_or(Ok(()), |k| check_expression(&k, variables, mode))
            .and_then(|_| {
                bc.clone()
                    .map_or(Ok(()), |k| check_expression(&k, variables, mode))
            }),
    }
}

fn check_expression(
    expr: &expression::Expression,
    variables: &HashSet<String>,
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
        let mut vars = HashSet::new();

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
        }

        errors::fallback_position(
            check_pattern(
                &rule.0,
                CheckerMode::Pattern {
                    function_names: &function_names,
                },
                &mut vars,
            ),
            &rule.2,
        )?;
        errors::fallback_position(
            check_expression(
                &rule.1,
                &vars,
                CheckerMode::Pattern {
                    function_names: &function_names,
                },
            ),
            &rule.2,
        )?
    }

    for function in &program.functions {
        for rule in &function.1 .0 {
            let mut vars = HashSet::new();
            check_pattern(&rule.0, CheckerMode::Function, &mut vars)?;
            errors::fallback_position(
                check_expression(&rule.1, &vars, CheckerMode::Function),
                &rule.2,
            )?
        }
    }

    Ok(())
}
