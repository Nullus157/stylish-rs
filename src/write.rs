use core::cell::RefCell;
use stylish::{Style, Arguments, Formatter};

pub trait Write {
    type Error;

    fn write_str(&mut self, s: &str, style: Style) -> Result<(), Self::Error>;

    fn write_fmt(&mut self, args: &Arguments<'_>) -> Result<(), Self::Error> {
        let mut trap = ErrorTrap::new(self);
        Formatter::new(&mut trap).write_fmt(args).or_else(|std::fmt::Error| trap.check())?;
        Ok(())
    }
}

struct ErrorTrap<W: Write> {
    inner: W,
    error: RefCell<Option<W::Error>>,
}

impl<W: Write> ErrorTrap<W> {
    fn new(inner: W) -> Self {
        Self {
            inner,
            error: RefCell::new(None),
        }
    }

    fn check(&mut self) -> Result<(), W::Error> {
        self.error.borrow_mut().take().map_or_else(|| Ok(()), Err)
    }
}

impl<W: Write> Write for ErrorTrap<W> {
    type Error = std::fmt::Error;

    fn write_str(&mut self, s: &str, style: Style) -> Result<(), Self::Error> {
        match self.inner.write_str(s, style) {
            Ok(()) => Ok(()),
            Err(err) => {
                *self.error.borrow_mut() = Some(err);
                Err(std::fmt::Error)
            }
        }
    }
}

impl<W: Write + ?Sized> Write for &mut W {
    type Error = W::Error;

    fn write_str(&mut self, s: &str, style: Style) -> Result<(), Self::Error> {
        (&mut **self).write_str(s, style)?;
        Ok(())
    }

    fn write_fmt(self: &mut Self, args: &Arguments<'_>) -> Result<(), Self::Error> {
        (&mut **self).write_fmt(args)?;
        Ok(())
    }
}

impl<W: Write + ?Sized> Write for Box<W> {
    type Error = W::Error;

    fn write_str(&mut self, s: &str, style: Style) -> Result<(), Self::Error> {
        (&mut **self).write_str(s, style)?;
        Ok(())
    }

    fn write_fmt(self: &mut Self, args: &Arguments<'_>) -> Result<(), Self::Error> {
        (&mut **self).write_fmt(args)?;
        Ok(())
    }
}
