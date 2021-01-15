use crate::Ansi;
use stylish_core::{Arguments, Write};

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        let res = $crate::format($crate::__export::format_args!($($arg)*));
        res
    }}
}

pub fn format(args: Arguments<'_>) -> String {
    let mut ansi = Ansi::new(String::new());
    ansi.write_fmt(args)
        .expect("a formatting trait implementation returned an error");
    ansi.finish().expect("String cannot fail")
}
