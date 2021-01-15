use crate::{Arguments, Formatter, Result, Style};

pub trait Write {
    fn write_str(&mut self, s: &str, style: Style) -> Result;

    fn write_fmt(mut self: &mut Self, args: &Arguments<'_>) -> Result {
        Formatter::new(&mut self).write_fmt(args)
    }
}

impl<W: Write + ?Sized> Write for &mut W {
    fn write_str(&mut self, s: &str, style: Style) -> Result {
        (&mut **self).write_str(s, style)?;
        Ok(())
    }

    fn write_fmt(&mut self, args: &Arguments<'_>) -> Result {
        (&mut **self).write_fmt(args)?;
        Ok(())
    }
}

impl<P: core::ops::DerefMut<Target: Write + ?Sized>> Write for P {
    default fn write_str(&mut self, s: &str, style: Style) -> Result {
        (&mut **self).write_str(s, style)
    }

    default fn write_fmt(&mut self, args: &Arguments<'_>) -> Result {
        (&mut **self).write_fmt(args)
    }
}
