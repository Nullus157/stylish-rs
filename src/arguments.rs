use stylish::{Debug, Display, FormatterArgs};

pub enum Argument<'a> {
    #[doc(hidden)]
    Lit(&'a str),
    #[doc(hidden)]
    Display(FormatterArgs<'a>, &'a dyn Display),
    #[doc(hidden)]
    Debug(FormatterArgs<'a>, &'a dyn Debug),
    #[doc(hidden)]
    StdDisplay(&'a dyn std::fmt::Display),
    #[doc(hidden)]
    StdDebug(&'a dyn std::fmt::Debug),
}

pub struct Arguments<'a> {
    pub pieces: &'a [Argument<'a>],
}
