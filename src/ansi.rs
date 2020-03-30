use std::fmt::Write;
use stylish::{Style, Result};

#[derive(Clone, Debug, Default)]
pub struct String {
    inner: std::string::String,
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

impl stylish::Write for String {
    fn write_str(&mut self, s: &str, style: Style) -> Result {
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
