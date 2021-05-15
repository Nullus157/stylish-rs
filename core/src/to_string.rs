use crate::{format, Display, String};

/// A trait for converting a value to a [`stylish::String`].
///
/// This trait is automatically implemented for any type which implements the [`stylish::Display`]
/// trait. As such, `ToString` shouldn’t be implemented directly: [`stylish::Display`] should be
/// implemented instead, and you get the `ToString` implementation for free.
pub trait ToStylishString {
    /// Converts the given value to a [`stylish::String`].
    ///
    /// ```rust
    /// struct Warning(&'static str);
    ///
    /// impl stylish::Display for Warning {
    ///     fn fmt(&self, f: &mut stylish::Formatter<'_>) -> stylish::Result {
    ///         f.with(stylish::Foreground(stylish::Color::Red)).write_str(self.0)
    ///     }
    /// }
    /// 
    /// use stylish::ToStylishString;
    ///
    /// let s: stylish::String = Warning("FIRE").to_stylish_string();
    /// assert_eq!(
    ///     stylish::html::format!("{:s}", Warning("FIRE")),
    ///     stylish::html::format!("{:s}", s),
    /// );
    /// ```
    fn to_stylish_string(&self) -> String;
}

impl<T> ToStylishString for T
where
    T: Display + ?Sized,
{
    fn to_stylish_string(&self) -> String {
        format!("{:s}", self)
    }
}
