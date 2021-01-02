use stylish::{style, Display};

pub enum Argument<'a> {
    Lit(&'a str),
    Display(&'a dyn Display),
    StdDisplay(&'a dyn std::fmt::Display),
    Debug(bool, &'a dyn std::fmt::Debug),
    With {
        restyle: &'a dyn style::Apply,
        arguments: Arguments<'a>,
    },
}

pub struct Arguments<'a> {
    pub pieces: &'a [Argument<'a>],
}
