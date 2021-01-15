use crate::util;
use stylish_core::{Result, Style, Write};

#[derive(Clone, Debug, Default)]
pub struct Ansi<T: std::fmt::Write> {
    inner: Option<T>,
    current: Option<Style>,
}

impl<T: std::fmt::Write> Ansi<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Some(inner),
            current: None,
        }
    }

    pub fn finish(mut self) -> Result<T> {
        if self.current.is_some() {
            write!(self.inner.as_mut().unwrap(), "[0m")?;
        }
        Ok(self.inner.take().unwrap())
    }
}

impl<T: std::fmt::Write> Write for Ansi<T> {
    fn write_str(&mut self, s: &str, style: Style) -> Result {
        if Some(style) != self.current {
            write!(
                self.inner.as_mut().unwrap(),
                "[{};{}m",
                util::color(style.color),
                util::intensity(style.intensity),
            )?;
            self.current = Some(style);
        }
        write!(self.inner.as_mut().unwrap(), "{}", s)?;
        Ok(())
    }
}

impl<T: std::fmt::Write> Drop for Ansi<T> {
    fn drop(&mut self) {
        if self.current.is_some() {
            panic!("Dropped Ansi without finishing it");
        }
    }
}
