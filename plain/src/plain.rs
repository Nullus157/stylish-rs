use stylish_core::{Result, Style, Write};

/// An adaptor to allow writing [`stylish`] attributed data to an output stream
/// by discarding style attributes.
///
/// ```rust
/// let mut writer = stylish::Plain::new(String::new());
/// stylish::write!(writer, "Hello {:(fg=red)}", "Ferris");
/// assert_eq!(writer.into_inner(), "Hello Ferris");
/// ```
#[derive(Clone, Debug, Default)]
pub struct Plain<T> {
    inner: T,
}

impl<T: core::fmt::Write> Plain<T> {
    /// Wrap the given output stream in this adaptor.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Inherent delegation to
    /// [`stylish::Write::write_fmt`](stylish_core::Write::write_fmt) to not
    /// require a trait import.
    pub fn write_fmt(&mut self, args: stylish_core::Arguments<'_>) -> Result {
        stylish_core::Write::write_fmt(self, args)
    }

    /// Get back the wrapped output stream.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: core::fmt::Write> Write for Plain<T> {
    fn write_str(&mut self, s: &str, _style: Style) -> Result {
        self.inner.write_str(s)
    }
}
