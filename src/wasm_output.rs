use std::io::{Error, ErrorKind, Result, Write};
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    fn handle_output(value: &str);
    fn handle_error(value: &str);
}

pub struct FunctionWriter {
    function: Box<dyn Fn(&str) -> ()>,
}

impl Write for FunctionWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        (self.function)(match std::str::from_utf8(buf) {
            Ok(v) => v,
            Err(e) => Result::Err(Error::new(ErrorKind::InvalidData, e))?,
        });

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl FunctionWriter {
    pub fn create_stdout() -> FunctionWriter {
        FunctionWriter {
            function: Box::new(|b: &str| handle_output(b)),
        }
    }

    pub fn create_stderr() -> FunctionWriter {
        FunctionWriter {
            function: Box::new(|b: &str| handle_error(b)),
        }
    }
}
