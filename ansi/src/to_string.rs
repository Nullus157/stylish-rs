use crate::format;
use alloc::string::String;
use stylish_core::Display;

/// A trait for converting a value to a [`String`] with integrated ANSI styling.
///
/// This trait is automatically implemented for any type which implements the
/// [`stylish::Display`] trait. As such, `ToAnsiString` shouldn’t be implemented
/// directly: [`stylish::Display`] should be implemented instead, and you get
/// the `ToAnsiString` implementation for free.
///
/// [`stylish::Display`]: `stylish_core::Display`
pub trait ToAnsiString {
    /// Converts the given value to a [`String`] with integrated ANSI styling.
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
    /// use stylish::ToAnsiString;
    ///
    /// assert_eq!(Warning("FIRE").to_ansi_string(), "\x1b[31mFIRE\x1b[0m");
    /// ```
    fn to_ansi_string(&self) -> String;
}

impl<T> ToAnsiString for T
where
    T: Display + ?Sized,
{
    fn to_ansi_string(&self) -> String {
        format!("{:s}", self)
    }
}
