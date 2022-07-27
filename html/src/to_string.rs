use alloc::string::String;

use stylish_core::Display;

use crate::format;

/// A trait for converting a value to a [`String`] with integrated HTML styling.
///
/// This trait is automatically implemented for any type which implements the
/// [`stylish::Display`] trait. As such, `ToHtmlString` shouldn’t be implemented
/// directly: [`stylish::Display`] should be implemented instead, and you get
/// the `ToHtmlString` implementation for free.
///
/// [`stylish::Display`]: `stylish_core::Display`
pub trait ToHtmlString {
    /// Converts the given value to a [`String`] with integrated HTML styling.
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
    /// use stylish::ToHtmlString;
    ///
    /// assert_eq!(
    ///     Warning("FIRE").to_html_string(),
    ///     "<span style=color:red>FIRE</span>"
    /// );
    /// ```
    fn to_html_string(&self) -> String;
}

impl<T> ToHtmlString for T
where
    T: Display + ?Sized,
{
    fn to_html_string(&self) -> String {
        format!("{:s}", self)
    }
}
