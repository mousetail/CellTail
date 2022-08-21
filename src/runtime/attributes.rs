pub enum IOFormat {
    Characters,
    Numbers,
}

pub enum InputSource {
    StdIn(IOFormat),
    Arg(IOFormat),
    Constant(Vec<usize>),
}

pub struct Attributes {
    input_mode: InputSource,
    output_mode: IOFormat,
    debug: bool,
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
