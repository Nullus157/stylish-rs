use crate::{formatter::FormatterArgs, Display, Formatter, Restyle, Result};

pub enum FormatTrait<'a> {
    Display(&'a dyn std::fmt::Display),
    Debug(&'a dyn std::fmt::Debug),
    Octal(&'a dyn std::fmt::Octal),
    LowerHex(&'a dyn std::fmt::LowerHex),
    UpperHex(&'a dyn std::fmt::UpperHex),
    Pointer(&'a dyn std::fmt::Pointer),
    Binary(&'a dyn std::fmt::Binary),
    LowerExp(&'a dyn std::fmt::LowerExp),
    UpperExp(&'a dyn std::fmt::UpperExp),
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
            Self::Octal(arg) => std_write!(f, Octal, arg),
            Self::LowerHex(arg) => std_write!(f, LowerHex, arg),
            Self::UpperHex(arg) => std_write!(f, UpperHex, arg),
            Self::Pointer(arg) => std_write!(f, Pointer, arg),
            Self::Binary(arg) => std_write!(f, Binary, arg),
            Self::LowerExp(arg) => std_write!(f, LowerExp, arg),
            Self::UpperExp(arg) => std_write!(f, UpperExp, arg),
            Self::Stylish(arg) => arg.fmt(f),
        }
    }
}
