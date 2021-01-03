use stylish::Style;

pub use stylish_macros::format_plain as format;

#[derive(Clone, Debug, Default)]
pub struct String {
    inner: std::string::String,
}

#[derive(Clone, Debug, Default)]
pub struct Write<T> {
    inner: T,
}

impl String {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_inner(self) -> std::string::String {
        self.inner
    }
}

impl<T> Write<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl stylish::fmt::Write for String {
    fn write_str(&mut self, s: &str, _style: Style) -> std::fmt::Result {
        self.inner.push_str(s);
        Ok(())
    }
}

impl<T: std::io::Write> stylish::io::Write for Write<T> {
    fn write_str(&mut self, s: &str, _style: Style) -> std::io::Result<()> {
        self.inner.write_all(s.as_bytes())?;
        Ok(())
    }
}
