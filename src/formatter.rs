use stylish::{Style, Write, style, Argument, Arguments};

pub struct Formatter<'a> {
    style: Style,
    write: &'a mut (dyn Write<Error = std::fmt::Error> + 'a),
}

impl<'a> Formatter<'a> {
    pub fn new(write: &'a mut (dyn Write<Error = std::fmt::Error> + 'a)) -> Self {
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

    pub fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.write.write_str(s, self.style)?;
        Ok(())
    }

    pub fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.write_str(c.encode_utf8(&mut [0; 4]))?;
        Ok(())
    }

    pub fn write_fmt(self: &mut Self, args: &Arguments<'_>) -> std::fmt::Result {
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

impl<'a> std::fmt::Write for Formatter<'a> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.write.write_str(s, self.style)?;
        Ok(())
    }
}
