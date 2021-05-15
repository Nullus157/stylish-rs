use crate::{Arguments, String, Write};

/// Create a [`stylish::String`] using interpolation of runtime elements.
///
/// The first argument `format!` receives is a format string literal, the rest are parameters
/// interpolated based on the format string. See the main [`stylish`] docs for more info on how the
/// interpolation is controlled.
///
/// ```rust
/// let s: stylish::String = stylish::format!("Hello {:(fg=green)}!", "World");
///
/// assert_eq!(
///     stylish::html::format!("{:s}", s),
///     "Hello <span style=color:green>World</span>!",
/// );
/// ```
#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        let res = $crate::format($crate::format_args!($($arg)*));
        res
    }};
}

// pub for macros
pub fn format(args: Arguments<'_>) -> String {
    let mut output = String::new();
    output
        .write_fmt(args)
        .expect("a formatting trait implementation returned an error");
    output
}
