use crate::{formatter::FormatterArgs, Formatter};

pub enum Argument<'a> {
    Lit(&'a str),

    Arg(
        FormatterArgs<'a>,
        stack_dst::ValueA<dyn Fn(&mut Formatter<'_>) -> std::fmt::Result + 'a, [usize; 3]>,
    ),
}

pub struct Arguments<'a> {
    #[doc(hidden)]
    pub pieces: &'a [Argument<'a>],
}

pub fn arg<'a>(
    args: FormatterArgs<'a>,
    f: impl Fn(&mut Formatter<'_>) -> std::fmt::Result + 'a,
) -> Argument<'a> {
    Argument::Arg(
        args,
        stack_dst::ValueA::new_stable(f, |p| p as _)
            .map_err(|_| ())
            .unwrap(),
    )
}
