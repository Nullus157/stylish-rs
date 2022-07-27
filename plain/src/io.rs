use stylish_core::{
    io::{Result, Write},
    Style,
};

/// An adaptor to allow writing [`stylish`] attributed data to an output stream
/// by discarding style attributes.
///
/// ```rust
/// let mut writer = stylish::io::Plain::new(Vec::new());
/// stylish::write!(writer, "Hello {:(fg=red)}", "Ferris")?;
/// assert_eq!(writer.into_inner(), b"Hello Ferris");
/// # Ok::<(), std::io::Error>(())
/// ```
#[derive(Clone, Debug, Default)]
pub struct Plain<T> {
    inner: T,
}

impl<T: std::io::Write> Plain<T> {
    /// Wrap the given output stream in this adaptor.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Inherent delegation to
    /// [`stylish::io::Write::write_fmt`](stylish_core::io::Write::write_fmt) to
    /// not require a trait import.
    pub fn write_fmt(&mut self, args: stylish_core::Arguments<'_>) -> Result<()> {
        stylish_core::io::Write::write_fmt(self, args)
    }

    /// Get back the wrapped output stream.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: std::io::Write> Write for Plain<T> {
    fn write(&mut self, s: &[u8], _style: Style) -> Result<usize> {
        self.inner.write(s)
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }

    fn write_all(&mut self, s: &[u8], _style: Style) -> Result<()> {
        self.inner.write_all(s)
    }
}
