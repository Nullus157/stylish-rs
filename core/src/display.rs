use crate::{Formatter, Result};

/// Format trait for the `stylish` format, `{:s}`.
///
/// `Display` is similar to [`core::fmt::Display`], but allows attaching additional style
/// attributes to the output.
pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

impl<T: Display + ?Sized> Display for &T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        (&**self).fmt(f)
    }
}
