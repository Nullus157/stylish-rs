use stylish::{style, Argument, Arguments, Style};

macro_rules! std_write {
    (@trait_str Display) => { "" };
    (@trait_str Debug) => { "?" };
    (@trait_str Octal) => { "o" };
    (@trait_str LowerHex) => { "x" };
    (@trait_str UpperHex) => { "X" };
    (@trait_str Pointer) => { "p" };
    (@trait_str Binary) => { "b" };
    (@trait_str LowerExp) => { "e" };
    (@trait_str UpperExp) => { "E" };

    ($self:ident, $trait:ident, $val:ident) => {{
        use std::fmt::Write;
        match $self.format {
            FormatterArgs {
                align: None,
                sign: None,
                zero: false,
                width: None,
                precision: None,
                alternate: false,
                restyle: _,
            } => std::write!(StdProxy($self), concat!("{:", std_write!(@trait_str $trait), "}"), $val),
            FormatterArgs {
                align: None,
                sign: None,
                zero: false,
                width: None,
                precision: None,
                alternate: true,
                restyle: _,
            } => std::write!(StdProxy($self), concat!("{:#", std_write!(@trait_str $trait), "}"), $val),
            FormatterArgs {
                align: Some((' ', Align::Left)),
                sign: None,
                zero: false,
                width: None,
                precision: None,
                alternate: false,
                restyle: _,
            } => std::write!(StdProxy($self), concat!("{:<", std_write!(@trait_str $trait), "}"), $val),
            FormatterArgs {
                align: Some((' ', Align::Left)),
                sign: None,
                zero: false,
                width: None,
                precision: None,
                alternate: true,
                restyle: _,
            } => std::write!(StdProxy($self), concat!("{:<#", std_write!(@trait_str $trait), "}"), $val),
            FormatterArgs { ..  } => todo!(),
        }
    }};
}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub enum Align {
    Left,
    Center,
    Right,
}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub enum Sign {
    Plus,
    Minus,
}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct FormatterArgs<'a> {
    pub align: Option<(char, Align)>,
    pub sign: Option<Sign>,
    pub zero: bool,
    pub width: Option<&'a usize>,
    pub precision: Option<&'a usize>,
    pub alternate: bool,
    pub restyle: &'a dyn style::Apply,
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
                Argument::DebugLowerHex(format, val) => val.fmt(&mut self.with(format))?,
                Argument::DebugUpperHex(format, val) => val.fmt(&mut self.with(format))?,
                Argument::Octal(format, val) => val.fmt(&mut self.with(format))?,
                Argument::LowerHex(format, val) => val.fmt(&mut self.with(format))?,
                Argument::UpperHex(format, val) => val.fmt(&mut self.with(format))?,
                Argument::Pointer(format, val) => val.fmt(&mut self.with(format))?,
                Argument::Binary(format, val) => val.fmt(&mut self.with(format))?,
                Argument::LowerExp(format, val) => val.fmt(&mut self.with(format))?,
                Argument::UpperExp(format, val) => val.fmt(&mut self.with(format))?,
                Argument::StdDisplay(val) => std_write!(self, Display, val)?,
                Argument::StdDebug(val) => std_write!(self, Debug, val)?,
                Argument::StdOctal(val) => std_write!(self, Octal, val)?,
                Argument::StdLowerHex(val) => std_write!(self, LowerHex, val)?,
                Argument::StdUpperHex(val) => std_write!(self, UpperHex, val)?,
                Argument::StdPointer(val) => std_write!(self, Pointer, val)?,
                Argument::StdBinary(val) => std_write!(self, Binary, val)?,
                Argument::StdLowerExp(val) => std_write!(self, LowerExp, val)?,
                Argument::StdUpperExp(val) => std_write!(self, UpperExp, val)?,
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
