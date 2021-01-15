use crate::{arguments::Argument, style::Apply, Arguments, Style, Write};

macro_rules! std_write {
    (@str $self:ident $val:ident [] { $({ $($field:ident : $pat:pat,)* } => (($($s:literal,)*), $(($($arg:tt)*),)*);)* }) => {{
        use std::fmt::Write;
        match $self.format {
            $(
                FormatterArgs { $($field : $pat,)* } => {
                    write!(StdProxy($self), concat!("{:", $($s,)* "}"), *$val, $($($arg)*,)*)
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
    pub restyle: &'a dyn Apply,
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

    pub fn with(&mut self, restyle: impl Apply) -> Formatter<'_> {
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

    pub(crate) fn write_std_display(&mut self, val: &impl std::fmt::Display) -> std::fmt::Result {
        std_write!(self, Display, val)
    }

    pub(crate) fn write_std_debug(&mut self, val: &impl std::fmt::Debug) -> std::fmt::Result {
        std_write!(self, Debug, val)
    }

    pub(crate) fn write_std_octal(&mut self, val: &impl std::fmt::Octal) -> std::fmt::Result {
        std_write!(self, Octal, val)
    }

    pub(crate) fn write_std_lower_hex(
        &mut self,
        val: &impl std::fmt::LowerHex,
    ) -> std::fmt::Result {
        std_write!(self, LowerHex, val)
    }

    pub(crate) fn write_std_upper_hex(
        &mut self,
        val: &impl std::fmt::UpperHex,
    ) -> std::fmt::Result {
        std_write!(self, UpperHex, val)
    }

    pub(crate) fn write_std_pointer(&mut self, val: &impl std::fmt::Pointer) -> std::fmt::Result {
        std_write!(self, Pointer, val)
    }

    pub(crate) fn write_std_binary(&mut self, val: &impl std::fmt::Binary) -> std::fmt::Result {
        std_write!(self, Binary, val)
    }

    pub(crate) fn write_std_lower_exp(
        &mut self,
        val: &impl std::fmt::LowerExp,
    ) -> std::fmt::Result {
        std_write!(self, LowerExp, val)
    }

    pub(crate) fn write_std_upper_exp(
        &mut self,
        val: &impl std::fmt::UpperExp,
    ) -> std::fmt::Result {
        std_write!(self, UpperExp, val)
    }

    pub fn write_fmt(&mut self, args: Arguments<'_>) -> std::fmt::Result {
        for piece in args.pieces {
            match piece {
                Argument::Lit(lit) => self.write_str(lit)?,
                Argument::Arg(format, fmt) => fmt(&mut self.with_args(format))?,
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

struct StdProxy<'a, 'b>(&'a mut Formatter<'b>);

impl<'a, 'b> std::fmt::Write for StdProxy<'a, 'b> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.write.write_str(s, self.0.style)?;
        Ok(())
    }
}
