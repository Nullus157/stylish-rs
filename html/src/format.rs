use alloc::string::String;

use stylish_core::Arguments;

use crate::Html;

#[cfg(feature = "macros")]
/// Create a [`String`] with inline ANSI escape codes styling its formatted
/// data.
///
/// ```rust
/// assert_eq!(
///     stylish::html::format!("Hello {:(fg=red)}", "Ferris"),
///     "Hello <span style=color:red>Ferris</span>",
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
/// the attributes into HTML elements.
///
/// ```rust
/// assert_eq!(
///     stylish::html::format(stylish::format_args!(
///         "Hello {:(fg=red)}",
///         "Ferris"
///     )),
///     "Hello <span style=color:red>Ferris</span>",
/// );
/// ```
pub fn format(args: Arguments<'_>) -> String {
    let mut html = Html::new(String::new());
    html.write_fmt(args)
        .expect("a formatting trait implementation returned an error");
    html.finish().expect("String cannot fail")
}
