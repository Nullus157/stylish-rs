use stylish::Style;

pub use stylish_macros::format_ansi as format;

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
        Self {
            inner,
            current: Style::default(),
        }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl stylish::fmt::Write for String {
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

impl<T: std::io::Write> stylish::io::Write for Write<T> {
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
