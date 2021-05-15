use crate::{Formatter, Result};

/// Format trait for the [`stylish` format, `{:s}`](stylish#implementing-a-style-for-a-type).
///
/// `Display` is similar to [`core::fmt::Display`], but allows attaching additional style
/// attributes to the output.
///
/// ```rust
/// struct Name(&'static str);
/// 
/// impl stylish::Display for Name {
///     fn fmt(&self, f: &mut stylish::Formatter<'_>) -> stylish::Result {
///         let color = match self.0 {
///             "Ferris" => stylish::Color::Red,
///             "Gorris" => stylish::Color::Cyan,
///             _ => stylish::Color::Default,
///         };
///         f.with(stylish::Foreground(color)).write_str(self.0)
///     }
/// }
/// 
/// let formatted = stylish::html::format!("Hello {:s} and {:s}", Name("Ferris"), Name("Gorris"));
/// assert_eq!(formatted, "Hello <span style=color:red>Ferris</span> and <span style=color:cyan>Gorris</span>");
/// ```
pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

impl<T: Display + ?Sized> Display for &T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        (&**self).fmt(f)
    }
}
