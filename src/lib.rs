#![no_std]
#![feature(extended_key_value_attributes)]
#![doc = stylish_core::__export::docs::krate!()]

#[cfg(feature = "std")]
extern crate std;

#[cfg(doc)]
extern crate self as stylish;

#[cfg(all(doc, not(feature = "std")))]
use core::fmt as doc_fmt;

#[cfg(all(doc, feature = "std"))]
use std::fmt as doc_fmt;

pub use stylish_core::{
    format_args, write, writeln, Arguments, Background, Color, Display, Error, Foreground,
    Formatter, Intensity, Restyle, Result, Style, StyleDiff, ToStylishString, Write,
};

#[cfg(feature = "alloc")]
pub use stylish_core::String;

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
