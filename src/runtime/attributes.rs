#[derive(Debug)]
pub enum IOFormat {
    Characters,
    Numbers,
}

#[derive(Debug)]
pub enum InputSource {
    StdIn(IOFormat),
    Arg(IOFormat),
    Constant(Vec<isize>),
}

#[derive(Debug)]
pub struct Attributes {
    pub input_mode: InputSource,
    pub output_mode: IOFormat,
    pub debug: bool,
    pub max_iterations: Option<isize>
}

impl Attributes {
    pub fn new() -> Attributes {
        Attributes {
            input_mode: InputSource::Arg(IOFormat::Characters),
            output_mode: IOFormat::Characters,
            debug: false,
            max_iterations: None
        }
    }
}
