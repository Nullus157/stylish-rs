//! Traits and associated types for writing [`stylish`] attributed data to
//! fallible IO sinks with ANSI escape codes.

use stylish_core::{
    io::{Result, Write},
    Style,
};

use crate::util;

/// An adaptor to allow writing [`stylish`] attributed data to an output stream
/// by turning attributes into inline ANSI escape codes.
///
/// ```rust
/// let mut writer = stylish::io::Ansi::new(Vec::new());
/// stylish::write!(writer, "Hello {:(fg=red)}", "Ferris")?;
/// assert_eq!(writer.finish()?, b"Hello \x1b[31mFerris\x1b[0m");
/// # Ok::<(), std::io::Error>(())
/// ```
#[derive(Clone, Debug, Default)]
pub struct Ansi<T: std::io::Write> {
    inner: T,
    current: Style,
}

impl<T: std::io::Write> Ansi<T> {
    /// Wrap the given output stream in this adaptor.
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            current: Style::default(),
        }
    }

    /// Inherent delegation to
    /// [`stylish::io::Write::write_fmt`](stylish_core::io::Write::write_fmt) to
    /// not require a trait import.
    pub fn write_fmt(&mut self, args: stylish_core::Arguments<'_>) -> Result<()> {
        stylish_core::io::Write::write_fmt(self, args)
    }

    /// Ensure the output stream is reset back to the default style and return
    /// it, if you don't call this the stream will be left in whatever style
    /// the last output data was.
    pub fn finish(mut self) -> std::io::Result<T> {
        if self.current != Style::default() {
            self.inner.write_all(b"\x1b[0m")?;
        }
        Ok(self.inner)
    }
}

/// Does not guarantee a single write, if the style changes will repeatedly
/// write to the underlying stream until the change is completed.
impl<T: std::io::Write> Write for Ansi<T> {
    fn write(&mut self, s: &[u8], style: Style) -> Result<usize> {
        if self.current != style && style == Style::default() {
            self.inner.write_all(b"\x1b[0m")?;
        } else {
            let diff = style.diff_from(self.current);
            let segments = [
                diff.foreground.map(util::foreground),
                diff.background.map(util::background),
                diff.intensity.map(util::intensity),
            ];
            let mut segments = segments.iter().filter_map(|&s| s);
            if let Some(segment) = segments.next() {
                self.inner.write_all(b"\x1b[")?;
                self.inner.write_all(segment.as_bytes())?;
                for segment in segments {
                    self.inner.write_all(b";")?;
                    self.inner.write_all(segment.as_bytes())?;
                }
                self.inner.write_all(b"m")?;
            }
        }
        self.current = style;

        self.inner.write(s)
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}
