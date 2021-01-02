use stylish::{style, Argument, Arguments, Style};

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct FormatterArgs<'a> {
    pub alternate: bool,
    pub restyle: &'a dyn style::Apply,
}

impl Default for FormatterArgs<'static> {
    fn default() -> Self {
        Self {
            alternate: false,
            restyle: &(),
        }
    }
}

pub struct Formatter<'a> {
    style: Style,
    format: FormatterArgs<'a>,
    write: &'a mut (dyn stylish::fmt::Write + 'a),
}

impl<'a> Formatter<'a> {
    #[doc(hidden)]
    pub fn new(write: &'a mut (dyn stylish::fmt::Write + 'a)) -> Self {
        Self {
            style: Style::default(),
            format: FormatterArgs::default(),
            write,
        }
    }

    fn with<'b>(&'b mut self, format: &FormatterArgs<'b>) -> Formatter<'b> {
        Formatter {
            write: &mut *self.write,
            format: *format,
            style: self.style.with(format.restyle),
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
                Argument::Display(format, val) => val.fmt(&mut self.with(format))?,
                Argument::Debug(format, val) => val.fmt(&mut self.with(format))?,
                Argument::StdDisplay(val) => {
                    use std::fmt::Write;
                    match self.format {
                        FormatterArgs {
                            alternate: false,
                            restyle: _,
                        } => std::write!(StdProxy(self), "{}", val)?,
                        FormatterArgs {
                            alternate: true,
                            restyle: _,
                        } => std::write!(StdProxy(self), "{:#}", val)?,
                    }
                }
                Argument::StdDebug(val) => {
                    use std::fmt::Write;
                    match self.format {
                        FormatterArgs {
                            alternate: false,
                            restyle: _,
                        } => std::write!(StdProxy(self), "{:?}", val)?,
                        FormatterArgs {
                            alternate: true,
                            restyle: _,
                        } => std::write!(StdProxy(self), "{:#?}", val)?,
                    }
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
