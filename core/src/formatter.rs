use crate::{arguments::Argument, Arguments, Display, Restyle, Style, Write};

#[derive(Clone, Copy)]
pub enum Align {
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy)]
pub enum Sign {
    Plus,
    Minus,
}

#[derive(Clone, Copy)]
pub enum DebugHex {
    Lower,
    Upper,
}

#[derive(Clone, Copy)]
pub struct FormatterArgs<'a> {
    pub align: Option<Align>,
    pub sign: Option<Sign>,
    pub alternate: bool,
    pub zero: bool,
    pub width: Option<&'a usize>,
    pub precision: Option<&'a usize>,
    pub debug_hex: Option<DebugHex>,
}

impl Default for FormatterArgs<'static> {
    fn default() -> Self {
        Self {
            align: None,
            sign: None,
            zero: false,
            width: None,
            precision: None,
            alternate: false,
            debug_hex: None,
        }
    }
}

pub struct Formatter<'a> {
    style: Style,
    pub(crate) format: FormatterArgs<'a>,
    write: &'a mut (dyn Write + 'a),
}

impl<'a> Formatter<'a> {
    pub(crate) fn new(write: &'a mut (dyn Write + 'a)) -> Self {
        Self {
            style: Style::default(),
            format: FormatterArgs::default(),
            write,
        }
    }

    pub fn with(&mut self, restyle: impl Restyle) -> Formatter<'_> {
        Formatter {
            write: &mut *self.write,
            format: self.format,
            style: self.style.with(restyle),
        }
    }

    fn with_args<'b>(&'b mut self, format: &FormatterArgs<'b>) -> Formatter<'b> {
        Formatter {
            write: &mut *self.write,
            format: *format,
            style: self.style,
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

    pub fn write_fmt(&mut self, args: Arguments<'_>) -> std::fmt::Result {
        for piece in args.pieces {
            match piece {
                Argument::Lit(lit) => self.write_str(lit)?,
                Argument::Arg { args, restyle, arg } => {
                    arg.fmt(&mut self.with(restyle).with_args(args))?
                }
            }
        }
        Ok(())
    }
}

impl<'a> Write for Formatter<'a> {
    fn write_str(&mut self, s: &str, style: Style) -> std::fmt::Result {
        self.with(style).write_str(s)
    }

    fn write_fmt(&mut self, args: Arguments<'_>) -> std::fmt::Result {
        self.write_fmt(args)
    }
}

impl<'a> std::fmt::Write for Formatter<'a> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.write_str(s)
    }
}
