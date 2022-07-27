use core::fmt;

use stylish_core::{Style, Write};

use crate::util;

/// An adaptor to allow writing [`stylish`] attributed data to an output stream
/// by turning attributes into inline ANSI escape codes.
///
/// ```rust
/// let mut writer = stylish::Ansi::new(String::new());
/// stylish::write!(writer, "Hello {:(fg=red)}", "Ferris");
/// assert_eq!(writer.finish()?, "Hello \x1b[31mFerris\x1b[0m");
/// # Ok::<(), core::fmt::Error>(())
/// ```
#[derive(Clone, Debug, Default)]
pub struct Ansi<T: core::fmt::Write> {
    inner: T,
    current: Style,
}

impl<T: core::fmt::Write> Ansi<T> {
    /// Wrap the given output stream in this adaptor.
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            current: Style::default(),
        }
    }

    /// Inherent delegation to
    /// [`stylish::Write::write_fmt`](stylish_core::Write::write_fmt) to not
    /// require a trait import.
    pub fn write_fmt(&mut self, args: stylish_core::Arguments<'_>) -> fmt::Result {
        stylish_core::Write::write_fmt(self, args)
    }

    /// Ensure the output stream is reset back to the default style and return
    /// it, if you don't call this the stream will be left in whatever style
    /// the last output data was.
    pub fn finish(mut self) -> Result<T, fmt::Error> {
        if self.current != Style::default() {
            self.inner.write_str("\x1b[0m")?;
        }
        Ok(self.inner)
    }
}

impl<T: fmt::Write> Write for Ansi<T> {
    fn write_str(&mut self, s: &str, style: Style) -> fmt::Result {
        if self.current != style && style == Style::default() {
            self.inner.write_str("\x1b[0m")?;
        } else {
            let diff = style.diff_from(self.current);
            let segments = [
                diff.foreground.map(util::foreground),
                diff.background.map(util::background),
                diff.intensity.map(util::intensity),
            ];
            let mut segments = segments.iter().filter_map(|&s| s);
            if let Some(segment) = segments.next() {
                self.inner.write_str("\x1b[")?;
                self.inner.write_str(segment)?;
                for segment in segments {
                    self.inner.write_str(";")?;
                    self.inner.write_str(segment)?;
                }
                self.inner.write_str("m")?;
            }
        }
        self.current = style;

        write!(self.inner, "{}", s)?;
        Ok(())
    }
}
