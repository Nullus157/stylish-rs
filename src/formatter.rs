use stylish::{Style, Write, Result, style, Argument, Arguments};

pub struct Formatter<'a> {
    style: Style,
    write: &'a mut (dyn Write + 'a),
}

impl<'a> Formatter<'a> {
    pub fn new(write: &'a mut (dyn Write + 'a)) -> Self {
        Self {
            style: Style::default(),
            write,
        }
    }

    pub fn with(&mut self, adj: impl style::Apply) -> Formatter<'_> {
        Formatter {
            write: &mut *self.write,
            style: self.style.with(&adj),
        }
    }

    pub fn write_str(&mut self, s: &str) -> Result {
        self.write.write_str(s, self.style)?;
        Ok(())
    }

    pub fn write_char(&mut self, c: char) -> Result {
        self.write_str(c.encode_utf8(&mut [0; 4]))?;
        Ok(())
    }
}

impl<'a> std::fmt::Write for Formatter<'a> {
    fn write_str(&mut self, s: &str) -> Result {
        self.write.write_str(s, self.style)?;
        Ok(())
    }
}

impl<'a> Write for Formatter<'a> {
    fn write_str(&mut self, s: &str, style: Style) -> Result {
        self.write.write_str(s, style)?;
        Ok(())
    }

    fn write_fmt(self: &mut Self, args: &Arguments<'_>) -> Result {
        for piece in args.pieces {
            match piece {
                Argument::Lit(lit) => self.write_str(lit)?,
                Argument::Val(val) => val.fmt(self)?,
                Argument::With { restyle, arguments } => {
                    self.with(restyle).write_fmt(arguments)?;
                }
            }
        }
        Ok(())
    }
}
