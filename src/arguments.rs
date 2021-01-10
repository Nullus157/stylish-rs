use stylish::{
    Binary, Debug, Display, FormatterArgs, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex,
};

pub enum Argument<'a> {
    #[doc(hidden)]
    Lit(&'a str),

    #[doc(hidden)]
    Display(FormatterArgs<'a>, &'a dyn Display),
    #[doc(hidden)]
    Debug(FormatterArgs<'a>, &'a dyn Debug),
    #[doc(hidden)]
    DebugLowerHex(FormatterArgs<'a>, &'a dyn Debug),
    #[doc(hidden)]
    DebugUpperHex(FormatterArgs<'a>, &'a dyn Debug),
    #[doc(hidden)]
    Octal(FormatterArgs<'a>, &'a dyn Octal),
    #[doc(hidden)]
    LowerHex(FormatterArgs<'a>, &'a dyn LowerHex),
    #[doc(hidden)]
    UpperHex(FormatterArgs<'a>, &'a dyn UpperHex),
    #[doc(hidden)]
    Pointer(FormatterArgs<'a>, &'a dyn Pointer),
    #[doc(hidden)]
    Binary(FormatterArgs<'a>, &'a dyn Binary),
    #[doc(hidden)]
    LowerExp(FormatterArgs<'a>, &'a dyn LowerExp),
    #[doc(hidden)]
    UpperExp(FormatterArgs<'a>, &'a dyn UpperExp),

    #[doc(hidden)]
    StdDisplay(&'a dyn std::fmt::Display),
    #[doc(hidden)]
    StdDebug(&'a dyn std::fmt::Debug),
    #[doc(hidden)]
    StdOctal(&'a dyn std::fmt::Octal),
    #[doc(hidden)]
    StdLowerHex(&'a dyn std::fmt::LowerHex),
    #[doc(hidden)]
    StdUpperHex(&'a dyn std::fmt::UpperHex),
    #[doc(hidden)]
    StdPointer(&'a dyn std::fmt::Pointer),
    #[doc(hidden)]
    StdBinary(&'a dyn std::fmt::Binary),
    #[doc(hidden)]
    StdLowerExp(&'a dyn std::fmt::LowerExp),
    #[doc(hidden)]
    StdUpperExp(&'a dyn std::fmt::UpperExp),
}

pub struct Arguments<'a> {
    pub pieces: &'a [Argument<'a>],
}
