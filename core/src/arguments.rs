use crate::{formatter::FormatterArgs, Formatter, Restyle};

pub enum Argument<'a> {
    Lit(&'a str),

    Arg {
        args: FormatterArgs<'a>,
        restyle: &'a dyn Restyle,
        fmt: stack_dst::ValueA<dyn Fn(&mut Formatter<'_>) -> std::fmt::Result + 'a, [usize; 3]>,
    },
}

pub struct Arguments<'a> {
    #[doc(hidden)]
    pub pieces: &'a [Argument<'a>],
}

pub fn arg<'a>(
    args: FormatterArgs<'a>,
    restyle: &'a dyn Restyle,
    fmt: impl Fn(&mut Formatter<'_>) -> std::fmt::Result + 'a,
) -> Argument<'a> {
    let fmt = stack_dst::ValueA::new_stable(fmt, |p| p as _)
        .map_err(|_| ())
        .unwrap();
    Argument::Arg { args, restyle, fmt }
}
