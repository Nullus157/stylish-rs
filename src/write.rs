use stylish::{Style, Result, Arguments, Formatter};

pub trait Write {
    fn write_str(&mut self, s: &str, style: Style) -> Result;

    fn write_fmt(mut self: &mut Self, args: &Arguments<'_>) -> Result {
        Formatter::new(&mut self).write_fmt(args)?;
        Ok(())
    }
}

impl<W: Write + ?Sized> Write for &mut W {
    fn write_str(&mut self, s: &str, style: Style) -> Result {
        (&mut **self).write_str(s, style)?;
        Ok(())
    }

    fn write_fmt(self: &mut Self, args: &Arguments<'_>) -> Result {
        (&mut **self).write_fmt(args)?;
        Ok(())
    }
}

impl<W: Write + ?Sized> Write for Box<W> {
    fn write_str(&mut self, s: &str, style: Style) -> Result {
        (&mut **self).write_str(s, style)?;
        Ok(())
    }
}
