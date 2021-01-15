use crate::Html;
use stylish_core::{Arguments, Write};

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        let res = $crate::format($crate::__export::format_args!($($arg)*));
        res
    }}
}

pub fn format(args: Arguments<'_>) -> String {
    let mut html = Html::new(String::new());
    html.write_fmt(args)
        .expect("a formatting trait implementation returned an error");
    html.finish().expect("String cannot fail")
}
