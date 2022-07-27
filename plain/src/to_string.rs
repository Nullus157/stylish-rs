use alloc::string::String;

use stylish_core::Display;

use crate::format;

/// A trait for converting a value to a [`String`] without styling.
///
/// This trait is automatically implemented for any type which implements the
/// [`stylish::Display`] trait. As such, `ToPlainString` shouldn’t be
/// implemented directly: [`stylish::Display`] should be implemented instead,
/// and you get the `ToPlainString` implementation for free.
///
/// [`stylish::Display`]: `stylish_core::Display`
pub trait ToPlainString {
    /// Converts the given value to a [`String`] without styling.
    ///
    /// ```rust
    /// struct Warning(&'static str);
    ///
    /// impl stylish::Display for Warning {
    ///     fn fmt(&self, f: &mut stylish::Formatter<'_>) -> stylish::Result {
    ///         f.with(stylish::Foreground(stylish::Color::Red))
    ///             .write_str(self.0)
    ///     }
    /// }
    ///
    /// use stylish::ToPlainString;
    ///
    /// assert_eq!(Warning("FIRE").to_plain_string(), "FIRE");
    /// ```
    fn to_plain_string(&self) -> String;
}

impl<T> ToPlainString for T
where
    T: Display + ?Sized,
{
    fn to_plain_string(&self) -> String {
        format!("{:s}", self)
    }
}
