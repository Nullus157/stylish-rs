use crate::Ansi;
use alloc::string::String;
use stylish_core::Arguments;

#[cfg(feature = "macros")]
/// Create a [`String`] with inline ANSI escape codes styling its formatted
/// data.
///
/// ```rust
/// assert_eq!(
///     stylish::ansi::format!("Hello {:(fg=red)}", "Ferris"),
///     "Hello \x1b[31mFerris\x1b[0m",
/// );
/// ```
#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        let res = $crate::format($crate::𓀄::format_args!($($arg)*));
        res
    }}
}

/// Render the given attributed [`Arguments`] into a [`String`] by converting
/// the attributes into inline ANSI escape codes.
///
/// ```rust
/// assert_eq!(
///     stylish::ansi::format(stylish::format_args!("Hello {:(fg=red)}", "Ferris")),
///     "Hello \x1b[31mFerris\x1b[0m",
/// );
/// ```
pub fn format(args: Arguments<'_>) -> String {
    let mut ansi = Ansi::new(String::new());
    ansi.write_fmt(args)
        .expect("a formatting trait implementation returned an error");
    ansi.finish().expect("String cannot fail")
}
