use stylish::Style;

#[derive(Clone, Debug, Default)]
pub struct String {
    inner: std::string::String,
}

impl String {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_inner(self) -> std::string::String {
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
