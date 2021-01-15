use crate::{Arguments, Style};

pub use std::io::{Error, ErrorKind};

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

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

pub trait Write {
    fn write(&mut self, s: &[u8], style: Style) -> Result<usize>;

    fn flush(&mut self) -> Result<()>;

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

    fn write_fmt(&mut self, args: Arguments<'_>) -> Result<()> {
        let mut trap = ErrorTrap::new(self);

        crate::Write::write_fmt(&mut trap, args).map_err(|crate::Error| trap.error())
    }

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
