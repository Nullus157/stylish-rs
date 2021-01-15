use crate::util;
use askama_escape::{escape, Html as AskamaHtml};
use stylish_core::{Result, Style, Write};

#[derive(Clone, Debug, Default)]
pub struct Html<T: std::fmt::Write> {
    inner: Option<T>,
    current: Option<Style>,
}

impl<T: std::fmt::Write> Html<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Some(inner),
            current: None,
        }
    }

    pub fn finish(mut self) -> Result<T> {
        if self.current.is_some() {
            write!(self.inner.as_mut().unwrap(), "</span>")?;
        }
        Ok(self.inner.take().unwrap())
    }
}

impl<T: std::fmt::Write> Write for Html<T> {
    fn write_str(&mut self, s: &str, style: Style) -> Result {
        if Some(style) != self.current {
            if self.current.is_some() {
                write!(self.inner.as_mut().unwrap(), "</span>")?;
            }
            write!(
                self.inner.as_mut().unwrap(),
                r#"<span style="color: {}; font-weight: {}">"#,
                util::color(style.color),
                util::intensity(style.intensity),
            )?;
            self.current = Some(style);
        }
        write!(self.inner.as_mut().unwrap(), "{}", escape(s, AskamaHtml))?;
        Ok(())
    }
}

impl<T: std::fmt::Write> Drop for Html<T> {
    fn drop(&mut self) {
        if self.current.is_some() {
            panic!("Dropped Html without finishing it");
        }
    }
}
