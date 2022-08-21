use std::io::*;

use crate::parser;
use crate::runtime::literal::Literal;
use crate::runtime::pattern_list::PatternList;

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

pub fn interpret(program: parser::Program, input: Vec<u8>) {
    let mut cells: Vec<Cell> = input
        .iter()
        .map(|i| Cell {
            value_from_left: Literal::Null,
            value_from_top: Literal::Number(*i as isize),
            value_from_right: Literal::Null,
        })
        .collect();

    let mut modified = true;
    while modified {
        modified = false;
        let mut next_value = cells.clone();
        let mut cell_offset = 0;
        for (index, cell) in cells.iter().enumerate() {
            if cell.value_from_left != Literal::Null
                || cell.value_from_top != Literal::Null
                || cell.value_from_right != Literal::Null
            {
                if let Some(raw_result) =
                    program
                        .rules
                        .apply_first_matching_pattern(Literal::Tuple(vec![
                            cell.value_from_left.clone(),
                            cell.value_from_top.clone(),
                            cell.value_from_right.clone(),
                        ]))
                {
                    let result = parse_literal(raw_result);

                    if index == 0 && result.0 != Literal::Null {
                        next_value.insert(0, Cell::new());
                        cell_offset += 1;
                        modified = true;
                    }

                    if index + cell_offset >= next_value.len() - 1 {
                        next_value.push(Cell::new());
                        modified = true;
                    }

                    if if (index + cell_offset == 0) {
                        &Literal::Null
                    } else {
                        &next_value[index + cell_offset - 1].value_from_right
                    } != &result.0
                        || next_value[index + cell_offset].value_from_top != result.1
                        || if index + cell_offset >= next_value.len() - 1 {
                            &Literal::Null
                        } else {
                            &next_value[index + cell_offset + 1].value_from_right
                        } != &result.2
                    {
                        modified = true;
                    }

                    if index + cell_offset > 0 {
                        next_value[index + cell_offset - 1].value_from_right = result.0;
                    }
                    next_value[index + cell_offset].value_from_top = result.1;
                    if index + cell_offset < next_value.len() - 1 {
                        next_value[index + cell_offset + 1].value_from_left = result.2;
                    }

                    break;
                }
            }
        }

        cells = next_value;

        for cell in &cells {
            print!(
                "(L={} M={} R={})\t",
                cell.value_from_left, cell.value_from_top, cell.value_from_right
            );
        }
        println!();

        std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
    }

    println!(
        "{}",
        cells
            .iter()
            .filter(|i| if let Literal::Null = i.value_from_top {
                false
            } else {
                true
            })
            .map(|i| if let Literal::Number(k) = i.value_from_top {
                k.try_into().ok().unwrap_or(80u8) as char
            } else {
                '?'
            })
            .collect::<String>()
    );
}
