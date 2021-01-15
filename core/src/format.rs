use crate::{Arguments, String, Write};

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        let res = $crate::format($crate::format_args!($($arg)*));
        res
    }};
}

pub fn format(args: Arguments<'_>) -> String {
    let mut output = String::new();
    output
        .write_fmt(args)
        .expect("a formatting trait implementation returned an error");
    output
}
