use crate::errors;
use crate::parser;
use crate::runtime::attributes;
use crate::runtime::literal::Literal;
use std::io::Read;

#[derive(Clone, PartialEq, Debug)]
struct Cell {
    value_from_left: Literal,
    value_from_top: Literal,
    value_from_right: Literal,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            value_from_left: Literal::Null,
            value_from_top: Literal::Null,
            value_from_right: Literal::Null,
        }
    }
}

fn parse_literal(lit: Literal) -> (Literal, Literal, Literal) {
    match lit {
        Literal::Tuple(t) => {
            assert_eq!(t.len(), 3);
            (t[0].clone(), t[1].clone(), t[2].clone())
        }
        b => (Literal::Null, b, Literal::Null),
    }
}

fn interpret_iteration(cells: &Vec<Cell>, program: &parser::Program) -> Vec<Cell> {
    let mut next_value = cells.clone();
    let mut cell_offset = 0;
    for (index, cell) in cells.iter().enumerate() {
        if cell.value_from_left != Literal::Null
            || cell.value_from_top != Literal::Null
            || cell.value_from_right != Literal::Null
        {
            if let Some(raw_result) = program.rules.apply_first_matching_pattern(
                Literal::Tuple(vec![
                    cell.value_from_left.clone(),
                    cell.value_from_top.clone(),
                    cell.value_from_right.clone(),
                ]),
                &program.functions,
            ) {
                let result = parse_literal(raw_result);

                if index == 0 && result.0 != Literal::Null {
                    next_value.insert(0, Cell::new());
                    cell_offset += 1;
                }

                if index + cell_offset >= next_value.len() - 1 {
                    next_value.push(Cell::new());
                }

                if index + cell_offset > 0 {
                    next_value[index + cell_offset - 1].value_from_right = result.0;
                }
                next_value[index + cell_offset].value_from_top = result.1;
                if index + cell_offset < next_value.len() - 1 {
                    next_value[index + cell_offset + 1].value_from_left = result.2;
                }
            }
        }
    }

    return next_value;
}

fn print_cells(cells: &Vec<Cell>) {
    for cell in cells {
        if cell.value_from_top != Literal::Null {
            print!(
                "({:0>4}, {:0>4}, {:0>4}) ",
                cell.value_from_left, cell.value_from_top, cell.value_from_right
            );
        }
    }
    println!();
}

fn interpret(
    program: &parser::Program,
    input: Vec<isize>,
) -> errors::CellTailResult<Vec<Option<isize>>> {
    let mut cells: Vec<Cell> = input
        .iter()
        .map(|i| Cell {
            value_from_left: Literal::Null,
            value_from_top: Literal::Number(*i as isize),
            value_from_right: Literal::Null,
        })
        .collect();

    if program.attributes.debug {
        print_cells(&cells);
    }

    let mut modified = true;
    while modified {
        let new_cells = interpret_iteration(&cells, program);
        modified = cells != new_cells;
        cells = new_cells;

        if program.attributes.debug {
            print_cells(&cells);

            std::thread::sleep(std::time::Duration::from_secs_f32(0.25))
        }

        // std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
    }

    Ok(cells
        .iter()
        .filter(|i| {
            if let Literal::Null = i.value_from_top {
                false
            } else {
                true
            }
        })
        .map(|i| {
            if let Literal::Number(k) = i.value_from_top {
                Some(k)
            } else {
                None
            }
        })
        .collect::<Vec<_>>())
}

fn get_contents(data: &str, format: &attributes::IOFormat) -> errors::CellTailResult<Vec<isize>> {
    match format {
        attributes::IOFormat::Characters => Ok(data.chars().map(|i| i as u8 as isize).collect()),
        attributes::IOFormat::Numbers => data
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                errors::CellTailError::new(
                    &errors::UnkownLocationError,
                    format!("Failed to parse command line arguments: {:?}", e),
                )
            }),
    }
}

pub fn run_program(
    program: parser::Program,
    command_line_arguments: Vec<String>,
) -> errors::CellTailResult<()> {
    let input = match &program.attributes.input_mode {
        attributes::InputSource::Arg(m) => {
            if command_line_arguments.len() != 1 {
                Err(errors::CellTailError::new(
                    &errors::UnkownLocationError,
                    format!("Expected a command line argument"),
                ))?
            }
            get_contents(&command_line_arguments[0], m)?
        }
        attributes::InputSource::StdIn(m) => {
            let mut file_contents: Vec<u8> = vec![];
            std::io::stdin()
                .read_to_end(&mut file_contents)
                .expect("Failed to read contents of STDIN");

            get_contents(std::str::from_utf8(&file_contents).unwrap(), m)?
        }
        attributes::InputSource::Constant(constant) => constant.to_vec(),
    };

    let result = interpret(&program, input)?;

    match &program.attributes.output_mode {
        attributes::IOFormat::Characters => {
            print!(
                "{}",
                result
                    .iter()
                    .map(|i| match i {
                        Some(i @ 0..=256) => *i as u8 as char,
                        _ => '?',
                    })
                    .collect::<String>()
            )
        }
        attributes::IOFormat::Numbers => {
            print!(
                "{}",
                result
                    .iter()
                    .map(|i| match i {
                        Some(i) => format!("{}, ", i),
                        _ => "???, ".to_owned(),
                    })
                    .collect::<String>()
            )
        }
    }

    Ok(())
}
