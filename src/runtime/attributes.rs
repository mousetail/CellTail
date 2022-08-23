pub enum IOFormat {
    Characters,
    Numbers,
}

pub enum InputSource {
    StdIn(IOFormat),
    Arg(IOFormat),
    Constant(Vec<isize>),
}

pub struct Attributes {
    pub input_mode: InputSource,
    pub output_mode: IOFormat,
    pub debug: bool,
}

impl Attributes {
    pub fn new() -> Attributes {
        Attributes {
            input_mode: InputSource::Arg(IOFormat::Characters),
            output_mode: IOFormat::Characters,
            debug: false,
        }
    }
}
