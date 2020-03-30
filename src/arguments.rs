use stylish::{Display, style};

pub enum Argument<'a> {
    Lit(&'a str),
    Val(&'a dyn Display),
    With {
        restyle: &'a dyn style::Apply,
        arguments: Arguments<'a>,
    }
}

pub struct Arguments<'a> {
    pub pieces: &'a [Argument<'a>]
}
