//! Traits and associated types for writing [`stylish`] attributed data to
//! fallible IO sinks.

use crate::{Arguments, Style};

pub use std::io::{Error, ErrorKind, Result};

struct ErrorTrap<W: Write> {
    inner: W,
    error: Option<Error>,
}

impl<W: Write> ErrorTrap<W> {
    fn new(inner: W) -> Self {
        Self { inner, error: None }
    }

    fn error(&mut self) -> Error {
        self.error
            .take()
            .unwrap_or_else(|| Error::new(ErrorKind::Other, "formatter error"))
    }
}

impl<W: Write> crate::Write for ErrorTrap<W> {
    fn write_str(&mut self, s: &str, style: Style) -> crate::Result {
        match self.inner.write_all(s.as_bytes(), style) {
            Ok(()) => Ok(()),
            Err(err) => {
                self.error = Some(err);
                Err(crate::Error)
            }
        }
    }
}

/// A trait for objects which are byte-oriented sinks and can handle attributed
/// data.
///
/// Writers are defined by two required methods, [`write`] and [`flush`]:
///
/// * The [`write`] method will attempt to write some data into the object,
///   returning how many bytes were successfully written.
/// * The [`flush`] method is useful for adaptors and explicit buffers
///   themselves for ensuring that all buffered data has been pushed out to the
///   'true sink'.
///
/// [`write`]: Write::write
/// [`flush`]: Write::flush
///
/// # Examples
///
/// ```rust
/// use stylish::{io::Write, Foreground, Color, Style};
///
/// let data = b"some bytes";
/// let style = Style::default().with(Foreground(Color::Blue));
///     
/// let mut pos = 0;
/// let mut output = stylish::io::plain(std::io::stdout());
///
/// while pos < data.len() {
///     let bytes_written = output.write(&data[pos..], style)?;
///     pos += bytes_written;
/// }
///
/// output.flush()?;
/// # Ok::<(), std::io::Error>(())
pub trait Write {
    /// Write a buffer into this writer with a specified style, returning how
    /// many bytes were written.
    ///
    /// This function will attempt to write the entire contents of `buf`, but
    /// the entire write may not succeed, or the write may also generate an
    /// error. A call to `write` represents at most one attempt to write to
    /// any wrapped object, plus whatever is needed to change the style.
    ///
    /// Calls to `write` are not guaranteed to block waiting for data to be
    /// written, and a write which would otherwise block can be indicated
    /// through an [`Err`] variant.
    ///
    /// If the return value is `Ok(n)` then it must be guaranteed that `n <=
    /// buf.len()`. A return value of `0` typically means that the
    /// underlying object is no longer able to accept bytes and will likely
    /// not be able to in the future as well, or that the buffer provided is
    /// empty.
    ///
    /// # Errors
    ///
    /// Each call to `write` may generate an I/O error indicating that the
    /// operation could not be completed. If an error is returned then no
    /// bytes in the buffer were written to this writer, but the writers
    /// state may have changed to match the new style.
    ///
    /// It is not considered an error if the entire buffer could not be written
    /// to this writer.
    ///
    /// An error of the [`ErrorKind::Interrupted`] kind is non-fatal and the
    /// write operation should be retried if there is nothing else to do.
    ///
    /// ```rust
    /// use stylish::{io::Write, Color, Foreground, Style};
    ///
    /// let mut output = stylish::io::plain(std::io::stdout());
    ///
    /// // Writes some prefix of the byte string, not necessarily all of it.
    /// output.write(
    ///     b"some bytes",
    ///     Style::default().with(Foreground(Color::Blue)),
    /// )?;
    ///
    /// output.flush()?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    fn write(&mut self, buf: &[u8], style: Style) -> Result<usize>;

    /// Flush this output stream, ensuring that all intermediately buffered
    /// contents reach their destination.
    ///
    /// # Errors
    ///
    /// It is considered an error if not all bytes could be written due to I/O
    /// errors or EOF being reached.
    ///
    /// ```rust
    /// use stylish::{io::Write, Color, Foreground, Style};
    ///
    /// let mut output = stylish::io::plain(std::io::stdout());
    ///
    /// output.write_all(
    ///     b"some bytes",
    ///     Style::default().with(Foreground(Color::Blue)),
    /// )?;
    ///
    /// output.flush()?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    fn flush(&mut self) -> Result<()>;

    /// Attempts to write an entire buffer into this writer using a specified
    /// style.
    ///
    /// This method will continuously call [`write`] until there is no more data
    /// to be written or an error of non-[`ErrorKind::Interrupted`] kind is
    /// returned. This method will not return until the entire buffer has
    /// been successfully written or such an error occurs.  The first
    /// error that is not of [`ErrorKind::Interrupted`] kind generated from this
    /// method will be returned.
    ///
    /// If the buffer contains no data, this will never call [`write`].
    ///
    /// # Errors
    ///
    /// This function will return the first error of
    /// non-[`ErrorKind::Interrupted`] kind that [`write`] returns.
    ///
    /// [`write`]: Write::write
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stylish::{io::Write, Color, Foreground, Style};
    ///
    /// let mut output = stylish::io::plain(std::io::stdout());
    ///
    /// output.write_all(
    ///     b"some bytes",
    ///     Style::default().with(Foreground(Color::Blue)),
    /// )?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    fn write_all(&mut self, mut buf: &[u8], style: Style) -> Result<()> {
        while !buf.is_empty() {
            match self.write(buf, style) {
                Ok(0) => {
                    return Err(Error::new(
                        ErrorKind::WriteZero,
                        "failed to write whole buffer",
                    ));
                }
                Ok(n) => buf = &buf[n..],
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    /// Writes a formatted string into this writer, returning any error
    /// encountered.
    ///
    /// This method is primarily used to interface with the
    /// [`stylish::format_args!`] macro, but it is rare that this should
    /// explicitly be called. The [`stylish::write!`] macro should be
    /// favored to invoke this method instead.
    ///
    /// This function internally uses the [`write_all`](Write::write_all) method
    /// on this trait and hence will continuously write data so long as no
    /// errors are received. This also means that partial writes are not
    /// indicated in this signature.
    ///
    /// # Errors
    ///
    /// This function will return any I/O error reported while formatting.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stylish::{io::Write, Color, Foreground, Style};
    ///
    /// let mut output = stylish::io::plain(std::io::stdout());
    ///
    /// output.write_fmt(stylish::format_args!("{:(fg=red)}", '☎'))?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    fn write_fmt(&mut self, args: Arguments<'_>) -> Result<()> {
        let mut trap = ErrorTrap::new(self);

        crate::Write::write_fmt(&mut trap, args).map_err(|crate::Error| trap.error())
    }

    /// Creates a "by reference" adaptor for this instance of `Write`.
    ///
    /// The returned adaptor also implements `Write` and will simply borrow this
    /// current writer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stylish::{io::Write, Color, Foreground, Style};
    ///
    /// let mut output = stylish::io::plain(std::io::stdout());
    ///
    /// let reference = output.by_ref();
    ///
    /// // we can use reference just like our original output
    /// reference.write_all(
    ///     b"some bytes",
    ///     Style::default().with(Foreground(Color::Blue)),
    /// )?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }
}

impl<W: Write + ?Sized> Write for &mut W {
    fn write(&mut self, s: &[u8], style: Style) -> Result<usize> {
        (&mut **self).write(s, style)
    }

    fn flush(&mut self) -> Result<()> {
        (&mut **self).flush()
    }

    fn write_all(&mut self, s: &[u8], style: Style) -> Result<()> {
        (&mut **self).write_all(s, style)
    }

    fn write_fmt(&mut self, args: Arguments<'_>) -> Result<()> {
        (&mut **self).write_fmt(args)
    }
}
