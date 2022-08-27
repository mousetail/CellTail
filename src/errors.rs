pub trait SourceCodePosition {
    fn get_start(&self) -> Option<usize>;
    fn get_end(&self) -> Option<usize>;
}

#[derive(Debug)]
pub struct PointError(pub usize);
#[derive(Debug)]
pub struct RangeError(pub usize, pub usize);

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

pub struct CellTailError {
    start: Option<usize>,
    end: Option<usize>,
    description: String,
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

    fn get_line_number(source: &Vec<char>, position: usize) -> usize {
        return source[..position].iter().filter(|i| **i == '\n').count();
    }

    pub fn print(&self, source: Vec<char>) {
        eprintln!("There was a error running the code");

        if let (Some(start_pos), Some(end_pos)) = (self.start, self.end) {
            eprintln!(
                "Line {} at ({start_pos} {end_pos}): {:?}",
                Self::get_line_number(&source, start_pos),
                source[start_pos..end_pos].iter().collect::<String>()
            )
        } else if let Some(pos) = self.start.or(self.end) {
            eprint!(
                "Line {} at ({pos}) {:?}",
                Self::get_line_number(&source, pos),
                source[pos.saturating_sub(5)..pos + 5]
                    .iter()
                    .collect::<String>()
            )
        } else {
            eprint!("At unkown location")
        }

        eprintln!("{}", self.description)
    }
}

pub type CellTailResult<T> = Result<T, CellTailError>;
