use crate::Style;

pub use std::io::{Error, Result};

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
            .unwrap_or_else(|| Error::new(std::io::ErrorKind::Other, "formatter error"))
    }
}

impl<W: Write> crate::Write for ErrorTrap<W> {
    fn write_str(&mut self, s: &str, style: Style) -> crate::Result {
        match self.inner.write_str(s, style) {
            Ok(()) => Ok(()),
            Err(err) => {
                self.error = Some(err);
                Err(crate::Error)
            }
        }
    }
}

pub trait Write {
    fn write_str(&mut self, s: &str, style: Style) -> Result<()>;

    fn write_fmt(&mut self, args: &crate::Arguments<'_>) -> Result<()> {
        let mut trap = ErrorTrap::new(self);

        crate::Write::write_fmt(&mut trap, args).map_err(|crate::Error| trap.error())
    }
}

impl<W: Write + ?Sized> Write for &mut W {
    fn write_str(&mut self, s: &str, style: Style) -> Result<()> {
        (&mut **self).write_str(s, style)
    }

    fn write_fmt(&mut self, args: &crate::Arguments<'_>) -> Result<()> {
        (&mut **self).write_fmt(args)
    }
}

impl<P: core::ops::DerefMut<Target: Write + ?Sized>> Write for P {
    default fn write_str(&mut self, s: &str, style: Style) -> Result<()> {
        (&mut **self).write_str(s, style)
    }

    default fn write_fmt(&mut self, args: &crate::Arguments<'_>) -> Result<()> {
        (&mut **self).write_fmt(args)
    }
}
