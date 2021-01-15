pub use stylish_core::{
    format_args, io, write, writeln, Arguments, Color, Display, Formatter, Intensity, Style, Write,
};

pub fn ansi<T: std::fmt::Write>(inner: T) -> ansi::Ansi<T> {
    ansi::Ansi::new(inner)
}

pub fn html<T: std::fmt::Write>(inner: T) -> html::Html<T> {
    html::Html::new(inner)
}

pub fn plain<T: std::fmt::Write>(inner: T) -> plain::Plain<T> {
    plain::Plain::new(inner)
}

#[doc(inline)]
pub use stylish_ansi as ansi;
#[doc(inline)]
pub use stylish_html as html;
#[doc(inline)]
pub use stylish_plain as plain;
