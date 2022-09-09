#[cfg(not(arch = "wasm32"))]
use atty::Stream;
use serde::Serialize;

#[cfg(not(arch = "wasm32"))]
fn is_tty() -> bool {
    atty::is(Stream::Stderr)
}

#[cfg(arch = "wasm32")]
fn is_tty() -> bool {
    false
}

pub trait SourceCodePosition {
    fn get_start(&self) -> Option<usize>;
    fn get_end(&self) -> Option<usize>;
}

#[derive(Debug)]
pub struct PointError(pub usize);
#[derive(Debug)]
pub struct RangeError(pub usize, pub usize);
#[derive(Debug)]
pub struct UnkownLocationError;

impl SourceCodePosition for PointError {
    fn get_start(&self) -> Option<usize> {
        Some(self.0)
    }
    fn get_end(&self) -> Option<usize> {
        None
    }
}

impl SourceCodePosition for RangeError {
    fn get_start(&self) -> Option<usize> {
        Some(self.0)
    }
    fn get_end(&self) -> Option<usize> {
        Some(self.1)
    }
}

impl SourceCodePosition for UnkownLocationError {
    fn get_start(&self) -> Option<usize> {
        None
    }
    fn get_end(&self) -> Option<usize> {
        None
    }
}

impl<T: SourceCodePosition> SourceCodePosition for Option<T> {
    fn get_start(&self) -> Option<usize> {
        self.as_ref().and_then(|i| i.get_start())
    }

    fn get_end(&self) -> Option<usize> {
        self.as_ref().and_then(|i| i.get_end())
    }
}

#[derive(Debug, Serialize)]
pub struct CellTailError {
    start: Option<usize>,
    end: Option<usize>,
    description: String,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct LinePosition {
    line_number: usize,
    column_number: usize,
    line_start: usize,
    line_end: usize,
}

impl CellTailError {
    pub fn new<T>(location: &T, message: String) -> CellTailError
    where
        T: SourceCodePosition + std::fmt::Debug,
    {
        if location.get_start() > location.get_end() {
            panic!("{:?} is invalid: start > end", location);
        }
        CellTailError {
            start: location.get_start(),
            end: location.get_end(),
            description: message,
        }
    }

    fn set_color<T: std::io::Write>(color: u8, destination: &mut T) {
        if is_tty() {
            write!(destination, "\x1b[{}m", color).unwrap()
        }
    }

    fn is_location_known(&self) -> bool {
        self.start.is_some() || self.end.is_some()
    }

    fn get_line_number(source: &Vec<char>, position: usize) -> LinePosition {
        let lines: Vec<_> = source
            .iter()
            .enumerate()
            .filter(|(_, i)| **i == '\n')
            .map(|(b, _)| b)
            .enumerate()
            .collect();

        let previous_line = lines
            .iter()
            .rev()
            .filter(|(_, original_pos)| *original_pos < position)
            .next();

        let next_line = lines
            .iter()
            .filter(|(_, original_pos)| *original_pos >= position)
            .next();

        match (previous_line, next_line) {
            (None, None) => LinePosition {
                line_number: 0,
                column_number: position,
                line_start: 0,
                line_end: source.len(),
            },
            (Some((line_number, line_start)), None) => LinePosition {
                line_number: *line_number,
                column_number: position - line_start,
                line_start: *line_start + 1,
                line_end: source.len(),
            },
            (None, Some((_line_end_number, line_end))) => LinePosition {
                line_number: 0,
                column_number: position,
                line_start: 0,
                line_end: *line_end,
            },
            (Some((line_number, line_start)), Some((_next_line_number, line_end))) => {
                LinePosition {
                    line_number: *line_number,
                    column_number: position - line_start,
                    line_start: *line_start + 1,
                    line_end: *line_end,
                }
            }
        }
    }

    fn highlight_line<T: std::io::Write>(
        source: &Vec<char>,
        line_start: usize,
        error_start: usize,
        line_end: usize,
        error_end: usize,
        output: &mut T,
    ) {
        Self::set_color(33, output);
        writeln!(
            output,
            "> \t{}",
            source[line_start..line_end].iter().collect::<String>()
        )
        .unwrap();
        writeln!(
            output,
            "> \t{}{}",
            " ".repeat(error_start),
            "^".repeat(error_end - error_start)
        )
        .unwrap();
        Self::set_color(0, output);
    }

    fn highlight_error<T: std::io::Write>(
        source: &Vec<char>,
        start_info: LinePosition,
        end_info: LinePosition,
        output: &mut T,
    ) {
        if start_info.line_number == end_info.line_number {
            Self::highlight_line(
                source,
                start_info.line_start,
                start_info.column_number,
                end_info.line_end,
                end_info.column_number,
                output,
            );
        } else {
            Self::highlight_line(
                source,
                start_info.line_start,
                start_info.column_number,
                start_info.line_end,
                start_info.line_end - start_info.line_start,
                output,
            );
            Self::highlight_line(
                source,
                end_info.line_start,
                0,
                end_info.line_end,
                end_info.column_number,
                output,
            )
        }
    }

    pub fn print<T: std::io::Write>(&self, source: Vec<char>, destination: &mut T) {
        Self::set_color(31, destination);
        writeln!(destination, "There was a error running the code").unwrap();
        Self::set_color(0, destination);

        if let (Some(start_pos), Some(end_pos)) = (self.start, self.end) {
            let line_info = Self::get_line_number(&source, start_pos);
            let line_end_info = Self::get_line_number(&source, end_pos);

            writeln!(
                destination,
                "Line {} Column {} (pos {start_pos}) to line {} column {} (pos: {end_pos}):",
                line_info.line_number,
                line_info.column_number,
                line_end_info.line_number,
                line_end_info.column_number,
            )
            .unwrap();

            Self::highlight_error(&source, line_info, line_end_info, destination);
        } else if let Some(pos) = self.start.or(self.end) {
            let line_info = Self::get_line_number(&source, pos);

            writeln!(
                destination,
                "At line {} column {}:",
                line_info.line_number, line_info.column_number
            )
            .unwrap();

            Self::highlight_error(
                &source,
                line_info,
                LinePosition {
                    column_number: line_info.column_number + 1,
                    ..line_info
                },
                destination,
            );
        } else {
            writeln!(destination, "At an unkown location: ").unwrap()
        }

        writeln!(destination, "{}", self.description).unwrap()
    }
}

pub type CellTailResult<T> = Result<T, CellTailError>;

pub fn fallback_position<T, E: SourceCodePosition + std::fmt::Debug>(
    res: CellTailResult<T>,
    fallback: &E,
) -> CellTailResult<T> {
    res.map_err(|u| {
        if u.is_location_known() {
            u
        } else {
            CellTailError::new(fallback, u.description)
        }
    })
}
