use stylish_core::{io, Result, Style, Write};

#[derive(Clone, Debug, Default)]
pub struct Plain<T> {
    inner: T,
}

impl<T> Plain<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: std::fmt::Write> Write for Plain<T> {
    fn write_str(&mut self, s: &str, _style: Style) -> Result {
        self.inner.write_str(s)
    }
}

impl<T: std::io::Write> io::Write for Plain<T> {
    fn write(&mut self, s: &[u8], _style: Style) -> io::Result<usize> {
        self.inner.write(s)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }

    fn write_all(&mut self, s: &[u8], _style: Style) -> io::Result<()> {
        self.inner.write_all(s)
    }
}
