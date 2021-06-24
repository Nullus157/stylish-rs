#[cfg(feature = "std")]
use stylish_core::io;
use stylish_core::{Result, Style, Write};

/// An adaptor to allow writing [`stylish`] attributed data to an output stream
/// by discarding style attributes.
///
/// ```rust
/// use stylish::Write;
/// let mut writer = stylish::plain::Plain::new(String::new());
/// stylish::write!(writer, "Hello {:(fg=red)}", "Ferris");
/// assert_eq!(writer.into_inner(), "Hello Ferris");
/// ```
#[derive(Clone, Debug, Default)]
pub struct Plain<T> {
    inner: T,
}

impl<T> Plain<T> {
    /// Wrap the given output stream in this adaptor.
    pub fn new(inner: T) -> Self {
        Self { inner }
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

#[cfg(feature = "std")]
impl<T: std::io::Write> io::Write for Plain<T> {
    fn write(&mut self, s: &[u8], _style: Style) -> io::Result<usize> {
        self.inner.write(s)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }

    fn write_all(&mut self, s: &[u8], _style: Style) -> io::Result<()> {
        self.inner.write_all(s)
    }
}
