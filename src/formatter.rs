use stylish::{style, Argument, Arguments, Style};

// macro_rules! write {
//     ($($t:tt)*) => { write($($t)*) }
// }

macro_rules! std_write {
    (@str $self:ident $val:ident [] { $({ $($field:ident : $pat:pat,)* } => (($($s:literal,)*), $(($($arg:tt)*),)*);)* }) => {{
        use std::fmt::Write;
        match $self.format {
            $(
                FormatterArgs { $($field : $pat,)* } => {
                    write!(StdProxy($self), concat!("{:", $($s,)* "}"), $val, $($($arg)*,)*)
                }
            )*
        }
    }};

    (@str $self:ident $val:ident [restyle $($flag:ident)*] { $({ $($field:ident : $pat:pat,)* } => (($($s:literal,)*), $(($($arg:tt)*),)*);)* }) => {
        std_write!(@str $self $val [$($flag)*] {
            $({ restyle: _, $($field : $pat,)* } => (($($s,)*), $(($($arg)*),)*);)*
        })
    };

    (@str $self:ident $val:ident [precision $($flag:ident)*] { $({ $($field:ident : $pat:pat,)* } => (($($s:literal,)*), $(($($arg:tt)*),)*);)* }) => {
        std_write!(@str $self $val [$($flag)*] {
            $({ precision: None, $($field : $pat,)* } => (($($s,)*), $(($($arg)*),)*);)*
            $({ precision: Some(precision), $($field : $pat,)* } => ((".precision$", $($s,)*), (precision=precision), $(($($arg)*),)*);)*
        })
    };

    (@str $self:ident $val:ident [width $($flag:ident)*] { $({ $($field:ident : $pat:pat,)* } => (($($s:literal,)*), $(($($arg:tt)*),)*);)* }) => {
        std_write!(@str $self $val [$($flag)*] {
            $({ width: None, $($field : $pat,)* } => (($($s,)*), $(($($arg)*),)*);)*
            $({ width: Some(width), $($field : $pat,)* } => (("width$", $($s,)*), (width=width), $(($($arg)*),)*);)*
        })
    };

    (@str $self:ident $val:ident [zero $($flag:ident)*] { $({ $($field:ident : $pat:pat,)* } => (($($s:literal,)*), $(($($arg:tt)*),)*);)* }) => {
        std_write!(@str $self $val [$($flag)*] {
            $({ zero: false, $($field : $pat,)* } => (($($s,)*), $(($($arg)*),)*);)*
            $({ zero: true, $($field : $pat,)* } => (("0", $($s,)*), $(($($arg)*),)*);)*
        })
    };

    (@str $self:ident $val:ident [alternate $($flag:ident)*] { $({ $($field:ident : $pat:pat,)* } => (($($s:literal,)*), $(($($arg:tt)*),)*);)* }) => {
        std_write!(@str $self $val [$($flag)*] {
            $({ alternate: false, $($field : $pat,)* } => (($($s,)*), $(($($arg)*),)*);)*
            $({ alternate: true, $($field : $pat,)* } => (("#", $($s,)*), $(($($arg)*),)*);)*
        })
    };

    (@str $self:ident $val:ident [sign $($flag:ident)*] { $({ $($field:ident : $pat:pat,)* } => (($($s:literal,)*), $(($($arg:tt)*),)*);)* }) => {
        std_write!(@str $self $val [$($flag)*] {
            $({ sign: None, $($field : $pat,)* } => (($($s,)*), $(($($arg)*),)*);)*
            $({ sign: Some(Sign::Plus), $($field : $pat,)* } => (("+", $($s,)*), $(($($arg)*),)*);)*
            $({ sign: Some(Sign::Minus), $($field : $pat,)* } => (("-", $($s,)*), $(($($arg)*),)*);)*
        })
    };

    (@str $self:ident $val:ident [align $($flag:ident)*] { $({ $($field:ident : $pat:pat,)* } => (($($s:literal,)*), $(($($arg:tt)*),)*);)* }) => {
        std_write!(@str $self $val [$($flag)*] {
            $({ align: None, $($field : $pat,)* } => (($($s,)*), $(($($arg)*),)*);)*
            $({ align: Some(Align::Left), $($field : $pat,)* } => (("<", $($s,)*), $(($($arg)*),)*);)*
            $({ align: Some(Align::Center), $($field : $pat,)* } => (("^", $($s,)*), $(($($arg)*),)*);)*
            $({ align: Some(Align::Right), $($field : $pat,)* } => ((">", $($s,)*), $(($($arg)*),)*);)*
        })
    };

    (@str $self:ident $val:ident [Display $($flag:ident)*]) => {
        std_write!(@str $self $val [$($flag)*] { { debug_hex: _, } => (("",),); })
    };
    (@str $self:ident $val:ident [Debug $($flag:ident)*]) => {
        std_write!(@str $self $val [$($flag)*] {
            { debug_hex: None, } => (("?",),);
            { debug_hex: Some(DebugHex::Lower), } => (("x?",),);
            { debug_hex: Some(DebugHex::Upper), } => (("X?",),);
        })
    };
    (@str $self:ident $val:ident [Octal $($flag:ident)*]) => {
        std_write!(@str $self $val [$($flag)*] { { debug_hex: _, } => (("o",),); })
    };
    (@str $self:ident $val:ident [LowerHex $($flag:ident)*]) => {
        std_write!(@str $self $val [$($flag)*] { { debug_hex: _, } => (("x",),); })
    };
    (@str $self:ident $val:ident [UpperHex $($flag:ident)*]) => {
        std_write!(@str $self $val [$($flag)*] { { debug_hex: _, } => (("X",),); })
    };
    (@str $self:ident $val:ident [Pointer $($flag:ident)*]) => {
        std_write!(@str $self $val [$($flag)*] { { debug_hex: _, } => (("p",),); })
    };
    (@str $self:ident $val:ident [Binary $($flag:ident)*]) => {
        std_write!(@str $self $val [$($flag)*] { { debug_hex: _, } => (("b",),); })
    };
    (@str $self:ident $val:ident [LowerExp $($flag:ident)*]) => {
        std_write!(@str $self $val [$($flag)*] { { debug_hex: _, } => (("e",),); })
    };
    (@str $self:ident $val:ident [UpperExp $($flag:ident)*]) => {
        std_write!(@str $self $val [$($flag)*] { { debug_hex: _, } => (("E",),); })
    };

    ($self:ident, $trait:ident, $val:ident) => {
        std_write!(@str $self $val [$trait restyle precision width zero alternate sign align])
    };
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
pub enum DebugHex {
    Lower,
    Upper,
}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct FormatterArgs<'a> {
    pub align: Option<Align>,
    pub sign: Option<Sign>,
    pub alternate: bool,
    pub zero: bool,
    pub width: Option<&'a usize>,
    pub precision: Option<&'a usize>,
    pub debug_hex: Option<DebugHex>,
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
            debug_hex: None,
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
