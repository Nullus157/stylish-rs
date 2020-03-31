use stylish::Style;

#[derive(Clone, Debug, Default)]
pub struct String {
    inner: std::string::String,
    current: Style,
}

#[derive(Clone, Debug, Default)]
pub struct Write<T> {
    inner: T,
    current: Style,
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
        Self { inner, current: Style::default() }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl stylish::Write for String {
    type Error = std::fmt::Error;

    fn write_str(&mut self, s: &str, style: Style) -> std::fmt::Result {
        use std::fmt::Write;

        if style != self.current {
            write!(
                self.inner,
                "[0m[{color};{intensity}m",
                color = style.color as u8,
                intensity = style.intensity as u8,
            )?;
            self.current = style;
        }
        self.inner.push_str(s);
        Ok(())
    }
}

impl<T: std::io::Write> stylish::Write for Write<T> {
    type Error = std::io::Error;

    fn write_str(&mut self, s: &str, style: Style) -> std::io::Result<()> {
        if style != self.current {
            write!(
                self.inner,
                "[0m[{color};{intensity}m",
                color = style.color as u8,
                intensity = style.intensity as u8,
            )?;
            self.current = style;
        }
        self.inner.write_all(s.as_bytes())?;
        Ok(())
    }
}
