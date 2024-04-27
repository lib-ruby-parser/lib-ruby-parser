mod parse;
pub use parse::{token_name, Parser};

#[cfg(feature = "debug-parser")]
macro_rules! println_if_debug_parser {
    () => {
        println!("");
    };
    ($($arg:tt)*) => {{
        use std::io::Write;
        let mut stderr = std::io::stderr();
        stderr.write_fmt(core::format_args!($($arg)*)).unwrap();
        stderr.write(b"\n").unwrap();
    }};
}
#[cfg(not(feature = "debug-parser"))]
macro_rules! println_if_debug_parser {
    () => {};
    ($($arg:tt)*) => {{}};
}
pub(crate) use println_if_debug_parser;
