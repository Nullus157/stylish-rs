use stylish::{style, Argument, Arguments, Style};

pub struct Formatter<'a> {
    style: Style,
    write: &'a mut (dyn stylish::Write<Error = std::fmt::Error> + 'a),
}

impl<'a> Formatter<'a> {
    pub fn new(write: &'a mut (dyn stylish::Write<Error = std::fmt::Error> + 'a)) -> Self {
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

    pub fn write_fmt(&mut self, args: &Arguments<'_>) -> std::fmt::Result {
        for piece in args.pieces {
            match piece {
                Argument::Lit(lit) => self.write_str(lit)?,
                Argument::Display(val) => val.fmt(self)?,
                Argument::StdDisplay(val) => {
                    use std::fmt::Write;
                    std::write!(StdProxy(self), "{}", val)?;
                }
                Argument::Debug(alternate, val) => {
                    use std::fmt::Write;

                    if *alternate {
                        std::write!(StdProxy(self), "{:#?}", val)?;
                    } else {
                        std::write!(StdProxy(self), "{:?}", val)?;
                    }
                }
                Argument::With { restyle, arguments } => {
                    self.with(restyle).write_fmt(arguments)?;
                }
            }
        }
        Ok(())
    }
}

struct StdProxy<'a, 'b>(&'a mut Formatter<'b>);

impl<'a, 'b> std::fmt::Write for StdProxy<'a, 'b> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.write.write_str(s, self.0.style)?;
        Ok(())
    }
}
