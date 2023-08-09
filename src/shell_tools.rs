#[cfg(not(arch = "wasm32"))]
use atty::Stream;

pub fn set_color<T: std::io::Write>(color: u8, destination: &mut T) {
    if is_tty() {
        write!(destination, "\x1b[{}m", color).unwrap()
    }
}

#[cfg(not(arch = "wasm32"))]
fn is_tty() -> bool {
    atty::is(Stream::Stderr)
}

#[cfg(arch = "wasm32")]
fn is_tty() -> bool {
    false
}