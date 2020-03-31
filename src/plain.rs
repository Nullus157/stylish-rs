use stylish::Style;

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

impl stylish::Write for String {
    type Error = std::fmt::Error;

    fn write_str(&mut self, s: &str, _style: Style) -> std::fmt::Result {
        self.inner.push_str(s);
        Ok(())
    }
}

impl<T: std::io::Write> stylish::Write for Write<T> {
    type Error = std::io::Error;

    fn write_str(&mut self, s: &str, _style: Style) -> std::io::Result<()> {
        self.inner.write_all(s.as_bytes())?;
        Ok(())
    }
}
