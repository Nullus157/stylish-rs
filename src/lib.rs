#![no_std]

pub use stylish_core::{
    format_args, write, writeln, Arguments, Color, Display, Formatter, Intensity, Style, Write,
};

#[cfg(feature = "std")]
pub use stylish_core::io;

pub fn ansi<T: core::fmt::Write>(inner: T) -> ansi::Ansi<T> {
    ansi::Ansi::new(inner)
}

pub fn html<T: core::fmt::Write>(inner: T) -> html::Html<T> {
    html::Html::new(inner)
}

pub fn plain<T: core::fmt::Write>(inner: T) -> plain::Plain<T> {
    plain::Plain::new(inner)
}

#[doc(inline)]
pub use stylish_ansi as ansi;
#[doc(inline)]
pub use stylish_html as html;
#[doc(inline)]
pub use stylish_plain as plain;
