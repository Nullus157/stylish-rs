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

/// The `format` function takes a [`stylish::Arguments`] struct and returns the resulting
/// attributed and formatted [`stylish::String`].
///
/// The [`stylish::Arguments`] instance can be created with the [`stylish::format_args!`] macro.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let s = stylish::format(stylish::format_args!("Hello, {:(fg=green)}!", "world"));
/// assert_eq!(
///     stylish::html::format!("{:s}", s),
///     "Hello, <span style=color:green>world</span>!"
/// );
/// ```
///
///
/// Please note that using [`stylish::format!`] might be preferable. Example:
///
/// ```rust
/// let s = stylish::format!("Hello, {:(fg=green)}!", "world");
/// assert_eq!(
///     stylish::html::format!("{:s}", s),
///     "Hello, <span style=color:green>world</span>!"
/// );
/// ```
pub fn format(args: Arguments<'_>) -> String {
    let mut output = String::new();
    output
        .write_fmt(args)
        .expect("a formatting trait implementation returned an error");
    output
}
