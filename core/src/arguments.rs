use crate::{formatter::FormatterArgs, Display, Formatter, Result, StyleDiff};

type StdFmtFn<'a> = dyn Fn(&mut core::fmt::Formatter<'_>) -> Result + 'a;

#[doc(hidden)] // workaround https://github.com/rust-lang/rust/issues/85522
pub struct StdFmt<'a>(stack_dst::ValueA<StdFmtFn<'a>, [usize; 3]>);

impl<'a> StdFmt<'a> {
    #[doc(hidden)] // workaround https://github.com/rust-lang/rust/issues/85526
    pub fn new(f: impl Fn(&mut core::fmt::Formatter<'_>) -> Result + 'a) -> StdFmt<'a> {
        // not possible(/easy) to correctly type the closure, but with the cast
        // inference works
        #[allow(trivial_casts)]
        StdFmt(
            stack_dst::ValueA::new_stable(f, |p| p as _)
                .map_err(|_| ())
                .expect("StdFmt was more than 3 words, this is a bug in stylish-core"),
        )
    }
}

impl core::fmt::Display for StdFmt<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result {
        (self.0)(f)
    }
}

impl core::fmt::Debug for StdFmt<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result {
        (self.0)(f)
    }
}

#[doc(hidden)] // workaround https://github.com/rust-lang/rust/issues/85522
#[allow(missing_debug_implementations)]
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

#[doc(hidden)] // workaround https://github.com/rust-lang/rust/issues/85522
#[allow(missing_debug_implementations)]
pub enum Argument<'a> {
    Lit(&'a str),

    Arg {
        args: &'a FormatterArgs<'a>,
        style: StyleDiff,
        arg: FormatTrait<'a>,
    },
}

/// A precompiled version of a format string and its by-reference arguments.
///
/// Currently this can only be constructed via [`stylish::format_args!`], but it
/// may be possible to dynamically construct this at runtime in the future.
///
/// ```rust
/// let args = stylish::format_args!("{:(bg=red)} Will Robinson", "Danger");
/// assert_eq!(
///     stylish::html::format!("{:s}", args),
///     "<span style=background-color:red>Danger</span> Will Robinson",
/// );
/// ```
#[allow(missing_debug_implementations)]
pub struct Arguments<'a> {
    #[doc(hidden)] // pub for macros
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

impl Display for Arguments<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for piece in self.pieces {
            match piece {
                Argument::Lit(lit) => f.write_str(lit)?,
                Argument::Arg { args, style, arg } => {
                    arg.fmt(&mut f.with(style).with_args(args))?
                }
            }
        }
        Ok(())
    }
}
