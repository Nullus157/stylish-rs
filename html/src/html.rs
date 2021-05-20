use crate::util;
use core::fmt;
use askama_escape::{escape, Html as AskamaHtml};
use stylish_core::{Style, Write};

#[derive(Clone, Debug, Default)]
pub struct Html<T: core::fmt::Write> {
    inner: T,
    current: Style,
}

impl<T: core::fmt::Write> Html<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            current: Style::default(),
        }
    }

    pub fn finish(mut self) -> Result<T, fmt::Error> {
        if self.current != Style::default() {
            self.inner.write_str("</span>")?;
        }
        Ok(self.inner)
    }
}

impl<T: fmt::Write> Write for Html<T> {
    fn write_str(&mut self, s: &str, style: Style) -> fmt::Result {
        if style == Style::default() {
            if self.current != Style::default() {
                self.inner.write_str("</span>")?;
            }
        } else if style != self.current {
            let diff = style.diff_from(Style::default());
            let segments = [
                diff.foreground.map(util::foreground),
                diff.background.map(util::background),
                diff.intensity.map(util::intensity),
            ];
            let mut segments = segments.iter().filter_map(|&s| s);
            if let Some(segment) = segments.next() {
                if self.current != Style::default() {
                    self.inner.write_str("</span>")?;
                }
                self.inner.write_str("<span style=")?;
                self.inner.write_str(segment)?;
                for segment in segments {
                    self.inner.write_str(";")?;
                    self.inner.write_str(segment)?;
                }
                self.inner.write_str(">")?;
            }
        }
        self.current = style;

        write!(self.inner, "{}", escape(s, AskamaHtml))?;

        Ok(())
    }
}
