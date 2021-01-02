pub mod fmt {
    use stylish::{Arguments, Formatter, Style};

    pub trait Write {
        fn write_str(&mut self, s: &str, style: Style) -> std::fmt::Result;

        fn write_fmt(mut self: &mut Self, args: &Arguments<'_>) -> std::fmt::Result {
            Formatter::new(&mut self).write_fmt(args)
        }
    }

    impl<W: Write + ?Sized> Write for &mut W {
        fn write_str(&mut self, s: &str, style: Style) -> std::fmt::Result {
            (&mut **self).write_str(s, style)?;
            Ok(())
        }

        fn write_fmt(&mut self, args: &Arguments<'_>) -> std::fmt::Result {
            (&mut **self).write_fmt(args)?;
            Ok(())
        }
    }

    impl<P: core::ops::DerefMut<Target: Write + ?Sized>> Write for P {
        default fn write_str(&mut self, s: &str, style: Style) -> std::fmt::Result {
            (&mut **self).write_str(s, style)
        }

        default fn write_fmt(&mut self, args: &Arguments<'_>) -> std::fmt::Result {
            (&mut **self).write_fmt(args)
        }
    }
}

pub mod io {
    use super::fmt;
    use core::cell::RefCell;
    use stylish::{Arguments, Formatter, Style};

    pub trait Write {
        fn write_str(&mut self, s: &str, style: Style) -> std::io::Result<()>;

        fn write_fmt(&mut self, args: &Arguments<'_>) -> std::io::Result<()> {
            struct ErrorTrap<W: Write> {
                inner: W,
                error: RefCell<Option<std::io::Error>>,
            }

            impl<W: Write> fmt::Write for ErrorTrap<W> {
                fn write_str(&mut self, s: &str, style: Style) -> std::fmt::Result {
                    match self.inner.write_str(s, style) {
                        Ok(()) => Ok(()),
                        Err(err) => {
                            *self.error.borrow_mut() = Some(err);
                            Err(std::fmt::Error)
                        }
                    }
                }

                fn write_fmt(&mut self, args: &Arguments<'_>) -> std::fmt::Result {
                    Formatter::new(self).write_fmt(args)?;
                    Ok(())
                }
            }

            let mut trap = ErrorTrap {
                inner: self,
                error: RefCell::new(None),
            };

            Formatter::new(&mut trap)
                .write_fmt(args)
                .or_else(|std::fmt::Error| {
                    trap.error.borrow_mut().take().map_or_else(|| Ok(()), Err)
                })?;

            Ok(())
        }
    }

    impl<W: Write + ?Sized> Write for &mut W {
        fn write_str(&mut self, s: &str, style: Style) -> std::io::Result<()> {
            (&mut **self).write_str(s, style)?;
            Ok(())
        }

        fn write_fmt(&mut self, args: &Arguments<'_>) -> std::io::Result<()> {
            (&mut **self).write_fmt(args)?;
            Ok(())
        }
    }

    impl<P: core::ops::DerefMut<Target: Write + ?Sized>> Write for P {
        default fn write_str(&mut self, s: &str, style: Style) -> std::io::Result<()> {
            (&mut **self).write_str(s, style)
        }

        default fn write_fmt(&mut self, args: &Arguments<'_>) -> std::io::Result<()> {
            (&mut **self).write_fmt(args)
        }
    }
}
