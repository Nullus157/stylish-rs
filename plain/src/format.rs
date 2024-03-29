use alloc::string::String;

use stylish_core::Arguments;

use crate::Plain;

#[cfg(feature = "macros")]
/// Create a [`String`] by discarding styling attributes.
///
/// ```rust
/// assert_eq!(
///     stylish::plain::format!("Hello {:(fg=red)}", "Ferris"),
///     "Hello Ferris",
/// );
/// ```
#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        let res = $crate::format($crate::𓀄::format_args!($($arg)*));
        res
    }}
}

/// Render the given attributed [`Arguments`] into a [`String`] by discarding
/// the style attributes.
///
/// ```rust
/// assert_eq!(
///     stylish::plain::format(stylish::format_args!(
///         "Hello {:(fg=red)}",
///         "Ferris"
///     )),
///     "Hello Ferris",
/// );
/// ```
#[inline]
pub fn format(args: Arguments<'_>) -> String {
    let mut output = Plain::new(String::new());
    output
        .write_fmt(args)
        .expect("a formatting trait implementation returned an error");
    output.into_inner()
}
