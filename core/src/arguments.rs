use crate::{formatter::FormatterArgs, Display, Formatter, Restyle, Result};

type StdFmtFn<'a> = dyn Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result + 'a;

pub struct StdFmt<'a>(stack_dst::ValueA<StdFmtFn<'a>, [usize; 3]>);

impl<'a> StdFmt<'a> {
    pub fn new(f: impl Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result + 'a) -> StdFmt<'a> {
        StdFmt(
            stack_dst::ValueA::new_stable(f, |p| p as _)
                .map_err(|_| ())
                .unwrap(),
        )
    }
}

impl std::fmt::Display for StdFmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.0)(f)
    }
}

impl std::fmt::Debug for StdFmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.0)(f)
    }
}

pub enum FormatTrait<'a> {
    Display(StdFmt<'a>),
    Debug(StdFmt<'a>),
    Octal(StdFmt<'a>),
    LowerHex(StdFmt<'a>),
    UpperHex(StdFmt<'a>),
    Pointer(StdFmt<'a>),
    Binary(StdFmt<'a>),
    LowerExp(StdFmt<'a>),
    UpperExp(StdFmt<'a>),
    Stylish(&'a dyn Display),
}

pub enum Argument<'a> {
    Lit(&'a str),

    Arg {
        args: FormatterArgs<'a>,
        restyle: &'a dyn Restyle,
        arg: FormatTrait<'a>,
    },
}

pub struct Arguments<'a> {
    #[doc(hidden)]
    pub pieces: &'a [Argument<'a>],
}

impl Display for FormatTrait<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Display(arg) => std_write!(f, Display, arg),
            Self::Debug(arg) => std_write!(f, Debug, arg),
            Self::Octal(arg) => std_write!(f, Display, arg),
            Self::LowerHex(arg) => std_write!(f, Display, arg),
            Self::UpperHex(arg) => std_write!(f, Display, arg),
            Self::Pointer(arg) => std_write!(f, Display, arg),
            Self::Binary(arg) => std_write!(f, Display, arg),
            Self::LowerExp(arg) => std_write!(f, Display, arg),
            Self::UpperExp(arg) => std_write!(f, Display, arg),
            Self::Stylish(arg) => arg.fmt(f),
        }
    }
}
