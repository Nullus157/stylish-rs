use crate::{Arguments, Formatter, Result, Style};

pub trait Write {
    fn write_str(&mut self, s: &str, style: Style) -> Result;

    fn write_char(&mut self, c: char, style: Style) -> Result {
        self.write_str(c.encode_utf8(&mut [0; 4]), style)
    }

    fn write_fmt(mut self: &mut Self, args: Arguments<'_>) -> Result {
        Formatter::new(&mut self).write_fmt(args)
    }
}

impl<W: Write + ?Sized> Write for &mut W {
    fn write_str(&mut self, s: &str, style: Style) -> Result {
        (&mut **self).write_str(s, style)
    }

    fn write_char(&mut self, c: char, style: Style) -> Result {
        (&mut **self).write_char(c, style)
    }

    fn write_fmt(&mut self, args: Arguments<'_>) -> Result {
        (&mut **self).write_fmt(args)
    }
}
